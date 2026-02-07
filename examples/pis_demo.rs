use brazilian_utils::pis;

fn main() {
    println!("=== Brazilian PIS (Programa de Integração Social) Utilities Demo ===\n");

    // Example 1: Remove symbols
    println!("1. Remove Symbols:");
    let formatted_pis_numbers = vec!["123.45678.90-0", "987.65432.10-3", "170.24354.75-3"];

    for pis_number in formatted_pis_numbers {
        let clean = pis::remove_symbols(pis_number);
        println!("   {} -> {}", pis_number, clean);
    }
    println!();

    // Example 2: Validate PIS numbers
    println!("2. Validate PIS Numbers:");

    let valid_numbers = vec![
        "12345678900",
        "98765432103",
        "17024354753",
        "82178537467",
        "55550207756",
    ];

    println!("   Valid PIS numbers:");
    for number in valid_numbers {
        let is_valid = pis::is_valid(number);
        let formatted = pis::format_pis(number);
        println!(
            "      {} -> {} -> {}",
            number,
            if is_valid { "✓ Valid" } else { "✗ Invalid" },
            formatted.unwrap_or("N/A".to_string())
        );
    }

    println!("\n   Invalid PIS numbers:");
    let invalid_numbers = vec![
        ("12345678901", "Wrong check digit"),
        ("123", "Too short"),
        ("123456789012", "Too long"),
        ("1234567890a", "Contains letter"),
        ("", "Empty"),
    ];

    for (number, reason) in invalid_numbers {
        let is_valid = pis::is_valid(number);
        println!(
            "      {} ({}) -> {}",
            if number.is_empty() { "(empty)" } else { number },
            reason,
            if is_valid { "✓ Valid" } else { "✗ Invalid" }
        );
    }
    println!();

    // Example 3: Format PIS numbers
    println!("3. Format PIS Numbers:");
    let numbers_to_format = vec!["12345678900", "98765432103", "17024354753", "82178537467"];

    for number in numbers_to_format {
        match pis::format_pis(number) {
            Some(formatted) => println!("   {} -> {}", number, formatted),
            None => println!("   {} -> Invalid (cannot format)", number),
        }
    }
    println!();

    // Example 4: Generate random PIS numbers
    println!("4. Generate Random Valid PIS Numbers:");
    for i in 1..=10 {
        let pis_number = pis::generate();
        let formatted = pis::format_pis(&pis_number).unwrap();
        let is_valid = pis::is_valid(&pis_number);
        println!(
            "   {:2}. {} -> {} (Valid: {})",
            i,
            pis_number,
            formatted,
            if is_valid { "✓" } else { "✗" }
        );
    }
    println!();

    // Example 5: Understanding the format
    println!("5. PIS Number Format:");
    println!("   ┌──────────────────────┐");
    println!("   │  XXX.XXXXX.XX-X      │  11 digits total");
    println!("   │  123.45678.90-0      │  Example");
    println!("   └──────────────────────┘");
    println!("   - 10 base digits + 1 check digit");
    println!("   - Check digit calculated using weighted sum");
    println!("   - Formatted as: XXX.XXXXX.XX-X");
    println!();

    // Example 6: Checksum demonstration
    println!("6. Checksum Calculation:");
    println!("   The check digit is calculated as follows:");
    println!("   - Multiply each of the first 10 digits by weights [3,2,9,8,7,6,5,4,3,2]");
    println!("   - Sum all products");
    println!("   - Calculate: 11 - (sum % 11)");
    println!("   - If result is 10 or 11, use 0");
    println!();

    let base = "1234567890";
    let check_digit = pis::checksum(base);
    let full_pis = format!("{}{}", base, check_digit);
    println!(
        "   Example: Base {} -> Check digit: {} -> Full PIS: {}",
        base, check_digit, full_pis
    );
    println!("   Formatted: {}", pis::format_pis(&full_pis).unwrap());
    println!();

    // Example 7: Validation and formatting workflow
    println!("7. Complete Workflow:");
    let unformatted = "98765432103";
    println!("   Step 1: Unformatted PIS: {}", unformatted);
    println!(
        "   Step 2: Validate: {}",
        if pis::is_valid(unformatted) {
            "✓ Valid"
        } else {
            "✗ Invalid"
        }
    );

    if let Some(formatted) = pis::format_pis(unformatted) {
        println!("   Step 3: Format: {}", formatted);
        let cleaned = pis::remove_symbols(&formatted);
        println!("   Step 4: Remove symbols: {}", cleaned);
        println!(
            "   Step 5: Verify roundtrip: {}",
            if cleaned == unformatted {
                "✓ Success"
            } else {
                "✗ Failed"
            }
        );
    }
}
