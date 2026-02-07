use std::char;

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

pub fn is_valid(input: &str) -> bool {
    if input.len() != SIZE || is_blacklisted(&input) {
        return false;
    }

    is_valid_checksum(&input)
}

fn is_blacklisted(input: &str) -> bool {
    BLACKLIST.contains(&input)
}

fn is_valid_checksum(input: &str) -> bool {
    [9, 10].iter().all(|&check| {
        let digits = &input[0..check];
        let mut weight = digits.len() + 1;
        let mut mod_val = digits.chars().fold(0, |acc, curr| -> usize {
            weight = weight - 1;
            acc + ((curr.to_digit(10).unwrap() as usize) * (weight + 1))
        }) % 11;

        mod_val = if mod_val < 2 { 0 } else { 11 - mod_val };
        let char_mod = char::from_digit(mod_val as u32, 10).unwrap();

        input.chars().nth(check).unwrap() == char_mod
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
