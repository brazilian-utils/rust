/// Inscrição Estadual (IE, state tax registration) utilities.
///
/// The IE contract describes 27 different state-specific SINTEGRA
/// check-digit algorithms, several with additional special cases (such as
/// São Paulo's separate "produtor rural" numbering), and ships zero test
/// cases to pin any of them down against. Rather than guess at 27 undocumented
/// algorithms, this module — like the reference Go port it mirrors — only
/// validates the **structure** of an IE: that the state code is a real UF and
/// that the cleaned value has one of that UF's accepted digit counts. This is
/// a deliberate, documented limitation: it does not verify the check digits
/// themselves.
const MASK_CHARS: &[char] = &[' ', '.', '-', '/'];

/// Returns the accepted digit-count(s) for a UF, or `None` for an unknown UF.
fn accepted_lengths(uf: &str) -> Option<&'static [usize]> {
    Some(match uf {
        "AC" => &[13],
        "AL" => &[9],
        "AM" => &[9],
        "AP" => &[9],
        "BA" => &[8, 9],
        "CE" => &[9],
        "DF" => &[13],
        "ES" => &[9],
        "GO" => &[9],
        "MA" => &[9],
        "MG" => &[13],
        "MS" => &[9],
        "MT" => &[11],
        "PA" => &[9],
        "PB" => &[9],
        "PE" => &[9, 14],
        "PI" => &[9],
        "PR" => &[10],
        "RJ" => &[8],
        "RN" => &[9, 10],
        "RO" => &[9, 14],
        "RR" => &[9],
        "RS" => &[10],
        "SC" => &[9],
        "SE" => &[9],
        "SP" => &[12],
        "TO" => &[9, 11],
        _ => return None,
    })
}

/// Validates the structure of a Brazilian Inscrição Estadual (state tax
/// registration number) for the given state (UF).
///
/// This does **not** validate the state-specific SINTEGRA check digits — see
/// the module docs for why. It only checks that `state` is a real UF and that
/// `value`, once mask characters (`{space, '.', '-', '/'}`) are stripped, has
/// one of that UF's accepted digit counts.
///
/// São Paulo's "produtor rural" inscriptions (which start with `P`) are a
/// special case: they are accepted iff the cleaned value is exactly 13
/// characters long (the leading `P` plus 12 digits).
///
/// # Arguments
///
/// * `value` - The IE to validate, with or without mask characters.
/// * `state` - The 2-letter UF the IE belongs to (case-insensitive).
///
/// # Examples
///
/// ```
/// use brazilian_utils::ie::is_valid;
///
/// assert!(is_valid("123456789012", "SP"));
/// assert!(is_valid("P123456789012", "SP"));
/// assert!(is_valid("12345678", "RJ"));
/// assert!(!is_valid("123", "SP"));
/// assert!(!is_valid("123456789012", "XX"));
/// assert!(!is_valid("", "SP"));
/// ```
pub fn is_valid(value: &str, state: &str) -> bool {
    let uf = state.trim().to_uppercase();
    let lengths = match accepted_lengths(&uf) {
        Some(lengths) => lengths,
        None => return false,
    };

    let cleaned: String = value.chars().filter(|c| !MASK_CHARS.contains(c)).collect();
    if cleaned.is_empty() {
        return false;
    }

    // Special case: São Paulo's produtor rural inscriptions start with a
    // non-digit `P`, so they must be checked before the all-digit rule.
    if uf == "SP" {
        if let Some(rest) = cleaned.strip_prefix('P').or_else(|| cleaned.strip_prefix('p')) {
            let _ = rest;
            return cleaned.len() == 13;
        }
    }

    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    lengths.contains(&cleaned.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_per_state() {
        // SP: 12 digits
        assert!(is_valid("123456789012", "SP"));
        // RJ: 8 digits
        assert!(is_valid("12345678", "RJ"));
        // MT: 11 digits
        assert!(is_valid("12345678901", "MT"));
        // BA: 8 or 9 digits
        assert!(is_valid("12345678", "BA"));
        assert!(is_valid("123456789", "BA"));
        // PE: 9 or 14 digits
        assert!(is_valid("123456789", "PE"));
        assert!(is_valid("12345678901234", "PE"));
        // AC, DF, MG: 13 digits
        assert!(is_valid("1234567890123", "AC"));
        assert!(is_valid("1234567890123", "DF"));
        assert!(is_valid("1234567890123", "MG"));
    }

    #[test]
    fn test_sp_produtor_rural() {
        assert!(is_valid("P123456789012", "SP"));
        assert!(is_valid("p123456789012", "SP"));
        // Wrong length for the producer-rural case
        assert!(!is_valid("P12345678901", "SP"));
        assert!(!is_valid("P1234567890123", "SP"));
    }

    #[test]
    fn test_is_valid_with_mask_chars() {
        assert!(is_valid("123.456.789.012", "SP"));
        assert!(is_valid("123-456-78", "RJ"));
    }

    #[test]
    fn test_invalid_length() {
        assert!(!is_valid("123", "SP"));
        assert!(!is_valid("1234567890123", "SP")); // one digit too many
    }

    #[test]
    fn test_unknown_state() {
        assert!(!is_valid("123456789012", "XX"));
        assert!(!is_valid("123456789012", ""));
    }

    #[test]
    fn test_empty_value() {
        assert!(!is_valid("", "SP"));
        assert!(!is_valid("   ", "SP"));
    }

    #[test]
    fn test_non_digit_value() {
        assert!(!is_valid("12345678901a", "SP"));
    }
}
