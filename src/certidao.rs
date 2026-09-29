/// Certidão (civil registry certificate) utilities.
///
/// Validates and parses the matrícula of a certidão de registro civil
/// (nascimento, casamento, óbito and the other acts kept by a serventia de
/// registro civil das pessoas naturais), per art. 473 of the Código
/// Nacional de Normas da Corregedoria Nacional de Justiça.

/// Applies a grouped mask to a digit string, as far as the digits go.
fn apply_grouped_mask(digits: &str, group_sizes: &[usize], separators: &[&str]) -> String {
    let mut result = String::new();
    let mut pos = 0;

    for (i, &size) in group_sizes.iter().enumerate() {
        if pos >= digits.len() {
            break;
        }
        let end = (pos + size).min(digits.len());
        result.push_str(&digits[pos..end]);
        pos = end;

        if pos >= digits.len() {
            break;
        }
        if i < separators.len() {
            result.push_str(separators[i]);
        }
    }

    result
}

const GROUP_SIZES: [usize; 9] = [6, 2, 2, 4, 1, 5, 3, 7, 2];

/// Calculates a CPF-style modulus-11 check digit over `digits` (weights
/// descending from `len + 1` to 2).
fn cpf_style_check_digit(digits: &str) -> u32 {
    let n = digits.len() as u32;
    let sum: u32 = digits
        .chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap_or(0) * (n + 1 - i as u32))
        .sum();
    let rem = sum % 11;
    if rem < 2 {
        0
    } else {
        11 - rem
    }
}

/// Formats a certidão matrícula into the printed mask of art. 473: the 32
/// digits grouped as 6 2 2 4 1 5 3 7 2, separated by spaces.
///
/// # Examples
///
/// ```
/// use brazilian_utils::certidao::format;
///
/// assert_eq!(
///     format("10453901552013100012021000012321"),
///     "104539 01 55 2013 1 00012 021 0000123 21"
/// );
/// assert_eq!(format("10453901"), "104539 01");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &GROUP_SIZES, &[" "; 8])
}

/// Book type of a certidão matrícula (the raw 1-9 code and its name).
fn book_type_name(code: u32) -> Option<&'static str> {
    match code {
        1 => Some("birth"),
        2 => Some("marriage"),
        3 => Some("religiousMarriage"),
        4 => Some("death"),
        5 => Some("stillbirth"),
        6 => Some("banns"),
        7 => Some("other"),
        8 => Some("emancipation"),
        9 => Some("interdiction"),
        _ => None,
    }
}

/// Parsed fields of a certidão matrícula, as returned by [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct CertidaoInfo {
    pub registry_cns: String,
    pub acervo: String,
    pub service: String,
    pub year: i32,
    pub book_type: &'static str,
    pub book_type_code: u32,
    pub book: String,
    pub page: String,
    pub term: String,
    pub check_digits: String,
}

fn clean_and_validate(value: &str) -> Option<String> {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '-' && *c != '/')
        .collect();

    if cleaned.len() != 32 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let service = &cleaned[8..10];
    if service != "55" {
        return None;
    }

    let book_type_code = cleaned[14..15].parse::<u32>().ok()?;
    book_type_name(book_type_code)?;

    let base30 = &cleaned[0..30];
    let dv1 = cpf_style_check_digit(base30);
    let with_dv1 = format!("{}{}", base30, dv1);
    let dv2 = cpf_style_check_digit(&with_dv1);
    let expected = format!("{}{}", dv1, dv2);

    if cleaned[30..32] != expected {
        return None;
    }

    Some(cleaned)
}

/// Validates the 32-digit matrícula of a certidão de registro civil.
///
/// # Examples
///
/// ```
/// use brazilian_utils::certidao::is_valid;
///
/// assert!(is_valid("10453901552013100012021000012321"));
/// assert!(is_valid("104539 01 55 2013 1 00012 021 0000123 21"));
/// assert!(!is_valid("10453901552013100012021000012322"));
/// ```
pub fn is_valid(value: &str) -> bool {
    clean_and_validate(value).is_some()
}

