/// Module for Brazilian Boleto (payment slip) utilities
/// 
/// This module provides validation functions for boleto digitable lines.
/// A boleto digitable line is a 47-digit numeric string that contains
/// payment information and verification digits.

/// Length of a valid boleto digitable line (47 digits)
const DIGITABLE_LINE_LENGTH: usize = 47;

/// Position of the check digit in the parsed barcode (mod 11)
const CHECK_DIGIT_MOD11_POSITION: usize = 4;

/// Weights used for mod 10 calculation (alternating 2 and 1)
const MOD10_WEIGHTS: [u32; 2] = [2, 1];

/// Configuration for mod 11 weights
const MOD11_WEIGHT_INITIAL: u32 = 2;
const MOD11_WEIGHT_END: u32 = 9;

/// Positions used to convert digitable line to barcode format
const DIGITABLE_LINE_TO_BOLETO_CONVERT_POSITIONS: [(usize, usize); 5] = [
    (0, 4),
    (32, 47),
    (4, 9),
    (10, 20),
    (21, 31),
];

/// Partials to verify using mod 10 algorithm
const PARTIALS_TO_VERIFY_MOD10: [(usize, usize, usize); 3] = [
    (0, 9, 9),      // start, end, digit_index
    (10, 20, 20),
    (21, 31, 31),
];

// HELPER FUNCTIONS
// ================

/// Removes all non-numeric characters from a string
/// 
/// # Arguments
/// 
/// * `input` - The input string
/// 
/// # Returns
/// 
/// A string containing only numeric characters
fn only_numbers(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Reverses a string
/// 
/// # Arguments
/// 
/// * `input` - The input string
/// 
/// # Returns
/// 
/// The reversed string
fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/// Validates if the length is correct
/// 
/// # Arguments
/// 
/// * `digitable_line` - The digitable line with only numbers
/// 
/// # Returns
/// 
/// `true` if length is 47, `false` otherwise
fn is_valid_length(digitable_line: &str) -> bool {
    digitable_line.len() == DIGITABLE_LINE_LENGTH
}

/// Calculate mod 10 check digit for a given partial string
/// 
/// The algorithm:
/// 1. Reverse the string
/// 2. Multiply each digit by alternating weights (2, 1, 2, 1, ...)
/// 3. If result > 9, add 1 + (result % 10), else add result
/// 4. Return (10 - (sum % 10)) if sum % 10 > 0, else 0
/// 
/// # Arguments
/// 
/// * `partial` - The partial string to calculate mod 10 for
/// 
/// # Returns
/// 
/// The calculated mod 10 digit
fn get_mod10(partial: &str) -> u32 {
    let partial_reversed = reverse(partial);
    let mut sum: u32 = 0;

    for (index, ch) in partial_reversed.chars().enumerate() {
        let digit = ch.to_digit(10).unwrap_or(0);
        let weight = MOD10_WEIGHTS[index % 2];
        let multiplier = digit * weight;

        if multiplier > 9 {
            sum += 1 + (multiplier % 10);
        } else {
            sum += multiplier;
        }
    }

    let mod10 = sum % 10;
    if mod10 > 0 {
        10 - mod10
    } else {
        0
    }
}

/// Validates the three partial sections using mod 10 algorithm
/// 
/// # Arguments
/// 
/// * `digitable_line` - The complete digitable line (47 digits)
/// 
/// # Returns
/// 
/// `true` if all three partials are valid, `false` otherwise
fn validate_digitable_line_partials(digitable_line: &str) -> bool {
    for &(start, end, digit_index) in &PARTIALS_TO_VERIFY_MOD10 {
        let partial = &digitable_line[start..end];
        let mod10 = get_mod10(partial);
        
        let digit_char = digitable_line.chars().nth(digit_index).unwrap();
        let digit = digit_char.to_digit(10).unwrap_or(0);

        if digit != mod10 {
            return false;
        }
    }
    true
}

/// Parse digitable line to barcode format by extracting specific positions
/// 
/// # Arguments
/// 
/// * `digitable_line` - The complete digitable line (47 digits)
/// 
/// # Returns
/// 
/// The parsed barcode string
fn parse_digitable_line(digitable_line: &str) -> String {
    let mut result = String::new();
    
    for &(start, end) in &DIGITABLE_LINE_TO_BOLETO_CONVERT_POSITIONS {
        result.push_str(&digitable_line[start..end]);
    }
    
    result
}

/// Calculate mod 11 check digit
/// 
/// The algorithm:
/// 1. Reverse the string
/// 2. Multiply each digit by weights cycling from 2 to 9
/// 3. Sum all results
/// 4. Calculate mod 11 of the sum
/// 5. If mod11 != 0 and != 1, return 11 - mod11, else return 1
/// 
/// # Arguments
/// 
/// * `value` - The value string to calculate mod 11 for
/// 
/// # Returns
/// 
/// The calculated mod 11 digit
fn get_mod11(value: &str) -> u32 {
    let mut weight = MOD11_WEIGHT_INITIAL;
    let mut sum: u32 = 0;
    let value_reversed = reverse(value);

    for ch in value_reversed.chars() {
        let digit = ch.to_digit(10).unwrap_or(0);
        sum += digit * weight;

        if weight < MOD11_WEIGHT_END {
            weight += 1;
        } else {
            weight = MOD11_WEIGHT_INITIAL;
        }
    }

    let mod11 = sum % 11;
    if mod11 != 0 && mod11 != 1 {
        11 - mod11
    } else {
        1
    }
}

/// Validates the mod 11 check digit
/// 
/// # Arguments
/// 
/// * `digitable_line` - The complete digitable line (47 digits)
/// 
/// # Returns
/// 
/// `true` if the mod 11 check digit is valid, `false` otherwise
fn validate_mod11_check_digit(digitable_line: &str) -> bool {
    let parsed = parse_digitable_line(digitable_line);
    
    // Concatenate everything except the check digit at position 4
    let value_without_check_digit = format!(
        "{}{}",
        &parsed[0..CHECK_DIGIT_MOD11_POSITION],
        &parsed[CHECK_DIGIT_MOD11_POSITION + 1..]
    );
    
    let mod11 = get_mod11(&value_without_check_digit);
    let check_digit_char = parsed.chars().nth(CHECK_DIGIT_MOD11_POSITION).unwrap();
    let check_digit = check_digit_char.to_digit(10).unwrap_or(0);

    check_digit == mod11
}

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

/// Removes boleto formatting and keeps only digits, capped to 47 digits (48
/// for a boleto de arrecadação, recognized by a leading `8`).
///
/// # Examples
///
/// ```
/// use brazilian_utils::boleto::parse;
///
/// assert_eq!(
///     parse("10491.44338 55119.000002 00000.000141 3 25230000093423"),
///     "10491443385511900000200000000141325230000093423"
/// );
/// ```
pub fn parse(value: &str) -> String {
    let digits = only_numbers(value);
    let cap = if digits.starts_with('8') { 48 } else { 47 };
    digits.chars().take(cap).collect()
}

/// Formats a boleto linha digitável with its printed mask.
///
/// The 47-digit cobrança bancária linha digitável is grouped as
/// `00000.00000 00000.000000 00000.000000 0 00000000000000`. A 48-digit
/// linha digitável starting with `8` (boleto de arrecadação) gets four
/// blocks of 11 digits, each followed by its check digit.
///
/// # Examples
///
/// ```
/// use brazilian_utils::boleto::format;
///
/// assert_eq!(
///     format("10491443385511900000200000000141325230000093423"),
///     "10491.44338 55119.000002 00000.000141 3 25230000093423"
/// );
/// assert_eq!(format("104914"), "10491.4");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.starts_with('8') && digits.len() > 44 {
        apply_grouped_mask(
            &digits,
            &[11, 1, 11, 1, 11, 1, 11, 1],
            &["-", " ", "-", " ", "-", " ", "-"],
        )
    } else {
        apply_grouped_mask(
            &digits,
            &[5, 5, 5, 6, 5, 6, 1, 14],
            &[".", " ", ".", " ", ".", " ", " "],
        )
    }
}

