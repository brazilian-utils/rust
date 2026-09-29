/// Pix key (chave Pix) utilities: identifies and validates a Pix key
/// against the DICT (Diretório de Identificadores de Contas Transacionais)
/// key formats — CPF, CNPJ, email, mobile phone or a random EVP key.
use crate::{cnpj, cpf, email, phone};

/// Identified Pix key, as returned by [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct PixKeyInfo {
    /// One of `"cpf"`, `"cnpj"`, `"email"`, `"phone"` or `"evp"`.
    pub key_type: String,
    /// The canonical value: digits for a CPF/CNPJ, a lowercase email, an
    /// E.164 phone, or a lowercase UUID for an EVP key.
    pub value: String,
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    for (i, b) in bytes.iter().enumerate() {
        match i {
            8 | 13 | 18 | 23 => {
                if *b != b'-' {
                    return false;
                }
            }
            _ => {
                if !(*b as char).is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

/// Identifies a Pix key and normalizes it to the canonical form the DICT
/// expects inside a BR Code; returns `None` when the value is not a valid
/// Pix key.
///
/// An 11-digit value valid as both CPF and mobile phone is a CPF, unless
/// written as a phone (`+55` prefix or DDD in parentheses). Landlines are
/// not Pix keys; an email longer than 77 characters is rejected.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pix_key::get_info;
///
/// let info = get_info("123.456.789-09").unwrap();
/// assert_eq!(info.key_type, "cpf");
/// assert_eq!(info.value, "12345678909");
///
/// let info = get_info("+5561912345678").unwrap();
/// assert_eq!(info.key_type, "phone");
/// assert_eq!(info.value, "+5561912345678");
/// ```
pub fn get_info(value: &str) -> Option<PixKeyInfo> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let looks_like_phone_format = trimmed.contains('+') || trimmed.contains('(');

    if !looks_like_phone_format {
        let digits: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() == 11 && digits.chars().all(|c| c.is_ascii_digit()) && cpf::is_valid(&digits)
        {
            return Some(PixKeyInfo {
                key_type: "cpf".to_string(),
                value: digits,
            });
        }
    }

    let digits14: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits14.len() == 14 && cnpj::is_valid(&digits14, None) {
        return Some(PixKeyInfo {
            key_type: "cnpj".to_string(),
            value: digits14,
        });
    }

    if trimmed.contains('@') && trimmed.chars().count() <= 77 && email::is_valid(trimmed) {
        return Some(PixKeyInfo {
            key_type: "email".to_string(),
            value: trimmed.to_lowercase(),
        });
    }

    let phone_digits = phone::parse(trimmed);
    let has_letters = trimmed.chars().any(|c| c.is_ascii_alphabetic());
    if !has_letters && phone::is_valid_mobile(&phone_digits, None) {
        return Some(PixKeyInfo {
            key_type: "phone".to_string(),
            value: format!("+55{}", phone_digits),
        });
    }

    if is_uuid_like(trimmed) {
        return Some(PixKeyInfo {
            key_type: "evp".to_string(),
            value: trimmed.to_lowercase(),
        });
    }

    None
}

/// Options for [`is_valid`].
#[derive(Debug, Clone, Default)]
pub struct IsValidPixKeyOptions {
    /// Restricts the accepted key types (`"cpf"`, `"cnpj"`, `"email"`,
    /// `"phone"`, `"evp"`). An empty list rejects everything; omitted,
    /// every type is accepted.
    pub types: Option<Vec<String>>,
}

/// Checks whether a value is a valid Pix key: a CPF, a CNPJ, an email, a
/// Brazilian mobile phone or a random EVP key, per the DICT key formats.
///
/// # Examples
///
/// ```
/// use brazilian_utils::pix_key::is_valid;
///
/// assert!(is_valid("123.456.789-09", None));
/// assert!(!is_valid("fulano@example", None)); // no TLD
/// assert!(!is_valid("1130000000", None)); // landline, not a Pix key
/// ```
pub fn is_valid(value: &str, options: Option<IsValidPixKeyOptions>) -> bool {
    let info = match get_info(value) {
        Some(info) => info,
        None => return false,
    };

    match options.and_then(|o| o.types) {
        Some(types) => types.iter().any(|t| t == &info.key_type),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_info() {
        assert_eq!(
            get_info("123.456.789-09"),
            Some(PixKeyInfo {
                key_type: "cpf".to_string(),
                value: "12345678909".to_string()
            })
        );
        assert_eq!(
            get_info("00.038.166/0001-05"),
            Some(PixKeyInfo {
                key_type: "cnpj".to_string(),
                value: "00038166000105".to_string()
            })
        );
        assert_eq!(
            get_info("fulano_da_silva.recebedor@example.com"),
            Some(PixKeyInfo {
                key_type: "email".to_string(),
                value: "fulano_da_silva.recebedor@example.com".to_string()
            })
        );
        assert_eq!(
            get_info("+5561912345678"),
            Some(PixKeyInfo {
                key_type: "phone".to_string(),
                value: "+5561912345678".to_string()
            })
        );
        assert_eq!(
            get_info("71c7d9be-4b85-4e43-9f1c-1f3b8b4e9a2d"),
            Some(PixKeyInfo {
                key_type: "evp".to_string(),
                value: "71c7d9be-4b85-4e43-9f1c-1f3b8b4e9a2d".to_string()
            })
        );
        assert_eq!(get_info("11257245286"), None);
        assert_eq!(get_info(""), None);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("123.456.789-09", None));
        assert!(is_valid("00.038.166/0001-05", None));
        assert!(is_valid("fulano_da_silva.recebedor@example.com", None));
        assert!(is_valid("+5561912345678", None));
        assert!(is_valid("71c7d9be-4b85-4e43-9f1c-1f3b8b4e9a2d", None));
        assert!(!is_valid("11257245286", None));
        assert!(!is_valid("fulano@example", None));
        assert!(!is_valid("1130000000", None));
        assert!(!is_valid("", None));
    }

    #[test]
    fn test_is_valid_with_type_restriction() {
        let opts = IsValidPixKeyOptions {
            types: Some(vec!["cpf".to_string()]),
        };
        assert!(is_valid("123.456.789-09", Some(opts.clone())));
        assert!(!is_valid("+5561912345678", Some(opts)));

        let none_allowed = IsValidPixKeyOptions { types: Some(vec![]) };
        assert!(!is_valid("123.456.789-09", Some(none_allowed)));
    }
}
