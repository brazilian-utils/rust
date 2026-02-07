//! Brazilian Voter ID (Título de Eleitor) utilities.
//!
//! This module provides functions to validate, format, and generate Brazilian
//! voter registration numbers (Título de Eleitor).

use rand::Rng;
use std::collections::HashMap;

/// Validates a Brazilian voter ID number.
///
/// A valid voter ID must:
/// - Contain only digits
/// - Have 12 digits (or 13 for SP and MG in special cases)
/// - Have a valid federative union code (01-28)
/// - Have valid check digits
///
/// # Arguments
///
/// * `voter_id` - A string slice containing the voter ID to validate
///
/// # Returns
///
/// `true` if the voter ID is valid, `false` otherwise
///
/// # Example
///
/// ```
/// use brazilian_utils::voter_id::is_valid;
///
/// assert_eq!(is_valid("690847092828"), true);
/// assert_eq!(is_valid("163204010922"), true);
/// assert_eq!(is_valid("123456789012"), false);
/// ```
pub fn is_valid(voter_id: &str) -> bool {
    // Must contain only digits
    if !voter_id.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Check length validity
    if !is_length_valid(voter_id) {
        return false;
    }

    // Extract parts
    let sequential_number = get_sequential_number(voter_id);
    let federative_union = get_federative_union(voter_id);
    let verifying_digits = get_verifying_digits(voter_id);

    // Ensure federative union is valid
    if !is_federative_union_valid(&federative_union) {
        return false;
    }

    // Validate first check digit
    let vd1 = calculate_vd1(&sequential_number, &federative_union);
    if vd1
        != verifying_digits
            .chars().next()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(99) as u8
    {
        return false;
    }

    // Validate second check digit
    let vd2 = calculate_vd2(&federative_union, vd1);
    if vd2
        != verifying_digits
            .chars()
            .nth(1)
            .and_then(|c| c.to_digit(10))
            .unwrap_or(99) as u8
    {
        return false;
    }

    true
}

/// Formats a voter ID for display with visual spaces.
///
/// # Arguments
///
/// * `voter_id` - A string slice containing the voter ID to format
///
/// # Returns
///
/// `Some(String)` with formatted voter ID as "XXXX XXXX XX XX", or `None` if invalid
///
/// # Example
///
/// ```
/// use brazilian_utils::voter_id::format_voter_id;
///
/// assert_eq!(format_voter_id("690847092828"), Some("6908 4709 28 28".to_string()));
/// assert_eq!(format_voter_id("163204010922"), Some("1632 0401 09 22".to_string()));
/// assert_eq!(format_voter_id("123456789012"), None);
/// ```
pub fn format_voter_id(voter_id: &str) -> Option<String> {
    if !is_valid(voter_id) {
        return None;
    }

    Some(format!(
        "{} {} {} {}",
        &voter_id[0..4],
        &voter_id[4..8],
        &voter_id[8..10],
        &voter_id[10..12]
    ))
}

/// Generates a random valid Brazilian voter registration.
///
/// # Arguments
///
/// * `federative_union` - Optional UF code (e.g., "SP", "RJ"). Default is "ZZ" for foreigners.
///
/// # Returns
///
/// `Some(String)` with a valid voter ID, or `None` if the UF is invalid
///
/// # Example
///
/// ```
/// use brazilian_utils::voter_id::{generate, is_valid};
///
/// let voter_id = generate(Some("SP")).unwrap();
/// assert_eq!(voter_id.len(), 12);
/// assert!(is_valid(&voter_id));
///
/// let voter_id_zz = generate(None).unwrap();
/// assert!(is_valid(&voter_id_zz));
/// ```
pub fn generate(federative_union: Option<&str>) -> Option<String> {
    let ufs = get_uf_map();

    let uf = federative_union.unwrap_or("ZZ").to_uppercase();

    if let Some(uf_number) = ufs.get(uf.as_str()) {
        if is_federative_union_valid(uf_number) {
            let mut rng = rand::thread_rng();
            let sequential_number = format!("{:08}", rng.gen_range(0..100000000));

            let vd1 = calculate_vd1(&sequential_number, uf_number);
            let vd2 = calculate_vd2(uf_number, vd1);

            return Some(format!("{}{}{}{}", sequential_number, uf_number, vd1, vd2));
        }
    }

    None
}

