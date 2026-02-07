pub mod cep;
pub mod cnh;
pub mod cnpj;
pub mod cpf;

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
}

