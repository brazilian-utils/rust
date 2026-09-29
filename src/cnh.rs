use rand::Rng;

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

/// Formats a CNH number as `000000000-00` (9 digits, hyphen, 2 check digits).
///
/// The mask is applied as far as the digits go.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnh::format;
///
/// assert_eq!(format("00000000119"), "000000001-19");
/// assert_eq!(format("0000000011"), "000000001-1");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[9, 2], &["-"])
}

/// Removes CNH formatting and keeps only digits, capped to 11 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnh::parse;
///
/// assert_eq!(parse("000000001-19"), "00000000119");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(11)
        .collect()
}

/// Generates a random valid CNH number: 11 digits, unformatted.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnh::{generate, is_valid_cnh};
///
/// let cnh = generate();
/// assert_eq!(cnh.len(), 11);
/// assert!(is_valid_cnh(&cnh));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    loop {
        let base: Vec<u32> = (0..9).map(|_| rng.gen_range(0..=9)).collect();

        // Reject a base whose (would-be) 11 digits are all the same.
        if base.iter().all(|&d| d == base[0]) {
            continue;
        }

        let first_remainder = compute_first_remainder(&base);
        let first_verificator = if first_remainder > 9 { 0 } else { first_remainder };

        let mut sum = 0;
        for (i, &digit) in base.iter().enumerate() {
            sum += digit * (i as u32 + 1);
        }
        let mut second_verificator = sum % 11;
        if first_remainder >= 10 {
            second_verificator = if (second_verificator as i32 - 2) < 0 {
                second_verificator + 9
            } else {
                second_verificator - 2
            };
        }
        if second_verificator > 9 {
            second_verificator = 0;
        }

        let base_str: String = base.iter().map(|d| d.to_string()).collect();
        return format!("{}{}{}", base_str, first_verificator, second_verificator);
    }
}

/// Validates the registration number for the Brazilian CNH (Carteira Nacional de Habilitação)
/// that was created in 2022.
///
/// Previous versions of the CNH are not supported in this version.
/// This function checks if the given CNH is valid based on the format and allowed characters,
/// verifying the verification digits.
///
/// # Arguments
///
/// * `cnh` - CNH string (symbols will be ignored).
///
/// # Returns
///
/// `true` if CNH has a valid format, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnh::is_valid_cnh;
///
/// assert_eq!(is_valid_cnh("12345678901"), false);
/// assert_eq!(is_valid_cnh("A2C45678901"), false);
/// assert_eq!(is_valid_cnh("98765432109"), true);
/// assert_eq!(is_valid_cnh("987654321-09"), true);
/// ```
pub fn is_valid_cnh(cnh: &str) -> bool {
    // Clean the input and check for numbers only
    let cnh_digits: String = cnh.chars().filter(|c| c.is_ascii_digit()).collect();

    if cnh_digits.is_empty() {
        return false;
    }

    if cnh_digits.len() != 11 {
        return false;
    }

    // Reject sequences as "00000000000", "11111111111", etc.
    if cnh_digits
        .chars()
        .all(|c| c == cnh_digits.chars().next().unwrap())
    {
        return false;
    }

    // Cast digits to list of integers
    let digits: Vec<u32> = cnh_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first_verificator = digits[9];
    let second_verificator = digits[10];

    // The decrement below the 11th digit depends on the *raw* remainder of the
    // first checksum (which can be 10), not on the printed 10th digit (0-9
    // after clamping). Keeping both around is what makes the DENATRAN
    // decrement rule reachable.
    let first_remainder = compute_first_remainder(&digits);

    // Checking the 10th digit
    if !check_first_verificator(first_remainder, first_verificator) {
        return false;
    }

    // Checking the 11th digit
    check_second_verificator(&digits, second_verificator, first_remainder)
}

/// Computes the raw remainder (0-10) used to derive the 10th digit of the CNH.
fn compute_first_remainder(digits: &[u32]) -> u32 {
    let mut sum = 0;
    for (i, &digit) in digits.iter().enumerate().take(9) {
        sum += digit * (9 - i as u32);
    }

    sum % 11
}

/// Uses the first checksum's raw remainder to verify the 10th digit of the CNH
fn check_first_verificator(first_remainder: u32, first_verificator: u32) -> bool {
    let result = if first_remainder > 9 { 0 } else { first_remainder };

    result == first_verificator
}

