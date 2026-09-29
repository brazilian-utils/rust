/// Brazilian municipality utilities.
///
/// Data source: IBGE Localidades API; see `src/data/municipalities.json`
/// for provenance.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;
use unicode_normalization::UnicodeNormalization;

/// Normalizes a name for pt-BR-ish collation: strips accents and lower-cases.
fn collation_key(value: &str) -> String {
    value
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect::<String>()
        .to_lowercase()
}

/// A municipality, as returned by [`get_by_code`] and [`list`].
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Municipality {
    pub code: String,
    pub name: String,
    #[serde(rename = "stateCode")]
    pub state_code: String,
}

#[derive(Debug, Deserialize)]
struct MunicipalityFile {
    data: Vec<Municipality>,
}

fn entries() -> &'static Vec<Municipality> {
    static ENTRIES: OnceLock<Vec<Municipality>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/municipalities.json");
        let parsed: MunicipalityFile =
            serde_json::from_str(JSON_DATA).expect("Failed to parse municipalities.json");
        parsed.data
    })
}

fn by_code() -> &'static HashMap<String, Municipality> {
    static BY_CODE: OnceLock<HashMap<String, Municipality>> = OnceLock::new();
    BY_CODE.get_or_init(|| entries().iter().cloned().map(|m| (m.code.clone(), m)).collect())
}

/// Looks a municipality up by its 7-digit IBGE code.
///
/// # Arguments
///
/// * `code` - The IBGE code, with or without mask symbols.
///
/// # Returns
///
/// The municipality, or `None` when `code` is not 7 digits long or matches
/// none.
///
/// # Examples
///
/// ```
/// use brazilian_utils::municipality::get_by_code;
///
/// let m = get_by_code("3550308").unwrap();
/// assert_eq!(m.name, "São Paulo");
/// assert_eq!(m.state_code, "SP");
///
/// assert_eq!(get_by_code("0000000"), None);
/// ```
pub fn get_by_code(code: &str) -> Option<Municipality> {
    let digits: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 7 {
        return None;
    }
    by_code().get(&digits).cloned()
}

/// Returns the Brazilian municipalities published by the IBGE, sorted by
/// name (pt-BR collation): all of them, or those of one state.
///
/// Only an omitted `state_code` returns the full list; an empty or unknown
/// code returns an empty list. `state_code` is case-sensitive.
///
/// # Examples
///
/// ```
/// use brazilian_utils::municipality::list;
///
/// let df = list(Some("DF"));
/// assert_eq!(df.len(), 1);
/// assert_eq!(df[0].name, "Brasília");
///
/// assert_eq!(list(Some("ZZ")), Vec::new());
/// ```
pub fn list(state_code: Option<&str>) -> Vec<Municipality> {
    let mut result: Vec<Municipality> = match state_code {
        None => entries().clone(),
        Some(code) => entries()
            .iter()
            .filter(|m| m.state_code == code)
            .cloned()
            .collect(),
    };
    result.sort_by(|a, b| collation_key(&a.name).cmp(&collation_key(&b.name)));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_by_code() {
        let sp = get_by_code("3550308").unwrap();
        assert_eq!(sp.name, "São Paulo");
        assert_eq!(sp.state_code, "SP");

        assert_eq!(get_by_code("355-030-8"), Some(sp.clone()));

        let mt = get_by_code("5101837").unwrap();
        assert_eq!(mt.name, "Boa Esperança do Norte");
        assert_eq!(mt.state_code, "MT");

        assert_eq!(get_by_code("0000000"), None);
        assert_eq!(get_by_code("123"), None);
        assert_eq!(get_by_code("12345678"), None);
        assert_eq!(get_by_code(""), None);
    }

    #[test]
    fn test_list() {
        let df = list(Some("DF"));
        assert_eq!(df.len(), 1);
        assert_eq!(df[0].name, "Brasília");

        assert_eq!(list(Some("ZZ")), Vec::new());
        assert_eq!(list(Some("sp")), Vec::new());

        let all = list(None);
        assert!(all.len() > 5000);
    }
}
