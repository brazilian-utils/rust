use std::{char, collections::HashSet};

use rand::random;

const ASCII_ZERO_CHAR_VALUE: u8 = 48;
const CPF_SIZE: usize = 11;
const CPF_CALCULATE_BASE_NUMBER: i32 = 11;
const CPF_FIRST_DIGIT_POSITION: usize = 10;
const CPF_SECOND_DIGIT_POSITION: usize = 11;


/// Verify if a given CPF (brazilian identification number) is a valid CPF
/// ```rust
/// #[test]
/// fn should_is_valid_cpf() {
///     let cpf = "00000000191";
///     assert!(is_valid(cpf));
/// }
/// 
/// #[test]
/// fn should_is_invalid_cpf() {
///     let cpf = "00000000091";
///     assert!(!is_valid(cpf));
/// }
/// ```
pub fn is_valid(input: &str) -> bool {
    if input.len() != CPF_SIZE || is_invalid_cpf(input) {
        return false;
    }
    is_valid_checksum(input)
}

/// Generate a valid CPF (brazilian identification number) number that
/// can be tested with is_valid method
/// ```rust
/// #[test]
/// fn should_is_valid_cpf() {
///     let cpf = generate_cpf();
///     assert!(is_valid(cpf));
/// }
/// ```
pub fn generate_cpf() -> String {
    let digits: Vec<u8> = (0..CPF_SIZE - 2)
        .map(|_| {
            let digit = random::<u32>() % 10;
            char::from_digit(digit, 10).unwrap() as u8
        })
        .collect();
    let digits_string = String::from_utf8(digits).unwrap();
    let validation_digits = calc_validation_digits(&digits_string);
    format!("{}{}", digits_string, validation_digits)
}

#[inline(always)]
fn is_invalid_cpf(input: &str) -> bool {
    let digits: HashSet<char> = input.chars().collect();
    digits.len() == 1
}

fn is_valid_checksum(input: &str) -> bool {
    input[9..] == calc_validation_digits(&input[..9])
}

fn calculate_digit(input: &str, digit_to_calc: usize) -> i32 {
    let digit_mod_calc = input[0..digit_to_calc - 1]
        .chars()
        .map(|ch| ch as i32 % ASCII_ZERO_CHAR_VALUE as i32)
        .fold(((digit_to_calc) as i32, 0), |prev: (i32, i32), act: i32| {
            let (mut multiply, mut val) = prev;
            val += act * multiply;
            multiply -= 1;
            (multiply, val)
        })
        .1
        % CPF_CALCULATE_BASE_NUMBER;
    let digit = CPF_CALCULATE_BASE_NUMBER - (digit_mod_calc);
    if digit >= 10 {
        0
    } else {
        digit
    }
}

fn calc_validation_digits(input: &str) -> String {
    let first_digit = (calculate_digit(input, CPF_FIRST_DIGIT_POSITION)) as u8;
    let second_digt =
        (calculate_digit(&format!("{input}{first_digit}"), CPF_SECOND_DIGIT_POSITION)) as u8;
    String::from_utf8(vec![
        first_digit + ASCII_ZERO_CHAR_VALUE,
        second_digt + ASCII_ZERO_CHAR_VALUE,
    ])
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_without_mask() {
        for input in ["96271845860", "40364478829", "00000000191"].iter() {
            assert!(is_valid(input), "expected '{input}' is a valid cpf");
        }
    }

    #[test]
    fn it_validates_invalid_list() {
        const INVALID_LIST: [&str; 12] = [
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
        for input in INVALID_LIST.iter() {
            assert!(!is_valid(input), "expected '{input}' is a invalid cpf");
        }
    }

    #[test]
    fn should_generate_a_valid_cpf() {
        let cpfs: Vec<String> = (0..1000).map(|_| generate_cpf()).collect();
        for input in cpfs {
            assert!(is_valid(&input), "expected cpf '{input}' is a valid cpf")
        }
    }
}
