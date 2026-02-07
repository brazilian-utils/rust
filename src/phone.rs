/// Phone (telefone) utilities for Brazilian phone numbers.
///
/// Supports both mobile and landline phone numbers.

use rand::Rng;

/// Removes common symbols from a Brazilian phone number string.
///
/// # Arguments
///
/// * `phone_number` - The phone number to remove symbols from.
///
/// # Returns
///
/// A new string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::remove_symbols;
///
/// assert_eq!(remove_symbols("(11)99402-9275"), "11994029275");
/// assert_eq!(remove_symbols("+55 11 9 9402-9275"), "5511994029275");
/// assert_eq!(remove_symbols("16 3501-4415"), "1635014415");
/// ```
pub fn remove_symbols(phone_number: &str) -> String {
    phone_number
        .replace("(", "")
        .replace(")", "")
        .replace("-", "")
        .replace("+", "")
        .replace(" ", "")
}

/// Checks if a phone number string matches the mobile format.
///
/// Mobile format: [1-9][1-9][9]XXXXXXXX (11 digits)
/// - First 2 digits: DDD (area code) - both must be 1-9
/// - Third digit: Must be 9 (mobile identifier)
/// - Remaining 8 digits: Any digit 0-9
///
/// # Arguments
///
/// * `phone_number` - The phone number string to validate.
///
/// # Returns
///
/// `true` if the phone matches mobile format, `false` otherwise.
fn is_valid_mobile(phone_number: &str) -> bool {
    if phone_number.len() != 11 {
        return false;
    }
    
    let chars: Vec<char> = phone_number.chars().collect();
    
    // Check if all characters are digits
    if !chars.iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // First two digits (DDD) must be 1-9
    if chars[0] < '1' || chars[0] > '9' || chars[1] < '1' || chars[1] > '9' {
        return false;
    }
    
    // Third digit must be 9 (mobile identifier)
    if chars[2] != '9' {
        return false;
    }
    
    true
}

/// Checks if a phone number string matches the landline format.
///
/// Landline format: [1-9][1-9][2-5]XXXXXXX (10 digits)
/// - First 2 digits: DDD (area code) - both must be 1-9
/// - Third digit: Must be 2-5 (landline identifier)
/// - Remaining 7 digits: Any digit 0-9
///
/// # Arguments
///
/// * `phone_number` - The phone number string to validate.
///
/// # Returns
///
/// `true` if the phone matches landline format, `false` otherwise.
fn is_valid_landline(phone_number: &str) -> bool {
    if phone_number.len() != 10 {
        return false;
    }
    
    let chars: Vec<char> = phone_number.chars().collect();
    
    // Check if all characters are digits
    if !chars.iter().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // First two digits (DDD) must be 1-9
    if chars[0] < '1' || chars[0] > '9' || chars[1] < '1' || chars[1] > '9' {
        return false;
    }
    
    // Third digit must be 2-5 (landline identifier)
    if chars[2] < '2' || chars[2] > '5' {
        return false;
    }
    
    true
}

/// Returns if a Brazilian phone number is valid.
///
/// It does not verify if the number actually exists.
///
/// # Arguments
///
/// * `phone_number` - The phone number to validate. Only digits, without country code.
///                    It should include two digits DDD (area code).
/// * `phone_type` - Optional phone type: "mobile" or "landline".
///                  If not specified, checks for either format.
///
/// # Returns
///
/// `true` if the phone number is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::is_valid;
///
/// // Valid mobile
/// assert!(is_valid("11994029275", None));
/// assert!(is_valid("11994029275", Some("mobile")));
///
/// // Valid landline
/// assert!(is_valid("1635014415", None));
/// assert!(is_valid("1635014415", Some("landline")));
///
/// // Invalid
/// assert!(!is_valid("123", None));
/// assert!(!is_valid("11994029275", Some("landline")));
/// ```
pub fn is_valid(phone_number: &str, phone_type: Option<&str>) -> bool {
    match phone_type {
        Some("mobile") => is_valid_mobile(phone_number),
        Some("landline") => is_valid_landline(phone_number),
        _ => is_valid_mobile(phone_number) || is_valid_landline(phone_number),
    }
}

