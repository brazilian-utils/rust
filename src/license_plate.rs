/// License plate (placa de veículo) utilities for Brazilian vehicle plates.
///
/// Supports both old format (LLLNNNN) and Mercosul format (LLLNLNN).
use rand::Rng;

/// Removes the dash (-) symbol from a license plate string.
///
/// # Arguments
///
/// * `license_plate` - A license plate string that may contain a dash.
///
/// # Returns
///
/// The license plate with the dash removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::remove_symbols;
///
/// assert_eq!(remove_symbols("ABC-1234"), "ABC1234");
/// assert_eq!(remove_symbols("abc123"), "abc123");
/// assert_eq!(remove_symbols("ABCD123"), "ABCD123");
/// ```
pub fn remove_symbols(license_plate: &str) -> String {
    license_plate.replace("-", "")
}

/// Formats a license plate into the correct pattern.
///
/// This function receives a license plate in any pattern (LLLNNNN or LLLNLNN)
/// and returns a formatted version.
///
/// # Arguments
///
/// * `license_plate` - A license plate string.
///
/// # Returns
///
/// The formatted license plate string or `None` if the input is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::format_license_plate;
///
/// // Old format (with dash)
/// assert_eq!(format_license_plate("ABC1234"), Some("ABC-1234".to_string()));
///
/// // Mercosul format (uppercase, no dash)
/// assert_eq!(format_license_plate("abc1e34"), Some("ABC1E34".to_string()));
///
/// // Invalid
/// assert_eq!(format_license_plate("ABC123"), None);
/// ```
pub fn format_license_plate(license_plate: &str) -> Option<String> {
    let license_plate = license_plate.to_uppercase();

    if is_valid_old_format(&license_plate) {
        return Some(format!("{}-{}", &license_plate[0..3], &license_plate[3..]));
    } else if is_valid_mercosul(&license_plate) {
        return Some(license_plate);
    }

    None
}

/// Checks if a license plate string matches the old format (LLLNNNN).
///
/// # Arguments
///
/// * `license_plate` - The license plate string to validate.
///
/// # Returns
///
/// `true` if the plate matches the old format, `false` otherwise.
fn is_valid_old_format(license_plate: &str) -> bool {
    let plate = license_plate.trim().to_uppercase();

    if plate.len() != 7 {
        return false;
    }

    let chars: Vec<char> = plate.chars().collect();

    // First 3 must be letters
    if !chars[0..3].iter().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    // Last 4 must be digits
    chars[3..7].iter().all(|c| c.is_ascii_digit())
}

/// Checks if a license plate string matches the Mercosul format (LLLNLNN).
///
/// # Arguments
///
/// * `license_plate` - The license plate string to validate.
///
/// # Returns
///
/// `true` if the plate matches the Mercosul format, `false` otherwise.
fn is_valid_mercosul(license_plate: &str) -> bool {
    let plate = license_plate.trim().to_uppercase();

    if plate.len() != 7 {
        return false;
    }

    let chars: Vec<char> = plate.chars().collect();

    // Positions 0, 1, 2 must be letters
    if !chars[0..3].iter().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    // Position 3 must be digit
    if !chars[3].is_ascii_digit() {
        return false;
    }

    // Position 4 must be letter
    if !chars[4].is_ascii_alphabetic() {
        return false;
    }

    // Positions 5, 6 must be digits
    chars[5..7].iter().all(|c| c.is_ascii_digit())
}

/// Returns if a Brazilian license plate number is valid.
///
/// It does not verify if the plate actually exists.
///
/// # Arguments
///
/// * `license_plate` - The license plate number to be validated.
/// * `format` - Optional format type: "old_format" or "mercosul".
///   If not specified, checks for either format.
///
/// # Returns
///
/// `true` if the plate number is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::is_valid;
///
/// // Valid old format
/// assert!(is_valid("ABC1234", None));
/// assert!(is_valid("ABC1234", Some("old_format")));
///
/// // Valid Mercosul format
/// assert!(is_valid("ABC1D23", None));
/// assert!(is_valid("ABC1D23", Some("mercosul")));
///
/// // Invalid
/// assert!(!is_valid("ABC123", None));
/// assert!(!is_valid("ABC1D23", Some("old_format")));
/// ```
pub fn is_valid(license_plate: &str, format: Option<&str>) -> bool {
    match format {
        Some("old_format") => is_valid_old_format(license_plate),
        Some("mercosul") => is_valid_mercosul(license_plate),
        _ => is_valid_old_format(license_plate) || is_valid_mercosul(license_plate),
    }
}

