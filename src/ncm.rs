/// NCM (Nomenclatura Comum do Mercosul) utilities.
///
/// Data source: Siscomex/Receita Federal/MDIC-Gecex NCM table; see
/// `src/data/ncm.json` for provenance. Only level-8 (leaf) codes currently
/// in force are included.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct NcmEntry {
    code: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct NcmFile {
    data: Vec<NcmEntry>,
}

fn table() -> &'static HashMap<String, String> {
    static TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/ncm.json");
        let parsed: NcmFile = serde_json::from_str(JSON_DATA).expect("Failed to parse ncm.json");
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
    parse_grouped(value, &[4, 2, 2])
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

/// Formats an NCM code with the mask `NNNN.NN.NN`; only the structure
/// changes (use [`is_valid`] to check the code).
///
/// # Examples
///
/// ```
/// use brazilian_utils::ncm::format;
///
/// assert_eq!(format("84713012"), "8471.30.12");
/// assert_eq!(format("84713"), "8471.3");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[4, 2, 2], &[".", "."])
}

/// Removes NCM formatting and keeps only digits, capped to 8 digits.
/// Nothing is left-padded.
///
/// # Examples
///
/// ```
/// use brazilian_utils::ncm::parse;
///
/// assert_eq!(parse("8471.30.12"), "84713012");
/// ```
pub fn parse(value: &str) -> String {
    value.chars().filter(|c| c.is_ascii_digit()).take(8).collect()
}

/// Checks whether an NCM code exists in the current table published by
/// Siscomex.
///
/// # Examples
///
/// ```
/// use brazilian_utils::ncm::is_valid;
///
/// assert!(is_valid("22030000"));
/// assert!(is_valid("2203.00.00"));
/// assert!(!is_valid("12345678"));
/// ```
pub fn is_valid(value: &str) -> bool {
    match normalize(value) {
        Some(code) => table().contains_key(&code),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("84713012"), "8471.30.12");
        assert_eq!(format("8471.30.12"), "8471.30.12");
        assert_eq!(format("84713"), "8471.3");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("22030000"));
        assert!(is_valid("2203.00.00"));
        assert!(is_valid("01012100"));
        assert!(!is_valid("12345678"));
        assert!(!is_valid("2203000"));
        assert!(!is_valid("abcdefgh"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("8471.30.12"), "84713012");
        assert_eq!(parse("84713012"), "84713012");
        assert_eq!(parse("84?ABC71.30.12abc"), "84713012");
        assert_eq!(parse(""), "");
        assert_eq!(parse("84713012999"), "84713012");
    }
}
