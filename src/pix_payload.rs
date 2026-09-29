/// Pix payload (BR Code) utilities: the string behind a Pix QR Code and
/// behind "Pix copia e cola", per the Banco Central do Brasil / EMV QRCPS
/// merchant-presented QR code manual.
use crate::pix_key;
use unicode_normalization::UnicodeNormalization;

const PIX_GUI: &str = "br.gov.bcb.pix";

/// A single top-level or nested TLV (tag-length-value) field.
struct Tlv {
    id: String,
    value: String,
}

/// Parses a BR Code (or any EMV QRCPS TLV string) into its top-level fields.
/// Returns `None` on any structural inconsistency (a length that runs past
/// the end of the string, or a non-numeric id/length).
fn parse_tlv(s: &str) -> Option<Vec<Tlv>> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut fields = Vec::new();

    while i < chars.len() {
        if i + 4 > chars.len() {
            return None;
        }
        let id: String = chars[i..i + 2].iter().collect();
        let len_str: String = chars[i + 2..i + 4].iter().collect();
        let len: usize = len_str.parse().ok()?;
        let start = i + 4;
        let end = start + len;
        if end > chars.len() {
            return None;
        }
        let value: String = chars[start..end].iter().collect();
        fields.push(Tlv { id, value });
        i = end;
    }

    Some(fields)
}

/// Builds a single TLV field `id + 2-digit length + value`.
fn build_tlv(id: &str, value: &str) -> String {
    format!("{}{:02}{}", id, value.chars().count(), value)
}

/// CRC-16/CCITT-FALSE (initial value `0xFFFF`, polynomial `0x1021`), as the
/// BR Code manual requires for object `63`.
fn crc16_ccitt(data: &str) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for b in data.bytes() {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// The parsed Merchant Account Information (Pix) template: the GUI, and
/// either the key (static) or the PSP URL (dynamic).
struct PixMai {
    key: Option<String>,
    url: Option<String>,
}

fn find_pix_mai(fields: &[Tlv]) -> Option<PixMai> {
    for f in fields {
        let id_num: u32 = f.id.parse().ok()?;
        if !(26..=51).contains(&id_num) {
            continue;
        }
        let sub = parse_tlv(&f.value)?;
        let gui = sub.iter().find(|s| s.id == "00")?;
        if gui.value != PIX_GUI {
            continue;
        }
        let key = sub.iter().find(|s| s.id == "01").map(|s| s.value.clone());
        let url = sub.iter().find(|s| s.id == "25").map(|s| s.value.clone());
        return Some(PixMai { key, url });
    }
    None
}

/// Validates a Pix BR Code payload; the key itself is not checked (use
/// [`crate::pix_key::is_valid`]).
///
/// Checks the TLV structure, the CRC-16 and the mandatory objects (format
/// indicator, category code, currency, country, merchant name and city);
/// one Merchant Account Information template (IDs 26 to 51) must carry the
/// `br.gov.bcb.pix` GUI with a key (static) or a PSP URL (dynamic), never
/// both.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pix_payload::is_valid;
///
/// let payload = "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3D";
/// assert!(is_valid(payload));
/// assert!(!is_valid(""));
/// ```
pub fn is_valid(value: &str) -> bool {
    if value.len() < 4 {
        return false;
    }

    let crc_str = &value[value.len() - 4..];
    if crc_str.len() != 4 || !crc_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }

    // Object 63 is "6304" + the 4 hex digits; the CRC covers everything up
    // to and including "6304".
    let crc_field_start = match value.rfind("6304") {
        Some(idx) if idx == value.len() - 8 => idx,
        _ => return false,
    };

    let expected_crc = crc16_ccitt(&value[..crc_field_start + 4]);
    let expected_hex = format!("{:04X}", expected_crc);
    if !crc_str.eq_ignore_ascii_case(&expected_hex) {
        return false;
    }

    let fields = match parse_tlv(value) {
        Some(f) => f,
        None => return false,
    };

    let get = |id: &str| fields.iter().find(|f| f.id == id).map(|f| f.value.as_str());

    if get("00") != Some("01") {
        return false;
    }
    if let Some(poi) = get("01") {
        if poi != "11" && poi != "12" {
            return false;
        }
    }
    if get("52").is_none() || get("53").is_none() {
        return false;
    }
    if get("58") != Some("BR") {
        return false;
    }
    if get("59").is_none() || get("60").is_none() {
        return false;
    }

    let mai = match find_pix_mai(&fields) {
        Some(m) => m,
        None => return false,
    };

    match (&mai.key, &mai.url) {
        (Some(_), None) | (None, Some(_)) => {}
        _ => return false, // both or neither
    }

    if let Some(amount) = get("54") {
        match amount.parse::<f64>() {
            Ok(v) if v > 0.0 => {}
            _ => return false,
        }
    }

    true
}

