use brazilian_utils::legal_nature::{is_valid, get_description, list_all};

fn main() {
    println!("=== Demonstração do Módulo Legal Nature (Natureza Jurídica) ===\n");
    
    // Validação de códigos
    println!("1. Validação de códigos:");
    let codes_to_test = vec![
        ("2062", true),
        ("206-2", true),
        ("1015", true),
        ("101-5", true),
        ("9999", false),
        ("0000", false),
        ("", false),
        ("abcd", false),
    ];
    
    for (code, expected) in &codes_to_test {
        let is_valid_result = is_valid(code);
        let status = if is_valid_result == *expected { "✓" } else { "✗" };
        let display_code = if code.is_empty() { "(vazio)" } else { code };
        println!("   {} {}: {} (esperado: {})", 
            status, 
            display_code,
            if is_valid_result { "VÁLIDO" } else { "INVÁLIDO" },
            if *expected { "VÁLIDO" } else { "INVÁLIDO" }
        );
    }
    
    // Obter descrições
    println!("\n2. Descrições de códigos válidos:");
    let codes_with_descriptions = vec![
        "2062",
        "1015",
        "2143",
        "2305",
        "3034",
        "3131",
        "4014",
        "5002",
    ];
    
    for code in &codes_with_descriptions {
        if let Some(description) = get_description(code) {
            println!("   {} - {}", code, description);
        }
    }
    
    // Códigos inválidos
    println!("\n3. Tentando obter descrições de códigos inválidos:");
    let invalid_codes = vec!["9999", "0000", "20A2"];
    for code in &invalid_codes {
        match get_description(code) {
            Some(desc) => println!("   {} - {}", code, desc),
            None => println!("   {} - (código inválido)", code),
        }
    }
    
    // Listando todas as naturezas jurídicas
    println!("\n4. Estatísticas da tabela:");
    let table = list_all();
    println!("   Total de códigos registrados: {}", table.len());
    
    // Contando por categoria (baseado no primeiro dígito)
    let mut categories: std::collections::HashMap<char, u32> = std::collections::HashMap::new();
    for code in table.keys() {
        if let Some(first_char) = code.chars().next() {
            *categories.entry(first_char).or_insert(0) += 1;
        }
    }
    
    println!("\n   Códigos por categoria:");
    let category_names = vec![
        ('1', "Administração Pública"),
        ('2', "Entidades Empresariais"),
        ('3', "Entidades sem Fins Lucrativos"),
        ('4', "Pessoas Físicas"),
        ('5', "Organizações Internacionais"),
    ];
    
    for (digit, name) in category_names {
        let count = categories.get(&digit).unwrap_or(&0);
        println!("   {} - {}: {} códigos", digit, name, count);
    }
    
    // Exemplos de códigos por seção
    println!("\n5. Exemplos de códigos por seção:");
    
    println!("\n   Administração Pública (1xxx):");
    for (code, desc) in table.iter().filter(|(k, _)| k.starts_with('1')).take(3) {
        println!("     {} - {}", code, desc);
    }
    
    println!("\n   Entidades Empresariais (2xxx):");
    for (code, desc) in table.iter().filter(|(k, _)| k.starts_with('2')).take(3) {
        println!("     {} - {}", code, desc);
    }
    
    println!("\n   Entidades sem Fins Lucrativos (3xxx):");
    for (code, desc) in table.iter().filter(|(k, _)| k.starts_with('3')).take(3) {
        println!("     {} - {}", code, desc);
    }
}
