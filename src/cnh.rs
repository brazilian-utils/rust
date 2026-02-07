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
/// assert_eq!(is_valid_cnh("98765432100"), true);
/// assert_eq!(is_valid_cnh("987654321-00"), true);
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
    if cnh_digits.chars().all(|c| c == cnh_digits.chars().next().unwrap()) {
        return false;
    }

    // Cast digits to list of integers
    let digits: Vec<u32> = cnh_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first_verificator = digits[9];
    let second_verificator = digits[10];

    // Checking the 10th digit
    if !check_first_verificator(&digits, first_verificator) {
        return false;
    }

    // Checking the 11th digit
    check_second_verificator(&digits, second_verificator, first_verificator)
}

/// Generates the first verification digit and uses it to verify the 10th digit of the CNH
fn check_first_verificator(digits: &[u32], first_verificator: u32) -> bool {
    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (9 - i as u32);
    }

    let remainder = sum % 11;
    let result = if remainder > 9 { 0 } else { remainder };

    result == first_verificator
}

/// Generates the second verification and uses it to verify the 11th digit of the CNH
fn check_second_verificator(
    digits: &[u32],
    second_verificator: u32,
    first_verificator: u32,
) -> bool {
    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (i as u32 + 1);
    }

    let mut result = sum % 11;

    if first_verificator > 9 {
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
        assert_eq!(is_valid_cnh("22222222222"), false);
        
        // Invalid: contains letters
        assert_eq!(is_valid_cnh("ABC70304734"), false);
        
        // Invalid: wrong length
        assert_eq!(is_valid_cnh("6619558737912"), false);
        
        // Valid with formatting
        assert_eq!(is_valid_cnh("097703047-34"), true);
        
        // Valid without formatting
        assert_eq!(is_valid_cnh("09770304734"), true);
        
        // Additional test cases
        assert_eq!(is_valid_cnh("12345678901"), false);
        assert_eq!(is_valid_cnh("A2C45678901"), false);
        assert_eq!(is_valid_cnh("98765432100"), true);
        assert_eq!(is_valid_cnh("987654321-00"), true);
        
        // Edge cases
        assert_eq!(is_valid_cnh(""), false);
        assert_eq!(is_valid_cnh("00000000000"), false);
        assert_eq!(is_valid_cnh("11111111111"), false);
    }
}
