/// CAEPF (Cadastro de Atividade Econômica da Pessoa Física) utilities.

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

/// Calculates a CNPJ-style modulus-11 check digit over `base`, using `weights`.
fn hashdigit(base: &str, weights: &[u32]) -> u32 {
    let sum: u32 = base
        .chars()
        .zip(weights.iter())
        .map(|(c, w)| c.to_digit(10).unwrap_or(0) * w)
        .sum();
    let rem = sum % 11;
    if rem < 2 {
        0
    } else {
        11 - rem
    }
}

/// Computes the 2-digit CAEPF check-digit pair for a 12-character base (the
/// 9-digit CPF base plus the 3-digit sequence).
///
/// Both digits follow the CNPJ modulus-11 algorithm; the resulting pair is
/// then shifted by 12, wrapping around 100.
fn check_digits(base12: &str) -> u32 {
    const WEIGHTS_13: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d13 = hashdigit(base12, &WEIGHTS_13);

    const WEIGHTS_14: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let with_d13 = format!("{}{}", base12, d13);
    let d14 = hashdigit(&with_d13, &WEIGHTS_14);

    let pair = d13 * 10 + d14;
    (pair + 12) % 100
}

/// Formats a CAEPF number with the mask `000.000.000/000-00`.
///
/// The mask is applied as far as the digits go.
///
/// # Examples
///
/// ```
/// use brazilian_utils::caepf::format;
///
/// assert_eq!(format("29311861000184"), "293.118.610/001-84");
/// assert_eq!(format("2931"), "293.1");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    apply_grouped_mask(&digits, &[3, 3, 3, 3, 2], &[".", ".", "/", "-"])
}

/// Validates a CAEPF number: 14 digits, the 9-digit CPF base of the holder,
/// a 3-digit sequence and 2 check digits.
///
/// A base whose 9 CPF digits are all the same is rejected.
///
/// # Examples
///
/// ```
/// use brazilian_utils::caepf::is_valid;
///
/// assert!(is_valid("293.118.610/001-84"));
/// assert!(is_valid("29311861000184"));
/// assert!(!is_valid("29311861000185"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '.' && *c != '/' && *c != '-')
        .collect();

    if cleaned.len() != 14 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let cpf_base = &cleaned[0..9];
    if cpf_base.chars().all(|c| c == cpf_base.chars().next().unwrap()) {
        return false;
    }

    let base12 = &cleaned[0..12];
    let expected = check_digits(base12);
    let actual: Option<u32> = cleaned[12..14].parse().ok();

    actual == Some(expected)
}

/// Removes CAEPF formatting and keeps only digits, capped to 14 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::caepf::parse;
///
/// assert_eq!(parse("293.118.610/001-84"), "29311861000184");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(14)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format("29311861000184"), "293.118.610/001-84");
        assert_eq!(format("293.118.610/001-84"), "293.118.610/001-84");
        assert_eq!(format("2931"), "293.1");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("293.118.610/001-84"));
        assert!(is_valid("29311861000184"));
        assert!(is_valid("41142260000101"));
        assert!(!is_valid("29311861000185"));
        assert!(!is_valid("00000000000000"));
        assert!(!is_valid("1234567890"));
        assert!(!is_valid("abc.118.610/001-84"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("293.118.610/001-84"), "29311861000184");
        assert_eq!(parse("29311861000184"), "29311861000184");
        assert_eq!(parse("293.?ABC118.610/001-84abc"), "29311861000184");
        assert_eq!(parse(""), "");
        assert_eq!(parse("29311861000184999"), "29311861000184");
    }
}
