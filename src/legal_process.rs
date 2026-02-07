/// Legal process (processo jurídico) utilities for Brazilian legal system.

use chrono::Datelike;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Structure to hold legal process validation data for each organ.
#[derive(Debug, Deserialize, Serialize)]
struct OrgaoData {
    id_tribunal: Vec<u32>,
    id_foro: Vec<u32>,
}

/// Load legal process validation data from JSON.
fn load_legal_process_data() -> HashMap<String, OrgaoData> {
    const JSON_DATA: &str = include_str!("data/legal_process_ids.json");
    serde_json::from_str(JSON_DATA).expect("Failed to parse legal_process_ids.json")
}

/// Removes specific symbols from a given legal process.
///
/// This function takes a legal process as input and removes all occurrences
/// of the '.' and '-' characters from it.
///
/// # Arguments
///
/// * `legal_process` - A legal process containing symbols to be removed.
///
/// # Returns
///
/// The legal process string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_process::remove_symbols;
///
/// assert_eq!(remove_symbols("123.45-678.901.234-56.7890"), "12345678901234567890");
/// assert_eq!(remove_symbols("9876543-21.0987.6.54.3210"), "98765432109876543210");
/// ```
pub fn remove_symbols(legal_process: &str) -> String {
    legal_process.replace('.', "").replace('-', "")
}

/// Format a legal process ID into a standard format.
///
/// This function formats a 20-digit string into the standard Brazilian legal
/// process format: NNNNNNN-DD.AAAA.J.TR.OOOO
///
/// # Arguments
///
/// * `legal_process_id` - A 20-digits string representing the legal process ID.
///
/// # Returns
///
/// The formatted legal process ID or `None` if the input is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_process::format_legal_process;
///
/// assert_eq!(
///     format_legal_process("12345678901234567890"),
///     Some("1234567-89.0123.4.56.7890".to_string())
/// );
/// assert_eq!(
///     format_legal_process("98765432109876543210"),
///     Some("9876543-21.0987.6.54.3210".to_string())
/// );
/// assert_eq!(format_legal_process("123"), None);
/// ```
pub fn format_legal_process(legal_process_id: &str) -> Option<String> {
    if legal_process_id.len() == 20 && legal_process_id.chars().all(|c| c.is_ascii_digit()) {
        let nnnnnnn = &legal_process_id[0..7];
        let dd = &legal_process_id[7..9];
        let aaaa = &legal_process_id[9..13];
        let j = &legal_process_id[13..14];
        let tr = &legal_process_id[14..16];
        let oooo = &legal_process_id[16..20];
        
        Some(format!("{}-{}.{}.{}.{}.{}", nnnnnnn, dd, aaaa, j, tr, oooo))
    } else {
        None
    }
}

/// Calculate the checksum for a legal process ID.
///
/// The checksum is calculated using modulo 97 arithmetic.
///
/// # Arguments
///
/// * `basenum` - The base number (without verification digits) as a string.
///
/// # Returns
///
/// The checksum value as a 2-digit string.
fn checksum(basenum: &str) -> String {
    if let Ok(num) = basenum.parse::<u128>() {
        let result = 97 - ((num * 100) % 97);
        format!("{:02}", result)
    } else {
        "00".to_string()
    }
}

/// Check if a legal process ID is valid.
///
/// This function validates the format and checksum of a legal process ID.
/// It does not verify if the legal process ID corresponds to a real legal process;
/// it only validates the format of the string.
///
/// # Arguments
///
/// * `legal_process_id` - A string representing the legal process ID (with or without symbols).
///
/// # Returns
///
/// `true` if the legal process ID is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_process::is_valid;
///
/// assert_eq!(is_valid("68476506020233030000"), true);
/// assert_eq!(is_valid("51808233620233030000"), true);
/// assert_eq!(is_valid("123"), false);
/// assert_eq!(is_valid("00000000000000000000"), false);
/// ```
pub fn is_valid(legal_process_id: &str) -> bool {
    let clean = remove_symbols(legal_process_id);
    
    // Check length
    if clean.len() != 20 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // Extract components
    let nnnnnnn = &clean[0..7];
    let dd = &clean[7..9];
    let aaaa = &clean[9..13];
    let j = &clean[13..14];
    let tr = &clean[14..16];
    let oooo = &clean[16..20];
    
    // Parse components
    let j_num = match j.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    let tr_num = match tr.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    let oooo_num = match oooo.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    // Load validation data
    let data = load_legal_process_data();
    let orgao_key = format!("orgao_{}", j_num);
    
    // Check if orgao exists
    let orgao_data = match data.get(&orgao_key) {
        Some(d) => d,
        None => return false,
    };
    
    // Validate tribunal and foro
    let valid_process = orgao_data.id_tribunal.contains(&tr_num) 
        && orgao_data.id_foro.contains(&oooo_num);
    
    if !valid_process {
        return false;
    }
    
    // Validate checksum
    let base_for_checksum = format!("{}{}{}{}{}", nnnnnnn, aaaa, j, tr, oooo);
    let expected_dd = checksum(&base_for_checksum);
    
    dd == expected_dd
}

