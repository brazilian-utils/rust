/// Legal Nature (Natureza Jurídica) utilities for Brazilian companies.
///
/// This module provides utilities for consulting and validating the official
/// *Natureza Jurídica* (Legal Nature) codes defined by CONCLA/IBGE.
///
/// Data source: CONCLA Tabela de Natureza Jurídica 2021 (Notas
/// Explicativas), the DREI legacy table (for retired/legacy codes) and the
/// CONCLA revision history; see `src/data/legalNature.json` for provenance.
/// The "retired" sub-list is best-effort, not necessarily exhaustive (see
/// that file's `note`).
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// A CONCLA category (the first digit of a legal nature code).
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LegalNatureCategory {
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RawLegalNatureEntry {
    code: String,
    description: String,
    category: LegalNatureCategory,
    legacy: bool,
    #[serde(rename = "currentCode", default)]
    current_code: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LegalNatureFile {
    data: Vec<RawLegalNatureEntry>,
    retired: Vec<RawLegalNatureEntry>,
}

/// A legal nature entry, as returned by [`get`] and [`list_by_category`].
#[derive(Debug, Clone, PartialEq)]
pub struct LegalNature {
    pub code: String,
    pub description: String,
    pub category: LegalNatureCategory,
    pub legacy: bool,
    pub current_code: Option<String>,
}

impl From<&RawLegalNatureEntry> for LegalNature {
    fn from(e: &RawLegalNatureEntry) -> Self {
        LegalNature {
            code: e.code.clone(),
            description: e.description.clone(),
            category: e.category.clone(),
            legacy: e.legacy,
            current_code: e.current_code.clone(),
        }
    }
}

struct Tables {
    in_force: Vec<RawLegalNatureEntry>,
    retired: Vec<RawLegalNatureEntry>,
    by_code: HashMap<String, usize>, // index into a combined vec
    combined: Vec<RawLegalNatureEntry>,
}

fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/legalNature.json");
        let parsed: LegalNatureFile =
            serde_json::from_str(JSON_DATA).expect("Failed to parse legalNature.json");

        let mut combined = parsed.data.clone();
        combined.extend(parsed.retired.clone());

        let by_code: HashMap<String, usize> = combined
            .iter()
            .enumerate()
            .map(|(i, e)| (e.code.clone(), i))
            .collect();

        Tables {
            in_force: parsed.data,
            retired: parsed.retired,
            by_code,
            combined,
        }
    })
}

/// Normalize a legal nature code by removing non-digit characters.
///
/// Accepts formats like "2062" or "206-2" and returns "2062".
fn normalize(code: &str) -> Option<String> {
    let digits: String = code.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() == 4 {
        Some(digits)
    } else {
        None
    }
}

/// Check if a string corresponds to a valid *Natureza Jurídica* (Legal
/// Nature) code: one of the 92 in-force codes, plus the codes a past
/// revision retired.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::is_valid;
///
/// assert_eq!(is_valid("2062"), true);
/// assert_eq!(is_valid("206-2"), true);
/// assert_eq!(is_valid("9999"), false);
/// ```
pub fn is_valid(code: &str) -> bool {
    match normalize(code) {
        Some(normalized) => tables().by_code.contains_key(&normalized),
        None => false,
    }
}

/// Retrieve the description of a *Natureza Jurídica* (Legal Nature) code.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::get_description;
///
/// assert_eq!(get_description("2062"), Some("Sociedade Empresária Limitada"));
/// assert_eq!(get_description("0000"), None);
/// ```
pub fn get_description(code: &str) -> Option<&'static str> {
    let normalized = normalize(code)?;
    let idx = *tables().by_code.get(&normalized)?;
    Some(tables().combined[idx].description.as_str())
}

/// Looks a legal nature code up in the CONCLA Natureza Jurídica 2021 table;
/// returns `None` for an unknown code.
///
/// A code retired by a past revision comes back flagged as legacy, with the
/// code it corresponds to today, or `None` when it has no successor.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::get;
///
/// let ln = get("2062").unwrap();
/// assert_eq!(ln.description, "Sociedade Empresária Limitada");
/// assert!(!ln.legacy);
///
/// let legacy = get("2208").unwrap();
/// assert!(legacy.legacy);
/// assert_eq!(legacy.current_code.as_deref(), Some("2275"));
/// ```
pub fn get(value: &str) -> Option<LegalNature> {
    let normalized = normalize(value)?;
    let idx = *tables().by_code.get(&normalized)?;
    Some(LegalNature::from(&tables().combined[idx]))
}