/// Returns the format of a license plate.
///
/// # Arguments
///
/// * `license_plate` - A license plate string without symbols.
///
/// # Returns
///
/// The format of the license plate ("LLLNNNN" for old format, "LLLNLNN" for Mercosul)
/// or `None` if the format is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::get_format;
///
/// assert_eq!(get_format("ABC1234"), Some("LLLNNNN".to_string()));
/// assert_eq!(get_format("ABC1D23"), Some("LLLNLNN".to_string()));
/// assert_eq!(get_format("ABCD123"), None);
/// ```
pub fn get_format(license_plate: &str) -> Option<String> {
    if is_valid_old_format(license_plate) {
        return Some("LLLNNNN".to_string());
    }

    if is_valid_mercosul(license_plate) {
        return Some("LLLNLNN".to_string());
    }

    None
}

/// Converts an old pattern license plate (LLLNNNN) to Mercosul format (LLLNLNN).
///
/// The conversion works by converting the second digit (position 4) to a letter:
/// 0->A, 1->B, 2->C, 3->D, 4->E, 5->F, 6->G, 7->H, 8->I, 9->J
///
/// # Arguments
///
/// * `license_plate` - A string representing the old pattern license plate.
///
/// # Returns
///
/// The converted Mercosul license plate or `None` if the input is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::convert_to_mercosul;
///
/// assert_eq!(convert_to_mercosul("ABC1234"), Some("ABC1C34".to_string()));
/// assert_eq!(convert_to_mercosul("ABC4567"), Some("ABC4F67".to_string()));
/// assert_eq!(convert_to_mercosul("ABC0000"), Some("ABC0A00".to_string()));
/// assert_eq!(convert_to_mercosul("ABC4*67"), None);
/// ```
pub fn convert_to_mercosul(license_plate: &str) -> Option<String> {
    if !is_valid_old_format(license_plate) {
        return None;
    }

    let mut chars: Vec<char> = license_plate.to_uppercase().chars().collect();

    // Convert the 5th character (second digit, position 4) to a letter
    // 0->A, 1->B, ..., 9->J
    if let Some(digit) = chars[4].to_digit(10) {
        chars[4] = char::from_u32('A' as u32 + digit).unwrap();
    }

    Some(chars.into_iter().collect())
}

