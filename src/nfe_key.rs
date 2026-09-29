/// NF-e key (DF-e access key / chave de acesso) utilities.
///
/// Covers the 44-digit access key shared by the NF-e, NFC-e, CT-e, MDF-e,
/// CT-e OS, GTV-e, BP-e, NF3e and NFCom (the CF-e-SAT, model 59, is out of
/// scope).
const XML_ID_PREFIXES: &[&str] = &["NFe", "CTe", "MDFe", "BPe", "NF3e", "NFCom"];

const VALID_MODELS: &[&str] = &["55", "65", "57", "58", "67", "64", "63", "66", "62"];

fn strip_prefix(value: &str) -> &str {
    let trimmed = value.trim();
    for prefix in XML_ID_PREFIXES {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return rest;
        }
    }
    trimmed
}

/// Maps the 2-digit `cUF` field to its state abbreviation.
fn state_code_for_cuf(cuf: &str) -> Option<&'static str> {
    Some(match cuf {
        "11" => "RO",
        "12" => "AC",
        "13" => "AM",
        "14" => "RR",
        "15" => "PA",
        "16" => "AP",
        "17" => "TO",
        "21" => "MA",
        "22" => "PI",
        "23" => "CE",
        "24" => "RN",
        "25" => "PB",
        "26" => "PE",
        "27" => "AL",
        "28" => "SE",
        "29" => "BA",
        "31" => "MG",
        "32" => "ES",
        "33" => "RJ",
        "35" => "SP",
        "41" => "PR",
        "42" => "SC",
        "43" => "RS",
        "50" => "MS",
        "51" => "MT",
        "52" => "GO",
        "53" => "DF",
        _ => return None,
    })
}

fn apply_grouped_mask(digits: &str, group_sizes: &[usize], separators: &[&str]) -> String {
    let mut result = String::new();
    let mut pos = 0;

    for (i, &size) in group_sizes.iter().enumerate() {
        if pos >= digits.len() {
            break;
        }
        let end = (pos + size).min(digits.len());
        result.push_str(&digits[pos..end]);
        pos = end;

        if pos >= digits.len() {
            break;
        }
        if i < separators.len() {
            result.push_str(separators[i]);
        }
    }

    result
}

/// Removes the formatting of a DF-e access key and keeps only digits,
/// capped to 44 digits. The XML `Id` prefixes are stripped first.
///
/// # Examples
///
/// ```
/// use brazilian_utils::nfe_key::parse;
///
/// assert_eq!(
///     parse("NFe35170458716523000119550010000000121000123458"),
///     "35170458716523000119550010000000121000123458"
/// );
/// ```
pub fn parse(value: &str) -> String {
    strip_prefix(value)
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(44)
        .collect()
}

/// Formats a DF-e access key into groups of 4 digits separated by spaces,
/// as the DANFE and the other auxiliary documents print it. Does not
/// validate (use [`is_valid`]).
///
/// # Examples
///
/// ```
/// use brazilian_utils::nfe_key::format;
///
/// assert_eq!(
///     format("35170458716523000119550010000000121000123458"),
///     "3517 0458 7165 2300 0119 5500 1000 0000 1210 0012 3458"
/// );
/// assert_eq!(format("12345"), "1234 5");
/// ```
pub fn format(value: &str) -> String {
    let digits = parse(value);
    apply_grouped_mask(&digits, &[4; 11], &[" "; 10])
}

/// Computes the modulus-11 check digit over the first 43 digits of a DF-e
/// access key (weights cycling 2 to 9, from the rightmost digit).
fn check_digit(base43: &str) -> u32 {
    let mut weight = 2u32;
    let mut sum = 0u32;
    for c in base43.chars().rev() {
        sum += c.to_digit(10).unwrap_or(0) * weight;
        weight = if weight == 9 { 2 } else { weight + 1 };
    }
    let rem = sum % 11;
    if rem < 2 {
        0
    } else {
        11 - rem
    }
}

