use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// TYPES
// ======

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Address {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub ibge: String,
    #[serde(default)]
    pub gia: String,
    pub ddd: String,
    pub siafi: String,
}

// ERRORS
// ======

#[derive(Debug)]
pub struct InvalidCEP {
    pub cep: String,
}

impl fmt::Display for InvalidCEP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CEP '{}' is invalid.", self.cep)
    }
}

impl Error for InvalidCEP {}

#[derive(Debug)]
pub struct CEPNotFound {
    pub message: String,
}

impl fmt::Display for CEPNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CEPNotFound {}

// FORMATTING
// ==========

/// Removes specific symbols from a given CEP (Postal Code).
///
/// This function takes a CEP (Postal Code) as input and removes all occurrences
/// of the '.' and '-' characters from it.
///
/// # Arguments
///
/// * `dirty` - The input CEP (Postal Code) containing symbols to be removed.
///
/// # Returns
///
/// A new string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cep::remove_symbols;
///
/// assert_eq!(remove_symbols("123-45.678.9"), "123456789");
/// assert_eq!(remove_symbols("abc.xyz"), "abcxyz");
/// ```
pub fn remove_symbols(dirty: &str) -> String {
    dirty.chars().filter(|c| *c != '.' && *c != '-').collect()
}

/// Formats a Brazilian CEP (Postal Code) into a standard format.
///
/// This function takes a CEP (Postal Code) as input and, if it is a valid
/// 8-digit CEP, formats it into the standard "12345-678" format.
///
/// # Arguments
///
/// * `cep` - The input CEP (Postal Code) to be formatted.
///
/// # Returns
///
/// The formatted CEP in the "12345-678" format if it's valid, None if it's not valid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cep::format_cep;
///
/// assert_eq!(format_cep("12345678"), Some("12345-678".to_string()));
/// assert_eq!(format_cep("12345"), None);
/// ```
pub fn format_cep(cep: &str) -> Option<String> {
    if is_valid(cep) {
        Some(format!("{}-{}", &cep[0..5], &cep[5..8]))
    } else {
        None
    }
}

// OPERATIONS
// ==========

/// Checks if a CEP (Postal Code) is valid.
///
/// To be considered valid, the input must be a string containing exactly 8 digits.
/// This function does not verify if the CEP is a real postal code; it only
/// validates the format of the string.
///
/// # Arguments
///
/// * `cep` - The string containing the CEP to be checked.
///
/// # Returns
///
/// `true` if the CEP is valid (8 digits), `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cep::is_valid;
///
/// assert_eq!(is_valid("12345678"), true);
/// assert_eq!(is_valid("12345"), false);
/// assert_eq!(is_valid("abcdefgh"), false);
/// ```
///
/// # Source
///
/// <https://en.wikipedia.org/wiki/Código_de_Endereçamento_Postal>
pub fn is_valid(cep: &str) -> bool {
    cep.len() == 8 && cep.chars().all(|c| c.is_ascii_digit())
}

/// Generates a random 8-digit CEP (Postal Code) number as a string.
///
/// # Returns
///
/// A randomly generated 8-digit CEP string.
///
/// # Examples
///
/// ```
/// use brazilian_utils::cep::{generate, is_valid};
///
/// let cep = generate();
/// assert_eq!(cep.len(), 8);
/// assert!(is_valid(&cep));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect()
}

// API FUNCTIONS
// =============

