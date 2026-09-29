/// Registro profissional (professional council registration) structural
/// utilities.
///
/// Checks the *structure* of a professional council registration number —
/// digit count and UF only, never a check digit (none of these councils
/// publish one).
use regex::Regex;

const VALID_UFS: &[&str] = &[
    "AC", "AL", "AP", "AM", "BA", "CE", "DF", "ES", "GO", "MA", "MT", "MS", "MG", "PA", "PB", "PR",
    "PE", "PI", "RJ", "RN", "RS", "RO", "RR", "SC", "SP", "SE", "TO",
];

/// Parameters for [`is_valid`].
#[derive(Debug, Clone, Default)]
pub struct IsValidRegistroProfissionalParams {
    /// The registration number to validate.
    pub value: String,
    /// The council: `"OAB"`, `"CRM"`, `"CRO"`, `"CRP"` or `"CRC"`
    /// (case-insensitive).
    pub council: String,
    /// The expected state (UF), when known. Ignored for CRP.
    pub state: Option<String>,
}

fn matches_state(uf: &str, expected: Option<&str>) -> bool {
    if !VALID_UFS.contains(&uf.to_uppercase().as_str()) {
        return false;
    }
    match expected {
        Some(state) => uf.eq_ignore_ascii_case(state),
        None => true,
    }
}

/// OAB and CRM: 4 to 6 digits plus the UF (`123456/SP`, `123456-SP`).
fn validate_oab_or_crm(value: &str, state: Option<&str>) -> bool {
    let re = Regex::new(r"^(\d{4,6})[/-]([A-Za-z]{2})$").unwrap();
    match re.captures(value) {
        Some(caps) => matches_state(&caps[2], state),
        None => false,
    }
}

/// CRO: 3 to 6 digits plus the UF.
fn validate_cro(value: &str, state: Option<&str>) -> bool {
    let re = Regex::new(r"^(\d{3,6})[/-]([A-Za-z]{2})$").unwrap();
    match re.captures(value) {
        Some(caps) => matches_state(&caps[2], state),
        None => false,
    }
}

/// CRP: a 2-digit regional code (`01` to `24`) plus 4 to 6 digits
/// (`06/12345`); the expected state is ignored.
fn validate_crp(value: &str) -> bool {
    let re = Regex::new(r"^(\d{2})[/-](\d{4,6})$").unwrap();
    match re.captures(value) {
        Some(caps) => {
            let region: u32 = caps[1].parse().unwrap_or(0);
            (1..=24).contains(&region)
        }
        None => false,
    }
}

/// CRC: UF, 6 digits, registration type (`O` or `P`) and a digit
/// (`SP-123456/O-3`), optionally a transfer suffix (`T-MG` or `S-MG`); the
/// expected state matches the originating UF.
fn validate_crc(value: &str, state: Option<&str>) -> bool {
    let re =
        Regex::new(r"^([A-Za-z]{2})-(\d{6})/([OPop])-(\d)(-[TSts]-[A-Za-z]{2})?$").unwrap();
    match re.captures(value) {
        Some(caps) => matches_state(&caps[1], state),
        None => false,
    }
}

/// Checks the structure of a professional council registration number.
///
/// # Examples
///
/// ```
/// use brazilian_utils::registro_profissional::{is_valid, IsValidRegistroProfissionalParams};
///
/// assert!(is_valid(&IsValidRegistroProfissionalParams {
///     value: "123456/SP".to_string(),
///     council: "OAB".to_string(),
///     state: None,
/// }));
/// assert!(is_valid(&IsValidRegistroProfissionalParams {
///     value: "SP-123456/O-3".to_string(),
///     council: "CRC".to_string(),
///     state: None,
/// }));
/// ```
pub fn is_valid(params: &IsValidRegistroProfissionalParams) -> bool {
    let council = params.council.to_uppercase();
    let value = params.value.trim();
    let state = params.state.as_deref();

    match council.as_str() {
        "OAB" | "CRM" => validate_oab_or_crm(value, state),
        "CRO" => validate_cro(value, state),
        "CRP" => validate_crp(value),
        "CRC" => validate_crc(value, state),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(value: &str, council: &str, state: Option<&str>) -> IsValidRegistroProfissionalParams {
        IsValidRegistroProfissionalParams {
            value: value.to_string(),
            council: council.to_string(),
            state: state.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_oab_crm() {
        assert!(is_valid(&params("123456/SP", "OAB", None)));
        assert!(is_valid(&params("12345-RJ", "CRM", Some("RJ"))));
        assert!(!is_valid(&params("12345-RJ", "CRM", Some("SP"))));
        assert!(!is_valid(&params("123/SP", "OAB", None))); // too short
        assert!(!is_valid(&params("123456/XX", "OAB", None))); // not a UF
    }

    #[test]
    fn test_cro() {
        assert!(is_valid(&params("123/SP", "CRO", None)));
        assert!(is_valid(&params("123456-MG", "CRO", None)));
    }

    #[test]
    fn test_crp() {
        assert!(is_valid(&params("06/12345", "CRP", None)));
        assert!(is_valid(&params("01-1234", "CRP", None)));
        assert!(!is_valid(&params("25/12345", "CRP", None))); // out of range
        assert!(!is_valid(&params("06/123", "CRP", None))); // too short
    }

    #[test]
    fn test_crc() {
        assert!(is_valid(&params("SP-123456/O-3", "CRC", None)));
        assert!(is_valid(&params("SP-123456/O-3", "CRC", Some("SP"))));
        assert!(!is_valid(&params("SP-123456/O-3", "CRC", Some("MG"))));
        assert!(is_valid(&params("SP-123456/O-3-T-MG", "CRC", None)));
    }

    #[test]
    fn test_unknown_council() {
        assert!(!is_valid(&params("123456/SP", "CREA", None)));
    }
}