/// Generate a valid license plate in the given format.
///
/// In case no format is provided, it will return a license plate in the Mercosul format.
///
/// # Arguments
///
/// * `format` - The desired format for the license plate.
///   "LLLNNNN" for the old pattern or "LLLNLNN" for the Mercosul one.
///   Default is "LLLNLNN".
///
/// # Returns
///
/// A randomly generated license plate number or `None` if the format is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::license_plate::generate;
///
/// // Generate Mercosul format (default)
/// let plate = generate(None);
/// assert!(plate.is_some());
/// assert_eq!(plate.unwrap().len(), 7);
///
/// // Generate old format
/// let plate = generate(Some("LLLNNNN"));
/// assert!(plate.is_some());
///
/// // Invalid format
/// assert_eq!(generate(Some("invalid")), None);
/// ```
pub fn generate(format: Option<&str>) -> Option<String> {
    let format = format.unwrap_or("LLLNLNN").to_uppercase();

    if format != "LLLNLNN" && format != "LLLNNNN" {
        return None;
    }

    let mut rng = rand::thread_rng();
    let mut result = String::new();

    for ch in format.chars() {
        if ch == 'L' {
            // Random uppercase letter A-Z
            let letter = char::from_u32('A' as u32 + rng.gen_range(0..26)).unwrap();
            result.push(letter);
        } else if ch == 'N' {
            // Random digit 0-9
            let digit = rng.gen_range(0..10);
            result.push_str(&digit.to_string());
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("ABC-1234"), "ABC1234");
        assert_eq!(remove_symbols("abc123"), "abc123");
        assert_eq!(remove_symbols("ABCD123"), "ABCD123");
        assert_eq!(remove_symbols("ABC-1D23"), "ABC1D23");
    }

    #[test]
    fn test_format_license_plate_old() {
        assert_eq!(
            format_license_plate("ABC1234"),
            Some("ABC-1234".to_string())
        );
        assert_eq!(
            format_license_plate("abc1234"),
            Some("ABC-1234".to_string())
        );
        assert_eq!(
            format_license_plate("XYZ9876"),
            Some("XYZ-9876".to_string())
        );
    }

    #[test]
    fn test_format_license_plate_mercosul() {
        assert_eq!(format_license_plate("ABC1D23"), Some("ABC1D23".to_string()));
        assert_eq!(format_license_plate("abc1e34"), Some("ABC1E34".to_string()));
    }

    #[test]
    fn test_format_license_plate_invalid() {
        assert_eq!(format_license_plate("ABC123"), None);
        assert_eq!(format_license_plate("ABCD1234"), None);
        assert_eq!(format_license_plate("AB1234"), None);
        assert_eq!(format_license_plate(""), None);
    }

    #[test]
    fn test_is_valid_old_format() {
        assert!(is_valid("ABC1234", Some("old_format")));
        assert!(is_valid("XYZ9876", Some("old_format")));
        assert!(is_valid("abc1234", Some("old_format")));

        assert!(!is_valid("ABC1D23", Some("old_format")));
        assert!(!is_valid("ABC123", Some("old_format")));
        assert!(!is_valid("ABCD1234", Some("old_format")));
    }

    #[test]
    fn test_is_valid_mercosul() {
        assert!(is_valid("ABC1D23", Some("mercosul")));
        assert!(is_valid("XYZ9A99", Some("mercosul")));
        assert!(is_valid("abc1e34", Some("mercosul")));

        assert!(!is_valid("ABC1234", Some("mercosul")));
        assert!(!is_valid("ABC12D3", Some("mercosul")));
        assert!(!is_valid("ABCD123", Some("mercosul")));
    }

    #[test]
    fn test_is_valid_any_format() {
        assert!(is_valid("ABC1234", None));
        assert!(is_valid("ABC1D23", None));
        assert!(is_valid("xyz9876", None));
        assert!(is_valid("abc1e34", None));

        assert!(!is_valid("ABC123", None));
        assert!(!is_valid("ABCD1234", None));
        assert!(!is_valid("", None));
    }

    #[test]
    fn test_get_format() {
        assert_eq!(get_format("ABC1234"), Some("LLLNNNN".to_string()));
        assert_eq!(get_format("abc1234"), Some("LLLNNNN".to_string()));
        assert_eq!(get_format("ABC1D23"), Some("LLLNLNN".to_string()));
        assert_eq!(get_format("abc1e34"), Some("LLLNLNN".to_string()));

        assert_eq!(get_format("ABC123"), None);
        assert_eq!(get_format("ABCD1234"), None);
    }

    #[test]
    fn test_convert_to_mercosul() {
        assert_eq!(convert_to_mercosul("ABC1234"), Some("ABC1C34".to_string()));
        assert_eq!(convert_to_mercosul("ABC4567"), Some("ABC4F67".to_string()));
        assert_eq!(convert_to_mercosul("ABC0000"), Some("ABC0A00".to_string()));
        assert_eq!(convert_to_mercosul("ABC9999"), Some("ABC9J99".to_string()));
        assert_eq!(convert_to_mercosul("abc1234"), Some("ABC1C34".to_string()));
    }

    #[test]
    fn test_convert_to_mercosul_invalid() {
        assert_eq!(convert_to_mercosul("ABC4*67"), None);
        assert_eq!(convert_to_mercosul("ABC123"), None);
        assert_eq!(convert_to_mercosul("ABC1D23"), None);
        assert_eq!(convert_to_mercosul("ABCD1234"), None);
    }

    #[test]
    fn test_generate_mercosul() {
        let plate = generate(None);
        assert!(plate.is_some());
        let plate = plate.unwrap();
        assert_eq!(plate.len(), 7);
        assert!(is_valid(&plate, Some("mercosul")));
    }

    #[test]
    fn test_generate_old_format() {
        let plate = generate(Some("LLLNNNN"));
        assert!(plate.is_some());
        let plate = plate.unwrap();
        assert_eq!(plate.len(), 7);
        assert!(is_valid(&plate, Some("old_format")));
    }

    #[test]
    fn test_generate_invalid_format() {
        assert_eq!(generate(Some("invalid")), None);
        assert_eq!(generate(Some("LLLLNNN")), None);
        assert_eq!(generate(Some("")), None);
    }

    #[test]
    fn test_generate_uniqueness() {
        let plate1 = generate(None).unwrap();
        let plate2 = generate(None).unwrap();
        let plate3 = generate(None).unwrap();

        // Very unlikely to generate the same plate twice
        assert!(plate1 != plate2 || plate2 != plate3);
    }
}