/// Options for [`list_all`].
#[derive(Debug, Clone, Default)]
pub struct GetLegalNaturesParams {
    /// Also include the retired/legacy codes. Defaults to `false`.
    pub include_retired: Option<bool>,
}

/// Return the legal nature table as a map from code to description.
///
/// Only the 92 codes in force by default; `params.include_retired` also
/// includes the retired codes.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::list_all;
///
/// let table = list_all(None);
/// assert_eq!(table.len(), 92);
/// assert_eq!(table.get("2062"), Some(&"Sociedade Empresária Limitada".to_string()));
/// ```
pub fn list_all(params: Option<GetLegalNaturesParams>) -> HashMap<String, String> {
    let include_retired = params.and_then(|p| p.include_retired).unwrap_or(false);
    let t = tables();
    let mut map: HashMap<String, String> = t
        .in_force
        .iter()
        .map(|e| (e.code.clone(), e.description.clone()))
        .collect();
    if include_retired {
        for e in &t.retired {
            map.insert(e.code.clone(), e.description.clone());
        }
    }
    map
}

/// Options for [`list_by_category`].
#[derive(Debug, Clone, Default)]
pub struct GetLegalNaturesByCategoryOptions {
    /// Also include the retired/legacy codes of the category. Defaults to
    /// `false`.
    pub include_retired: Option<bool>,
}

/// Returns every legal nature of a CONCLA category (the first digit of the
/// code), sorted by code.
///
/// Categories: 1 Administração Pública, 2 Entidades Empresariais, 3
/// Entidades sem Fins Lucrativos, 4 Pessoas Físicas, 5 Organizações
/// Internacionais e Outras Instituições Extraterritoriais.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::list_by_category;
///
/// let category5 = list_by_category("5", None);
/// assert_eq!(category5.len(), 3);
/// assert_eq!(category5[0].code, "5010");
///
/// assert_eq!(list_by_category("0", None), Vec::new());
/// ```
pub fn list_by_category(
    category: &str,
    options: Option<GetLegalNaturesByCategoryOptions>,
) -> Vec<LegalNature> {
    let category = category.trim();
    if category.len() != 1 || !category.chars().all(|c| c.is_ascii_digit()) {
        return Vec::new();
    }

    let include_retired = options.and_then(|o| o.include_retired).unwrap_or(false);
    let t = tables();

    let mut result: Vec<LegalNature> = t
        .in_force
        .iter()
        .filter(|e| e.code.starts_with(category))
        .map(LegalNature::from)
        .collect();

    if include_retired {
        result.extend(
            t.retired
                .iter()
                .filter(|e| e.code.starts_with(category))
                .map(LegalNature::from),
        );
    }

    result.sort_by(|a, b| a.code.cmp(&b.code));
    result
}

/// Formats a legal nature code as `NNN-N`; use [`is_valid`] to check the
/// code. The mask is applied as far as the digits go.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::format;
///
/// assert_eq!(format("2062"), "206-2");
/// assert_eq!(format("206"), "206");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() <= 3 {
        digits
    } else {
        format!("{}-{}", &digits[0..3], &digits[3..digits.len().min(4)])
    }
}

/// Generates a random valid legal nature code (4 digits), drawn only among
/// the 92 codes in force, never a retired one.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::{generate, is_valid};
///
/// let code = generate();
/// assert_eq!(code.len(), 4);
/// assert!(is_valid(&code));
/// ```
pub fn generate() -> String {
    let in_force = &tables().in_force;
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..in_force.len());
    in_force[idx].code.clone()
}

