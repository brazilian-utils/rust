/// Credit/debit card (payment card) utilities.

/// Validates a payment card number (credit or debit) using the Luhn
/// algorithm: 12 to 19 digits and a Luhn check digit.
///
/// No brand detection, issuer range lookup, expiry or CVV check is
/// performed. A number whose digits are all the same is rejected even
/// though it passes Luhn.
///
/// # Arguments
///
/// * `value` - The card number, with or without whitespace, `.`, `-` or `/`
///   between the digits.
///
/// # Returns
///
/// `true` if `value` is a structurally valid card number, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::credit_card::is_valid;
///
/// assert!(is_valid("4111111111111111"));
/// assert!(is_valid("4111 1111 1111 1111"));
/// assert!(!is_valid("4111111111111112"));
/// assert!(!is_valid("0000000000000000"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '-' && *c != '/')
        .collect();

    if cleaned.len() < 12 || cleaned.len() > 19 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if cleaned.chars().all(|c| c == cleaned.chars().next().unwrap()) {
        return false;
    }

    luhn_checksum(&cleaned) % 10 == 0
}

/// Computes the Luhn sum of a digit string (from the rightmost digit).
fn luhn_checksum(digits: &str) -> u32 {
    digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap_or(0);
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("4111111111111111"));
        assert!(is_valid("5555555555554444"));
        assert!(is_valid("378282246310005"));
        assert!(is_valid("4111 1111 1111 1111"));
        assert!(!is_valid("4111111111111112"));
        assert!(!is_valid("0000000000000000"));
        assert!(!is_valid("60110000000"));
        assert!(!is_valid("abcdabcdabcd"));
        assert!(!is_valid(""));
    }
}
