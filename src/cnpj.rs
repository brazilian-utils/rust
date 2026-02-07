use rand::Rng;

const SIZE: usize = 14;

// FORMATTING
// ==========

/// Removes specific symbols from a CNPJ (Brazilian Company Registration Number) string.
///
/// This function takes a CNPJ string as input and removes all occurrences of
/// the '.', '/' and '-' characters from it.
///
/// # Arguments
///
/// * `dirty` - The CNPJ string containing symbols to be removed.
///
/// # Returns
///
/// A new string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::remove_symbols;
///
/// assert_eq!(remove_symbols("12.345.678/9012-34"), "12345678901234");
/// assert_eq!(remove_symbols("98.765.432/1098-76"), "98765432109876");
/// ```
pub fn remove_symbols(dirty: &str) -> String {
    dirty.chars().filter(|c| *c != '.' && *c != '/' && *c != '-').collect()
}

/// Formats a CNPJ (Brazilian Company Registration Number) string for visual display.
///
/// This function takes a CNPJ string as input, validates its format, and
/// formats it with standard visual aid symbols for display purposes.
///
/// # Arguments
///
/// * `cnpj` - The CNPJ string to be formatted for display.
///
/// # Returns
///
/// The formatted CNPJ with visual aid symbols if it's valid, None if it's not valid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::format_cnpj;
///
/// assert_eq!(format_cnpj("03560714000142"), Some("03.560.714/0001-42".to_string()));
/// assert_eq!(format_cnpj("98765432100100"), None);
/// ```
pub fn format_cnpj(cnpj: &str) -> Option<String> {
    if !is_valid(cnpj) {
        return None;
    }

    Some(format!(
        "{}.{}.{}/{}-{}",
        &cnpj[0..2],
        &cnpj[2..5],
        &cnpj[5..8],
        &cnpj[8..12],
        &cnpj[12..14]
    ))
}

// OPERATIONS
// ==========

/// Validates a CNPJ (Brazilian Company Registration Number) by comparing its
/// verifying checksum digits to its base number.
///
/// This function checks the validity of a CNPJ by comparing its verifying
/// checksum digits to its base number. The input should be a string of digits
/// with the appropriate length.
///
/// # Arguments
///
/// * `cnpj` - The CNPJ to be validated.
///
/// # Returns
///
/// `true` if the checksum digits match the base number, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::validate;
///
/// assert_eq!(validate("03560714000142"), true);
/// assert_eq!(validate("00111222000133"), false);
/// ```
pub fn validate(cnpj: &str) -> bool {
    if !cnpj.chars().all(|c| c.is_ascii_digit()) || cnpj.len() != SIZE {
        return false;
    }

    // Check if all digits are the same
    if cnpj.chars().all(|c| c == cnpj.chars().next().unwrap()) {
        return false;
    }

    // Validate both checksum digits
    let digit_13 = hashdigit(cnpj, 13);
    let digit_14 = hashdigit(cnpj, 14);

    cnpj.chars().nth(12).unwrap().to_digit(10).unwrap() == digit_13 as u32
        && cnpj.chars().nth(13).unwrap().to_digit(10).unwrap() == digit_14 as u32
}

/// Returns whether or not the verifying checksum digits of the given CNPJ
/// match its base number.
///
/// This function does not verify the existence of the CNPJ; it only
/// validates the format of the string.
///
/// # Arguments
///
/// * `cnpj` - The CNPJ to be validated, a 14-digit string.
///
/// # Returns
///
/// `true` if the checksum digits match the base number, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::is_valid;
///
/// assert_eq!(is_valid("03560714000142"), true);
/// assert_eq!(is_valid("00111222000133"), false);
/// ```
pub fn is_valid(cnpj: &str) -> bool {
    validate(cnpj)
}

/// Generates a random valid CNPJ digit string.
///
/// An optional branch number parameter can be given; it defaults to 1.
///
/// # Arguments
///
/// * `branch` - An optional branch number to be included in the CNPJ.
///
/// # Returns
///
/// A randomly generated valid CNPJ string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::{generate, is_valid};
///
/// let cnpj = generate(Some(1));
/// assert!(is_valid(&cnpj));
///
/// let cnpj2 = generate(None);
/// assert!(is_valid(&cnpj2));
/// ```
pub fn generate(branch: Option<u32>) -> String {
    let mut rng = rand::thread_rng();
    
    let mut branch_num = branch.unwrap_or(1);
    branch_num %= 10000;
    if branch_num == 0 {
        branch_num = 1;
    }
    
    let branch_str = format!("{:04}", branch_num);
    let base_num = format!("{:08}", rng.gen_range(0..=99999999));
    let base = format!("{}{}", base_num, branch_str);
    
    let checksum = compute_checksum(&base);
    format!("{}{}", base, checksum)
}

/// Calculates the checksum digit at the given position for the provided CNPJ.
///
/// The input must contain all elements before position.
///
/// # Arguments
///
/// * `cnpj` - The CNPJ for which the checksum digit is calculated.
/// * `position` - The position of the checksum digit to be calculated (13 or 14).
///
/// # Returns
///
/// The calculated checksum digit.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::hashdigit;
///
/// assert_eq!(hashdigit("12345678901234", 13), 3);
/// assert_eq!(hashdigit("00000000000000", 13), 0);
/// ```
pub fn hashdigit(cnpj: &str, position: usize) -> usize {
    let weights: Vec<usize> = if position == 13 {
        vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
    } else {
        vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]
    };

    let sum: usize = cnpj
        .chars()
        .take(position - 1)
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap() as usize * weights[i])
        .sum();

    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        11 - remainder
    }
}