/// Check if the length of the voter ID is valid.
///
/// Typically 12 digits, but can be 13 for SP and MG (edge case with 9-digit sequential number).
fn is_length_valid(voter_id: &str) -> bool {
    let len = voter_id.len();

    if len == 12 {
        return true;
    }

    // Edge case: SP and MG with 9-digit sequential number
    if len == 13 {
        let federative_union = get_federative_union(voter_id);
        return federative_union == "01" || federative_union == "02";
    }

    false
}

/// Get the sequential number (first 8 digits) from a voter ID.
fn get_sequential_number(voter_id: &str) -> String {
    voter_id[..8].to_string()
}

/// Get the federative union code (2 digits) from a voter ID.
///
/// Indexed backwards since sequential number can be 8 or 9 digits.
fn get_federative_union(voter_id: &str) -> String {
    let len = voter_id.len();
    voter_id[len - 4..len - 2].to_string()
}

/// Get the verifying digits (last 2 digits) from a voter ID.
fn get_verifying_digits(voter_id: &str) -> String {
    let len = voter_id.len();
    voter_id[len - 2..].to_string()
}

/// Check if a federative union code is valid (01-28).
fn is_federative_union_valid(federative_union: &str) -> bool {
    if let Ok(num) = federative_union.parse::<u8>() {
        (1..=28).contains(&num)
    } else {
        false
    }
}

/// Calculate the first verifying digit.
///
/// Uses weights [2, 3, 4, 5, 6, 7, 8, 9] for the 8-digit sequential number.
pub fn calculate_vd1(sequential_number: &str, federative_union: &str) -> u8 {
    if sequential_number.len() < 8 {
        return 0;
    }

    let weights = [2, 3, 4, 5, 6, 7, 8, 9];
    let mut sum = 0;

    for (i, weight) in weights.iter().enumerate() {
        if let Some(digit) = sequential_number
            .chars()
            .nth(i)
            .and_then(|c| c.to_digit(10))
        {
            sum += digit * weight;
        }
    }

    let rest = (sum % 11) as u8;
    let mut vd1 = rest;

    // Edge case: rest == 0 and UF is SP (01) or MG (02)
    if rest == 0 && (federative_union == "01" || federative_union == "02") {
        vd1 = 1;
    }

    // Edge case: rest == 10
    if rest == 10 {
        vd1 = 0;
    }

    vd1
}

/// Calculate the second verifying digit.
///
/// Uses weights [7, 8, 9] for the federative union digits and first verifying digit.
pub fn calculate_vd2(federative_union: &str, vd1: u8) -> u8 {
    if federative_union.len() < 2 {
        return 0;
    }

    let weights = [7, 8, 9];
    let mut sum = 0;

    if let Some(d1) = federative_union.chars().next().and_then(|c| c.to_digit(10)) {
        sum += d1 * weights[0];
    }
    if let Some(d2) = federative_union.chars().nth(1).and_then(|c| c.to_digit(10)) {
        sum += d2 * weights[1];
    }
    sum += vd1 as u32 * weights[2];

    let rest = (sum % 11) as u8;
    let mut vd2 = rest;

    // Edge case: rest == 0 and UF is SP (01) or MG (02)
    if rest == 0 && (federative_union == "01" || federative_union == "02") {
        vd2 = 1;
    }

    // Edge case: rest == 10
    if rest == 10 {
        vd2 = 0;
    }

    vd2
}

