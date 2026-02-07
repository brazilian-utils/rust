pub mod cep;
pub mod cnh;
pub mod cnpj;
pub mod cpf;
pub mod currency;
pub mod date_utils;
pub mod email;
pub mod legal_nature;
pub mod legal_process;
pub mod license_plate;
pub mod phone;

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

    #[test]
    fn test_email_module_accessible() {
        // Test valid emails
        assert!(email::is_valid("brutils@brutils.com"));
        assert!(email::is_valid("user.name+tag@example.co.uk"));
        assert!(email::is_valid("user@example.com"));
        
        // Test invalid emails
        assert!(!email::is_valid("invalid-email@brutils"));
        assert!(!email::is_valid(".invalid@example.com"));
        assert!(!email::is_valid(""));
        assert!(!email::is_valid("user@"));
        assert!(!email::is_valid("@example.com"));
    }

    #[test]
    fn test_legal_nature_module_accessible() {
        // Test valid codes
        assert!(legal_nature::is_valid("2062"));
        assert!(legal_nature::is_valid("206-2"));
        assert!(legal_nature::is_valid("1015"));
        
        // Test invalid codes
        assert!(!legal_nature::is_valid("9999"));
        assert!(!legal_nature::is_valid("0000"));
        assert!(!legal_nature::is_valid(""));
        
        // Test get_description
        assert_eq!(
            legal_nature::get_description("2062"),
            Some("Sociedade Empresária Limitada")
        );
        assert_eq!(
            legal_nature::get_description("101-5"),
            Some("Órgão Público do Poder Executivo Federal")
        );
        assert_eq!(legal_nature::get_description("9999"), None);
        
        // Test list_all
        let table = legal_nature::list_all();
        assert!(table.len() > 40);
        assert_eq!(
            table.get("2062"),
            Some(&"Sociedade Empresária Limitada".to_string())
        );
    }

    #[test]
    fn test_legal_process_module_accessible() {
        // Test remove_symbols
        assert_eq!(
            legal_process::remove_symbols("6439067-89.2023.4.04.5902"),
            "64390678920234045902"
        );
        
        // Test format_legal_process
        assert_eq!(
            legal_process::format_legal_process("23141945820055070079"),
            Some("2314194-58.2005.5.07.0079".to_string())
        );
        assert_eq!(legal_process::format_legal_process("123"), None);
        
        // Test is_valid
        assert!(legal_process::is_valid("10188748220234018200"));
        assert!(legal_process::is_valid("45532346920234025107"));
        assert!(!legal_process::is_valid("00000000000000000000"));
        assert!(!legal_process::is_valid("123"));
        
        // Test generate
        let id = legal_process::generate(None, Some(5));
        assert!(id.is_some());
        assert_eq!(id.unwrap().len(), 20);
    }

    #[test]
    fn test_license_plate_module_accessible() {
        // Test remove_symbols
        assert_eq!(license_plate::remove_symbols("ABC-1234"), "ABC1234");
        
        // Test format_license_plate
        assert_eq!(
            license_plate::format_license_plate("ABC1234"),
            Some("ABC-1234".to_string())
        );
        assert_eq!(
            license_plate::format_license_plate("ABC1D23"),
            Some("ABC1D23".to_string())
        );
        
        // Test is_valid
        assert!(license_plate::is_valid("ABC1234", None));
        assert!(license_plate::is_valid("ABC1D23", None));
        assert!(!license_plate::is_valid("ABC123", None));
        
        // Test get_format
        assert_eq!(
            license_plate::get_format("ABC1234"),
            Some("LLLNNNN".to_string())
        );
        assert_eq!(
            license_plate::get_format("ABC1D23"),
            Some("LLLNLNN".to_string())
        );
        
        // Test convert_to_mercosul
        assert_eq!(
            license_plate::convert_to_mercosul("ABC1234"),
            Some("ABC1C34".to_string())
        );
        
        // Test generate
        let plate = license_plate::generate(None);
        assert!(plate.is_some());
        assert_eq!(plate.unwrap().len(), 7);
    }

    #[test]
    fn test_phone_module_accessible() {
        // Test remove_symbols
        assert_eq!(phone::remove_symbols("(11)99402-9275"), "11994029275");
        
        // Test is_valid
        assert!(phone::is_valid("11994029275", None));
        assert!(phone::is_valid("1635014415", None));
        assert!(phone::is_valid("11994029275", Some("mobile")));
        assert!(phone::is_valid("1635014415", Some("landline")));
        assert!(!phone::is_valid("123", None));
        
        // Test format_phone
        assert_eq!(
            phone::format_phone("11994029275"),
            Some("(11)99402-9275".to_string())
        );
        assert_eq!(
            phone::format_phone("1635014415"),
            Some("(16)3501-4415".to_string())
        );
        assert_eq!(phone::format_phone("123"), None);
        
        // Test remove_international_dialing_code
        assert_eq!(
            phone::remove_international_dialing_code("5511994029275"),
            "11994029275"
        );
        
        // Test generate
        let phone_number = phone::generate(None);
        assert!(phone_number.len() == 10 || phone_number.len() == 11);
        
        let mobile = phone::generate(Some("mobile"));
        assert_eq!(mobile.len(), 11);
        
        let landline = phone::generate(Some("landline"));
        assert_eq!(landline.len(), 10);
    }
}