/// Generates a valid random Brazilian bank slip (boleto) number: a 47-digit
/// cobrança bancária linha digitável that passes [`is_valid`].
///
/// # Examples
///
/// ```
/// use brazilian_utils::boleto::{generate, is_valid};
///
/// let boleto = generate();
/// assert_eq!(boleto.len(), 47);
/// assert!(is_valid(&boleto));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let rand_digits = |rng: &mut rand::rngs::ThreadRng, n: usize| -> String {
        (0..n).map(|_| rng.gen_range(0..=9).to_string()).collect()
    };

    let field0_4 = rand_digits(&mut rng, 4);
    let field4_9 = rand_digits(&mut rng, 5);
    let part1_data = format!("{}{}", field0_4, field4_9);
    let check1 = get_mod10(&part1_data);

    let field10_20 = rand_digits(&mut rng, 10);
    let check2 = get_mod10(&field10_20);

    let field21_31 = rand_digits(&mut rng, 10);
    let check3 = get_mod10(&field21_31);

    let field33_47 = rand_digits(&mut rng, 14);

    let barcode_without_dv = format!(
        "{}{}{}{}{}",
        field0_4, field33_47, field4_9, field10_20, field21_31
    );
    let dv = get_mod11(&barcode_without_dv);

    format!(
        "{}{}{}{}{}{}{}{}{}",
        field0_4, field4_9, check1, field10_20, check2, field21_31, check3, dv, field33_47
    )
}

// PUBLIC API
// ==========