/// Function responsible for formatting a telephone number.
///
/// # Arguments
///
/// * `phone` - The phone number to format.
///
/// # Returns
///
/// The formatted phone number, or `None` if the number is not valid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::format_phone;
///
/// // Mobile
/// assert_eq!(format_phone("11994029275"), Some("(11)99402-9275".to_string()));
///
/// // Landline
/// assert_eq!(format_phone("1635014415"), Some("(16)3501-4415".to_string()));
///
/// // Invalid
/// assert_eq!(format_phone("333333"), None);
/// ```
pub fn format_phone(phone: &str) -> Option<String> {
    if !is_valid(phone, None) {
        return None;
    }
    
    let ddd = &phone[0..2];
    let phone_number = &phone[2..];
    let len = phone_number.len();
    
    Some(format!(
        "({}){}-{}",
        ddd,
        &phone_number[0..len - 4],
        &phone_number[len - 4..]
    ))
}

/// Generate a valid DDD (area code) number.
///
/// # Returns
///
/// A 2-digit DDD string where both digits are between 1-9.
fn generate_ddd_number() -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}", rng.gen_range(1..=9), rng.gen_range(1..=9))
}

/// Generate a valid and random mobile phone number.
///
/// # Returns
///
/// An 11-digit mobile phone number string.
fn generate_mobile_phone() -> String {
    let mut rng = rand::thread_rng();
    let ddd = generate_ddd_number();
    let client_number: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();
    
    format!("{}9{}", ddd, client_number)
}

/// Generate a valid and random landline phone number.
///
/// # Returns
///
/// A 10-digit landline phone number string.
fn generate_landline_phone() -> String {
    let mut rng = rand::thread_rng();
    let ddd = generate_ddd_number();
    let first_digit = rng.gen_range(2..=5);
    let remaining = format!("{:07}", rng.gen_range(0..=9999999));
    
    format!("{}{}{}", ddd, first_digit, remaining)
}

/// Generate a valid and random phone number.
///
/// # Arguments
///
/// * `phone_type` - Optional type: "landline" or "mobile".
///                  If not specified, generates either type randomly.
///
/// # Returns
///
/// A randomly generated valid phone number.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::generate;
///
/// // Generate any type
/// let phone = generate(None);
/// assert!(phone.len() == 10 || phone.len() == 11);
///
/// // Generate mobile
/// let mobile = generate(Some("mobile"));
/// assert_eq!(mobile.len(), 11);
///
/// // Generate landline
/// let landline = generate(Some("landline"));
/// assert_eq!(landline.len(), 10);
/// ```
pub fn generate(phone_type: Option<&str>) -> String {
    match phone_type {
        Some("mobile") => generate_mobile_phone(),
        Some("landline") => generate_landline_phone(),
        _ => {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                generate_mobile_phone()
            } else {
                generate_landline_phone()
            }
        }
    }
}

