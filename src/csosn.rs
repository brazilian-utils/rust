/// CSOSN (Código de Situação da Operação no Simples Nacional) utilities.

/// The 10 CSOSN codes of the official table (Ajuste SINIEF 07/2005 / NF-e
/// MOC Tabela 5.3 Tabela B).
const VALID_CODES: &[&str] = &[
    "101", "102", "103", "201", "202", "203", "300", "400", "500", "900",
];

/// Validates if a CSOSN code is valid: one of the 10 codes of the official
/// table.
///
/// Accepts the bare 3 digits as a string, or a non-negative integer; a
/// CSOSN has no printed grouping, so a masked value is rejected.
///
/// # Arguments
///
/// * `value` - The CSOSN code to check.
///
/// # Returns
///
/// `true` if `value` is one of the 10 official CSOSN codes.
///
/// # Examples
///
/// ```
/// use brazilian_utils::csosn::is_valid;
///
/// assert!(is_valid("101"));
/// assert!(is_valid("900"));
/// assert!(!is_valid("999"));
/// assert!(!is_valid("1-01"));
/// ```
pub fn is_valid(value: &str) -> bool {
    VALID_CODES.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("101"));
        assert!(is_valid("102"));
        assert!(is_valid("103"));
        assert!(is_valid("201"));
        assert!(is_valid("202"));
        assert!(is_valid("203"));
        assert!(is_valid("300"));
        assert!(is_valid("400"));
        assert!(is_valid("500"));
        assert!(is_valid("900"));
        assert!(!is_valid("999"));
        assert!(!is_valid("10"));
        assert!(!is_valid("1-01"));
        assert!(!is_valid("abc"));
        assert!(!is_valid(""));
    }
}
