/// Email validation utilities.
use regex::Regex;
use std::sync::OnceLock;

/// Get the compiled email validation regex pattern.
///
/// This function returns a reference to a statically compiled regex pattern
/// that is initialized only once and reused for all subsequent calls.
fn email_regex() -> &'static Regex {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    EMAIL_REGEX.get_or_init(|| {
        // Note: Rust's regex crate doesn't support lookahead/lookbehind,
        // so we check for starting dot separately in the is_valid function
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

/// Check if a string corresponds to a valid email address.
///
/// This function validates an email address according to the rules generally
/// following the specifications defined by RFC 5322 (updated by RFC 5322bis),
/// which is the widely accepted standard for email address formats.
///
/// # Arguments
///
/// * `email` - The input string to be checked.
///
/// # Returns
///
/// Returns `true` if the email is a valid email address, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::email::is_valid;
///
/// assert_eq!(is_valid("brutils@brutils.com"), true);
/// assert_eq!(is_valid("user.name+tag@example.co.uk"), true);
/// assert_eq!(is_valid("invalid-email@brutils"), false);
/// assert_eq!(is_valid(".invalid@example.com"), false);
/// assert_eq!(is_valid("invalid@.example.com"), false);
/// assert_eq!(is_valid(""), false);
/// ```
pub fn is_valid(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }

    // Check if email starts with a dot (not allowed)
    if email.starts_with('.') {
        return false;
    }

    // Check for consecutive dots (not allowed)
    if email.contains("..") {
        return false;
    }

    // Check if there's an @ symbol
    if !email.contains('@') {
        return false;
    }

    // Split by @ to check domain separately
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local_part = parts[0];
    let domain = parts[1];

    // Check if local part ends with a dot (not allowed)
    if local_part.ends_with('.') {
        return false;
    }

    // Check if domain starts with a dot (not allowed)
    if domain.starts_with('.') {
        return false;
    }

    email_regex().is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_correct_emails() {
        assert!(is_valid("brutils@brutils.com"));
        assert!(is_valid("user@example.com"));
        assert!(is_valid("user.name@example.com"));
        assert!(is_valid("user+tag@example.com"));
        assert!(is_valid("user_name@example.com"));
        assert!(is_valid("user-name@example.com"));
        assert!(is_valid("user123@example.com"));
        assert!(is_valid("123user@example.com"));
        assert!(is_valid("user@sub.example.com"));
        assert!(is_valid("user@example.co.uk"));
        assert!(is_valid("user.name+tag@example.co.uk"));
    }

    #[test]
    fn test_is_valid_incorrect_emails() {
        // Empty string
        assert!(!is_valid(""));

        // Missing @
        assert!(!is_valid("userexample.com"));

        // Missing domain
        assert!(!is_valid("user@"));

        // Missing local part
        assert!(!is_valid("@example.com"));

        // Missing TLD
        assert!(!is_valid("user@example"));
        assert!(!is_valid("invalid-email@brutils"));

        // TLD too short (less than 2 characters)
        assert!(!is_valid("user@example.c"));

        // Starts with dot
        assert!(!is_valid(".user@example.com"));

        // Domain starts with dot
        assert!(!is_valid("user@.example.com"));

        // Multiple @
        assert!(!is_valid("user@@example.com"));
        assert!(!is_valid("user@exam@ple.com"));

        // Invalid characters
        assert!(!is_valid("user name@example.com")); // space
        assert!(!is_valid("user@exam ple.com")); // space in domain

        // Ends with dot
        assert!(!is_valid("user.@example.com"));

        // Domain ends with dot before TLD
        assert!(!is_valid("user@example.com."));
    }

    #[test]
    fn test_is_valid_edge_cases() {
        // Very long but valid
        assert!(is_valid("verylongemailaddressname@verylongdomainname.com"));

        // Multiple subdomains
        assert!(is_valid("user@mail.sub.example.com"));

        // Numbers in TLD (not allowed in this regex, TLD must be letters)
        assert!(!is_valid("user@example.123"));

        // Hyphen in domain
        assert!(is_valid("user@my-domain.com"));

        // Consecutive dots (technically invalid but let's test)
        assert!(!is_valid("user..name@example.com"));
    }

    #[test]
    fn test_is_valid_special_characters() {
        // Allowed special characters in local part
        assert!(is_valid("user%test@example.com"));
        assert!(is_valid("user+mailbox@example.com"));

        // Disallowed special characters
        assert!(!is_valid("user#test@example.com"));
        assert!(!is_valid("user@test@example.com"));
        assert!(!is_valid("user!test@example.com"));
    }
}
