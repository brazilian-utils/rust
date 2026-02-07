pub mod cep;
pub mod cnh;
pub mod cnpj;
pub mod cpf;
pub mod currency;
pub mod date_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpf_module_accessible() {
        // Test that CPF module functions are accessible
        assert!(cpf::is_valid("11144477735"));
        assert!(!cpf::is_valid("00000000000"));
        
        // Test validate
        assert!(cpf::validate("82178537464"));
        assert!(!cpf::validate("12345678901"));
        
        // Test format_cpf
        assert_eq!(
            cpf::format_cpf("82178537464"),
            Some("821.785.374-64".to_string())
        );
        assert_eq!(cpf::format_cpf("00000000000"), None);
        
        // Test remove_symbols
        assert_eq!(cpf::remove_symbols("821.785.374-64"), "82178537464");
        
        // Test generate
        let generated = cpf::generate();
        assert_eq!(generated.len(), 11);
        assert!(cpf::is_valid(&generated));
        
        // Test hashdigit
        assert_eq!(cpf::hashdigit("52599927765", 10), 6);
        assert_eq!(cpf::hashdigit("52599927765", 11), 5);
        
        // Test compute_checksum
        assert_eq!(cpf::compute_checksum("525131277"), "65");
    }

    #[test]
    fn test_cep_module_accessible() {
        // Test that CEP module functions are accessible
        assert!(cep::is_valid("01310200"));
        assert!(!cep::is_valid("12345"));
        
        assert_eq!(cep::format_cep("01310200"), Some("01310-200".to_string()));
        assert_eq!(cep::remove_symbols("01310-200"), "01310200");
        
        let generated = cep::generate();
        assert!(cep::is_valid(&generated));
    }

    #[test]
    fn test_cnh_module_accessible() {
        // Test that CNH module functions are accessible
        assert!(cnh::is_valid_cnh("09770304734"));
        assert!(!cnh::is_valid_cnh("00000000000"));
    }

    #[test]
    fn test_cnpj_module_accessible() {
        // Test that CNPJ module functions are accessible
        assert!(cnpj::is_valid("03560714000142"));
        assert!(!cnpj::is_valid("00000000000000"));
        
        assert_eq!(
            cnpj::format_cnpj("03560714000142"),
            Some("03.560.714/0001-42".to_string())
        );
        assert_eq!(cnpj::remove_symbols("03.560.714/0001-42"), "03560714000142");
        
        let generated = cnpj::generate(None);
        assert!(cnpj::is_valid(&generated));
        assert_eq!(generated.len(), 14);
    }

    #[test]
    fn test_currency_module_accessible() {
        // Test that Currency module functions are accessible
        assert_eq!(
            currency::format_currency(1234.56),
            Some("R$ 1.234,56".to_string())
        );
        assert_eq!(
            currency::format_currency(0.0),
            Some("R$ 0,00".to_string())
        );
        assert_eq!(
            currency::format_currency(-9876.54),
            Some("R$ -9.876,54".to_string())
        );
        assert_eq!(currency::format_currency(f64::NAN), None);
    }

    #[test]
    fn test_date_utils_module_accessible() {
        use chrono::NaiveDate;
        
        // Test convert_date_to_text
        assert_eq!(
            date_utils::convert_date_to_text("01/01/2024"),
            Some("Primeiro de janeiro de dois mil e vinte e quatro".to_string())
        );
        assert_eq!(date_utils::convert_date_to_text("invalid"), None);
        
        // Test is_holiday for national holidays
        let new_year = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert_eq!(date_utils::is_holiday(new_year, None), Some(true));
        
        let regular_day = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        assert_eq!(date_utils::is_holiday(regular_day, None), Some(false));
        
        // Test is_holiday with invalid UF
        assert_eq!(date_utils::is_holiday(new_year, Some("XX")), None);
    }
}

