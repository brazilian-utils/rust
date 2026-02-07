//! Brazilian RENAVAM (Registro Nacional de Veículos Automotores) utilities.
//!
//! This module provides functions to validate and generate Brazilian vehicle
//! registration numbers (RENAVAM).

use rand::Rng;

/// Weights used for RENAVAM check digit calculation.
/// Applied to the first 10 digits in reverse order.
const WEIGHTS: [u32; 10] = [2, 3, 4, 5, 6, 7, 8, 9, 2, 3];

/// Calculates the check digit for a RENAVAM base number.
///
/// # Arguments
///
/// * `base_renavam` - A string slice containing the first 10 digits of the RENAVAM
///
/// # Returns
///
/// The calculated check digit (0-9)
///
/// # Example
///
/// ```
/// use brazilian_utils::renavam::calculate_checksum;
///
/// let check_digit = calculate_checksum("8676959730");
/// assert_eq!(check_digit, 8);
/// ```
pub fn calculate_checksum(base_renavam: &str) -> u32 {
    if base_renavam.len() != 10 {
        return 0;
    }

    let digits: Vec<u32> = base_renavam
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 10 {
        return 0;
    }

    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(digit, weight)| digit * weight)
        .sum();

    let remainder = sum % 11;
    let check_digit = 11 - remainder;

    if check_digit >= 10 {
        0
    } else {
        check_digit
    }
}

/// Validates a Brazilian RENAVAM number.
///
/// A valid RENAVAM must:
/// - Contain exactly 11 digits
/// - Not have all digits the same
/// - Have a valid check digit (last digit)
///
/// # Arguments
///
/// * `renavam` - A string slice containing the RENAVAM to validate
///
/// # Returns
///
/// `true` if the RENAVAM is valid, `false` otherwise
///
/// # Example
///
/// ```
/// use brazilian_utils::renavam::is_valid;
///
/// assert_eq!(is_valid("86769597308"), true);
/// assert_eq!(is_valid("12345678901"), false);
/// assert_eq!(is_valid("11111111111"), false);
/// ```
pub fn is_valid(renavam: &str) -> bool {
    // Must be exactly 11 digits
    if renavam.len() != 11 {
        return false;
    }

    // Must contain only digits
    if !renavam.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Cannot have all digits the same
    let first_char = renavam.chars().next().unwrap();
    if renavam.chars().all(|c| c == first_char) {
        return false;
    }

    // Validate check digit
    let base = &renavam[..10];
    let check_digit = renavam.chars().nth(10).and_then(|c| c.to_digit(10));

    if let Some(digit) = check_digit {
        calculate_checksum(base) == digit
    } else {
        false
    }
}

/// Generates a valid random RENAVAM number.
///
/// # Returns
///
/// A String containing a valid 11-digit RENAVAM
///
/// # Example
///
/// ```
/// use brazilian_utils::renavam::{generate, is_valid};
///
/// let renavam = generate();
/// assert_eq!(renavam.len(), 11);
/// assert!(is_valid(&renavam));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    loop {
        // Generate 10 random digits
        let base: String = (0..10).map(|_| rng.gen_range(0..10).to_string()).collect();

        // Check if all digits are the same
        let first_char = base.chars().next().unwrap();
        if base.chars().all(|c| c == first_char) {
            continue;
        }

        // Calculate check digit
        let check_digit = calculate_checksum(&base);

        // Return complete RENAVAM
        return format!("{}{}", base, check_digit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        // Test cases based on known valid RENAVAM numbers
        assert_eq!(calculate_checksum("8676959730"), 8);
        assert_eq!(calculate_checksum("0123456789"), 7);
        assert_eq!(calculate_checksum("9876543210"), 3);
    }

    #[test]
    fn test_is_valid() {
        // Valid RENAVAM numbers
        assert!(is_valid("86769597308"));
        assert!(is_valid("01234567897"));
        assert!(is_valid("98765432103"));

        // Invalid: wrong check digit
        assert!(!is_valid("86769597309"));
        assert!(!is_valid("12345678901"));

        // Invalid: all digits the same
        assert!(!is_valid("11111111111"));
        assert!(!is_valid("00000000000"));

        // Invalid: wrong length
        assert!(!is_valid("123"));
        assert!(!is_valid("123456789012"));

        // Invalid: contains non-digits
        assert!(!is_valid("1234567890a"));
        assert!(!is_valid("12345678 01"));

        // Invalid: empty
        assert!(!is_valid(""));
    }

    #[test]
    fn test_generate() {
        // Generate should create valid RENAVAM
        let renavam = generate();
        assert_eq!(renavam.len(), 11);
        assert!(is_valid(&renavam));
    }

    #[test]
    fn test_generate_uniqueness() {
        // Generate multiple RENAVAM numbers and check they're different
        let mut renavams = std::collections::HashSet::new();
        for _ in 0..100 {
            let renavam = generate();
            assert!(is_valid(&renavam));
            renavams.insert(renavam);
        }
        // Should have generated at least 95 unique RENAVAM numbers
        assert!(renavams.len() >= 95);
    }

    #[test]
    fn test_checksum_invalid_input() {
        // Test with invalid input lengths
        assert_eq!(calculate_checksum("123"), 0);
        assert_eq!(calculate_checksum("12345678901"), 0);
        assert_eq!(calculate_checksum(""), 0);
    }
}
