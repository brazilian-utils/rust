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
    fn it_validates_without_mask() {
        for input in ["96271845860", "40364478829"].iter() {
            assert_eq!(is_valid(input), true);
        }

        for input in BLACKLIST.iter() {
            assert_eq!(is_valid(input), false);
        }
    }
}
