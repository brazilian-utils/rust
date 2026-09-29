/// Brazilian passport number utilities.
use rand::Rng;

/// Removes non-alphanumeric characters from a passport number, upper-cases
/// it and caps it to 8 characters.
///
/// # Examples
///
/// ```
/// use brazilian_utils::passport::parse;
///
/// assert_eq!(parse("Ab123456"), "AB123456");
/// assert_eq!(parse(" AB 123 456 "), "AB123456");
/// assert_eq!(parse("AB123456789"), "AB123456");
/// ```
pub fn parse(passport: &str) -> String {
    passport
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .take(8)
        .collect()
}

/// Formats a passport number for display: the same operation as [`parse`].
///
/// # Examples
///
/// ```
/// use brazilian_utils::passport::format;
///
/// assert_eq!(format("AB123456"), "AB123456");
/// assert_eq!(format("acd12736"), "ACD12736");
/// assert_eq!(format("AB-123.456"), "AB123456");
/// ```
pub fn format(passport: &str) -> String {
    parse(passport)
}

/// Validates a Brazilian passport number: 2 letters followed by 6 digits,
/// after removing non-alphanumeric characters (case-insensitive).
///
/// There is no check digit, so a well-formed number is not necessarily a
/// real passport.
///
/// # Examples
///
/// ```
/// use brazilian_utils::passport::is_valid;
///
/// assert!(is_valid("AA111111"));
/// assert!(is_valid("ab123456"));
/// assert!(is_valid("AB-123456"));
/// assert!(!is_valid("1112223334-"));
/// ```
pub fn is_valid(passport: &str) -> bool {
    let cleaned = parse(passport);
    if cleaned.len() != 8 {
        return false;
    }
    let chars: Vec<char> = cleaned.chars().collect();
    chars[0..2].iter().all(|c| c.is_ascii_alphabetic())
        && chars[2..8].iter().all(|c| c.is_ascii_digit())
}

/// Generates a random valid Brazilian passport number: 2 uppercase letters
/// followed by 6 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::passport::{generate, is_valid};
///
/// let passport = generate();
/// assert_eq!(passport.len(), 8);
/// assert!(is_valid(&passport));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let letter1 = char::from_u32('A' as u32 + rng.gen_range(0..26)).unwrap();
    let letter2 = char::from_u32('A' as u32 + rng.gen_range(0..26)).unwrap();
    let digits: String = (0..6).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}{}{}", letter1, letter2, digits)
}

/// Removes the formatting symbols (keeps everything else).
///
/// # Examples
///
/// ```
/// use brazilian_utils::passport::remove_symbols;
///
/// assert_eq!(remove_symbols("AB-123.456"), "AB123456");
/// ```
pub fn remove_symbols(value: &str) -> String {
    value
        .chars()
        .filter(|c| !matches!(c, '.' | '-' | '/' | '(' | ')' | ',' | '*' | ' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("AB123456"), "AB123456");
        assert_eq!(format("acd12736"), "ACD12736");
        assert_eq!(format("AB-123.456"), "AB123456");
        assert_eq!(format("AB12"), "AB12");
        assert_eq!(format(""), "");
        assert_eq!(format("AB123456789"), "AB123456");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("AA111111"));
        assert!(is_valid("CL125167"));
        assert!(is_valid("ab123456"));
        assert!(is_valid("AB-123456"));
        assert!(is_valid("AB.123.456"));
        assert!(!is_valid("1"));
        assert!(!is_valid("1112223334-"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("Ab123456"), "AB123456");
        assert_eq!(parse(" AB 123 456 "), "AB123456");
        assert_eq!(parse("-AB1-23-4-56-"), "AB123456");
        assert_eq!(parse("AB123456789"), "AB123456");
        assert_eq!(parse(""), "");
    }

    #[test]
    fn test_generate() {
        for _ in 0..20 {
            let passport = generate();
            assert_eq!(passport.len(), 8);
            assert!(is_valid(&passport));
        }
    }
}