/// Function responsible for removing an international dialing code from a phone number.
///
/// # Arguments
///
/// * `phone_number` - The phone number with potential international code.
///
/// # Returns
///
/// The phone number without international code, or the same phone number if no code found.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::remove_international_dialing_code;
///
/// assert_eq!(remove_international_dialing_code("5511994029275"), "11994029275");
/// assert_eq!(remove_international_dialing_code("1635014415"), "1635014415");
/// assert_eq!(remove_international_dialing_code("+5511994029275"), "+11994029275");
/// ```
pub fn remove_international_dialing_code(phone_number: &str) -> String {
    let cleaned = phone_number.replace(" ", "");
    
    // Check if starts with +55 or 55 and has more than 11 digits
    if cleaned.len() > 11 {
        if cleaned.starts_with("+55") {
            return cleaned.replacen("55", "", 1);
        } else if cleaned.starts_with("55") {
            return cleaned.replacen("55", "", 1);
        }
    }
    
    phone_number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("(11)99402-9275"), "11994029275");
        assert_eq!(remove_symbols("+55 11 9 9402-9275"), "5511994029275");
        assert_eq!(remove_symbols("16 3501-4415"), "1635014415");
        assert_eq!(remove_symbols("1635014415"), "1635014415");
    }

    #[test]
    fn test_is_valid_mobile() {
        assert!(is_valid("11994029275", Some("mobile")));
        assert!(is_valid("21987654321", Some("mobile")));
        assert!(is_valid("85912345678", Some("mobile")));
        
        assert!(!is_valid("1635014415", Some("mobile")));
        assert!(!is_valid("11894029275", Some("mobile"))); // 8 instead of 9
        assert!(!is_valid("1194029275", Some("mobile"))); // Too short
        assert!(!is_valid("119940292751", Some("mobile"))); // Too long
    }

    #[test]
    fn test_is_valid_landline() {
        assert!(is_valid("1635014415", Some("landline")));
        assert!(is_valid("1133334444", Some("landline")));
        assert!(is_valid("8532221111", Some("landline")));
        
        assert!(!is_valid("11994029275", Some("landline")));
        assert!(!is_valid("1635014415", Some("mobile")));
        assert!(!is_valid("163501441", Some("landline"))); // Too short
        assert!(!is_valid("16350144151", Some("landline"))); // Too long
        assert!(!is_valid("1665014415", Some("landline"))); // 6 not in 2-5 range
    }

    #[test]
    fn test_is_valid_any_type() {
        assert!(is_valid("11994029275", None));
        assert!(is_valid("1635014415", None));
        assert!(is_valid("21987654321", None));
        assert!(is_valid("1133334444", None));
        
        assert!(!is_valid("123", None));
        assert!(!is_valid("11894029275", None));
        assert!(!is_valid("1665014415", None));
    }

    #[test]
    fn test_format_phone() {
        assert_eq!(format_phone("11994029275"), Some("(11)99402-9275".to_string()));
        assert_eq!(format_phone("1635014415"), Some("(16)3501-4415".to_string()));
        assert_eq!(format_phone("21987654321"), Some("(21)98765-4321".to_string()));
        
        assert_eq!(format_phone("333333"), None);
        assert_eq!(format_phone("123"), None);
    }

    #[test]
    fn test_generate_mobile() {
        let mobile = generate(Some("mobile"));
        assert_eq!(mobile.len(), 11);
        assert!(is_valid(&mobile, Some("mobile")));
    }

    #[test]
    fn test_generate_landline() {
        let landline = generate(Some("landline"));
        assert_eq!(landline.len(), 10);
        assert!(is_valid(&landline, Some("landline")));
    }

    #[test]
    fn test_generate_any_type() {
        let phone = generate(None);
        assert!(phone.len() == 10 || phone.len() == 11);
        assert!(is_valid(&phone, None));
    }

    #[test]
    fn test_generate_uniqueness() {
        let phone1 = generate(None);
        let phone2 = generate(None);
        let phone3 = generate(None);
        
        // Very unlikely to generate the same phone twice
        assert!(phone1 != phone2 || phone2 != phone3);
    }

    #[test]
    fn test_remove_international_dialing_code() {
        assert_eq!(remove_international_dialing_code("5511994029275"), "11994029275");
        assert_eq!(remove_international_dialing_code("551635014415"), "1635014415");
        assert_eq!(remove_international_dialing_code("+5511994029275"), "+11994029275");
        
        // Should not remove if length is 11 or less
        assert_eq!(remove_international_dialing_code("11994029275"), "11994029275");
        assert_eq!(remove_international_dialing_code("1635014415"), "1635014415");
    }

    #[test]
    fn test_remove_international_dialing_code_with_spaces() {
        assert_eq!(remove_international_dialing_code("55 11 99402 9275"), "11994029275");
    }
}