/// Parsed fields of a Pix BR Code payload, as returned by [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct PixPayloadInfo {
    pub key: Option<String>,
    pub url: Option<String>,
    pub merchant_name: String,
    pub merchant_city: String,
    /// `"static"` or `"dynamic"`.
    pub point_of_initiation: String,
    pub amount: Option<f64>,
    pub txid: Option<String>,
}

/// Parses a Pix BR Code payload into its fields; returns `None` for
/// anything [`is_valid`] rejects, never a partial result.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pix_payload::get_info;
///
/// let payload = "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3D";
/// let info = get_info(payload).unwrap();
/// assert_eq!(info.key.as_deref(), Some("123e4567-e12b-12d1-a456-426655440000"));
/// assert_eq!(info.merchant_name, "Fulano de Tal");
/// assert_eq!(info.point_of_initiation, "static");
/// ```
pub fn get_info(value: &str) -> Option<PixPayloadInfo> {
    if !is_valid(value) {
        return None;
    }

    let fields = parse_tlv(value)?;
    let get = |id: &str| fields.iter().find(|f| f.id == id).map(|f| f.value.clone());

    let mai = find_pix_mai(&fields)?;
    let dynamic = mai.url.is_some() || get("01").as_deref() == Some("12");

    let (amount, txid) = if dynamic {
        (None, None)
    } else {
        let amount = get("54").and_then(|a| a.parse::<f64>().ok());
        let txid = get("62").and_then(|add| {
            let sub = parse_tlv(&add)?;
            sub.iter().find(|s| s.id == "05").map(|s| s.value.clone())
        });
        let txid = txid.filter(|t| t != "***");
        (amount, txid)
    };

    Some(PixPayloadInfo {
        key: mai.key,
        url: mai.url,
        merchant_name: get("59")?,
        merchant_city: get("60")?,
        point_of_initiation: if dynamic { "dynamic" } else { "static" }.to_string(),
        amount,
        txid,
    })
}

/// Removes diacritical marks from a string (Unicode NFD decomposition,
/// dropping combining marks).
fn remove_accents(value: &str) -> String {
    value
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect()
}

/// Parameters for [`generate`]. Exactly one of `key` or `url` must be set.
#[derive(Debug, Clone, Default)]
pub struct GeneratePixPayloadParams {
    pub key: Option<String>,
    pub url: Option<String>,
    pub merchant_name: String,
    pub merchant_city: String,
    pub amount: Option<f64>,
    pub txid: Option<String>,
}

