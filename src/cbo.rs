/// CBO (Classificação Brasileira de Ocupações) utilities.
///
/// Data source: MTE's official CBO 2002 occupation list; see
/// `src/data/cbo.json` for provenance.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct CboEntry {
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CboFile {
    data: Vec<CboEntry>,
}

fn table() -> &'static HashMap<String, String> {
    static TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/cbo.json");
        let parsed: CboFile = serde_json::from_str(JSON_DATA).expect("Failed to parse cbo.json");
        parsed.data.into_iter().map(|e| (e.code, e.description)).collect()
    })
}

/// Parses a value against a grouped digit pattern: either bare digits (no
/// longer than the total width, left-padded with zeros), or the full masked
/// form (each group boundary separated by one of ` `, `.`, `-` or `/`).
fn parse_grouped(value: &str, group_sizes: &[usize]) -> Option<String> {
    let total: usize = group_sizes.iter().sum();
    let chars: Vec<char> = value.chars().collect();

    if !chars.is_empty() && chars.len() <= total && chars.iter().all(|c| c.is_ascii_digit()) {
        return Some(format!("{:0>width$}", value, width = total));
    }

    let expected_len = total + (group_sizes.len() - 1);
    if chars.len() != expected_len {
        return None;
    }

    let mut code = String::new();
    let mut pos = 0;
    for (i, &size) in group_sizes.iter().enumerate() {
        for _ in 0..size {
            if !chars[pos].is_ascii_digit() {
                return None;
            }
            code.push(chars[pos]);
            pos += 1;
        }
        if i < group_sizes.len() - 1 {
            if !matches!(chars[pos], ' ' | '.' | '-' | '/') {
                return None;
            }
            pos += 1;
        }
    }
    Some(code)
}

fn normalize(value: &str) -> Option<String> {
    parse_grouped(value, &[4, 2])
}

/// Removes CBO formatting and keeps only digits, capped to 6 digits.
/// Nothing is left-padded.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cbo::parse;
///
/// assert_eq!(parse("2124-05"), "212405");
/// assert_eq!(parse("212405999"), "212405");
/// ```
pub fn parse(value: &str) -> String {
    value.chars().filter(|c| c.is_ascii_digit()).take(6).collect()
}

/// Checks whether a CBO code exists in the official CBO 2002 occupation
/// table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cbo::is_valid;
///
/// assert!(is_valid("212405"));
/// assert!(is_valid("2124-05"));
/// assert!(!is_valid("000000"));
/// ```
pub fn is_valid(value: &str) -> bool {
    match normalize(value) {
        Some(code) => table().contains_key(&code),
        None => false,
    }
}

/// A CBO code and its official title, as returned by [`get`].
#[derive(Debug, Clone, PartialEq)]
pub struct Cbo {
    pub code: String,
    pub description: String,
}

/// Looks a CBO code up in the official CBO 2002 occupation table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cbo::get;
///
/// let cbo = get("212405").unwrap();
/// assert_eq!(cbo.description, "Analista de desenvolvimento de sistemas");
/// assert_eq!(get("223150"), None);
/// ```
pub fn get(value: &str) -> Option<Cbo> {
    let code = normalize(value)?;
    table().get(&code).map(|description| Cbo {
        code,
        description: description.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("212405"));
        assert!(is_valid("2124-05"));
        assert!(is_valid("010205"));
        assert!(!is_valid("223150"));
        assert!(!is_valid("000000"));
        assert!(!is_valid("21240"));
        assert!(!is_valid("abcdef"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get() {
        let cbo = get("212405").unwrap();
        assert_eq!(cbo.code, "212405");
        assert_eq!(cbo.description, "Analista de desenvolvimento de sistemas");

        let masked = get("2124-05").unwrap();
        assert_eq!(masked.code, "212405");

        let leading_zero = get("0102-05").unwrap();
        assert_eq!(leading_zero.code, "010205");
        assert_eq!(leading_zero.description, "Oficial da aeronáutica");

        assert_eq!(get("223150"), None);
        assert_eq!(get("000000"), None);
        assert_eq!(get(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("2124-05"), "212405");
        assert_eq!(parse("212405"), "212405");
        assert_eq!(parse("21?ABC24-05abc"), "212405");
        assert_eq!(parse(""), "");
        assert_eq!(parse("212405999"), "212405");
    }
}
