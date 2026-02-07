/// Legal Nature (Natureza Jurídica) utilities for Brazilian companies.
///
/// This module provides utilities for consulting and validating the official
/// *Natureza Jurídica* (Legal Nature) codes defined by the Receita Federal do Brasil (RFB).
///
/// The codes and descriptions in this module are sourced from the official
/// **Tabela de Natureza Jurídica** (RFB), as provided in the document used
/// by the Cadastro Nacional (e.g., FCN).
///
/// Source: https://www.gov.br/empresas-e-negocios/pt-br/drei/links-e-downloads/arquivos/TABELADENATUREZAJURDICA.pdf
use std::collections::HashMap;
use std::sync::OnceLock;

/// Get the complete legal nature codes table.
fn legal_nature_table() -> &'static HashMap<&'static str, &'static str> {
    static TABLE: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();

        // 1. ADMINISTRAÇÃO PÚBLICA
        map.insert("1015", "Órgão Público do Poder Executivo Federal");
        map.insert(
            "1023",
            "Órgão Público do Poder Executivo Estadual ou do Distrito Federal",
        );
        map.insert("1031", "Órgão Público do Poder Executivo Municipal");
        map.insert("1040", "Órgão Público do Poder Legislativo Federal");
        map.insert(
            "1058",
            "Órgão Público do Poder Legislativo Estadual ou do Distrito Federal",
        );
        map.insert("1066", "Órgão Público do Poder Legislativo Municipal");
        map.insert("1074", "Órgão Público do Poder Judiciário Federal");
        map.insert("1082", "Órgão Público do Poder Judiciário Estadual");
        map.insert("1104", "Autarquia Federal");
        map.insert("1112", "Autarquia Estadual ou do Distrito Federal");
        map.insert("1120", "Autarquia Municipal");
        map.insert("1139", "Fundação Federal");
        map.insert("1147", "Fundação Estadual ou do Distrito Federal");
        map.insert("1155", "Fundação Municipal");
        map.insert("1163", "Órgão Público Autônomo da União");
        map.insert(
            "1171",
            "Órgão Público Autônomo Estadual ou do Distrito Federal",
        );
        map.insert("1180", "Órgão Público Autônomo Municipal");

        // 2. ENTIDADES EMPRESARIAIS
        map.insert("2011", "Empresa Pública");
        map.insert("2038", "Sociedade de Economia Mista");
        map.insert("2046", "Sociedade Anônima Aberta");
        map.insert("2054", "Sociedade Anônima Fechada");
        map.insert("2062", "Sociedade Empresária Limitada");
        map.insert("2070", "Sociedade Empresária em Nome Coletivo");
        map.insert("2089", "Sociedade Empresária em Comandita Simples");
        map.insert("2097", "Sociedade Empresária em Comandita por Ações");
        map.insert(
            "2100",
            "Sociedade Mercantil de Capital e Indústria (extinta pelo NCC/2002)",
        );
        map.insert("2127", "Sociedade Empresária em Conta de Participação");
        map.insert("2135", "Empresário (Individual)");
        map.insert("2143", "Cooperativa");
        map.insert("2151", "Consórcio de Sociedades");
        map.insert("2160", "Grupo de Sociedades");
        map.insert(
            "2178",
            "Estabelecimento, no Brasil, de Sociedade Estrangeira",
        );
        map.insert(
            "2194",
            "Estabelecimento, no Brasil, de Empresa Binacional Argentino-Brasileira",
        );
        map.insert("2208", "Entidade Binacional Itaipu");
        map.insert("2216", "Empresa Domiciliada no Exterior");
        map.insert("2224", "Clube/Fundo de Investimento");
        map.insert("2232", "Sociedade Simples Pura");
        map.insert("2240", "Sociedade Simples Limitada");
        map.insert("2259", "Sociedade em Nome Coletivo");
        map.insert("2267", "Sociedade em Comandita Simples");
        map.insert("2275", "Sociedade Simples em Conta de Participação");
        map.insert("2305", "Empresa Individual de Responsabilidade Limitada");

        // 3. ENTIDADES SEM FINS LUCRATIVOS
        map.insert("3034", "Serviço Notarial e Registral (Cartório)");
        map.insert("3042", "Organização Social");
        map.insert(
            "3050",
            "Organização da Sociedade Civil de Interesse Público (Oscip)",
        );
        map.insert(
            "3069",
            "Outras Formas de Fundações Mantidas com Recursos Privados",
        );
        map.insert("3077", "Serviço Social Autônomo");
        map.insert("3085", "Condomínio Edilícios");
        map.insert(
            "3093",
            "Unidade Executora (Programa Dinheiro Direto na Escola)",
        );
        map.insert("3107", "Comissão de Conciliação Prévia");
        map.insert("3115", "Entidade de Mediação e Arbitragem");
        map.insert("3123", "Partido Político");
        map.insert("3131", "Entidade Sindical");
        map.insert(
            "3204",
            "Estabelecimento, no Brasil, de Fundação ou Associação Estrangeiras",
        );
        map.insert("3212", "Fundação ou Associação Domiciliada no Exterior");
        map.insert("3999", "Outras Formas de Associação");

        // 4. PESSOAS FÍSICAS
        map.insert("4014", "Empresa Individual Imobiliária");
        map.insert("4022", "Segurado Especial");
        map.insert("4081", "Contribuinte individual");

        // 5. ORGANIZAÇÕES INTERNACIONAIS E OUTRAS INSTITUIÇÕES EXTRATERRITORIAIS
        map.insert(
            "5002",
            "Organização Internacional e Outras Instituições Extraterritoriais",
        );

        map
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