/// Generates a Pix BR Code payload; exactly one of a key (static) or a PSP
/// URL (dynamic) must be given, otherwise returns `None`.
///
/// A key is normalized as [`crate::pix_key::get_info`] does. A URL makes
/// the payload dynamic (Point of Initiation `12`) and cannot carry an
/// amount or txid; it is host and path without scheme, at most 77
/// characters. The amount has two decimal places; a value with more, or
/// that rounds to `0.00`, is rejected. The txid is 1 to 25 alphanumerics
/// (default `***`). Merchant name and city lose their accents and are
/// truncated to 25 and 15 characters.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pix_payload::{generate, GeneratePixPayloadParams};
/// use brazilian_utils::pix_payload::is_valid;
///
/// let payload = generate(GeneratePixPayloadParams {
///     key: Some("123e4567-e12b-12d1-a456-426655440000".to_string()),
///     merchant_name: "Fulano de Tal".to_string(),
///     merchant_city: "Brasilia".to_string(),
///     ..Default::default()
/// }).unwrap();
/// assert!(is_valid(&payload));
/// ```
pub fn generate(params: GeneratePixPayloadParams) -> Option<String> {
    let has_key = params.key.is_some();
    let has_url = params.url.is_some();
    if has_key == has_url {
        return None; // neither or both given
    }

    if has_url && (params.amount.is_some() || params.txid.is_some()) {
        return None;
    }

    let merchant_name: String = remove_accents(&params.merchant_name)
        .chars()
        .take(25)
        .collect();
    let merchant_city: String = remove_accents(&params.merchant_city)
        .chars()
        .take(15)
        .collect();
    if merchant_name.is_empty() || merchant_city.is_empty() {
        return None;
    }

    let mut payload = String::new();
    payload.push_str(&build_tlv("00", "01"));

    if let Some(url) = &params.url {
        if url.len() > 77 || url.contains("://") {
            return None;
        }
        payload.push_str(&build_tlv("01", "12"));
        let mai = format!("{}{}", build_tlv("00", PIX_GUI), build_tlv("25", url));
        payload.push_str(&build_tlv("26", &mai));
    } else if let Some(key) = &params.key {
        let normalized = pix_key::get_info(key)?.value;
        let mai = format!("{}{}", build_tlv("00", PIX_GUI), build_tlv("01", &normalized));
        payload.push_str(&build_tlv("26", &mai));
    }

    payload.push_str(&build_tlv("52", "0000"));
    payload.push_str(&build_tlv("53", "986"));

    if let Some(amount) = params.amount {
        let cents = (amount * 100.0).round();
        if cents <= 0.0 || (amount * 100.0 - cents).abs() > 1e-6 {
            return None;
        }
        payload.push_str(&build_tlv("54", &format!("{:.2}", amount)));
    }

    payload.push_str(&build_tlv("58", "BR"));
    payload.push_str(&build_tlv("59", &merchant_name));
    payload.push_str(&build_tlv("60", &merchant_city));

    if !has_url {
        let txid = params.txid.clone().unwrap_or_else(|| "***".to_string());
        if txid.is_empty()
            || txid.chars().count() > 25
            || !txid.chars().all(|c| c.is_ascii_alphanumeric() || c == '*')
        {
            return None;
        }
        payload.push_str(&build_tlv("62", &build_tlv("05", &txid)));
    }

    payload.push_str("6304");
    let crc = crc16_ccitt(&payload);
    payload.push_str(&format!("{:04X}", crc));

    Some(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATIC_PAYLOAD: &str = "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3D";
    const AMOUNT_TXID_PAYLOAD: &str = "00020126580014br.gov.bcb.pix0136bee05743-4291-4f3c-9259-595df1307ba1520400005303986540510.005802BR5914Alexandre Lima6019Presidente Prudente62180514Um-Id-Qualquer6304D475";

    #[test]
    fn test_is_valid() {
        assert!(is_valid(STATIC_PAYLOAD));
        assert!(is_valid(AMOUNT_TXID_PAYLOAD));
        let wrong_crc = "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3E";
        assert!(!is_valid(wrong_crc));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get_info() {
        let info = get_info(STATIC_PAYLOAD).unwrap();
        assert_eq!(
            info.key.as_deref(),
            Some("123e4567-e12b-12d1-a456-426655440000")
        );
        assert_eq!(info.merchant_name, "Fulano de Tal");
        assert_eq!(info.merchant_city, "BRASILIA");
        assert_eq!(info.point_of_initiation, "static");
        assert_eq!(info.amount, None);
        assert_eq!(info.txid, None);

        let info2 = get_info(AMOUNT_TXID_PAYLOAD).unwrap();
        assert_eq!(
            info2.key.as_deref(),
            Some("bee05743-4291-4f3c-9259-595df1307ba1")
        );
        assert_eq!(info2.merchant_name, "Alexandre Lima");
        assert_eq!(info2.merchant_city, "Presidente Prudente");
        assert_eq!(info2.amount, Some(10.0));
        assert_eq!(info2.txid.as_deref(), Some("Um-Id-Qualquer"));

        assert_eq!(get_info(""), None);
    }

    #[test]
    fn test_generate_roundtrip() {
        let payload = generate(GeneratePixPayloadParams {
            key: Some("123e4567-e12b-12d1-a456-426655440000".to_string()),
            merchant_name: "Fulano de Tal".to_string(),
            merchant_city: "Brasilia".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert!(is_valid(&payload));
        let info = get_info(&payload).unwrap();
        assert_eq!(
            info.key.as_deref(),
            Some("123e4567-e12b-12d1-a456-426655440000")
        );

        // Neither key nor url given
        assert_eq!(
            generate(GeneratePixPayloadParams {
                merchant_name: "Fulano".to_string(),
                merchant_city: "Brasilia".to_string(),
                ..Default::default()
            }),
            None
        );
    }
}
