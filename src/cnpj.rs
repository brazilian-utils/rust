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
    dirty
        .chars()
        .filter(|c| *c != '.' && *c != '/' && *c != '-')
        .collect()
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
    if !is_valid(cnpj, None) {
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

/// Removes CNPJ formatting and returns the normalized value (digits only),
/// capped to 14 characters.
///
/// # Arguments
///
/// * `value` - A CNPJ string that may contain formatting symbols or other characters.
///
/// # Returns
///
/// A string with only the digits of `value`, capped to 14 characters.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cnpj::parse;
///
/// assert_eq!(parse("46.843.485/0001-86"), "46843485000186");
/// assert_eq!(parse("46843485000186123"), "46843485000186");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(14)
        .collect()
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

/// The 12-character alphanumeric weights (positions 0-11) used to compute
/// the first check digit of a v2 (IN RFB 2.119) CNPJ.
const ALNUM_WEIGHTS_1: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// The 13-character alphanumeric weights (positions 0-12, i.e. including the
/// just-computed first check digit) used to compute the second check digit
/// of a v2 (IN RFB 2.119) CNPJ.
const ALNUM_WEIGHTS_2: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// The alphanumeric alphabet used by the base of a v2 CNPJ.
const ALNUM_ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Value of a single alphanumeric CNPJ character: `ascii_code - 48`, so
/// `'0'..'9'` map to `0..9` and `'A'..'Z'` map to `17..42`. Returns `None` for
/// anything outside `0-9A-Z` (the caller must upper-case first).
fn alnum_char_value(c: char) -> Option<u32> {
    if c.is_ascii_digit() || ('A'..='Z').contains(&c) {
        Some(c as u32 - '0' as u32)
    } else {
        None
    }
}

/// Computes a modulus-11 check digit over `chars`, weighted by `weights`
/// (same rule as [`hashdigit`], generalized to alphanumeric characters).
fn alnum_check_digit(chars: &[char], weights: &[u32]) -> u32 {
    let sum: u32 = chars
        .iter()
        .zip(weights.iter())
        .map(|(c, w)| alnum_char_value(*c).unwrap_or(0) * w)
        .sum();
    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        11 - remainder
    }
}

/// Validates a 14-character CNPJ under the v2 (IN RFB 2.119) alphanumeric
/// format: the first 12 characters may be digits or uppercase letters, and
/// the 2 check digits (always numeric) are a modulus-11 checksum over the
/// preceding characters, treating each as `ascii_code - 48`.
fn is_valid_checksum_alnum(cnpj: &str) -> bool {
    if cnpj.len() != SIZE {
        return false;
    }

    let upper: Vec<char> = cnpj.chars().map(|c| c.to_ascii_uppercase()).collect();
    if !upper
        .iter()
        .all(|c| c.is_ascii_digit() || ('A'..='Z').contains(c))
    {
        return false;
    }

    // The check digits themselves are always numeric.
    if !upper[12].is_ascii_digit() || !upper[13].is_ascii_digit() {
        return false;
    }

    let dv1 = alnum_check_digit(&upper[0..12], &ALNUM_WEIGHTS_1);
    let dv2 = alnum_check_digit(&upper[0..13], &ALNUM_WEIGHTS_2);

    upper[12].to_digit(10) == Some(dv1) && upper[13].to_digit(10) == Some(dv2)
}

/// Returns whether or not the verifying checksum digits of the given CNPJ
/// match its base number.
///
/// This function does not verify the existence of the CNPJ; it only
/// validates the format of the string.
///
/// # Arguments
///
/// * `cnpj` - The CNPJ to be validated, a 14-character string.
/// * `version` - The CNPJ format: `1` (the default, numeric-only) or `2`
///   (the alphanumeric format of IN RFB 2.119, which additionally accepts
///   uppercase letters in the first 12 characters).
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
/// assert_eq!(is_valid("03560714000142", None), true);
/// assert_eq!(is_valid("00111222000133", None), false);
/// ```
pub fn is_valid(cnpj: &str, version: Option<u8>) -> bool {
    match version.unwrap_or(1) {
        2 => is_valid_checksum_alnum(cnpj),
        _ => validate(cnpj),
    }
}

