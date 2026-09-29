/// Brazilian state (UF) utilities.
///
/// Data source: IBGE Localidades API and the IANA tz database; see
/// `src/data/states.json` for provenance.
use serde::Deserialize;
use std::sync::OnceLock;
use unicode_normalization::UnicodeNormalization;

/// A Brazilian state, as returned by [`list`] and the lookup functions.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct State {
    pub code: String,
    pub name: String,
    #[serde(rename = "regionCode")]
    pub region_code: String,
    #[serde(rename = "regionName")]
    pub region_name: String,
    #[serde(rename = "ibgeCode")]
    pub ibge_code: u32,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
struct StateFile {
    data: Vec<State>,
}

fn entries() -> &'static Vec<State> {
    static ENTRIES: OnceLock<Vec<State>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/states.json");
        let parsed: StateFile =
            serde_json::from_str(JSON_DATA).expect("Failed to parse states.json");
        parsed.data
    })
}

/// Normalizes a string for a loose match: removes accents, lower-cases,
/// trims and collapses internal whitespace.
fn normalize(value: &str) -> String {
    let no_accents: String = value
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect();
    no_accents
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns the state whose 2-digit IBGE code (`cUF`) matches `code`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::state::get_by_ibge_code;
///
/// let sp = get_by_ibge_code(35).unwrap();
/// assert_eq!(sp.code, "SP");
///
/// assert_eq!(get_by_ibge_code(0), None);
/// ```
pub fn get_by_ibge_code(code: u32) -> Option<State> {
    entries().iter().find(|s| s.ibge_code == code).cloned()
}

/// Returns the two-letter code (sigla) of a state from its full name.
///
/// The match ignores accents, case and surrounding whitespace; internal
/// whitespace collapses into one space.
///
/// # Examples
///
/// ```
/// use brazilian_utils::state::get_code_by_name;
///
/// assert_eq!(get_code_by_name("São Paulo"), Some("SP".to_string()));
/// assert_eq!(get_code_by_name("sao paulo"), Some("SP".to_string()));
/// assert_eq!(get_code_by_name("Neverland"), None);
/// ```
pub fn get_code_by_name(name: &str) -> Option<String> {
    let target = normalize(name);
    entries()
        .iter()
        .find(|s| normalize(&s.name) == target)
        .map(|s| s.code.clone())
}

/// Returns the full name of a state from its two-letter code (sigla).
///
/// The match ignores case and surrounding whitespace.
///
/// # Examples
///
/// ```
/// use brazilian_utils::state::get_name_by_code;
///
/// assert_eq!(get_name_by_code("sp"), Some("São Paulo".to_string()));
/// assert_eq!(get_name_by_code("ZZ"), None);
/// ```
pub fn get_name_by_code(code: &str) -> Option<String> {
    let target = code.trim().to_uppercase();
    entries()
        .iter()
        .find(|s| s.code == target)
        .map(|s| s.name.clone())
}

/// Returns the IANA time zone (tzdata zone) of a state: the zone of its
/// capital.
///
/// The match ignores case and surrounding whitespace.
///
/// # Examples
///
/// ```
/// use brazilian_utils::state::get_timezone;
///
/// assert_eq!(get_timezone("SP"), Some("America/Sao_Paulo".to_string()));
/// assert_eq!(get_timezone("AM"), Some("America/Manaus".to_string()));
/// assert_eq!(get_timezone("ZZ"), None);
/// ```
pub fn get_timezone(state_code: &str) -> Option<String> {
    let target = state_code.trim().to_uppercase();
    entries()
        .iter()
        .find(|s| s.code == target)
        .map(|s| s.timezone.clone())
}

/// Returns the 27 Brazilian federative units, sorted by name (pt-BR
/// collation).
///
/// # Examples
///
/// ```
/// use brazilian_utils::state::list;
///
/// let states = list();
/// assert_eq!(states.len(), 27);
/// assert_eq!(states[0].code, "AC");
/// ```
pub fn list() -> Vec<State> {
    let mut all = entries().clone();
    all.sort_by(|a, b| normalize(&a.name).cmp(&normalize(&b.name)));
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_by_ibge_code() {
        let sp = get_by_ibge_code(35).unwrap();
        assert_eq!(sp.code, "SP");
        assert_eq!(sp.region_code, "SE");

        let df = get_by_ibge_code(53).unwrap();
        assert_eq!(df.code, "DF");

        assert_eq!(get_by_ibge_code(0), None);
        assert_eq!(get_by_ibge_code(99), None);
    }

    #[test]
    fn test_get_code_by_name() {
        assert_eq!(get_code_by_name("São Paulo"), Some("SP".to_string()));
        assert_eq!(get_code_by_name("Sao Paulo"), Some("SP".to_string()));
        assert_eq!(get_code_by_name("sao paulo"), Some("SP".to_string()));
        assert_eq!(get_code_by_name("  São Paulo  "), Some("SP".to_string()));
        assert_eq!(
            get_code_by_name("Rio Grande do Norte"),
            Some("RN".to_string())
        );
        assert_eq!(
            get_code_by_name("Distrito Federal"),
            Some("DF".to_string())
        );
        assert_eq!(get_code_by_name("Neverland"), None);
        assert_eq!(get_code_by_name(""), None);
    }

    #[test]
    fn test_get_name_by_code() {
        assert_eq!(get_name_by_code("SP"), Some("São Paulo".to_string()));
        assert_eq!(get_name_by_code("sp"), Some("São Paulo".to_string()));
        assert_eq!(get_name_by_code("  RJ  "), Some("Rio de Janeiro".to_string()));
        assert_eq!(get_name_by_code("DF"), Some("Distrito Federal".to_string()));
        assert_eq!(get_name_by_code("ZZ"), None);
        assert_eq!(get_name_by_code(""), None);
    }

    #[test]
    fn test_get_timezone() {
        assert_eq!(get_timezone("SP"), Some("America/Sao_Paulo".to_string()));
        assert_eq!(get_timezone("AM"), Some("America/Manaus".to_string()));
        assert_eq!(get_timezone("AC"), Some("America/Rio_Branco".to_string()));
        assert_eq!(get_timezone("BA"), Some("America/Bahia".to_string()));
        assert_eq!(get_timezone("sp"), Some("America/Sao_Paulo".to_string()));
        assert_eq!(get_timezone("  SP  "), Some("America/Sao_Paulo".to_string()));
        assert_eq!(get_timezone("ZZ"), None);
        assert_eq!(get_timezone(""), None);
    }

    #[test]
    fn test_list() {
        let states = list();
        assert_eq!(states.len(), 27);

        let expected_order = [
            "AC", "AL", "AP", "AM", "BA", "CE", "DF", "ES", "GO", "MA", "MT", "MS", "MG", "PA",
            "PB", "PR", "PE", "PI", "RJ", "RN", "RS", "RO", "RR", "SC", "SP", "SE", "TO",
        ];
        let actual: Vec<&str> = states.iter().map(|s| s.code.as_str()).collect();
        assert_eq!(actual, expected_order);
    }
}
