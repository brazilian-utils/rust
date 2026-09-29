/// CEI (Cadastro Específico do INSS) utilities.
///
/// The CEI identifies a construction work or other specific activity with
/// the INSS. It has been superseded by the CNO for construction works, but
/// kept the same 12-digit numbering and check-digit algorithm (see the
/// [`crate::cno`] module).

/// Applies a grouped mask to a digit string, as far as the digits go.
///
/// `group_sizes.len()` must be `separators.len() + 1`.
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

/// Calculates the CEI/CNO check digit for an 11-digit base.
///
/// Weights the base by 7, 4, 1, 8, 5, 2, 1, 6, 3, 7, 4; adds the tens of the
/// sum to its units; takes the complement to 10 of the resulting units
/// digit (10 maps to 0).
pub(crate) fn checksum(base: &str) -> u32 {
    const WEIGHTS: [u32; 11] = [7, 4, 1, 8, 5, 2, 1, 6, 3, 7, 4];
    let sum: u32 = base
        .chars()
        .zip(WEIGHTS.iter())
        .map(|(c, w)| c.to_digit(10).unwrap_or(0) * w)
        .sum();

    let tens = (sum / 10) % 10;
    let units = sum % 10;
    let s2 = tens + units;

    (10 - (s2 % 10)) % 10
}

/// Formats a CEI number with the mask `00.000.00000/00`.
///
/// The mask is applied as far as the digits go.
///
/// # Arguments
///
/// * `value` - The CEI value (digits, optionally already masked).
///
/// # Returns
///
/// The masked string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cei::format;
///
/// assert_eq!(format("277297118187"), "27.729.71181/87");
/// assert_eq!(format("27729"), "27.729");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[2, 3, 5, 2], &[".", ".", "/"])
}

/// Validates a CEI number: 12 digits, 11 base digits and one check digit.
///
/// A value whose digits are all the same is rejected.
///
/// # Arguments
///
/// * `value` - The CEI value to validate, bare or masked.
///
/// # Returns
///
/// `true` if `value` is a valid CEI, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cei::is_valid;
///
/// assert!(is_valid("11.583.00249/85"));
/// assert!(is_valid("115830024985"));
/// assert!(!is_valid("115830024984"));
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

/// Removes CEI formatting and keeps only digits, capped to 12 digits.
///
/// # Arguments
///
/// * `value` - A CEI string that may contain formatting symbols.
///
/// # Returns
///
/// A string with only the digits of `value`, capped to 12 characters.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cei::parse;
///
/// assert_eq!(parse("27.729.71181/87"), "277297118187");
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
        assert_eq!(format("277297118187"), "27.729.71181/87");
        assert_eq!(format("11.583.00249/85"), "11.583.00249/85");
        assert_eq!(format("27729"), "27.729");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("11.583.00249/85"));
        assert!(is_valid("115830024985"));
        assert!(is_valid("277297118187"));
        assert!(!is_valid("115830024984"));
        assert!(!is_valid("000000000000"));
        assert!(!is_valid("1234567890"));
        assert!(!is_valid("aa.583.00249/85"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("27.729.71181/87"), "277297118187");
        assert_eq!(parse("277297118187"), "277297118187");
        assert_eq!(parse("27.?ABC729.71181/87abc"), "277297118187");
        assert_eq!(parse(""), "");
        assert_eq!(parse("277297118187999"), "277297118187");
    }
}