/// Calculates the verifying checksum digits for a given CNPJ base number.
///
/// This function computes the verifying checksum digits for a provided CNPJ
/// base number. The `basenum` should be a digit-string of the appropriate length.
///
/// # Arguments
///
/// * `basenum` - The base number of the CNPJ for which verifying checksum digits are calculated.
///
/// # Returns
///
/// The verifying checksum digits as a string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::compute_checksum;
///
/// assert_eq!(compute_checksum("123456789012"), "30");
/// assert_eq!(compute_checksum("000000000000"), "00");
/// ```
pub fn compute_checksum(basenum: &str) -> String {
    let digit1 = hashdigit(basenum, 13);
    let with_digit1 = format!("{}{}", basenum, digit1);
    let digit2 = hashdigit(&with_digit1, 14);
    
    format!("{}{}", digit1, digit2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("00000000000"), "00000000000");
        assert_eq!(remove_symbols("12.345.678/0001-90"), "12345678000190");
        assert_eq!(remove_symbols("134..2435/.-1892.-"), "13424351892");
        assert_eq!(remove_symbols("abc1230916*!*&#"), "abc1230916*!*&#");
        assert_eq!(
            remove_symbols("ab.c1.--.2-3/09.-1-./6/-.*.-!*&#"),
            "abc1230916*!*&#"
        );
        assert_eq!(remove_symbols("/...---.../"), "");
    }

    #[test]
    fn test_format_cnpj() {
        // Valid CNPJs should be formatted
        assert_eq!(
            format_cnpj("03560714000142"),
            Some("03.560.714/0001-42".to_string())
        );
        assert_eq!(
            format_cnpj("01838723000127"),
            Some("01.838.723/0001-27".to_string())
        );
        assert_eq!(
            format_cnpj("34665388000161"),
            Some("34.665.388/0001-61".to_string())
        );

        // Invalid CNPJs should return None
        assert_eq!(format_cnpj("98765432100100"), None);
        assert_eq!(format_cnpj("00111222000133"), None);
        assert_eq!(format_cnpj("00000000000000"), None);
        assert_eq!(format_cnpj("12345"), None);
    }

    #[test]
    fn test_validate() {
        // Valid CNPJs
        assert_eq!(validate("34665388000161"), true);
        assert_eq!(validate("03560714000142"), true);
        assert_eq!(validate("01838723000127"), true);

        // Invalid CNPJs
        assert_eq!(validate("52599927000100"), false);
        assert_eq!(validate("00000000000"), false);
        assert_eq!(validate("00000000000000"), false);
        assert_eq!(validate("11111111111111"), false);
        assert_eq!(validate("00111222000133"), false);
    }

    #[test]
    fn test_is_valid() {
        // When CNPJ's len is different of 14, returns False
        assert_eq!(is_valid("1"), false);

        // When CNPJ does not contain only digits, returns False
        assert_eq!(is_valid("1112223334445-"), false);

        // When CNPJ has only the same digit, returns false
        assert_eq!(is_valid("11111111111111"), false);

        // When rest_1 is lt 2 and the 13th digit is not 0, returns False
        assert_eq!(is_valid("1111111111315"), false);

        // When rest_1 is gte 2 and the 13th digit is not (11 - rest), returns False
        assert_eq!(is_valid("1111111111115"), false);

        // When rest_2 is lt 2 and the 14th digit is not 0, returns False
        assert_eq!(is_valid("11111111121205"), false);

        // When rest_2 is gte 2 and the 14th digit is not (11 - rest), returns False
        assert_eq!(is_valid("11111111113105"), false);

        // When CNPJ is valid
        assert_eq!(is_valid("34665388000161"), true);
        assert_eq!(is_valid("01838723000127"), true);
    }

    #[test]
    fn test_generate() {
        // Test that generate creates valid CNPJs
        for _ in 0..1000 {
            let cnpj = generate(None);
            assert!(is_valid(&cnpj));
            assert_eq!(cnpj.len(), 14);
        }

        // Test with specific branch numbers
        for branch in [1, 100, 1234, 9999] {
            let cnpj = generate(Some(branch));
            assert!(is_valid(&cnpj));
            assert_eq!(cnpj.len(), 14);
        }
    }

    #[test]
    fn test_hashdigit() {
        assert_eq!(hashdigit("00000000000000", 13), 0);
        assert_eq!(hashdigit("00000000000000", 14), 0);
        assert_eq!(hashdigit("52513127000292", 13), 9);
        assert_eq!(hashdigit("52513127000292", 14), 9);
        assert_eq!(hashdigit("12345678901234", 13), 3);
    }

    #[test]
    fn test_compute_checksum() {
        assert_eq!(compute_checksum("000000000000"), "00");
        assert_eq!(compute_checksum("525131270002"), "99");
        assert_eq!(compute_checksum("123456789012"), "30");
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(is_valid(""), false);

        // Too short
        assert_eq!(is_valid("123456789012"), false);

        // Too long
        assert_eq!(is_valid("123456789012345"), false);

        // Contains letters
        assert_eq!(is_valid("1234567890123a"), false);

        // All same digit
        assert_eq!(is_valid("00000000000000"), false);
        assert_eq!(is_valid("99999999999999"), false);
    }

    #[test]
    fn test_generate_with_zero_branch() {
        // Branch 0 should become 1
        let cnpj = generate(Some(0));
        assert!(is_valid(&cnpj));
        // Branch should be "0001"
        assert_eq!(&cnpj[8..12], "0001");
    }

    #[test]
    fn test_generate_branch_modulo() {
        // Branch larger than 9999 should wrap around
        let cnpj = generate(Some(10000));
        assert!(is_valid(&cnpj));
        // Should wrap to 0, then become 1
        assert_eq!(&cnpj[8..12], "0001");
    }
}
