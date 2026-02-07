/// PIS (Programa de Integração Social) utilities for Brazilian social integration numbers.

use rand::Rng;

/// Weights used for PIS checksum calculation.
const WEIGHTS: [u32; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// Remove formatting symbols from a PIS.
///
/// This function takes a PIS string with formatting symbols and returns
/// a cleaned version with no symbols.
///
/// # Arguments
///
/// * `pis` - A PIS string that may contain formatting symbols.
///
/// # Returns
///
/// A cleaned PIS string with no formatting symbols.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pis::remove_symbols;
///
/// assert_eq!(remove_symbols("123.456.789-00"), "12345678900");
/// assert_eq!(remove_symbols("987.654.321-03"), "98765432103");
/// assert_eq!(remove_symbols("12345678900"), "12345678900");
/// ```
pub fn remove_symbols(pis: &str) -> String {
    pis.replace(".", "").replace("-", "")
}

/// Calculate the checksum digit of the given base PIS string.
///
/// # Arguments
///
/// * `base_pis` - The first 10 digits of a PIS number as a string.
///
/// # Returns
///
/// The checksum digit (0-9).
pub fn checksum(base_pis: &str) -> u32 {
    let pis_digits: Vec<u32> = base_pis
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    let pis_sum: u32 = pis_digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(digit, weight)| digit * weight)
        .sum();
    
    let check_digit = 11 - (pis_sum % 11);
    
    if check_digit == 10 || check_digit == 11 {
        0
    } else {
        check_digit
    }
}

/// Returns whether or not the verifying checksum digit of the given PIS
/// matches its base number.
///
/// # Arguments
///
/// * `pis` - PIS number as a string of proper length.
///
/// # Returns
///
/// `true` if PIS is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pis::is_valid;
///
/// assert!(is_valid("12345678900"));
/// assert!(is_valid("98765432103"));
/// assert!(!is_valid("12345678901"));
/// assert!(!is_valid("123"));
/// ```
pub fn is_valid(pis: &str) -> bool {
    if pis.len() != 11 {
        return false;
    }
    
    if !pis.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    let expected_check_digit = checksum(&pis[..10]);
    let actual_check_digit = pis.chars().nth(10).and_then(|c| c.to_digit(10));
    
    match actual_check_digit {
        Some(digit) => digit == expected_check_digit,
        None => false,
    }
}

/// Format a valid PIS string with standard visual aid symbols.
///
/// This function takes a valid numbers-only PIS string as input
/// and adds standard formatting visual aid symbols for display.
///
/// # Arguments
///
/// * `pis` - A valid numbers-only PIS string.
///
/// # Returns
///
/// A formatted PIS string with standard visual aid symbols,
/// or `None` if the input is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pis::format_pis;
///
/// assert_eq!(format_pis("12345678900"), Some("123.45678.90-0".to_string()));
/// assert_eq!(format_pis("98765432103"), Some("987.65432.10-3".to_string()));
/// assert_eq!(format_pis("123"), None);
/// ```
pub fn format_pis(pis: &str) -> Option<String> {
    if !is_valid(pis) {
        return None;
    }
    
    Some(format!(
        "{}.{}.{}-{}",
        &pis[0..3],
        &pis[3..8],
        &pis[8..10],
        &pis[10..11]
    ))
}

/// Generate a random valid Brazilian PIS number.
///
/// This function generates a random PIS number with the following characteristics:
/// - It has 11 digits
/// - It passes the weight calculation check
///
/// # Returns
///
/// A randomly generated valid PIS number as a string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pis::generate;
///
/// let pis = generate();
/// assert_eq!(pis.len(), 11);
/// assert!(pis.chars().all(|c| c.is_ascii_digit()));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let base = format!("{:010}", rng.gen_range(0..10000000000u64));
    let check_digit = checksum(&base);
    
    format!("{}{}", base, check_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("123.456.789-09"), "12345678909");
        assert_eq!(remove_symbols("987.654.321-00"), "98765432100");
        assert_eq!(remove_symbols("12345678909"), "12345678909");
        assert_eq!(remove_symbols("170.24354.75-7"), "17024354757");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("1234567890"), 0);
        assert_eq!(checksum("9876543210"), 3);
        assert_eq!(checksum("1702435475"), 3);
        assert_eq!(checksum("8217853746"), 7);
        assert_eq!(checksum("5555020775"), 6);
    }

    #[test]
    fn test_is_valid() {
        // Valid PIS numbers (generated and verified)
        assert!(is_valid("12345678900"));
        assert!(is_valid("98765432103"));
        assert!(is_valid("17024354753"));
        assert!(is_valid("82178537467"));
        assert!(is_valid("55550207756"));
        
        // Invalid PIS numbers
        assert!(!is_valid("12345678901")); // Wrong check digit
        assert!(!is_valid("123")); // Too short
        assert!(!is_valid("123456789012")); // Too long
        assert!(!is_valid("1234567890a")); // Contains letter
        assert!(!is_valid("")); // Empty
    }

    #[test]
    fn test_format_pis() {
        assert_eq!(
            format_pis("12345678900"),
            Some("123.45678.90-0".to_string())
        );
        assert_eq!(
            format_pis("98765432103"),
            Some("987.65432.10-3".to_string())
        );
        assert_eq!(
            format_pis("17024354753"),
            Some("170.24354.75-3".to_string())
        );
        
        // Invalid inputs
        assert_eq!(format_pis("123"), None);
        assert_eq!(format_pis("12345678901"), None); // Wrong check digit
    }

    #[test]
    fn test_generate() {
        // Generate 10 PIS numbers and validate them
        for _ in 0..10 {
            let pis = generate();
            assert_eq!(pis.len(), 11);
            assert!(pis.chars().all(|c| c.is_ascii_digit()));
            assert!(is_valid(&pis));
        }
    }

    #[test]
    fn test_generate_uniqueness() {
        let pis1 = generate();
        let pis2 = generate();
        let pis3 = generate();
        
        // Very unlikely to generate the same PIS twice
        assert!(pis1 != pis2 || pis2 != pis3);
    }

    #[test]
    fn test_format_and_remove_symbols_roundtrip() {
        let pis = "12345678900";
        let formatted = format_pis(pis).unwrap();
        let cleaned = remove_symbols(&formatted);
        assert_eq!(cleaned, pis);
    }

    #[test]
    fn test_generated_pis_can_be_formatted() {
        let pis = generate();
        let formatted = format_pis(&pis);
        assert!(formatted.is_some());
        
        let cleaned = remove_symbols(&formatted.unwrap());
        assert_eq!(cleaned, pis);
    }
}