/// Returns whether every character of `digits` is the same.
fn is_all_repeated(digits: &str) -> bool {
    let mut chars = digits.chars();
    match chars.next() {
        Some(first) => chars.all(|c| c == first),
        None => false,
    }
}

/// Returns whether `digits` is a strictly ascending run of consecutive
/// digits (e.g. `"01234567"` or `"12345678"`).
fn is_ascending_sequential(digits: &str) -> bool {
    let chars: Vec<u32> = digits.chars().filter_map(|c| c.to_digit(10)).collect();
    if chars.len() != digits.len() || chars.len() < 2 {
        return false;
    }
    chars.windows(2).all(|w| w[1] == w[0] + 1)
}

/// Returns whether `digits` is a strictly descending run of consecutive
/// digits (e.g. `"87654321"` or `"76543210"`).
fn is_descending_sequential(digits: &str) -> bool {
    let chars: Vec<u32> = digits.chars().filter_map(|c| c.to_digit(10)).collect();
    if chars.len() != digits.len() || chars.len() < 2 {
        return false;
    }
    chars.windows(2).all(|w| w[1] + 1 == w[0])
}

/// Validates a 44-digit DF-e access key.
///
/// The `cUF` must be a state, the model one of the DF-e models this module
/// covers and the document number (`nNF`) not all zeros. The check digit is
/// a modulus 11 over the first 43 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::nfe_key::is_valid;
///
/// assert!(is_valid("35170458716523000119550010000000121000123458"));
/// assert!(is_valid("NFe35170458716523000119550010000000121000123458"));
/// assert!(!is_valid("35170458716523000119550010000000001000123457"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let digits = parse(value);

    if digits.len() != 44 {
        return false;
    }

    let cuf = &digits[0..2];
    if state_code_for_cuf(cuf).is_none() {
        return false;
    }

    let model = &digits[20..22];
    if !VALID_MODELS.contains(&model) {
        return false;
    }

    let nnf = &digits[25..34];
    if nnf.chars().all(|c| c == '0') {
        return false;
    }

    // Rule B03-10: only applies to NF-e (55) and NFC-e (65). The `cNF` field
    // must not be all-repeated-digits, ascending-sequential,
    // descending-sequential, or equal to `nNF`.
    if model == "55" || model == "65" {
        let cnf = &digits[35..43];
        if is_all_repeated(cnf) || is_ascending_sequential(cnf) || is_descending_sequential(cnf) {
            return false;
        }
        if let (Ok(cnf_num), Ok(nnf_num)) = (cnf.parse::<u64>(), nnf.parse::<u64>()) {
            if cnf_num == nnf_num {
                return false;
            }
        }
    }

    let base43 = &digits[0..43];
    let expected = check_digit(base43);
    let actual = digits.chars().nth(43).and_then(|c| c.to_digit(10));

    actual == Some(expected)
}

/// Parsed fields of a DF-e access key, as returned by [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct NfeKeyInfo {
    pub state_code: String,
    pub year: i32,
    pub month: u32,
    pub tax_id: String,
    pub model: String,
    pub series: u32,
    pub number: u64,
    pub emission_type: u32,
    pub code: String,
    pub check_digit: u32,
}