/// Removes legal nature formatting and keeps only digits, capped to 4
/// digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::parse;
///
/// assert_eq!(parse("206-2"), "2062");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(4)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_formats() {
        assert!(is_valid("2062"));
        assert!(is_valid("206-2"));
        assert!(is_valid("101-5"));
        assert!(is_valid("1015"));
    }

    #[test]
    fn test_is_valid_known_codes() {
        assert!(is_valid("1015"));
        assert!(is_valid("2062"));
        assert!(is_valid("2143"));
        assert!(is_valid("3034"));
        assert!(is_valid("4014"));
        assert!(is_valid("5010"));
    }

    #[test]
    fn test_is_valid_retired_codes() {
        // Retired codes are still accepted by is_valid.
        assert!(is_valid("2208"));
        assert!(is_valid("2100"));
    }

    #[test]
    fn test_is_valid_invalid_codes() {
        assert!(!is_valid(""));
        assert!(!is_valid("20"));
        assert!(!is_valid("20623"));
        assert!(!is_valid("abcd"));
        assert!(!is_valid("---"));
        assert!(!is_valid("9999"));
        assert!(!is_valid("0000"));
        assert!(!is_valid("3329"));
        assert!(!is_valid("2241"));
        assert!(!is_valid("3311"));
    }

    #[test]
    fn test_get_description_known() {
        assert_eq!(
            get_description("2062"),
            Some("Sociedade Empresária Limitada")
        );
        assert_eq!(
            get_description("101-5"),
            Some("Órgão Público do Poder Executivo Federal")
        );
        assert_eq!(get_description("2143"), Some("Cooperativa"));
        assert_eq!(get_description("2240"), Some("Sociedade Simples Limitada"));
        assert_eq!(get_description("224-0"), Some("Sociedade Simples Limitada"));
    }

    #[test]
    fn test_get_description_invalid() {
        assert_eq!(get_description("9999"), None);
        assert_eq!(get_description("0000"), None);
        assert_eq!(get_description("20A2"), None);
        assert_eq!(get_description(""), None);
    }

    #[test]
    fn test_get() {
        let ln = get("2062").unwrap();
        assert_eq!(ln.code, "2062");
        assert_eq!(ln.description, "Sociedade Empresária Limitada");
        assert_eq!(ln.category.code, "2");
        assert!(!ln.legacy);

        let masked = get("206-2").unwrap();
        assert_eq!(masked.code, "2062");

        let legacy = get("2208").unwrap();
        assert_eq!(legacy.description, "Entidade Binacional Itaipu");
        assert!(legacy.legacy);
        assert_eq!(legacy.current_code.as_deref(), Some("2275"));

        assert_eq!(get("0000"), None);
        assert_eq!(get("206"), None);
        assert_eq!(get(""), None);
    }

    #[test]
    fn test_list_all() {
        let table = list_all(None);
        assert_eq!(table.len(), 92);
        assert_eq!(
            table.get("2062"),
            Some(&"Sociedade Empresária Limitada".to_string())
        );
        assert_eq!(table.get("2208"), None); // retired, excluded by default

        let with_retired = list_all(Some(GetLegalNaturesParams {
            include_retired: Some(true),
        }));
        assert_eq!(with_retired.len(), 98);
        assert!(with_retired.contains_key("2208"));
    }

    #[test]
    fn test_list_by_category() {
        let category5 = list_by_category("5", None);
        assert_eq!(category5.len(), 3);
        assert_eq!(category5[0].code, "5010");
        assert_eq!(category5[1].code, "5029");
        assert_eq!(category5[2].code, "5037");

        let category4 = list_by_category("4", None);
        assert_eq!(category4.len(), 6);

        assert_eq!(list_by_category("0", None), Vec::new());
        assert_eq!(list_by_category("9", None), Vec::new());
        assert_eq!(list_by_category("2062", None), Vec::new());
        assert_eq!(list_by_category("", None), Vec::new());
    }

    #[test]
    fn test_format() {
        assert_eq!(format("2062"), "206-2");
        assert_eq!(format("206-2"), "206-2");
        assert_eq!(format("206"), "206");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_generate() {
        for _ in 0..20 {
            let code = generate();
            assert_eq!(code.len(), 4);
            assert!(is_valid(&code));
            let entry = get(&code).unwrap();
            assert!(!entry.legacy);
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("206-2"), "2062");
        assert_eq!(parse("2062"), "2062");
        assert_eq!(parse("206299"), "2062");
        assert_eq!(parse(""), "");
    }
}