/// Fetches address information from a given CEP (Postal Code) using the ViaCEP API.
///
/// # Arguments
///
/// * `cep` - The CEP (Postal Code) to be used in the search.
/// * `raise_exceptions` - Whether to raise exceptions when the CEP is invalid or not found.
///
/// # Returns
///
/// An `Address` struct containing the address information if the CEP is found,
/// `None` otherwise.
///
/// # Errors
///
/// * `InvalidCEP` - When the input CEP is invalid and `raise_exceptions` is `true`.
/// * `CEPNotFound` - When the input CEP is not found and `raise_exceptions` is `true`.
///
/// # Examples
///
/// ```no_run
/// use brazilian_utils::cep::get_address_from_cep;
///
/// match get_address_from_cep("01310200", false) {
///     Ok(Some(address)) => println!("CEP: {}", address.cep),
///     Ok(None) => println!("CEP not found"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Reference
///
/// <https://viacep.com.br/>
pub fn get_address_from_cep(
    cep: &str,
    raise_exceptions: bool,
) -> Result<Option<Address>, Box<dyn Error>> {
    let base_api_url = "https://viacep.com.br/ws/{}/json/";

    let clean_cep = remove_symbols(cep);
    let cep_is_valid = is_valid(&clean_cep);

    if !cep_is_valid {
        if raise_exceptions {
            return Err(Box::new(InvalidCEP {
                cep: cep.to_string(),
            }));
        }
        return Ok(None);
    }

    let url = base_api_url.replace("{}", &clean_cep);

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            let json: serde_json::Value = response.json()?;

            if json.get("erro").is_some() {
                if raise_exceptions {
                    return Err(Box::new(CEPNotFound {
                        message: cep.to_string(),
                    }));
                }
                return Ok(None);
            }

            let address: Address = serde_json::from_value(json)?;
            Ok(Some(address))
        }
        Err(_e) => {
            if raise_exceptions {
                return Err(Box::new(CEPNotFound {
                    message: cep.to_string(),
                }));
            }
            Ok(None)
        }
    }
}

/// Fetches CEP (Postal Code) options from a given address using the ViaCEP API.
///
/// # Arguments
///
/// * `federal_unit` - The two-letter abbreviation of the Brazilian state.
/// * `city` - The name of the city.
/// * `street` - The name (or substring) of the street.
/// * `raise_exceptions` - Whether to raise exceptions when the address is invalid or not found.
///
/// # Returns
///
/// A list of `Address` structs containing the address information if the address is found,
/// `None` otherwise.
///
/// # Errors
///
/// * `ValueError` - When the input UF is invalid and `raise_exceptions` is `true`.
/// * `CEPNotFound` - When the input address is not found and `raise_exceptions` is `true`.
///
/// # Examples
///
/// ```no_run
/// use brazilian_utils::cep::get_cep_information_from_address;
///
/// match get_cep_information_from_address("SP", "São Paulo", "Avenida Paulista", false) {
///     Ok(Some(addresses)) => {
///         for addr in addresses {
///             println!("CEP: {}", addr.cep);
///         }
///     }
///     Ok(None) => println!("Address not found"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Reference
///
/// <https://viacep.com.br/>
pub fn get_cep_information_from_address(
    federal_unit: &str,
    city: &str,
    street: &str,
    raise_exceptions: bool,
) -> Result<Option<Vec<Address>>, Box<dyn Error>> {
    // Valid Brazilian state abbreviations
    const VALID_UFS: &[&str] = &[
        "AC", "AL", "AP", "AM", "BA", "CE", "DF", "ES", "GO", "MA", "MT", "MS", "MG", "PA", "PB",
        "PR", "PE", "PI", "RJ", "RN", "RS", "RO", "RR", "SC", "SP", "SE", "TO",
    ];

    let federal_unit_upper = federal_unit.to_uppercase();

    if !VALID_UFS.contains(&federal_unit_upper.as_str()) {
        if raise_exceptions {
            return Err(format!("Invalid UF: {}", federal_unit).into());
        }
        return Ok(None);
    }

    let base_api_url = "https://viacep.com.br/ws/{}/{}/{}/json/";

    // Normalize strings: remove accents and replace spaces with %20
    let parsed_city = normalize_string(city);
    let parsed_street = normalize_string(street);

    let url = base_api_url
        .replace("{}", &federal_unit_upper)
        .replacen("{}", &parsed_city, 1)
        .replacen("{}", &parsed_street, 1);

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            let addresses: Vec<Address> = response.json()?;

            if addresses.is_empty() {
                if raise_exceptions {
                    return Err(Box::new(CEPNotFound {
                        message: format!("{} - {} - {}", federal_unit, city, street),
                    }));
                }
                return Ok(None);
            }

            Ok(Some(addresses))
        }
        Err(_e) => {
            if raise_exceptions {
                return Err(Box::new(CEPNotFound {
                    message: format!("{} - {} - {}", federal_unit, city, street),
                }));
            }
            Ok(None)
        }
    }
}

