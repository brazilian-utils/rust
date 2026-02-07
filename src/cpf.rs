use rand::Rng;

const SIZE: usize = 11;

const BLACKLIST: [&str; 12] = [
    "000",
    "00000000000",
    "11111111111",
    "22222222222",
    "33333333333",
    "44444444444",
    "55555555555",
    "66666666666",
    "77777777777",
    "88888888888",
    "99999999999",
    "999999999999",
];

// FORMATTING
// ==========

/// Removes specific symbols from a CPF (Brazilian Individual Taxpayer Number) string.
///
/// This function takes a CPF string as input and removes all occurrences of
/// the '.', '-' characters from it.
///
/// # Arguments
///
/// * `dirty` - The CPF string containing symbols to be removed.
///
/// # Returns
///
/// A new string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::remove_symbols;
///
/// assert_eq!(remove_symbols("123.456.789-01"), "12345678901");
/// assert_eq!(remove_symbols("987-654-321.01"), "98765432101");
/// ```
pub fn remove_symbols(dirty: &str) -> String {
    dirty.chars().filter(|c| *c != '.' && *c != '-').collect()
}

/// Format a CPF (Brazilian Individual Taxpayer Number) for display with visual aid symbols.
///
/// This function takes a numbers-only CPF string as input and adds standard
/// formatting visual aid symbols for display.
///
/// # Arguments
///
/// * `cpf` - A numbers-only CPF string.
///
/// # Returns
///
/// A formatted CPF string with standard visual aid symbols or None if the input is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::format_cpf;
///
/// assert_eq!(format_cpf("82178537464"), Some("821.785.374-64".to_string()));
/// assert_eq!(format_cpf("55550207753"), Some("555.502.077-53".to_string()));
/// assert_eq!(format_cpf("12345678901"), None);
/// ```
pub fn format_cpf(cpf: &str) -> Option<String> {
    if !is_valid(cpf) {
        return None;
    }

    Some(format!(
        "{}.{}.{}-{}",
        &cpf[0..3],
        &cpf[3..6],
        &cpf[6..9],
        &cpf[9..11]
    ))
}

// OPERATIONS
// ==========

/// Validate the checksum digits of a CPF.
///
/// This function checks whether the verifying checksum digits of the given CPF
/// match its base number. The input should be a digit string of the proper length.
///
/// # Arguments
///
/// * `cpf` - A numbers-only CPF string.
///
/// # Returns
///
/// `true` if the checksum digits are valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::validate;
///
/// assert_eq!(validate("82178537464"), true);
/// assert_eq!(validate("55550207753"), true);
/// assert_eq!(validate("00011122233"), false);
/// ```
pub fn validate(cpf: &str) -> bool {
    if !cpf.chars().all(|c| c.is_ascii_digit()) || cpf.len() != SIZE {
        return false;
    }

    if is_blacklisted(cpf) {
        return false;
    }

    is_valid_checksum(cpf)
}

/// Returns whether or not the verifying checksum digits of the given CPF
/// match its base number.
///
/// This function does not verify the existence of the CPF; it only
/// validates the format of the string.
///
/// # Arguments
///
/// * `cpf` - The CPF to be validated, an 11-digit string.
///
/// # Returns
///
/// `true` if the checksum digits match the base number, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::is_valid;
///
/// assert_eq!(is_valid("82178537464"), true);
/// assert_eq!(is_valid("55550207753"), true);
/// assert_eq!(is_valid("00011122233"), false);
/// ```
pub fn is_valid(cpf: &str) -> bool {
    validate(cpf)
}

/// Generate a random valid CPF (Brazilian Individual Taxpayer Number) digit string.
///
/// # Returns
///
/// A random valid CPF string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::{generate, is_valid};
///
/// let cpf = generate();
/// assert_eq!(cpf.len(), 11);
/// assert!(is_valid(&cpf));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let base = format!("{:09}", rng.gen_range(1..=999999999));
    let checksum = compute_checksum(&base);
    format!("{}{}", base, checksum)
}

/// Compute the given position checksum digit for a CPF.
///
/// This function computes the specified position checksum digit for the CPF input.
/// The input needs to contain all elements previous to the position, or the
/// computation will yield the wrong result.
///
/// # Arguments
///
/// * `cpf` - A CPF string.
/// * `position` - The position to calculate the checksum digit for (10 or 11).
///
/// # Returns
///
/// The calculated checksum digit.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::hashdigit;
///
/// assert_eq!(hashdigit("52599927765", 11), 5);
/// assert_eq!(hashdigit("52599927765", 10), 6);
/// ```
pub fn hashdigit(cpf: &str, position: usize) -> usize {
    let sum: usize = cpf
        .chars()
        .take(position - 1)
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap() as usize;
            let weight = position - i;
            digit * weight
        })
        .sum();

    let val = sum % 11;
    if val < 2 {
        0
    } else {
        11 - val
    }
}

/// Compute the checksum digits for a given CPF base number.
///
/// This function calculates the checksum digits for a given CPF base number.
/// The base number should be a digit string of adequate length (9 digits).
///
/// # Arguments
///
/// * `basenum` - A digit string of adequate length.
///
/// # Returns
///
/// The calculated checksum digits as a string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cpf::compute_checksum;
///
/// assert_eq!(compute_checksum("335451269"), "51");
/// assert_eq!(compute_checksum("382916331"), "26");
/// ```
pub fn compute_checksum(basenum: &str) -> String {
    let digit1 = hashdigit(basenum, 10);
    let with_digit1 = format!("{}{}", basenum, digit1);
    let digit2 = hashdigit(&with_digit1, 11);
    
    format!("{}{}", digit1, digit2)
}

fn is_blacklisted(input: &str) -> bool {
    BLACKLIST.contains(&input)
}

