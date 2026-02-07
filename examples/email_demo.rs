use brazilian_utils::email::is_valid;

fn main() {
    println!("=== Demonstração do Módulo Email ===\n");

    // Emails válidos
    println!("1. Emails válidos:");
    let valid_emails = vec![
        "brutils@brutils.com",
        "user@example.com",
        "user.name@example.com",
        "user+tag@example.com",
        "user_name@example.com",
        "user-name@example.com",
        "user123@example.com",
        "user@sub.example.com",
        "user@example.co.uk",
        "user.name+tag@example.co.uk",
    ];

    for email in &valid_emails {
        println!(
            "   ✓ {}: {}",
            email,
            if is_valid(email) {
                "VÁLIDO"
            } else {
                "INVÁLIDO"
            }
        );
    }

    // Emails inválidos
    println!("\n2. Emails inválidos:");
    let invalid_emails = vec![
        "",
        "userexample.com",        // Sem @
        "user@",                  // Sem domínio
        "@example.com",           // Sem parte local
        "user@example",           // Sem TLD
        "invalid-email@brutils",  // TLD inválido
        "user@example.c",         // TLD muito curto
        ".user@example.com",      // Começa com ponto
        "user@.example.com",      // Domínio começa com ponto
        "user@@example.com",      // Múltiplos @
        "user@exam@ple.com",      // Múltiplos @
        "user name@example.com",  // Espaço na parte local
        "user@exam ple.com",      // Espaço no domínio
        "user.@example.com",      // Termina com ponto antes do @
        "user..name@example.com", // Pontos consecutivos
        "user@example.com.",      // Termina com ponto
    ];

    for email in &invalid_emails {
        let display_email = if email.is_empty() { "(vazio)" } else { email };
        println!(
            "   ✗ {}: {}",
            display_email,
            if is_valid(email) {
                "VÁLIDO"
            } else {
                "INVÁLIDO"
            }
        );
    }

    // Casos especiais
    println!("\n3. Casos especiais com caracteres permitidos:");
    let special_emails = vec![
        ("user%test@example.com", true),
        ("user+mailbox@example.com", true),
        ("user#test@example.com", false),
        ("user!test@example.com", false),
    ];

    for (email, expected) in &special_emails {
        let is_valid_result = is_valid(email);
        let status = if is_valid_result == *expected {
            "✓"
        } else {
            "✗"
        };
        println!(
            "   {} {}: {} (esperado: {})",
            status,
            email,
            if is_valid_result {
                "VÁLIDO"
            } else {
                "INVÁLIDO"
            },
            if *expected { "VÁLIDO" } else { "INVÁLIDO" }
        );
    }
}