// HELPER FUNCTIONS
// ================

/// Normalizes a string by removing accents and replacing spaces with %20
fn normalize_string(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;

    s.nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect::<String>()
        .replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("00000000"), "00000000");
        assert_eq!(remove_symbols("01310-200"), "01310200");
        assert_eq!(remove_symbols("01..310.-200.-"), "01310200");
        assert_eq!(remove_symbols("abc01310200*!*&#"), "abc01310200*!*&#");
        assert_eq!(
            remove_symbols("ab.c1.--.3-102.-0-.0-.*.-!*&#"),
            "abc1310200*!*&#"
        );
        assert_eq!(remove_symbols("...---..."), "");
    }

    #[test]
    fn test_is_valid() {
        // When CEP's len is different of 8, returns False
        assert!(!is_valid("1"));
        assert!(!is_valid("12345"));
        assert!(!is_valid("123456789"));

        // When CEP does not contain only digits, returns False
        assert!(!is_valid("1234567-"));
        assert!(!is_valid("abcdefgh"));
        assert!(!is_valid("1234567a"));

        // When CEP is valid
        assert!(is_valid("99999999"));
        assert!(is_valid("88390000"));
        assert!(is_valid("01310200"));
        assert!(is_valid("12345678"));
        assert!(is_valid("00000000"));
    }

    #[test]
    fn test_format_cep() {
        // Valid CEPs should be formatted
        assert_eq!(format_cep("01310200"), Some("01310-200".to_string()));
        assert_eq!(format_cep("12345678"), Some("12345-678".to_string()));
        assert_eq!(format_cep("00000000"), Some("00000-000".to_string()));
        assert_eq!(format_cep("99999999"), Some("99999-999".to_string()));

        // Invalid CEPs should return None
        assert_eq!(format_cep("12345"), None);
        assert_eq!(format_cep("013102009"), None);
        assert_eq!(format_cep("abcdefgh"), None);
        assert_eq!(format_cep("1234567-"), None);
        assert_eq!(format_cep(""), None);
    }

    #[test]
    fn test_generate() {
        // Test that generate creates valid CEPs
        for _ in 0..1000 {
            let cep = generate();
            assert_eq!(cep.len(), 8);
            assert!(is_valid(&cep));
            assert!(cep.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_normalize_string() {
        // Test accent removal
        assert_eq!(normalize_string("São Paulo"), "Sao%20Paulo");
        assert_eq!(normalize_string("Brasília"), "Brasilia");
        assert_eq!(normalize_string("Goiânia"), "Goiania");

        // Test space to %20 conversion
        assert_eq!(normalize_string("Belo Horizonte"), "Belo%20Horizonte");
        assert_eq!(normalize_string("Rio de Janeiro"), "Rio%20de%20Janeiro");

        // Test combined
        assert_eq!(normalize_string("João Pessoa"), "Joao%20Pessoa");
    }

    #[test]
    fn test_remove_symbols_empty() {
        assert_eq!(remove_symbols(""), "");
    }

    #[test]
    fn test_remove_symbols_only_symbols() {
        assert_eq!(remove_symbols(".-.-.-"), "");
        assert_eq!(remove_symbols("..."), "");
        assert_eq!(remove_symbols("---"), "");
    }

    #[test]
    fn test_is_valid_edge_cases() {
        // Empty string
        assert!(!is_valid(""));

        // Only zeros
        assert!(is_valid("00000000"));

        // Only nines
        assert!(is_valid("99999999"));
    }

    #[test]
    fn test_format_cep_with_cleaned_input() {
        // Even if input has symbols, format_cep expects clean input
        // So these should fail format validation
        assert_eq!(format_cep("01310-200"), None); // contains dash
    }
}