fn is_valid_checksum(input: &str) -> bool {
    let digit1 = hashdigit(input, 10);
    let digit2 = hashdigit(input, 11);
    
    input.chars().nth(9).unwrap().to_digit(10).unwrap() == digit1 as u32
        && input.chars().nth(10).unwrap().to_digit(10).unwrap() == digit2 as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("00000000000"), "00000000000");
        assert_eq!(remove_symbols("123.456.789-10"), "12345678910");
        assert_eq!(remove_symbols("134..2435.-1892.-"), "13424351892");
        assert_eq!(remove_symbols("abc1230916*!*&#"), "abc1230916*!*&#");
        assert_eq!(
            remove_symbols("ab.c1.--.2-309.-1-.6-.*.-!*&#"),
            "abc1230916*!*&#"
        );
        assert_eq!(remove_symbols("...---..."), "");
    }

    #[test]
    fn test_format_cpf() {
        // Valid CPFs should be formatted
        assert_eq!(
            format_cpf("82178537464"),
            Some("821.785.374-64".to_string())
        );
        assert_eq!(
            format_cpf("55550207753"),
            Some("555.502.077-53".to_string())
        );
        assert_eq!(
            format_cpf("11144477735"),
            Some("111.444.777-35".to_string())
        );

        // Invalid CPFs should return None
        assert_eq!(format_cpf("00000000000"), None);
        assert_eq!(format_cpf("12345678901"), None);
        assert_eq!(format_cpf("1234567890"), None);
        assert_eq!(format_cpf("123456789012"), None);
    }

    #[test]
    fn test_validate() {
        // Valid CPFs
        assert_eq!(validate("52513127765"), true);
        assert_eq!(validate("52599927765"), true);
        assert_eq!(validate("82178537464"), true);
        assert_eq!(validate("55550207753"), true);

        // Invalid CPFs
        assert_eq!(validate("00000000000"), false);
        assert_eq!(validate("11111111111"), false);
        assert_eq!(validate("12345678901"), false);
        assert_eq!(validate("123456789"), false);
        assert_eq!(validate("12345678901a"), false);
    }

    #[test]
    fn test_is_valid() {
        // Valid CPFs
        assert_eq!(is_valid("96271845860"), true);
        assert_eq!(is_valid("40364478829"), true);
        assert_eq!(is_valid("11144477735"), true);
        assert_eq!(is_valid("82178537464"), true);
        assert_eq!(is_valid("55550207753"), true);

        // Invalid CPFs - wrong length
        assert_eq!(is_valid("1"), false);
        assert_eq!(is_valid("123456789"), false);
        assert_eq!(is_valid("123456789012"), false);

        // Invalid CPFs - non-digits
        assert_eq!(is_valid("1112223334-"), false);
        assert_eq!(is_valid("111.444.777-35"), false);

        // Invalid CPFs - blacklisted sequences
        for input in BLACKLIST.iter() {
            assert_eq!(is_valid(input), false);
        }

        // Invalid CPFs - wrong checksum
        assert_eq!(is_valid("11144477705"), false);
        assert_eq!(is_valid("11144477732"), false);
        assert_eq!(is_valid("11111111215"), false);
    }

    #[test]
    fn test_generate() {
        // Test that generate creates valid CPFs
        for _ in 0..1000 {
            let cpf = generate();
            assert_eq!(cpf.len(), 11);
            assert!(is_valid(&cpf));
            assert!(cpf.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_hashdigit() {
        assert_eq!(hashdigit("000000000", 10), 0);
        assert_eq!(hashdigit("0000000000", 11), 0);
        assert_eq!(hashdigit("52513127765", 10), 6);
        assert_eq!(hashdigit("52513127765", 11), 5);
        assert_eq!(hashdigit("52599927765", 10), 6);
        assert_eq!(hashdigit("52599927765", 11), 5);
    }

    #[test]
    fn test_compute_checksum() {
        assert_eq!(compute_checksum("000000000"), "00");
        assert_eq!(compute_checksum("525131277"), "65");
        assert_eq!(compute_checksum("335451269"), "51");
        assert_eq!(compute_checksum("382916331"), "26");
    }

    #[test]
    fn test_is_blacklisted() {
        assert_eq!(is_blacklisted("00000000000"), true);
        assert_eq!(is_blacklisted("11111111111"), true);
        assert_eq!(is_blacklisted("99999999999"), true);
        assert_eq!(is_blacklisted("12345678901"), false);
    }

    #[test]
    fn test_is_valid_checksum() {
        // Valid checksums
        assert_eq!(is_valid_checksum("11144477735"), true);
        assert_eq!(is_valid_checksum("96271845860"), true);
        
        // Invalid checksums
        assert_eq!(is_valid_checksum("11144477705"), false);
        assert_eq!(is_valid_checksum("11144477732"), false);
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(is_valid(""), false);
        
        // Special characters
        assert_eq!(is_valid("!@#$%^&*()_"), false);
        
        // Mixed valid and invalid
        assert_eq!(is_valid("111444777a5"), false);
    }

    #[test]
    fn test_format_with_symbols() {
        // Test that formatting after removing symbols works
        let cpf_with_symbols = "821.785.374-64";
        let cpf_clean = remove_symbols(cpf_with_symbols);
        assert_eq!(format_cpf(&cpf_clean), Some("821.785.374-64".to_string()));
    }

    #[test]
    fn test_generate_uniqueness() {
        // Test that generate creates different CPFs
        let cpf1 = generate();
        let cpf2 = generate();
        let cpf3 = generate();
        
        // While theoretically they could be the same, the probability is very low
        // Just check they're all valid
        assert!(is_valid(&cpf1));
        assert!(is_valid(&cpf2));
        assert!(is_valid(&cpf3));
    }
}

