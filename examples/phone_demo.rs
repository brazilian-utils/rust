use brazilian_utils::phone;

fn main() {
    println!("=== Brazilian Phone Number Utilities Demo ===\n");

    // Example 1: Remove symbols
    println!("1. Remove Symbols:");
    let formatted_phones = vec![
        "(11)99402-9275",
        "+55 11 9 9402-9275",
        "16 3501-4415",
    ];
    
    for phone_number in formatted_phones {
        let clean = phone::remove_symbols(phone_number);
        println!("   {} -> {}", phone_number, clean);
    }
    println!();

    // Example 2: Validate phone numbers
    println!("2. Validate Phone Numbers:");
    
    println!("   Mobile numbers:");
    let mobile_numbers = vec![
        "11994029275",
        "21987654321",
        "85912345678",
    ];
    
    for number in mobile_numbers {
        let is_valid = phone::is_valid(number, Some("mobile"));
        println!("      {} -> {}", number, if is_valid { "✓ Valid mobile" } else { "✗ Invalid" });
    }
    
    println!("\n   Landline numbers:");
    let landline_numbers = vec![
        "1635014415",
        "1133334444",
        "8532221111",
    ];
    
    for number in landline_numbers {
        let is_valid = phone::is_valid(number, Some("landline"));
        println!("      {} -> {}", number, if is_valid { "✓ Valid landline" } else { "✗ Invalid" });
    }
    
    println!("\n   Invalid examples:");
    let invalid_numbers = vec![
        ("123", "Too short"),
        ("11894029275", "Missing '9' for mobile"),
        ("1665014415", "Invalid landline prefix"),
        ("119940292751", "Too long"),
    ];
    
    for (number, reason) in invalid_numbers {
        let is_valid = phone::is_valid(number, None);
        println!("      {} ({}) -> {}", number, reason, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    println!();

    // Example 3: Format phone numbers
    println!("3. Format Phone Numbers:");
    let numbers_to_format = vec![
        ("11994029275", "Mobile"),
        ("1635014415", "Landline"),
        ("21987654321", "Mobile"),
        ("1133334444", "Landline"),
    ];
    
    for (number, type_name) in numbers_to_format {
        match phone::format_phone(number) {
            Some(formatted) => println!("   {} ({}) -> {}", number, type_name, formatted),
            None => println!("   {} -> Invalid", number),
        }
    }
    println!();

    // Example 4: Remove international dialing code
    println!("4. Remove International Dialing Code:");
    let international_numbers = vec![
        "5511994029275",
        "551635014415",
        "+5511994029275",
        "11994029275", // No code to remove
    ];
    
    for number in international_numbers {
        let cleaned = phone::remove_international_dialing_code(number);
        println!("   {} -> {}", number, cleaned);
    }
    println!();

    // Example 5: Generate random phone numbers
    println!("5. Generate Random Phone Numbers:");
    
    println!("   Mobile numbers:");
    for _ in 0..3 {
        let mobile = phone::generate(Some("mobile"));
        let formatted = phone::format_phone(&mobile).unwrap();
        println!("      {}", formatted);
    }
    
    println!("\n   Landline numbers:");
    for _ in 0..3 {
        let landline = phone::generate(Some("landline"));
        let formatted = phone::format_phone(&landline).unwrap();
        println!("      {}", formatted);
    }
    
    println!("\n   Random type:");
    for _ in 0..3 {
        let phone_number = phone::generate(None);
        let formatted = phone::format_phone(&phone_number).unwrap();
        let phone_type = if phone_number.len() == 11 { "mobile" } else { "landline" };
        println!("      {} ({})", formatted, phone_type);
    }
    println!();

    // Example 6: Understanding the formats
    println!("6. Phone Number Formats:");
    println!("   Mobile (11 digits):");
    println!("   ┌────────────────────┐");
    println!("   │ (DD) 9XXXX-XXXX    │  DDD + 9 + 8 digits");
    println!("   │ (11) 99402-9275    │  Example");
    println!("   └────────────────────┘");
    println!("   - DD: Area code (DDD) - both digits 1-9");
    println!("   - 9: Mobile identifier (always 9)");
    println!("   - XXXX-XXXX: Subscriber number");
    println!();
    println!("   Landline (10 digits):");
    println!("   ┌────────────────────┐");
    println!("   │ (DD) [2-5]XXX-XXXX │  DDD + [2-5] + 7 digits");
    println!("   │ (16) 3501-4415     │  Example");
    println!("   └────────────────────┘");
    println!("   - DD: Area code (DDD) - both digits 1-9");
    println!("   - [2-5]: Landline identifier (2, 3, 4, or 5)");
    println!("   - XXX-XXXX: Subscriber number");
    println!();
    println!("7. DDD (Area Codes) Examples:");
    let ddd_examples = vec![
        (11, "São Paulo - SP"),
        (21, "Rio de Janeiro - RJ"),
        (31, "Belo Horizonte - MG"),
        (41, "Curitiba - PR"),
        (51, "Porto Alegre - RS"),
        (61, "Brasília - DF"),
        (71, "Salvador - BA"),
        (81, "Recife - PE"),
        (85, "Fortaleza - CE"),
    ];
    
    for (code, city) in ddd_examples {
        println!("   {} - {}", code, city);
    }
}
