/// Bank (COMPE / ISPB) utilities.
///
/// Data source: Banco Central do Brasil STR participants list; see
/// `src/data/banks.json` for provenance. Only institutions with a COMPE
/// code are included.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// A bank entry, as returned by [`get_by_code`], [`get_by_ispb`] and [`list`].
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Bank {
    pub code: String,
    pub ispb: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct BankFile {
    data: Vec<Bank>,
}

fn entries() -> &'static Vec<Bank> {
    static ENTRIES: OnceLock<Vec<Bank>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/banks.json");
        let parsed: BankFile = serde_json::from_str(JSON_DATA).expect("Failed to parse banks.json");
        parsed.data
    })
}

fn by_code() -> &'static HashMap<String, Bank> {
    static BY_CODE: OnceLock<HashMap<String, Bank>> = OnceLock::new();
    BY_CODE.get_or_init(|| entries().iter().cloned().map(|b| (b.code.clone(), b)).collect())
}

fn by_ispb() -> &'static HashMap<String, Bank> {
    static BY_ISPB: OnceLock<HashMap<String, Bank>> = OnceLock::new();
    BY_ISPB.get_or_init(|| entries().iter().cloned().map(|b| (b.ispb.clone(), b)).collect())
}

/// Looks a bank up by its 3-digit COMPE code in the Banco Central do Brasil
/// STR participants list.
///
/// # Arguments
///
/// * `code` - The COMPE code, with or without leading zeros.
///
/// # Returns
///
/// The bank, or `None` when no bank has that code.
///
/// # Examples
///
/// ```
/// use brazilian_utils::bank::get_by_code;
///
/// let bank = get_by_code("1").unwrap();
/// assert_eq!(bank.code, "001");
/// assert_eq!(bank.name, "Banco do Brasil S.A.");
///
/// assert_eq!(get_by_code("999"), None);
/// ```
pub fn get_by_code(code: &str) -> Option<Bank> {
    if !code.chars().all(|c| c.is_ascii_digit()) || code.is_empty() {
        return None;
    }
    let padded = format!("{:0>3}", code);
    by_code().get(&padded).cloned()
}

/// Looks a bank up by its 8-digit ISPB.
///
/// # Arguments
///
/// * `value` - The ISPB, with or without leading zeros.
///
/// # Returns
///
/// The bank, or `None` when no institution with a COMPE code has that ISPB.
///
/// # Examples
///
/// ```
/// use brazilian_utils::bank::get_by_ispb;
///
/// let bank = get_by_ispb("60701190").unwrap();
/// assert_eq!(bank.code, "341");
///
/// assert_eq!(get_by_ispb("99999999"), None);
/// ```
pub fn get_by_ispb(value: &str) -> Option<Bank> {
    if !value.chars().all(|c| c.is_ascii_digit()) || value.is_empty() {
        return None;
    }
    let padded = format!("{:0>8}", value);
    by_ispb().get(&padded).cloned()
}

/// Returns every bank with a COMPE code from the Banco Central do Brasil
/// STR participants list.
///
/// # Examples
///
/// ```
/// use brazilian_utils::bank::list;
///
/// let banks = list();
/// assert!(banks.iter().any(|b| b.code == "001"));
/// ```
pub fn list() -> Vec<Bank> {
    entries().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_by_code() {
        let bb = get_by_code("001").unwrap();
        assert_eq!(bb.ispb, "00000000");
        assert_eq!(bb.name, "Banco do Brasil S.A.");

        assert_eq!(get_by_code("1"), Some(bb.clone()));
        assert_eq!(get_by_code("999"), None);
        assert_eq!(get_by_code("abc"), None);
        assert_eq!(get_by_code(""), None);

        let itau = get_by_code("341").unwrap();
        assert_eq!(itau.ispb, "60701190");
        assert_eq!(itau.name, "ITAÚ UNIBANCO S.A.");
    }

    #[test]
    fn test_get_by_ispb() {
        let bb = get_by_ispb("00000000").unwrap();
        assert_eq!(bb.code, "001");

        let itau = get_by_ispb("60701190").unwrap();
        assert_eq!(itau.code, "341");

        assert_eq!(get_by_ispb("99999999"), None);
        assert_eq!(get_by_ispb("abc"), None);
        assert_eq!(get_by_ispb(""), None);
    }

    #[test]
    fn test_list() {
        let banks = list();
        assert!(banks.len() >= 71);
        assert!(banks.iter().any(|b| b.code == "001"));
        assert!(banks.iter().any(|b| b.code == "341"));
    }
}
