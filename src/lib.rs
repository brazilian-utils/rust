pub mod area_code;
pub mod bank;
pub mod bank_account;
pub mod boleto;
pub mod caepf;
pub mod cbo;
pub mod cei;
pub mod cep;
pub mod certidao;
pub mod cnae;
pub mod cnh;
pub mod cfop;
pub mod cno;
pub mod cns;
pub mod csosn;
pub mod cst;
pub mod municipality;
pub mod ncm;
pub mod state;
pub mod text;
pub mod credit_card;
pub mod iban;
pub mod passport;
pub mod registro_profissional;
pub mod vin;
pub mod cnpj;
pub mod cpf;
pub mod currency;
pub mod date_utils;
pub mod email;
pub mod ie;
pub mod legal_nature;
pub mod legal_process;
pub mod license_plate;
pub mod nfe_key;
pub mod number;
pub mod phone;
pub mod pix_key;
pub mod pix_payload;
pub mod pis;
pub mod renavam;
pub mod voter_id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boleto_module_accessible() {
        // Test that Boleto module functions are accessible
        assert!(boleto::is_valid("00190000090114971860168524522114675860000102656"));
        assert!(boleto::is_valid("0019000009 01149.718601 68524.522114 6 75860000102656"));
        assert!(!boleto::is_valid("00190000020114971860168524522114675860000102656"));
        assert!(!boleto::is_valid(""));
        
