/// VIN (Vehicle Identification Number / chassi) utilities.

/// Transliterates a VIN character to its check-digit value, per the North
/// American (NHTSA) rule. Returns `None` for `I`, `O`, `Q` or any other
/// non-VIN character.
fn transliterate(c: char) -> Option<u32> {
    match c {
        '0'..='9' => c.to_digit(10),
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'F' => Some(6),
        'G' => Some(7),
        'H' => Some(8),
        'J' => Some(1),
        'K' => Some(2),
        'L' => Some(3),
        'M' => Some(4),
        'N' => Some(5),
        'P' => Some(7),
        'R' => Some(9),
        'S' => Some(2),
        'T' => Some(3),
        'U' => Some(4),
        'V' => Some(5),
        'W' => Some(6),
        'X' => Some(7),
        'Y' => Some(8),
        'Z' => Some(9),
        _ => None,
    }
}

const WEIGHTS: [u32; 17] = [8, 7, 6, 5, 4, 3, 2, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2];

/// Validates a VIN (chassi) structurally: 17 characters, none of the
/// excluded letters `I`, `O`, `Q`, and the check digit at position 9
/// (North American rule), case-insensitive.
///
/// A VIN whose characters are all the same is rejected even when the check
/// digit matches. Brazilian rules do not mandate the check digit, so many
/// Brazilian-built VINs fail it.
///
/// # Arguments
///
/// * `value` - The VIN to validate.
///
/// # Returns
///
/// `true` if `value` is a structurally valid, checksum-passing VIN.
///
/// # Examples
///
/// ```
/// use brazilian_utils::vin::is_valid;
///
/// assert!(is_valid("1HGCM82633A004352"));
/// assert!(is_valid("1m8gdm9axkp042788"));
/// assert!(!is_valid("1HGCM82633A004353"));
/// assert!(!is_valid("1HGCM8263IA004352")); // contains 'I'
/// ```
pub fn is_valid(value: &str) -> bool {
    if value.len() != 17 {
        return false;
    }

    let upper = value.to_uppercase();
    let chars: Vec<char> = upper.chars().collect();

    if chars.iter().any(|c| matches!(c, 'I' | 'O' | 'Q')) {
        return false;
    }

    if chars.iter().all(|c| *c == chars[0]) {
        return false;
    }

    let mut sum: u32 = 0;
    for (i, c) in chars.iter().enumerate() {
        match transliterate(*c) {
            Some(v) => sum += v * WEIGHTS[i],
            None => return false,
        }
    }

    let remainder = sum % 11;
    let expected_char = if remainder == 10 {
        'X'
    } else {
        char::from_digit(remainder, 10).unwrap()
    };

    chars[8] == expected_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("1HGCM82633A004352"));
        assert!(is_valid("1M8GDM9AXKP042788"));
        assert!(is_valid("1m8gdm9axkp042788"));
        assert!(!is_valid("1HGCM82633A004353"));
        assert!(!is_valid("1HGCM8263IA004352"));
        assert!(!is_valid("1HGCM82633A00435"));
        assert!(!is_valid("00000000000000000"));
        assert!(!is_valid(""));
    }
}