/// Get the mapping of UF codes to their numeric representations.
fn get_uf_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("SP", "01");
    map.insert("MG", "02");
    map.insert("RJ", "03");
    map.insert("RS", "04");
    map.insert("BA", "05");
    map.insert("PR", "06");
    map.insert("CE", "07");
    map.insert("PE", "08");
    map.insert("SC", "09");
    map.insert("GO", "10");
    map.insert("MA", "11");
    map.insert("PB", "12");
    map.insert("PA", "13");
    map.insert("ES", "14");
    map.insert("PI", "15");
    map.insert("RN", "16");
    map.insert("AL", "17");
    map.insert("MT", "18");
    map.insert("MS", "19");
    map.insert("DF", "20");
    map.insert("SE", "21");
    map.insert("AM", "22");
    map.insert("RO", "23");
    map.insert("AC", "24");
    map.insert("AP", "25");
    map.insert("RR", "26");
    map.insert("TO", "27");
    map.insert("ZZ", "28"); // For foreigners
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // Valid voter IDs
        assert!(is_valid("690847092828"));
        assert!(is_valid("163204010922"));

        // Invalid: wrong check digits
        assert!(!is_valid("123456789012"));
        assert!(!is_valid("690847092829"));

        // Invalid: wrong length
        assert!(!is_valid("123"));
        assert!(!is_valid("12345678901234"));

        // Invalid: contains non-digits
        assert!(!is_valid("6908470928a8"));

        // Invalid: empty
        assert!(!is_valid(""));
    }

    #[test]
    fn test_format_voter_id() {
        assert_eq!(
            format_voter_id("690847092828"),
            Some("6908 4709 28 28".to_string())
        );
        assert_eq!(
            format_voter_id("163204010922"),
            Some("1632 0401 09 22".to_string())
        );
        assert_eq!(format_voter_id("123456789012"), None);
        assert_eq!(format_voter_id("123"), None);
    }

    #[test]
    fn test_generate() {
        // Generate for SP
        let voter_id = generate(Some("SP")).unwrap();
        assert_eq!(voter_id.len(), 12);
        assert!(is_valid(&voter_id));
        assert_eq!(get_federative_union(&voter_id), "01");

        // Generate for default (ZZ)
        let voter_id_zz = generate(None).unwrap();
        assert_eq!(voter_id_zz.len(), 12);
        assert!(is_valid(&voter_id_zz));
        assert_eq!(get_federative_union(&voter_id_zz), "28");

        // Invalid UF
        assert_eq!(generate(Some("XX")), None);
    }

    #[test]
    fn test_generate_uniqueness() {
        let mut voter_ids = std::collections::HashSet::new();
        for _ in 0..100 {
            let voter_id = generate(Some("RJ")).unwrap();
            assert!(is_valid(&voter_id));
            voter_ids.insert(voter_id);
        }
        // Should have generated at least 95 unique voter IDs
        assert!(voter_ids.len() >= 95);
    }

    #[test]
    fn test_calculate_vd1() {
        // Test with known valid voter ID: 690847092828
        assert_eq!(calculate_vd1("69084709", "28"), 2);

        // Test with known valid voter ID: 163204010922
        assert_eq!(calculate_vd1("16320401", "09"), 2);
    }

    #[test]
    fn test_calculate_vd2() {
        // Test with known valid voter ID: 690847092828
        assert_eq!(calculate_vd2("28", 2), 8);

        // Test with known valid voter ID: 163204010922
        assert_eq!(calculate_vd2("09", 2), 2);
    }

    #[test]
    fn test_get_sequential_number() {
        assert_eq!(get_sequential_number("690847092828"), "69084709");
        assert_eq!(get_sequential_number("163204010922"), "16320401");
    }

    #[test]
    fn test_get_federative_union() {
        assert_eq!(get_federative_union("690847092828"), "28");
        assert_eq!(get_federative_union("163204010922"), "09");
    }

    #[test]
    fn test_get_verifying_digits() {
        assert_eq!(get_verifying_digits("690847092828"), "28");
        assert_eq!(get_verifying_digits("163204010922"), "22");
    }

    #[test]
    fn test_is_federative_union_valid() {
        assert!(is_federative_union_valid("01"));
        assert!(is_federative_union_valid("28"));
        assert!(!is_federative_union_valid("00"));
        assert!(!is_federative_union_valid("29"));
        assert!(!is_federative_union_valid("XX"));
    }

    #[test]
    fn test_is_length_valid() {
        assert!(is_length_valid("690847092828")); // 12 digits
        assert!(!is_length_valid("123")); // Too short
        assert!(!is_length_valid("12345678901234")); // Too long (14)
    }

    #[test]
    fn test_edge_case_sp_mg() {
        // Test edge case for SP (01) and MG (02) where rest = 0
        // When rest % 11 = 0, VD should be 1 for SP and MG

        // Generate some voter IDs for SP and MG
        for _ in 0..10 {
            let voter_id_sp = generate(Some("SP")).unwrap();
            assert!(is_valid(&voter_id_sp));

            let voter_id_mg = generate(Some("MG")).unwrap();
            assert!(is_valid(&voter_id_mg));
        }
    }
}