/// Generates a random valid numeric (v1) CNPJ digit string.
///
/// An optional branch number parameter can be given; it defaults to 1.
fn generate_numeric(branch: Option<u32>) -> String {
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

/// Computes the 2 check digits for a 12-character alphanumeric CNPJ base
/// (v2, IN RFB 2.119).
fn compute_checksum_alnum(base12: &str) -> String {
    let mut chars: Vec<char> = base12.chars().map(|c| c.to_ascii_uppercase()).collect();
    let dv1 = alnum_check_digit(&chars, &ALNUM_WEIGHTS_1);
    chars.push(std::char::from_digit(dv1, 10).unwrap());
    let dv2 = alnum_check_digit(&chars, &ALNUM_WEIGHTS_2);
    format!("{}{}", dv1, dv2)
}

/// Generates a random valid alphanumeric (v2, IN RFB 2.119) CNPJ string: 8
/// random alphanumeric characters, followed by a 4-digit zero-padded branch
/// number (1-9999, random when `branch` is not given), followed by the 2
/// computed check digits.
fn generate_alnum(branch: Option<u32>) -> String {
    let mut rng = rand::thread_rng();

    let base8: String = (0..8)
        .map(|_| ALNUM_ALPHABET[rng.gen_range(0..ALNUM_ALPHABET.len())] as char)
        .collect();

    let mut branch_num = branch.unwrap_or_else(|| rng.gen_range(1..=9999));
    branch_num %= 10000;
    if branch_num == 0 {
        branch_num = 1;
    }
    let branch_str = format!("{:04}", branch_num);

    let base12 = format!("{}{}", base8, branch_str);
    let checksum = compute_checksum_alnum(&base12);
    format!("{}{}", base12, checksum)
}

/// Generates a random valid CNPJ digit string.
///
/// An optional branch number parameter can be given; it defaults to 1 for
/// the numeric (v1) format, or a random 1-9999 branch for the alphanumeric
/// (v2) format.
///
/// # Arguments
///
/// * `branch` - An optional branch number to be included in the CNPJ.
/// * `version` - The CNPJ format to generate: `1` (the default, numeric-only)
///   or `2` (the alphanumeric format of IN RFB 2.119).
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
/// let cnpj = generate(Some(1), None);
/// assert!(is_valid(&cnpj, None));
///
/// let cnpj2 = generate(None, None);
/// assert!(is_valid(&cnpj2, None));
///
/// let cnpj3 = generate(None, Some(2));
/// assert!(is_valid(&cnpj3, Some(2)));
/// ```
pub fn generate(branch: Option<u32>, version: Option<u8>) -> String {
    match version.unwrap_or(1) {
        2 => generate_alnum(branch),
        _ => generate_numeric(branch),
    }
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
        assert!(validate("34665388000161"));
        assert!(validate("03560714000142"));
        assert!(validate("01838723000127"));

        // Invalid CNPJs
        assert!(!validate("52599927000100"));
        assert!(!validate("00000000000"));
        assert!(!validate("00000000000000"));
        assert!(!validate("11111111111111"));
        assert!(!validate("00111222000133"));
    }

    #[test]
    fn test_is_valid() {
        // When CNPJ's len is different of 14, returns False
        assert!(!is_valid("1", None));

        // When CNPJ does not contain only digits, returns False
        assert!(!is_valid("1112223334445-", None));

        // When CNPJ has only the same digit, returns false
        assert!(!is_valid("11111111111111", None));

        // When rest_1 is lt 2 and the 13th digit is not 0, returns False
        assert!(!is_valid("1111111111315", None));

        // When rest_1 is gte 2 and the 13th digit is not (11 - rest), returns False
        assert!(!is_valid("1111111111115", None));

        // When rest_2 is lt 2 and the 14th digit is not 0, returns False
        assert!(!is_valid("11111111121205", None));

        // When rest_2 is gte 2 and the 14th digit is not (11 - rest), returns False
        assert!(!is_valid("11111111113105", None));

        // When CNPJ is valid
        assert!(is_valid("34665388000161", None));
        assert!(is_valid("01838723000127", None));
    }

    #[test]
    fn test_generate() {
        // Test that generate creates valid CNPJs
        for _ in 0..1000 {
            let cnpj = generate(None, None);
            assert!(is_valid(&cnpj, None));
            assert_eq!(cnpj.len(), 14);
        }

        // Test with specific branch numbers
        for branch in [1, 100, 1234, 9999] {
            let cnpj = generate(Some(branch), None);
            assert!(is_valid(&cnpj, None));
            assert_eq!(cnpj.len(), 14);
        }
    }

    #[test]
    fn test_v2_alnum_round_trip() {
        // A v2-generated CNPJ must validate as v2.
        for _ in 0..200 {
            let cnpj = generate(None, Some(2));
            assert_eq!(cnpj.len(), 14);
            assert!(is_valid(&cnpj, Some(2)));
        }

        // Specific branch numbers still work for v2.
        for branch in [1, 100, 1234, 9999] {
            let cnpj = generate(Some(branch), Some(2));
            assert!(is_valid(&cnpj, Some(2)));
            assert_eq!(&cnpj[8..12], format!("{:04}", branch));
        }
    }

    #[test]
    fn test_v1_rejects_alnum() {
        // The default (v1, numeric-only) path must keep rejecting
        // alphanumeric strings, even ones that are valid v2 CNPJs.
        let alnum_cnpj = generate(None, Some(2));
        assert!(!is_valid(&alnum_cnpj, None));
        assert!(!is_valid(&alnum_cnpj, Some(1)));
        assert!(!is_valid("12ABC34501DE35", None));
    }

    #[test]
    fn test_v2_checksum_matches_known_base() {
        // Base "12ABC34501DE" (12 chars). Check digits computed by hand
        // following the same rule as Go's `isValidChecksumAlnum` /
        // `GenerateChecksumAlnum` (ascii_code - 48 per character, weights
        // [5,4,3,2,9,8,7,6,5,4,3,2] then [6,5,4,3,2,9,8,7,6,5,4,3,2], mod 11):
        // dv1 = 3, dv2 = 5, so the full CNPJ is "12ABC34501DE35".
        let base = "12ABC34501DE";
        let checksum = compute_checksum_alnum(base);
        assert_eq!(checksum, "35");

        let cnpj = format!("{}{}", base, checksum);
        assert_eq!(cnpj, "12ABC34501DE35");
        assert!(is_valid_checksum_alnum(&cnpj));
        assert!(is_valid(&cnpj, Some(2)));
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
        assert!(!is_valid("", None));

        // Too short
        assert!(!is_valid("123456789012", None));

        // Too long
        assert!(!is_valid("123456789012345", None));

        // Contains letters
        assert!(!is_valid("1234567890123a", None));

        // All same digit
        assert!(!is_valid("00000000000000", None));
        assert!(!is_valid("99999999999999", None));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("46.843.485/0001-86"), "46843485000186");
        assert_eq!(parse("46843485000186"), "46843485000186");
        assert_eq!(parse("46.?ABC843.485/0001-86abc"), "46843485000186");
        assert_eq!(parse(""), "");
        assert_eq!(parse("46843485000186123"), "46843485000186");
    }

    #[test]
    fn test_generate_with_zero_branch() {
        // Branch 0 should become 1
        let cnpj = generate(Some(0), None);
        assert!(is_valid(&cnpj, None));
        // Branch should be "0001"
        assert_eq!(&cnpj[8..12], "0001");
    }

    #[test]
    fn test_generate_branch_modulo() {
        // Branch larger than 9999 should wrap around
        let cnpj = generate(Some(10000), None);
        assert!(is_valid(&cnpj, None));
        // Should wrap to 0, then become 1
        assert_eq!(&cnpj[8..12], "0001");
    }
}
