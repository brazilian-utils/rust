use brazilian_utils::legal_process;

fn main() {
    println!("=== Brazilian Legal Process ID Utilities Demo ===\n");

    // Example 1: Remove symbols
    println!("1. Remove Symbols:");
    let formatted_id = "6439067-89.2023.4.04.5902";
    let clean_id = legal_process::remove_symbols(formatted_id);
    println!("   Input:  {}", formatted_id);
    println!("   Output: {}\n", clean_id);

    // Example 2: Format legal process ID
    println!("2. Format Legal Process ID:");
    let unformatted = "23141945820055070079";
    match legal_process::format_legal_process(unformatted) {
        Some(formatted) => {
            println!("   Input:  {}", unformatted);
            println!("   Output: {}", formatted);
        }
        None => println!("   Invalid ID: {}", unformatted),
    }
    println!();

    // Example 3: Validate legal process IDs
    println!("3. Validate Legal Process IDs:");
    let valid_ids = vec![
        "10188748220234018200",
        "45532346920234025107",
        "2314194-58.2005.5.07.0079",
    ];
    
    for id in valid_ids {
        let is_valid = legal_process::is_valid(id);
        println!("   {} -> {}", id, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    
    println!("\n   Invalid examples:");
    let invalid_ids = vec![
        "00000000000000000000",
        "123",
        "10188748220234018201", // wrong checksum
    ];
    
    for id in invalid_ids {
        let is_valid = legal_process::is_valid(id);
        println!("   {} -> {}", id, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    println!();

    // Example 4: Generate random legal process IDs
    println!("4. Generate Random Legal Process IDs:");
    
    // Generate with current year and random orgao
    match legal_process::generate(None, None) {
        Some(id) => {
            let formatted = legal_process::format_legal_process(&id).unwrap();
            println!("   Current year, random orgao: {}", formatted);
        }
        None => println!("   Failed to generate ID"),
    }
    
    // Generate for specific organs
    for orgao in [1, 4, 5, 8] {
        match legal_process::generate(None, Some(orgao)) {
            Some(id) => {
                let formatted = legal_process::format_legal_process(&id).unwrap();
                println!("   Orgao {}: {}", orgao, formatted);
            }
            None => println!("   Failed to generate ID for orgao {}", orgao),
        }
    }
    println!();

    // Example 5: Understanding the format
    println!("5. Legal Process ID Format:");
    println!("   NNNNNNN-DD.AAAA.J.TR.OOOO");
    println!("   └─┬─┘  └┬┘ └┬─┘ │ └┬┘ └┬──┘");
    println!("     │     │   │   │  │   └─ Foro (4 digits)");
    println!("     │     │   │   │  └───── Tribunal (2 digits)");
    println!("     │     │   │   └──────── Orgao (1 digit, 1-9)");
    println!("     │     │   └──────────── Year (4 digits)");
    println!("     │     └──────────────── Check digits (2 digits)");
    println!("     └────────────────────── Sequential number (7 digits)");
    println!();
    
    println!("6. Legal Organs (Justiça):");
    let organs = vec![
        (1, "Federal"),
        (2, "Trabalho (Labor)"),
        (3, "Eleitoral (Electoral)"),
        (4, "Militar (Military)"),
        (5, "Estadual (State)"),
        (6, "Distrito Federal e Territórios"),
        (7, "Tribunal Superior"),
        (8, "Estadual (State) - Additional"),
        (9, "Outros (Others)"),
    ];
    
    for (code, name) in organs {
        println!("   {} - {}", code, name);
    }
}