/// Parses the matrícula of a certidão de registro civil into its fields;
/// returns `None` when it is not valid (same rules as [`is_valid`]).
///
/// # Examples
///
/// ```
/// use brazilian_utils::certidao::get_info;
///
/// let info = get_info("104539 01 55 2013 1 00012 021 0000123 21").unwrap();
/// assert_eq!(info.registry_cns, "104539");
/// assert_eq!(info.book_type, "birth");
/// ```
pub fn get_info(value: &str) -> Option<CertidaoInfo> {
    let cleaned = clean_and_validate(value)?;

    let book_type_code: u32 = cleaned[14..15].parse().ok()?;
    let book_type = book_type_name(book_type_code)?;

    Some(CertidaoInfo {
        registry_cns: cleaned[0..6].to_string(),
        acervo: cleaned[6..8].to_string(),
        service: cleaned[8..10].to_string(),
        year: cleaned[10..14].parse().ok()?,
        book_type,
        book_type_code,
        book: cleaned[15..20].to_string(),
        page: cleaned[20..23].to_string(),
        term: cleaned[23..30].to_string(),
        check_digits: cleaned[30..32].to_string(),
    })
}

/// Removes the formatting of a certidão matrícula and keeps only digits,
/// capped to 32 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::certidao::parse;
///
/// assert_eq!(
///     parse("104539 01 55 2013 1 00012 021 0000123 21"),
///     "10453901552013100012021000012321"
/// );
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(
            format("10453901552013100012021000012321"),
            "104539 01 55 2013 1 00012 021 0000123 21"
        );
        assert_eq!(
            format("104539.01.55.2013.1.00012.021.0000123-21"),
            "104539 01 55 2013 1 00012 021 0000123 21"
        );
        assert_eq!(format("10453901"), "104539 01");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("10453901552013100012021000012321"));
        assert!(is_valid("104539 01 55 2013 1 00012 021 0000123 21"));
        assert!(is_valid("094003 01 55 2011 1 00110 002 0051917 43"));
        assert!(is_valid("104539/01/55/2013/1/00012/021/0000123/21"));
        assert!(!is_valid("10453901552013100012021000012322"));
        assert!(!is_valid("104539015520131000120210000123"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get_info() {
        let info = get_info("104539 01 55 2013 1 00012 021 0000123 21").unwrap();
        assert_eq!(info.registry_cns, "104539");
        assert_eq!(info.acervo, "01");
        assert_eq!(info.service, "55");
        assert_eq!(info.year, 2013);
        assert_eq!(info.book_type, "birth");
        assert_eq!(info.book_type_code, 1);
        assert_eq!(info.book, "00012");
        assert_eq!(info.page, "021");
        assert_eq!(info.term, "0000123");
        assert_eq!(info.check_digits, "21");

        let info2 = get_info("094300 01 55 2010 1 00020 112 0000120-87").unwrap();
        assert_eq!(info2.registry_cns, "094300");
        assert_eq!(info2.book, "00020");
        assert_eq!(info2.page, "112");
        assert_eq!(info2.term, "0000120");
        assert_eq!(info2.check_digits, "87");

        assert_eq!(get_info("10453901552013100012021000012322"), None);
        assert_eq!(get_info("not-a-matricula"), None);
        assert_eq!(get_info(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("104539 01 55 2013 1 00012 021 0000123 21"),
            "10453901552013100012021000012321"
        );
        assert_eq!(
            parse("104539.01.55.2013.1.00012.021.0000123-21abc"),
            "10453901552013100012021000012321"
        );
        assert_eq!(
            parse("10453901552013100012021000012321999"),
            "10453901552013100012021000012321"
        );
        assert_eq!(parse(""), "");
    }
}