/// Generate a random legal process ID number.
///
/// # Arguments
///
/// * `year` - The year for the legal process ID (default is the current year).
///            The year should not be in the past.
/// * `orgao` - The organization code (1-9) for the legal process ID.
///
/// # Returns
///
/// A randomly generated legal process ID, or `None` if the arguments are invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_process::generate;
///
/// // Generate with current year and random orgao
/// let id = generate(None, None);
/// assert!(id.is_some());
/// assert_eq!(id.unwrap().len(), 20);
///
/// // Generate with current year and specific orgao
/// let id = generate(None, Some(5));
/// assert!(id.is_some());
/// ```
pub fn generate(year: Option<i32>, orgao: Option<u32>) -> Option<String> {
    let current_year = chrono::Local::now().year();
    let year = year.unwrap_or(current_year);
    
    // Validate year (not in the past)
    if year < current_year {
        return None;
    }
    
    // Random orgao if not provided
    let mut rng = rand::thread_rng();
    let orgao = orgao.unwrap_or_else(|| rng.gen_range(1..=9));
    
    // Validate orgao (1-9)
    if !(1..=9).contains(&orgao) {
        return None;
    }
    
    // Load validation data
    let data = load_legal_process_data();
    let orgao_key = format!("orgao_{}", orgao);
    
    let orgao_data = match data.get(&orgao_key) {
        Some(d) => d,
        None => return None,
    };
    
    // Generate random components
    let nnnnnnn = format!("{:07}", rng.gen_range(0..10000000));
    
    // Pick random tribunal and foro
    let tr_idx = rng.gen_range(0..orgao_data.id_tribunal.len());
    let tr = format!("{:02}", orgao_data.id_tribunal[tr_idx]);
    
    let foro_idx = rng.gen_range(0..orgao_data.id_foro.len());
    let oooo = format!("{:04}", orgao_data.id_foro[foro_idx]);
    
    // Calculate checksum
    let base_for_checksum = format!("{}{}{}{}{}", nnnnnnn, year, orgao, tr, oooo);
    let dd = checksum(&base_for_checksum);
    
    Some(format!("{}{}{}{}{}{}", nnnnnnn, dd, year, orgao, tr, oooo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(
            remove_symbols("123.45-678.901.234-56.7890"),
            "12345678901234567890"
        );
        assert_eq!(
            remove_symbols("9876543-21.0987.6.54.3210"),
            "98765432109876543210"
        );
        assert_eq!(remove_symbols("6439067-89.2023.4.04.5902"), "64390678920234045902");
        assert_eq!(remove_symbols("64390678920234045902"), "64390678920234045902");
    }

    #[test]
    fn test_format_legal_process() {
        assert_eq!(
            format_legal_process("12345678901234567890"),
            Some("1234567-89.0123.4.56.7890".to_string())
        );
        assert_eq!(
            format_legal_process("98765432109876543210"),
            Some("9876543-21.0987.6.54.3210".to_string())
        );
        assert_eq!(
            format_legal_process("23141945820055070079"),
            Some("2314194-58.2005.5.07.0079".to_string())
        );
        assert_eq!(
            format_legal_process("00000000000000000000"),
            Some("0000000-00.0000.0.00.0000".to_string())
        );
    }

    #[test]
    fn test_format_legal_process_invalid() {
        assert_eq!(format_legal_process("123"), None);
        assert_eq!(format_legal_process("0000000000000000000"), None); // 19 digits
        assert_eq!(format_legal_process("000000000000000000000"), None); // 21 digits
        assert_eq!(format_legal_process("0000000000000000000a"), None); // contains letter
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("546611720238150014"), "77");
        assert_eq!(checksum("403818720238230498"), "50");
    }

    #[test]
    fn test_is_valid() {
        // Valid cases
        assert!(is_valid("10188748220234018200"));
        assert!(is_valid("45532346920234025107"));
        
        // Invalid checksum
        assert!(!is_valid("10188748220239918200"));
        
        // All zeros
        assert!(!is_valid("00000000000000000000"));
        
        // Wrong length
        assert!(!is_valid("455323469202340251"));
        assert!(!is_valid("455323469202340257123123123"));
        
        // Contains non-digits
        assert!(!is_valid("455323423QQWEQWSsasd&*(()"));
    }

    #[test]
    fn test_is_valid_with_symbols() {
        // Valid formatted
        assert!(is_valid("1018874-82.2023.4.01.8200"));
        assert!(is_valid("4553234-69.2023.4.02.5107"));
    }

    #[test]
    fn test_generate() {
        let current_year = chrono::Local::now().year();
        
        // Generate with defaults
        let id = generate(None, None);
        assert!(id.is_some());
        let id_str = id.unwrap();
        assert_eq!(id_str.len(), 20);
        
        // Check year in generated ID
        let year_part = &id_str[9..13];
        assert_eq!(year_part, current_year.to_string());
        
        // Generate with specific year
        let id = generate(Some(3000), None);
        assert!(id.is_some());
        let id_str = id.unwrap();
        assert_eq!(&id_str[9..13], "3000");
        
        // Generate with specific orgao
        let id = generate(None, Some(4));
        assert!(id.is_some());
        let id_str = id.unwrap();
        assert_eq!(&id_str[13..14], "4");
    }

    #[test]
    fn test_generate_invalid() {
        let current_year = chrono::Local::now().year();
        
        // Year in the past
        assert_eq!(generate(Some(current_year - 1), None), None);
        
        // Invalid orgao (0 or > 9)
        assert_eq!(generate(None, Some(0)), None);
        assert_eq!(generate(None, Some(10)), None);
    }

    #[test]
    fn test_generate_is_valid() {
        // Generated IDs should be valid
        for _ in 0..10 {
            let id = generate(None, None);
            assert!(id.is_some());
            let id_str = id.unwrap();
            assert!(is_valid(&id_str), "Generated ID should be valid: {}", id_str);
        }
    }
}