/// Parses a DF-e access key into its fields; returns `None` when the key is
/// not valid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::nfe_key::get_info;
///
/// let info = get_info("35170458716523000119550010000000121000123458").unwrap();
/// assert_eq!(info.state_code, "SP");
/// assert_eq!(info.year, 2017);
/// assert_eq!(info.month, 4);
/// ```
pub fn get_info(value: &str) -> Option<NfeKeyInfo> {
    if !is_valid(value) {
        return None;
    }

    let digits = parse(value);
    let cuf = &digits[0..2];
    let state_code = state_code_for_cuf(cuf)?.to_string();
    let year = 2000 + digits[2..4].parse::<i32>().ok()?;
    let month = digits[4..6].parse().ok()?;
    let tax_id = digits[6..20].to_string();
    let model = digits[20..22].to_string();
    let series = digits[22..25].parse().ok()?;
    let number = digits[25..34].parse().ok()?;
    let emission_type = digits[34..35].parse().ok()?;
    let code = digits[35..43].to_string();
    let check_digit = digits[43..44].parse().ok()?;

    Some(NfeKeyInfo {
        state_code,
        year,
        month,
        tax_id,
        model,
        series,
        number,
        emission_type,
        code,
        check_digit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_KEY: &str = "35170458716523000119550010000000121000123458";

    #[test]
    fn test_format() {
        assert_eq!(
            format(VALID_KEY),
            "3517 0458 7165 2300 0119 5500 1000 0000 1210 0012 3458"
        );
        assert_eq!(
            format("NFe35170458716523000119550010000000121000123458"),
            "3517 0458 7165 2300 0119 5500 1000 0000 1210 0012 3458"
        );
        assert_eq!(format("12345"), "1234 5");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid(VALID_KEY));
        assert!(is_valid(
            "3517 0458 7165 2300 0119 5500 1000 0000 1210 0012 3458"
        ));
        assert!(is_valid(&format!("NFe{}", VALID_KEY)));
        assert!(!is_valid(
            "35170458716523000119550010000000001000123457"
        ));
        assert!(!is_valid("3517045871652300011955001000000012100012345"));
        assert!(!is_valid(""));
    }

    /// Builds a 44-digit key from `VALID_KEY` with the model (digits 20..22)
    /// and/or the `cNF` field (digits 35..43) overridden, recomputing the
    /// check digit so the result is structurally valid.
    fn key_with(model: Option<&str>, cnf: Option<&str>) -> String {
        let mut digits: Vec<char> = VALID_KEY.chars().collect();
        if let Some(model) = model {
            for (i, c) in model.chars().enumerate() {
                digits[20 + i] = c;
            }
        }
        if let Some(cnf) = cnf {
            for (i, c) in cnf.chars().enumerate() {
                digits[35 + i] = c;
            }
        }
        let base43: String = digits[0..43].iter().collect();
        let dv = check_digit(&base43);
        format!("{}{}", base43, dv)
    }

    #[test]
    fn test_b03_10_rejects_sequential_cnf_for_model_55() {
        // Ascending sequential cNF.
        assert!(!is_valid(&key_with(None, Some("01234567"))));
        assert!(!is_valid(&key_with(None, Some("12345678"))));

        // Descending sequential cNF.
        assert!(!is_valid(&key_with(None, Some("87654321"))));
        assert!(!is_valid(&key_with(None, Some("76543210"))));
    }

    #[test]
    fn test_b03_10_only_applies_to_models_55_and_65() {
        // Regression test: a structurally-valid model-57 (CT-e) key with an
        // all-repeated cNF must validate true, since B03-10 does not apply
        // outside models 55/65.
        assert!(is_valid(&key_with(Some("57"), Some("00000000"))));

        // Same all-repeated cNF, but model 55: must still be rejected.
        assert!(!is_valid(&key_with(Some("55"), Some("00000000"))));

        // And model 65 (NFC-e): must also be rejected.
        assert!(!is_valid(&key_with(Some("65"), Some("00000000"))));
    }

    #[test]
    fn test_get_info() {
        let info = get_info(VALID_KEY).unwrap();
        assert_eq!(info.state_code, "SP");
        assert_eq!(info.year, 2017);
        assert_eq!(info.month, 4);
        assert_eq!(info.tax_id, "58716523000119");
        assert_eq!(info.model, "55");
        assert_eq!(info.series, 1);
        assert_eq!(info.number, 12);
        assert_eq!(info.emission_type, 1);
        assert_eq!(info.code, "00012345");
        assert_eq!(info.check_digit, 8);

        assert_eq!(
            get_info("35170458716523000119550010000000121000123459"),
            None
        );
        assert_eq!(get_info(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("3517 0458 7165 2300 0119 5500 1000 0000 1210 0012 3458"),
            VALID_KEY
        );
        assert_eq!(parse(&format!("NFe{}", VALID_KEY)), VALID_KEY);
        assert_eq!(parse("3517 0458"), "35170458");
        assert_eq!(parse(""), "");
    }
}
