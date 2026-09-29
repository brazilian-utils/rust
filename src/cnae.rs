/// CNAE (Classificação Nacional de Atividades Econômicas) utilities.
///
/// Data source: IBGE CONCLA API, CNAE-Subclasses 2.3; see
/// `src/data/cnae.json` for provenance.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct CnaeEntry {
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CnaeFile {
    data: Vec<CnaeEntry>,
}

fn table() -> &'static HashMap<String, String> {
    static TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/cnae.json");
        let parsed: CnaeFile = serde_json::from_str(JSON_DATA).expect("Failed to parse cnae.json");
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
    parse_grouped(value, &[4, 1, 2])
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

/// Formats a CNAE subclass code with the mask `NNNN-N/NN`; only the
/// structure changes (use [`is_valid`] to check the code).
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnae::format;
///
/// assert_eq!(format("6201501"), "6201-5/01");
/// assert_eq!(format("62015"), "6201-5");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[4, 1, 2], &["-", "/"])
}

/// Removes CNAE formatting and keeps only digits, capped to 7 digits.
/// Nothing is left-padded.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnae::parse;
///
/// assert_eq!(parse("6201-5/01"), "6201501");
/// ```
pub fn parse(value: &str) -> String {
    value.chars().filter(|c| c.is_ascii_digit()).take(7).collect()
}

/// Checks whether a CNAE subclass code exists in the CNAE-Subclasses 2.3
/// table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnae::is_valid;
///
/// assert!(is_valid("6201501"));
/// assert!(is_valid("6201-5/01"));
/// assert!(!is_valid("0000000"));
/// ```
pub fn is_valid(value: &str) -> bool {
    match normalize(value) {
        Some(code) => table().contains_key(&code),
        None => false,
    }
}

/// A CNAE subclass code and its official description, as returned by
/// [`get`].
#[derive(Debug, Clone, PartialEq)]
pub struct Cnae {
    pub code: String,
    pub description: String,
}

/// Looks a CNAE subclass code up in the CNAE-Subclasses 2.3 table.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnae::get;
///
/// let cnae = get("6201501").unwrap();
/// assert_eq!(cnae.description, "DESENVOLVIMENTO DE PROGRAMAS DE COMPUTADOR SOB ENCOMENDA");
/// ```
pub fn get(value: &str) -> Option<Cnae> {
    let code = normalize(value)?;
    table().get(&code).map(|description| Cnae {
        code,
        description: description.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("6201501"), "6201-5/01");
        assert_eq!(format("6201-5/01"), "6201-5/01");
        assert_eq!(format("62015"), "6201-5");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("6201501"));
        assert!(is_valid("6201-5/01"));
        assert!(is_valid("0111301"));
        assert!(!is_valid("0000000"));
        assert!(!is_valid("620150"));
        assert!(!is_valid("62015011"));
        assert!(!is_valid("abcdefg"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get() {
        let cnae = get("6201501").unwrap();
        assert_eq!(cnae.code, "6201501");
        assert_eq!(
            cnae.description,
            "DESENVOLVIMENTO DE PROGRAMAS DE COMPUTADOR SOB ENCOMENDA"
        );

        let masked = get("6201-5/01").unwrap();
        assert_eq!(masked.code, "6201501");

        let leading_zero = get("0111-3/01").unwrap();
        assert_eq!(leading_zero.code, "0111301");
        assert_eq!(leading_zero.description, "CULTIVO DE ARROZ");

        assert_eq!(get("0000000"), None);
        assert_eq!(get("620150"), None);
        assert_eq!(get(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("6201-5/01"), "6201501");
        assert_eq!(parse("6201501"), "6201501");
        assert_eq!(parse("62?ABC01-5/01abc"), "6201501");
        assert_eq!(parse(""), "");
        assert_eq!(parse("6201501999"), "6201501");
    }
}