        // Test validate alias
        assert!(boleto::validate("00190000090114971860168524522114675860000102656"));
        assert!(!boleto::validate("000111"));
    }

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
        assert!(cnpj::is_valid("03560714000142", None));
        assert!(!cnpj::is_valid("00000000000000", None));

        assert_eq!(
            cnpj::format_cnpj("03560714000142"),
            Some("03.560.714/0001-42".to_string())
        );
        assert_eq!(cnpj::remove_symbols("03.560.714/0001-42"), "03560714000142");

        let generated = cnpj::generate(None, None);
        assert!(cnpj::is_valid(&generated, None));
        assert_eq!(generated.len(), 14);

        // v2 (alphanumeric, IN RFB 2.119)
        let generated_v2 = cnpj::generate(None, Some(2));
        assert!(cnpj::is_valid(&generated_v2, Some(2)));
        assert_eq!(generated_v2.len(), 14);
    }

    #[test]
    fn test_currency_module_accessible() {
        // Test that Currency module functions are accessible
        assert_eq!(
            currency::format_currency(1234.56),
            Some("1.234,56".to_string())
        );
        assert_eq!(currency::format_currency(0.0), Some("0,00".to_string()));
        assert_eq!(
            currency::format_currency(-9876.54),
            Some("-9.876,54".to_string())
        );
        assert_eq!(currency::format_currency(f64::NAN), None);
        assert_eq!(currency::parse("R$ 1.234,56"), 1234.56);
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
        assert_eq!(
            date_utils::is_holiday(Some(date_utils::IsHolidayParams {
                date: Some(new_year),
                uf: None,
            })),
            Some(true)
        );

        let regular_day = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        assert_eq!(
            date_utils::is_holiday(Some(date_utils::IsHolidayParams {
                date: Some(regular_day),
                uf: None,
            })),
            Some(false)
        );

        // Test is_holiday with invalid UF
        assert_eq!(
            date_utils::is_holiday(Some(date_utils::IsHolidayParams {
                date: Some(new_year),
                uf: Some("XX".to_string()),
            })),
            None
        );

        // Test convert_to_words and business-day helpers
        assert_eq!(
            date_utils::convert_to_words("01/01/2024"),
            "primeiro de janeiro de dois mil e vinte e quatro"
        );
        assert!(date_utils::is_business_day(regular_day, None));
        assert!(!date_utils::is_business_day(new_year, None));
        assert!(!date_utils::get_holidays(2024).is_empty());
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
        let table = legal_nature::list_all(None);
        assert!(table.len() > 40);
        assert_eq!(
            table.get("2062"),
            Some(&"Sociedade Empresária Limitada".to_string())
        );

        // Test get, list_by_category, format, generate, parse
        assert!(legal_nature::get("2062").unwrap().category.code == "2");
        assert_eq!(legal_nature::list_by_category("5", None).len(), 3);
        assert_eq!(legal_nature::format("2062"), "206-2");
        assert_eq!(legal_nature::parse("206-2"), "2062");
        let generated = legal_nature::generate();
        assert!(legal_nature::is_valid(&generated));
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
        assert!(legal_process::is_valid("10188748320234018200"));
        assert!(legal_process::is_valid("45532347020234025107"));
        assert!(!legal_process::is_valid("00000000000000000000"));
        assert!(!legal_process::is_valid("123"));

        // Test generate
        let id = legal_process::generate(Some(legal_process::GenerateProcessoJuridicoParams {
            year: None,
            court: Some(5),
        }));
        assert!(id.is_some());
        assert_eq!(id.unwrap().len(), 20);

        // Test parse
        assert_eq!(
            legal_process::parse("0002080-25.2012.5.15.0049"),
            "00020802520125150049"
        );
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
        assert!(license_plate::is_valid("ABC1234"));
        assert!(license_plate::is_valid("ABC1D23"));
        assert!(!license_plate::is_valid("ABC123"));

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
        assert_eq!(license_plate::convert_to_mercosul("ABC1234"), "ABC1C34");

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
        assert!(!phone::is_valid("123", None));

        // Test is_valid_mobile / is_valid_landline / is_valid_service
        assert!(phone::is_valid_mobile("11994029275", None));
        assert!(phone::is_valid_landline("1635014415"));
        assert!(phone::is_valid_service("0800 123 4567"));

        // Test format_phone
        assert_eq!(phone::format_phone("988887777"), "98888-7777");
        assert_eq!(phone::format_phone(""), "");

        // Test parse
        assert_eq!(phone::parse("(11) 98888-7777"), "11988887777");

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

    #[test]
    fn test_pis_module_accessible() {
        // Test remove_symbols
        assert_eq!(pis::remove_symbols("123.456.789-09"), "12345678909");

        // Test is_valid
        assert!(pis::is_valid("12345678900"));
        assert!(pis::is_valid("98765432103"));
        assert!(!pis::is_valid("12345678901"));
        assert!(!pis::is_valid("123"));

        // Test format_pis
        assert_eq!(
            pis::format_pis("12345678900"),
            Some("123.45678.90-0".to_string())
        );
        assert_eq!(
            pis::format_pis("98765432103"),
            Some("987.65432.10-3".to_string())
        );
        assert_eq!(pis::format_pis("123"), None);

        // Test generate
        let pis_number = pis::generate();
        assert_eq!(pis_number.len(), 11);
        assert!(pis::is_valid(&pis_number));
    }

    #[test]
    fn test_renavam_module_accessible() {
        // Test is_valid
        assert!(renavam::is_valid("86769597308"));
        assert!(renavam::is_valid("01234567897"));
        assert!(!renavam::is_valid("12345678901"));
        assert!(!renavam::is_valid("11111111111"));
        assert!(!renavam::is_valid("123"));

        // Test calculate_checksum
        assert_eq!(renavam::calculate_checksum("8676959730"), 8);
        assert_eq!(renavam::calculate_checksum("0123456789"), 7);

        // Test generate
        let renavam_number = renavam::generate();
        assert_eq!(renavam_number.len(), 11);
        assert!(renavam::is_valid(&renavam_number));
    }

    #[test]
    fn test_voter_id_module_accessible() {
        // Test is_valid
        assert!(voter_id::is_valid("690847092828"));
        assert!(voter_id::is_valid("163204010922"));
        assert!(!voter_id::is_valid("123456789012"));
        assert!(!voter_id::is_valid("123"));

        // Test format_voter_id
        assert_eq!(
            voter_id::format_voter_id("690847092828"),
            Some("6908 4709 28 28".to_string())
        );
        assert_eq!(
            voter_id::format_voter_id("163204010922"),
            Some("1632 0401 09 22".to_string())
        );
        assert_eq!(voter_id::format_voter_id("123"), None);

        // Test generate
        let voter_id_sp = voter_id::generate(Some("SP")).unwrap();
        assert_eq!(voter_id_sp.len(), 12);
        assert!(voter_id::is_valid(&voter_id_sp));

        let voter_id_default = voter_id::generate(None).unwrap();
        assert_eq!(voter_id_default.len(), 12);
        assert!(voter_id::is_valid(&voter_id_default));

        // Test calculate_vd1 and calculate_vd2
        assert_eq!(voter_id::calculate_vd1("69084709", "28"), 2);
        assert_eq!(voter_id::calculate_vd2("28", 2), 8);
    }
}
