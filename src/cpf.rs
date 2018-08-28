const SIZE: usize = 11;

const CHECK_DIGITS: [usize; 2] = [9, 10];

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

    true
}

fn is_blacklisted(input: &str) -> bool {
    BLACKLIST.contains(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates() {
        let valid_inputs = ["02287813020", "02915205027"];
        let invalid_inputs = ["00000000000", "11111111111"];

        for input in valid_inputs.iter() {
            assert_eq!(is_valid(input), true);
        }

        for input in invalid_inputs.iter() {
            assert_eq!(is_valid(input), false);
        }
    }
}
