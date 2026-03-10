use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpf([u8; Self::SIZE]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseCpfError {
    WrongLength,
    NonNumeric,
    WrongChecksum,
}

impl Cpf {
    const SIZE: usize = 11;

    pub fn generate() -> Self {
        todo!()
    }

    pub fn compute_checksum(base: &[u8]) -> [u8; 2] {
        todo!()
    }

    fn remove_symbols(s: &str) -> String {
        s.chars().filter(|&c| c != '.' && c != '-').collect()
    }
}

impl FromStr for Cpf {
    type Err = ParseCpfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Self::remove_symbols(s.trim());
        if s.len() != Self::SIZE {
            return Err(ParseCpfError::WrongLength);
        }

        let mut digits = [0; Self::SIZE];
        for (c, d) in s.chars().zip(digits.iter_mut()) {
            *d = c.to_digit(10).ok_or(ParseCpfError::NonNumeric)? as u8;
        }

        Ok(Cpf(digits))
    }
}

impl Display for Cpf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CPF_LIST: [&str; 12] = [
        "11144477735",
        "111.444.777-35",
        "111-444-777-35",
        "111.444.777.35",
        "111444777-35",
        "  111444777-35  ",
        "40364478829",
        "52513127765",
        "52599927765",
        "55550207753",
        "82178537464",
        "96271845860",
    ];

    const WRONG_LENGTH_LIST: [&str; 3] = ["1", "1234567890", "123456789012"];

    const NON_DIGITS_LIST: [&str; 3] = ["b1144477735", "1234567890a", "12345!78901"];

    const WRONG_CHECKSUM_LIST: [&str; 3] = ["11144477705", "11144477732", "11111111215"];

    #[test]
    fn test_parse_valid() {
        for s in VALID_CPF_LIST {
            assert!(s.parse::<Cpf>().is_ok());
        }
    }

    #[test]
    fn test_parse_wrong_length() {
        for s in WRONG_LENGTH_LIST {
            assert_eq!(s.parse::<Cpf>(), Err(ParseCpfError::WrongLength));
        }
    }

    #[test]
    fn test_parse_non_digits() {
        for s in NON_DIGITS_LIST {
            assert_eq!(s.parse::<Cpf>(), Err(ParseCpfError::NonNumeric));
        }
    }

    #[test]
    fn test_parse_wrong_checksum() {
        for s in WRONG_CHECKSUM_LIST {
            assert_eq!(s.parse::<Cpf>(), Err(ParseCpfError::WrongChecksum));
        }
    }

    #[test]
    fn test_generate() {
        for _ in 0..1000 {
            let cpf = Cpf::generate();
            assert_eq!(cpf.0.len(), 11);
            assert!(cpf.0.iter().all(|x| (0..=9).contains(x)))
        }
    }

    #[test]
    fn test_compute_checksum() {
        for s in VALID_CPF_LIST {
            let cpf = s.parse::<Cpf>().unwrap();
            let split_idx = s.len() - 2;
            assert_eq!(
                Cpf::compute_checksum(&cpf.0[0..split_idx]),
                cpf.0[split_idx..11],
            );
        }
    }

    #[test]
    fn test_display() {
        for s in VALID_CPF_LIST {
            assert_eq!(
                Cpf::from_str(s).unwrap().to_string(),
                format!("{}.{}.{}-{}", &s[0..3], &s[3..6], &s[6..9], &s[9..11])
            )
        }
    }
}