/// Validates if a given boleto digitable line is valid
/// 
/// A boleto digitable line is a 47-digit numeric identifier used in Brazilian
/// bank slips (boletos). This function validates:
/// - The length (must be 47 digits after removing non-numeric characters)
/// - Three mod 10 check digits for different sections
/// - One mod 11 check digit for the entire barcode
/// 
/// # Arguments
/// 
/// * `digitable_line` - The digitable line to validate (can contain spaces or dots)
/// 
/// # Returns
/// 
/// `true` if the digitable line is valid, `false` otherwise
/// 
/// # Examples
/// 
/// ```
/// use brazilian_utils::boleto::is_valid;
/// 
/// assert!(is_valid("00190000090114971860168524522114675860000102656"));
/// assert!(is_valid("0019000009 01149.718601 68524.522114 6 75860000102656"));
/// assert!(!is_valid("00190000020114971860168524522114675860000102656"));
/// assert!(!is_valid(""));
/// ```
pub fn is_valid(digitable_line: &str) -> bool {
    let digitable_line_numbers = only_numbers(digitable_line);

    if !is_valid_length(&digitable_line_numbers) {
        return false;
    }

    if !validate_digitable_line_partials(&digitable_line_numbers) {
        return false;
    }

    validate_mod11_check_digit(&digitable_line_numbers)
}

/// Alias for `is_valid` function for consistency with other modules
/// 
/// # Arguments
/// 
/// * `digitable_line` - The digitable line to validate
/// 
/// # Returns
/// 
/// `true` if the digitable line is valid, `false` otherwise
/// 
/// # Examples
/// 
/// ```
/// use brazilian_utils::boleto::validate;
/// 
/// assert!(validate("00190000090114971860168524522114675860000102656"));
/// assert!(!validate("00190000020114971860168524522114675860000102656"));
/// ```
pub fn validate(digitable_line: &str) -> bool {
    is_valid(digitable_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_numbers() {
        assert_eq!(only_numbers("123.456.789-01"), "12345678901");
        assert_eq!(only_numbers("0019000009 01149.718601"), "001900000901149718601");
        assert_eq!(only_numbers("abc123def456"), "123456");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("12345"), "54321");
        assert_eq!(reverse("abc"), "cba");
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_is_valid_length() {
        assert!(is_valid_length("00190000090114971860168524522114675860000102656"));
        assert!(!is_valid_length(""));
        assert!(!is_valid_length("000111"));
        assert!(!is_valid_length("0019000009011497186016852452211467586000010265600"));
    }

    #[test]
    fn test_get_mod10() {
        // Test cases derived from the validation algorithm
        // For "001900000", the expected mod 10 at position 9 should be 9
        let result = get_mod10("001900000");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_validate_digitable_line_partials() {
        // Valid boleto
        assert!(validate_digitable_line_partials(
            "00190000090114971860168524522114675860000102656"
        ));
        
        // Invalid first partial (digit at position 9)
        assert!(!validate_digitable_line_partials(
            "00190000020114971860168524522114675860000102656"
        ));
    }

    #[test]
    fn test_parse_digitable_line() {
        let parsed = parse_digitable_line("00190000090114971860168524522114675860000102656");
        // Expected: positions [0..4] + [32..47] + [4..9] + [10..20] + [21..31]
        // = "0019" + "675860000102656" + "00000" + "01149718601" + "68524522114"
        assert_eq!(parsed.len(), 44);
    }

    #[test]
    fn test_validate_mod11_check_digit() {
        // Valid boleto - should pass mod11 check
        assert!(validate_mod11_check_digit(
            "00190000090114971860168524522114675860000102656"
        ));
        
        // This boleto passes mod10 checks but fails mod11
        // (position 33 changed from 6 to 9, which is in the barcode check digit area)
        assert!(!validate_mod11_check_digit(
            "00190000090114971860168524522114975860000102656"
        ));
    }

    #[test]
    fn test_is_valid_empty_string() {
        assert!(!is_valid(""));
    }

    #[test]
    fn test_is_valid_short_string() {
        assert!(!is_valid("000111"));
    }

    #[test]
    fn test_is_valid_invalid_first_check_digit() {
        assert!(!is_valid("00190000020114971860168524522114675860000102656"));
    }

    #[test]
    fn test_is_valid_invalid_main_check_digit() {
        assert!(!is_valid("00190000090114971860168524522114975860000102656"));
    }

    #[test]
    fn test_is_valid_valid_boleto() {
        assert!(is_valid("00190000090114971860168524522114675860000102656"));
    }

    #[test]
    fn test_is_valid_with_formatting() {
        assert!(is_valid("0019000009 01149.718601 68524.522114 6 75860000102656"));
    }

    #[test]
    fn test_format() {
        assert_eq!(
            format("10491443385511900000200000000141325230000093423"),
            "10491.44338 55119.000002 00000.000141 3 25230000093423"
        );
        assert_eq!(format("104914"), "10491.4");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("10491.44338 55119.000002 00000.000141 3 25230000093423"),
            "10491443385511900000200000000141325230000093423"
        );
        assert_eq!(
            parse("84610000000-5 24610029110-2 00546033900-4 69589506108-0"),
            "846100000005246100291102005460339004695895061080"
        );
        assert_eq!(parse(""), "");
    }

    #[test]
    fn test_generate() {
        for _ in 0..20 {
            let boleto = generate();
            assert_eq!(boleto.len(), 47);
            assert!(is_valid(&boleto), "generated boleto should be valid: {}", boleto);
        }
    }

    #[test]
    fn test_validate_alias() {
        assert!(validate("00190000090114971860168524522114675860000102656"));
        assert!(!validate("00190000020114971860168524522114675860000102656"));
    }
}
