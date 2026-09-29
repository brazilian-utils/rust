/// CST (Código de Situação Tributária) utilities.
///
/// Covers four sub-tables: the ICMS origin digit (Tabela A), the ICMS CST
/// (Tabela B), the IPI CST and the shared PIS/COFINS CST. The 3-digit ICMS
/// CST is the origin digit concatenated with the 2-digit Tabela B code.
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct CstEntry {
    code: String,
}

#[derive(Debug, Deserialize)]
struct CstSubTable {
    entries: Vec<CstEntry>,
}

#[derive(Debug, Deserialize)]
struct CstData {
    #[serde(rename = "icmsOrigin")]
    icms_origin: CstSubTable,
    #[serde(rename = "icmsCst")]
    icms_cst: CstSubTable,
    #[serde(rename = "ipiCst")]
    ipi_cst: CstSubTable,
    #[serde(rename = "pisCofinsCst")]
    pis_cofins_cst: CstSubTable,
}

#[derive(Debug, Deserialize)]
struct CstFile {
    data: CstData,
}

struct Tables {
    icms_origin: HashSet<String>,
    icms_cst: HashSet<String>,
    ipi_cst: HashSet<String>,
    pis_cofins_cst: HashSet<String>,
}

fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| {
        const JSON_DATA: &str = include_str!("data/cst.json");
        let parsed: CstFile = serde_json::from_str(JSON_DATA).expect("Failed to parse cst.json");
        let to_set = |t: CstSubTable| t.entries.into_iter().map(|e| e.code).collect();
        Tables {
            icms_origin: to_set(parsed.data.icms_origin),
            icms_cst: to_set(parsed.data.icms_cst),
            ipi_cst: to_set(parsed.data.ipi_cst),
            pis_cofins_cst: to_set(parsed.data.pis_cofins_cst),
        }
    })
}

fn is_icms_valid(origin: char, cst2: &str) -> bool {
    let t = tables();
    t.icms_origin.contains(&origin.to_string()) && t.icms_cst.contains(cst2)
}

/// Options for [`is_valid`].
#[derive(Debug, Clone, Default)]
pub struct IsValidCstOptions {
    /// Restricts the table: `"icms"`, `"ipi"`, `"pis"` or `"cofins"`
    /// (PIS and COFINS share a table). Omitted or unknown, every table is
    /// accepted.
    pub tax: Option<String>,
}

/// Checks whether a CST code is valid for a tax.
///
/// - `options.tax` picks the table: `icms` (3 digits, origem `0`-`8` plus
///   the Tabela B code), `ipi`, `pis` or `cofins` (2 digits). Omitted or
///   unknown, every table is accepted.
/// - A single digit is padded to the 3-digit ICMS form; a 2-digit string is
///   a Tabela B code. The 3-digit ICMS form may have a single separator
///   after the origin digit.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cst::is_valid;
///
/// assert!(is_valid("110", None));
/// assert!(is_valid("00", None)); // valid IPI CST
/// assert!(is_valid("07", None)); // valid PIS/COFINS CST
/// assert!(!is_valid("999", None));
/// assert!(!is_valid("0-0", None));
/// ```
pub fn is_valid(value: &str, options: Option<IsValidCstOptions>) -> bool {
    let tax = options
        .and_then(|o| o.tax)
        .map(|t| t.to_lowercase())
        .filter(|t| matches!(t.as_str(), "icms" | "ipi" | "pis" | "cofins"));
    let allow = |t: &str| tax.is_none() || tax.as_deref() == Some(t);

    let chars: Vec<char> = value.chars().collect();

    match chars.len() {
        1 if chars[0].is_ascii_digit() => {
            if !allow("icms") {
                return false;
            }
            let cst2 = format!("0{}", chars[0]);
            is_icms_valid('0', &cst2)
        }
        2 if chars.iter().all(|c| c.is_ascii_digit()) => {
            let code: String = chars.iter().collect();
            let t = tables();
            (allow("ipi") && t.ipi_cst.contains(&code))
                || ((allow("pis") || allow("cofins")) && t.pis_cofins_cst.contains(&code))
        }
        3 if chars.iter().all(|c| c.is_ascii_digit()) => {
            if !allow("icms") {
                return false;
            }
            let cst2: String = chars[1..3].iter().collect();
            is_icms_valid(chars[0], &cst2)
        }
        4 if chars[0].is_ascii_digit()
            && matches!(chars[1], ' ' | '.' | '-' | '/')
            && chars[2].is_ascii_digit()
            && chars[3].is_ascii_digit() =>
        {
            if !allow("icms") {
                return false;
            }
            let cst2: String = chars[2..4].iter().collect();
            is_icms_valid(chars[0], &cst2)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tax(t: &str) -> Option<IsValidCstOptions> {
        Some(IsValidCstOptions {
            tax: Some(t.to_string()),
        })
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("110", None));
        assert!(is_valid("00", None));
        assert!(is_valid("07", None));
        assert!(!is_valid("999", None));
        assert!(!is_valid("0-0", None));
    }

    #[test]
    fn test_is_valid_with_tax_filter() {
        assert!(is_valid("110", tax("icms")));
        assert!(!is_valid("00", tax("icms"))); // 2 digits isn't a valid ICMS form
        assert!(is_valid("00", tax("ipi")));
        assert!(!is_valid("07", tax("ipi")));
        assert!(is_valid("07", tax("pis")));
        assert!(is_valid("07", tax("cofins")));
        assert!(is_valid("1-10", tax("icms")));
    }
}
