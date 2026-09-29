/// CNO (Cadastro Nacional de Obras) utilities.
///
/// The CNO registers a construction work with the Receita Federal; it
/// replaced the CEI for construction works and kept the same numbering and
/// check-digit algorithm (see [`crate::cei`]).
use crate::cei::checksum;

/// Applies a grouped mask to a digit string, as far as the digits go.
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

/// Formats a CNO number with the mask `00.000.00000/00` (the CEI numbering).
///
/// # Examples
///
/// ```
/// use brazilian_utils::cno::format;
///
/// assert_eq!(format("111130137368"), "11.113.01373/68");
/// assert_eq!(format("11113"), "11.113");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[2, 3, 5, 2], &[".", ".", "/"])
}

/// Validates a CNO number: 12 digits, 11 base digits and one check digit.
///
/// Same rules as [`crate::cei::is_valid`].
///
/// # Examples
///
/// ```
/// use brazilian_utils::cno::is_valid;
///
/// assert!(is_valid("110840168062"));
/// assert!(is_valid("11.084.01680/62"));
/// assert!(!is_valid("110840168063"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '/' && *c != '-')
        .collect();

    if cleaned.len() != 12 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if cleaned.chars().all(|c| c == cleaned.chars().next().unwrap()) {
        return false;
    }

    let base = &cleaned[0..11];
    let expected = checksum(base);
    let actual = cleaned.chars().nth(11).and_then(|c| c.to_digit(10));

    actual == Some(expected)
}

/// Removes CNO formatting and keeps only digits, capped to 12 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cno::parse;
///
/// assert_eq!(parse("11.113.01373/68"), "111130137368");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(12)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("111130137368"), "11.113.01373/68");
        assert_eq!(format("11.084.01680/62"), "11.084.01680/62");
        assert_eq!(format("11113"), "11.113");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("110840168062"));
        assert!(is_valid("11.084.01680/62"));
        assert!(is_valid("401800097960"));
        assert!(!is_valid("110840168063"));
        assert!(!is_valid("000000000000"));
        assert!(!is_valid("1234567890"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("11.113.01373/68"), "111130137368");
        assert_eq!(parse("111130137368"), "111130137368");
        assert_eq!(parse("11.?ABC113.01373/68abc"), "111130137368");
        assert_eq!(parse(""), "");
        assert_eq!(parse("111130137368999"), "111130137368");
    }
}