/// Check if a string corresponds to a valid *Natureza Jurídica* (Legal Nature) code.
///
/// This function validates a legal nature code according to the official table from
/// Receita Federal do Brasil (RFB). The code is normalized before validation,
/// accepting formats like "2062" or "206-2".
///
/// # Arguments
///
/// * `code` - The code to be validated. Accepts either "NNNN" or "NNN-N".
///
/// # Returns
///
/// Returns `true` if the normalized code exists in the official table, `false` otherwise.
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
///
/// # Note
///
/// Validation is based solely on the presence of the code in the official RFB table.
/// It does not verify the current legal status or registration of the entity.
pub fn is_valid(code: &str) -> bool {
    if let Some(normalized) = normalize(code) {
        legal_nature_table().contains_key(normalized.as_str())
    } else {
        false
    }
}

/// Retrieve the description of a *Natureza Jurídica* (Legal Nature) code.
///
/// This function returns the official description for a given legal nature code
/// from the Receita Federal do Brasil (RFB) table.
///
/// # Arguments
///
/// * `code` - The code to look up. Accepts either "NNNN" or "NNN-N".
///
/// # Returns
///
/// The full description if the code is valid, otherwise `None`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::get_description;
///
/// assert_eq!(get_description("2062"), Some("Sociedade Empresária Limitada"));
/// assert_eq!(get_description("101-5"), Some("Órgão Público do Poder Executivo Federal"));
/// assert_eq!(get_description("0000"), None);
/// ```
pub fn get_description(code: &str) -> Option<&'static str> {
    if let Some(normalized) = normalize(code) {
        legal_nature_table().get(normalized.as_str()).copied()
    } else {
        None
    }
}

/// Return a copy of the full *Natureza Jurídica* (Legal Nature) table.
///
/// This function returns a HashMap containing all legal nature codes and their
/// corresponding descriptions from the official RFB table.
///
/// # Returns
///
/// A HashMap mapping 4-digit codes to their descriptions.
///
/// # Examples
///
/// ```
/// use brazilian_utils::legal_nature::list_all;
///
/// let table = list_all();
/// assert!(table.len() > 0);
/// assert_eq!(table.get("2062"), Some(&"Sociedade Empresária Limitada".to_string()));
/// ```
pub fn list_all() -> HashMap<String, String> {
    legal_nature_table()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_formats() {
        // Accept both "NNNN" and "NNN-N" formats
        assert!(is_valid("2062"));
        assert!(is_valid("206-2"));
        assert!(is_valid("101-5"));
        assert!(is_valid("1015"));
    }

    #[test]
    fn test_is_valid_known_codes() {
        // Known valid codes from different sections
        assert!(is_valid("1015")); // Administração Pública
        assert!(is_valid("2062")); // Entidades Empresariais
        assert!(is_valid("2143")); // Cooperativa
        assert!(is_valid("2305")); // EIRELI
        assert!(is_valid("3034")); // Entidades sem fins lucrativos
        assert!(is_valid("3131")); // Entidade Sindical
        assert!(is_valid("3212")); // Fundação no Exterior
        assert!(is_valid("4014")); // Pessoas Físicas
        assert!(is_valid("5002")); // Organizações Internacionais
    }

    #[test]
    fn test_is_valid_invalid_codes() {
        assert!(!is_valid("")); // Empty
        assert!(!is_valid("20")); // Too short
        assert!(!is_valid("20623")); // Too long
        assert!(!is_valid("abcd")); // Non-digits
        assert!(!is_valid("---")); // No digits
        assert!(!is_valid("9999")); // Not in table
        assert!(!is_valid("0000")); // Not in table
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
        assert_eq!(
            get_description("5002"),
            Some("Organização Internacional e Outras Instituições Extraterritoriais")
        );
    }

    #[test]
    fn test_get_description_invalid() {
        assert_eq!(get_description("9999"), None);
        assert_eq!(get_description("0000"), None);
        assert_eq!(get_description("20A2"), None);
        assert_eq!(get_description(""), None);
    }

    #[test]
    fn test_list_all() {
        let table = list_all();

        // Check that we have all codes
        assert!(table.len() > 40); // Should have around 50+ codes

        // Check a few known entries
        assert_eq!(
            table.get("2062"),
            Some(&"Sociedade Empresária Limitada".to_string())
        );
        assert_eq!(
            table.get("1015"),
            Some(&"Órgão Público do Poder Executivo Federal".to_string())
        );
    }

    #[test]
    fn test_list_all_returns_copy() {
        let mut table = list_all();
        assert_eq!(
            table.get("2062"),
            Some(&"Sociedade Empresária Limitada".to_string())
        );

        // Modify the copy
        table.insert("2062".to_string(), "X".to_string());

        // Original should be unchanged
        assert_eq!(
            get_description("2062"),
            Some("Sociedade Empresária Limitada")
        );
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("2062"), Some("2062".to_string()));
        assert_eq!(normalize("206-2"), Some("2062".to_string()));
        assert_eq!(normalize("20-62"), Some("2062".to_string()));
        assert_eq!(normalize("2 0 6 2"), Some("2062".to_string()));
        assert_eq!(normalize("20"), None); // Too short
        assert_eq!(normalize("20623"), None); // Too long
        assert_eq!(normalize("abcd"), None); // No digits
    }
}
