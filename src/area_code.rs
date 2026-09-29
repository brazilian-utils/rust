/// DDD (Brazilian area code) utilities.
///
/// Data source: Anatel Plano Geral de Numeração (Resolução nº 749/2022); see
/// `src/data/areaCodes.json` for provenance.
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
struct AreaCodeEntry {
    #[serde(rename = "areaCode")]
    area_code: u32,
    #[serde(rename = "stateCode")]
    state_code: String,
    #[serde(rename = "stateName")]
    state_name: String,
    #[serde(rename = "regionCode")]
    region_code: String,
    #[serde(rename = "regionName")]
    region_name: String,
    #[serde(rename = "stateCodes")]
    state_codes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AreaCodeFile {
    data: Vec<AreaCodeEntry>,
}

fn entries() -> &'static Vec<AreaCodeEntry> {
    static ENTRIES: OnceLock<Vec<AreaCodeEntry>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/areaCodes.json");
        let parsed: AreaCodeFile =
            serde_json::from_str(JSON_DATA).expect("Failed to parse areaCodes.json");
        parsed.data
    })
}

fn by_code() -> &'static HashMap<u32, AreaCodeEntry> {
    static BY_CODE: OnceLock<HashMap<u32, AreaCodeEntry>> = OnceLock::new();
    BY_CODE.get_or_init(|| {
        entries()
            .iter()
            .cloned()
            .map(|e| (e.area_code, e))
            .collect()
    })
}

/// The state and region a DDD (area code) belongs to, as returned by
/// [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct AreaCodeInfo {
    pub area_code: u32,
    pub state_code: String,
    pub state_name: String,
    pub region_code: String,
    pub region_name: String,
    pub state_codes: Vec<String>,
}

/// Returns the state and region a DDD (area code) belongs to, among the 67
/// DDDs in use under the Anatel Plano Geral de Numeração.
///
/// For the four DDDs that straddle a state border (61, 42, 47 and 49) the
/// list of states also has the other state, the seat first.
///
/// # Arguments
///
/// * `area_code` - The DDD to look up.
///
/// # Returns
///
/// The area code's info, or `None` when it is not in use.
///
/// # Examples
///
/// ```
/// use brazilian_utils::area_code::get_info;
///
/// let info = get_info(11).unwrap();
/// assert_eq!(info.state_code, "SP");
///
/// let df = get_info(61).unwrap();
/// assert_eq!(df.state_codes, vec!["DF".to_string(), "GO".to_string()]);
///
/// assert_eq!(get_info(20), None);
/// ```
pub fn get_info(area_code: u32) -> Option<AreaCodeInfo> {
    by_code().get(&area_code).map(|e| AreaCodeInfo {
        area_code: e.area_code,
        state_code: e.state_code.clone(),
        state_name: e.state_name.clone(),
        region_code: e.region_code.clone(),
        region_name: e.region_name.clone(),
        state_codes: e.state_codes.clone(),
    })
}

/// Returns every DDD (area code) that serves a state, sorted in ascending
/// order.
///
/// A DDD that straddles a border is listed under every state it serves.
///
/// # Arguments
///
/// * `state_code` - The state (UF) to look up, matched case-insensitively.
///
/// # Returns
///
/// The DDDs serving `state_code`, or an empty list when it is not a
/// Brazilian state.
///
/// # Examples
///
/// ```
/// use brazilian_utils::area_code::list_by_state;
///
/// assert_eq!(list_by_state("SP"), vec![11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// assert_eq!(list_by_state("sc"), vec![42, 47, 48, 49]);
/// assert_eq!(list_by_state("XX"), Vec::<u32>::new());
/// ```
pub fn list_by_state(state_code: &str) -> Vec<u32> {
    let upper = state_code.to_uppercase();
    let mut result: Vec<u32> = entries()
        .iter()
        .filter(|e| e.state_codes.iter().any(|s| s == &upper))
        .map(|e| e.area_code)
        .collect();
    result.sort_unstable();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_info() {
        let info = get_info(11).unwrap();
        assert_eq!(info.state_code, "SP");
        assert_eq!(info.state_name, "São Paulo");
        assert_eq!(info.region_code, "SE");
        assert_eq!(info.state_codes, vec!["SP".to_string()]);

        let df = get_info(61).unwrap();
        assert_eq!(df.state_code, "DF");
        assert_eq!(df.state_codes, vec!["DF".to_string(), "GO".to_string()]);

        let pr = get_info(42).unwrap();
        assert_eq!(pr.state_code, "PR");
        assert_eq!(pr.state_codes, vec!["PR".to_string(), "SC".to_string()]);

        assert_eq!(get_info(0), None);
        assert_eq!(get_info(20), None);
    }

    #[test]
    fn test_list_by_state() {
        assert_eq!(list_by_state("SP"), vec![11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(list_by_state("sp"), vec![11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(list_by_state("AC"), vec![68]);
        assert_eq!(list_by_state("PE"), vec![81, 87]);
        assert_eq!(list_by_state("SC"), vec![42, 47, 48, 49]);
        assert_eq!(list_by_state("XX"), Vec::<u32>::new());
        assert_eq!(list_by_state(""), Vec::<u32>::new());
    }
}
