use brazilian_utils::renavam::{calculate_checksum, generate, is_valid};

fn main() {
    println!("=== Brazilian RENAVAM (Registro Nacional de Veículos Automotores) Utilities Demo ===\n");

    // 1. Validate RENAVAM Numbers
    println!("1. Validate RENAVAM Numbers:");
    println!("   Valid RENAVAM numbers:");
    let valid_renavams = vec!["86769597308", "01234567897", "98765432103"];
    for renavam in &valid_renavams {
        if is_valid(renavam) {
            println!("      {} -> ✓ Valid", renavam);
        }
    }

    println!("\n   Invalid RENAVAM numbers:");
    let invalid_cases = vec![
        ("12345678901", "Wrong check digit"),
        ("11111111111", "All digits same"),
        ("123", "Too short"),
        ("123456789012", "Too long"),
        ("1234567890a", "Contains letter"),
        ("", "Empty"),
    ];
    for (renavam, reason) in &invalid_cases {
        println!("      {} ({}) -> ✗ Invalid", renavam, reason);
    }

    // 2. Calculate Check Digits
    println!("\n2. Calculate Check Digits:");
    let bases = vec!["8676959730", "0123456789", "9876543210"];
    for base in &bases {
        let check = calculate_checksum(base);
        println!("   Base: {} -> Check digit: {} -> Full: {}{}", base, check, base, check);
    }

    // 3. Generate Random Valid RENAVAM Numbers
    println!("\n3. Generate Random Valid RENAVAM Numbers:");
    for i in 1..=10 {
        let renavam = generate();
        let valid = if is_valid(&renavam) { "✓" } else { "✗" };
        println!("   {:2}. {} (Valid: {})", i, renavam, valid);
    }

    // 4. RENAVAM Number Format
    println!("\n4. RENAVAM Number Format:");
    println!("   ┌──────────────────────┐");
    println!("   │  XXXXXXXXXXX         │  11 digits total");
    println!("   │  86769597308         │  Example");
    println!("   └──────────────────────┘");
    println!("   - 10 base digits + 1 check digit");
    println!("   - Check digit calculated using weighted sum");
    println!("   - No formatting symbols (just 11 digits)");

    // 5. Checksum Calculation
    println!("\n5. Checksum Calculation:");
    println!("   The check digit is calculated as follows:");
    println!("   - Reverse the first 10 digits");
    println!("   - Multiply each digit by weights [2,3,4,5,6,7,8,9,2,3]");
    println!("   - Sum all products");
    println!("   - Calculate: 11 - (sum % 11)");
    println!("   - If result is 10 or 11, use 0");
    
    let example_base = "8676959730";
    let example_check = calculate_checksum(example_base);
    println!("\n   Example: Base {} -> Check digit: {} -> Full RENAVAM: {}{}", 
             example_base, example_check, example_base, example_check);

    // 6. Complete Workflow
    println!("\n6. Complete Workflow:");
    let test_renavam = "86769597308";
    println!("   Step 1: RENAVAM: {}", test_renavam);
    println!("   Step 2: Validate: {} {}", 
             test_renavam,
             if is_valid(test_renavam) { "✓ Valid" } else { "✗ Invalid" });
    println!("   Step 3: Extract base: {}", &test_renavam[..10]);
    println!("   Step 4: Calculate check: {}", calculate_checksum(&test_renavam[..10]));
    println!("   Step 5: Verify: {} {}",
             test_renavam,
             if calculate_checksum(&test_renavam[..10]).to_string() == test_renavam.chars().last().unwrap().to_string() {
                 "✓ Check digit matches"
             } else {
                 "✗ Check digit mismatch"
             });
}
