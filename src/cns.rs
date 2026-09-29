/// CNS (Cartão Nacional de Saúde) utilities.
///
/// The CNS is the unique identifier of a SUS (Sistema Único de Saúde) user,
/// health professional or health facility.

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

/// Formats a CNS number into groups of 3-4-4-4 digits separated by spaces.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cns::format;
///
/// assert_eq!(format("123456789010001"), "123 4567 8901 0001");
/// assert_eq!(format("1234"), "123 4");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[3, 4, 4, 4], &[" ", " ", " "])
}

/// Validates a CNS number: 15 digits, starting with 1, 2, 7, 8 or 9 (a
/// number starting with 5 is rejected, following ANVISA), with a weighted
/// modulus-11 check (weights 15 down to 1 over all 15 digits, remainder
/// must be 0).
///
/// Accepts the bare digits or the printed 3-4-4-4 groups split by
/// whitespace, `.`, `-` or `/`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cns::is_valid;
///
/// assert!(is_valid("123456789010000"));
/// assert!(is_valid("123 4567 8901 0000"));
/// assert!(is_valid("898000000043208"));
/// assert!(!is_valid("123456789010001"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '-' && *c != '/')
        .collect();

    if cleaned.len() != 15 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    match cleaned.chars().next() {
        Some('1') | Some('2') | Some('7') | Some('8') | Some('9') => {}
        _ => return false,
    }

    let sum: u32 = cleaned
        .chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap_or(0) * (15 - i as u32))
        .sum();

    sum % 11 == 0
}

/// Removes CNS formatting and keeps only digits, capped to 15 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cns::parse;
///
/// assert_eq!(parse("123 4567 8901 0000"), "123456789010000");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(15)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("123456789010001"), "123 4567 8901 0001");
        assert_eq!(format("898 0000 0004 3208"), "898 0000 0004 3208");
        assert_eq!(format("1234"), "123 4");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("123456789010000"));
        assert!(is_valid("123 4567 8901 0000"));
        assert!(is_valid("898000000043208"));
        assert!(is_valid("700000000000005"));
        assert!(!is_valid("123456789010001"));
        assert!(!is_valid("312345678901234"));
        assert!(!is_valid("12345678901"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("123 4567 8901 0000"), "123456789010000");
        assert_eq!(parse("123456789010000"), "123456789010000");
        assert_eq!(parse("123.?ABC4567 8901-0000abc"), "123456789010000");
        assert_eq!(parse(""), "");
        assert_eq!(parse("123456789010000999"), "123456789010000");
    }
}
