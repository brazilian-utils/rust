/// CFOP (Código Fiscal de Operações e Prestações) utilities.
///
/// Data source: CONFAZ Anexo II (Convênio SINIEF s/nº 1970, as amended); see
/// `src/data/cfop.json` for provenance. Only operable codes are included
/// (group/subgroup headings, ending in `00` or `50`, are not in the table).
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct CfopEntry {
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CfopData {
    data: Vec<CfopEntry>,
}

fn table() -> &'static HashMap<String, String> {
    static TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/cfop.json");
        let parsed: CfopData =
            serde_json::from_str(JSON_DATA).expect("Failed to parse cfop.json");
        parsed
            .data
            .into_iter()
            .map(|e| (e.code, e.description))
            .collect()
    })
}

/// Normalizes a CFOP input into its bare 4-digit form, rejecting anything
/// not written in an accepted form (4 digits, `N.NNN` with a single
/// separator, or a plain non-negative integer string). Nothing is padded,
/// since no CFOP starts with a zero.
fn normalize(value: &str) -> Option<String> {
    if value.len() == 4 && value.chars().all(|c| c.is_ascii_digit()) {
        return Some(value.to_string());
    }

    let chars: Vec<char> = value.chars().collect();
    if chars.len() == 5
        && chars[0].is_ascii_digit()
        && matches!(chars[1], ' ' | '.' | '-' | '/')
        && chars[2..5].iter().all(|c| c.is_ascii_digit())
    {
        return Some(format!("{}{}{}{}", chars[0], chars[2], chars[3], chars[4]));
    }

    None
}

/// Removes CFOP formatting and keeps only digits, capped to 4 digits.
/// Nothing is padded.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cfop::parse;
///
/// assert_eq!(parse("5.102"), "5102");
/// assert_eq!(parse("5102999"), "5102");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(4)
        .collect()
}

/// Checks whether a CFOP code exists in the consolidated table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cfop::is_valid;
///
/// assert!(is_valid("5102"));
/// assert!(is_valid("5.102"));
/// assert!(!is_valid("1100")); // group heading
/// assert!(!is_valid(""));
/// ```
pub fn is_valid(value: &str) -> bool {
    match normalize(value) {
        Some(code) => table().contains_key(&code),
        None => false,
    }
}

/// A CFOP code and its official description, as returned by [`get`].
#[derive(Debug, Clone, PartialEq)]
pub struct Cfop {
    pub code: String,
    pub description: String,
}

/// Looks a CFOP code up in the official table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cfop::get;
///
/// let cfop = get("5102").unwrap();
/// assert!(cfop.description.starts_with("Venda de mercadoria"));
/// assert_eq!(get("1100"), None);
/// ```
pub fn get(value: &str) -> Option<Cfop> {
    let code = normalize(value)?;
    table().get(&code).map(|description| Cfop {
        code,
        description: description.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("5102"));
        assert!(is_valid("5.102"));
        assert!(is_valid("7504"));
        assert!(!is_valid("0000"));
        assert!(!is_valid("1100"));
        assert!(!is_valid("510"));
        assert!(!is_valid("abcd"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get() {
        assert_eq!(get("1100"), None);
        assert_eq!(get("510"), None);
        assert_eq!(get(""), None);

        let cfop = get("5102").unwrap();
        assert_eq!(cfop.code, "5102");
        assert!(cfop.description.contains("Venda de mercadoria"));

        let masked = get("5.102").unwrap();
        assert_eq!(masked.code, "5102");
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("5.102"), "5102");
        assert_eq!(parse("5102"), "5102");
        assert_eq!(parse("5?ABC.102abc"), "5102");
        assert_eq!(parse(""), "");
        assert_eq!(parse("5102999"), "5102");
    }
}