/// Generates the second verification and uses it to verify the 11th digit of the CNH
///
/// `first_remainder` must be the *raw* remainder of the first checksum (0-10),
/// not the printed 10th digit: per the DENATRAN algorithm, whenever that
/// remainder is 10 (printed as 0), the second checksum is decremented by 2
/// (wrapping around 11 if negative).
fn check_second_verificator(digits: &[u32], second_verificator: u32, first_remainder: u32) -> bool {
    let mut sum = 0;
    for (i, &digit) in digits.iter().enumerate().take(9) {
        sum += digit * (i as u32 + 1);
    }

    let mut result = sum % 11;

    if first_remainder >= 10 {
        result = if (result as i32 - 2) < 0 {
            result + 9
        } else {
            result - 2
        };
    }

    if result > 9 {
        result = 0;
    }

    result == second_verificator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_cnh() {
        // Invalid: repeated sequence
        assert!(!is_valid_cnh("22222222222"));
        assert!(!is_valid_cnh("00000000000"));
        assert!(!is_valid_cnh("11111111111"));
        assert!(!is_valid_cnh("33333333333"));
        assert!(!is_valid_cnh("99999999999"));

        // Invalid: contains letters
        assert!(!is_valid_cnh("ABC70304734"));
        assert!(!is_valid_cnh("A2C45678901"));
        assert!(!is_valid_cnh("1234567890A"));

        // Invalid: wrong length
        assert!(!is_valid_cnh("6619558737912"));
        assert!(!is_valid_cnh("123456789"));
        assert!(!is_valid_cnh("1234567890"));
        assert!(!is_valid_cnh("123456789012"));

        // Valid with formatting
        assert!(is_valid_cnh("097703047-34"));
        assert!(is_valid_cnh("987654321-09"));

        // Valid without formatting
        assert!(is_valid_cnh("09770304734"));
        assert!(is_valid_cnh("98765432109"));

        // "98765432100" was mistakenly treated as valid before the decrement
        // fix (the first checksum's raw remainder here is 10, so the second
        // checksum must be decremented by 2 mod 11, giving 9 - not 0).
        assert!(!is_valid_cnh("98765432100"));

        // Additional test cases - invalid checksum
        assert!(!is_valid_cnh("12345678901"));

        // Edge cases
        assert!(!is_valid_cnh(""));
        assert!(!is_valid_cnh("           "));
        assert!(!is_valid_cnh("---"));
    }

    #[test]
    fn test_check_first_verificator() {
        // Test with valid CNH: 09770304734
        let digits = vec![0, 9, 7, 7, 0, 3, 0, 4, 7, 3, 4];
        let remainder = compute_first_remainder(&digits);
        assert!(check_first_verificator(remainder, 3));

        // Test with invalid first verificator
        assert!(!check_first_verificator(remainder, 5));
    }

    #[test]
    fn test_check_second_verificator() {
        // Test with valid CNH: 09770304734
        let digits = vec![0, 9, 7, 7, 0, 3, 0, 4, 7, 3, 4];
        assert!(check_second_verificator(&digits, 4, 3));

        // Test with invalid second verificator
        assert!(!check_second_verificator(&digits, 5, 3));
    }

    #[test]
    fn test_is_valid_cnh_symbols_removed() {
        // Test that various symbols are removed
        assert!(is_valid_cnh("097-703-047-34"));
        assert!(is_valid_cnh("097.703.047.34"));
        assert!(is_valid_cnh("097 703 047 34"));
        assert!(is_valid_cnh("(097)703-047-34"));
    }

    #[test]
    fn test_is_valid_cnh_mixed_invalid() {
        // Mixed letters and numbers
        assert!(!is_valid_cnh("0977O3O4734")); // O instead of 0
        assert!(!is_valid_cnh("097703O4734"));
    }

    #[test]
    fn test_edge_cases_first_verificator_greater_than_9() {
        // When first verificator is > 9, special logic applies
        // This would require finding a real CNH that triggers this
        // For now, just ensure the function handles it
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0];
        // Just make sure it doesn't panic
        let _ = check_second_verificator(&digits, 0, 10);
    }

    #[test]
    fn test_format() {
        assert_eq!(format("00000000119"), "000000001-19");
        assert_eq!(format("000.000.001-19"), "000000001-19");
        assert_eq!(format("0000000011"), "000000001-1");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("000000001-19"), "00000000119");
        assert_eq!(parse("00000000119"), "00000000119");
        assert_eq!(parse("000.abc000001-19"), "00000000119");
        assert_eq!(parse(""), "");
        assert_eq!(parse("00000000119123"), "00000000119");
    }

    #[test]
    fn test_generate() {
        for _ in 0..50 {
            let cnh = generate();
            assert_eq!(cnh.len(), 11);
            assert!(is_valid_cnh(&cnh));
        }
    }

    #[test]
    fn test_is_valid_cnh_decrement_rule() {
        // Regression test for the DENATRAN decrement rule: when the raw
        // remainder of the first checksum is 10 (so the printed 10th digit
        // is 0), the second checksum must be decremented by 2. Before the
        // fix, is_valid_cnh compared against the printed digit (always 0-9)
        // instead of the raw remainder, so this branch was unreachable and
        // valid CNHs like this one were rejected.
        assert!(is_valid_cnh("10433218109"));
    }
}
