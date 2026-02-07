use brazilian_utils::license_plate;

fn main() {
    println!("=== Brazilian License Plate Utilities Demo ===\n");

    // Example 1: Remove symbols
    println!("1. Remove Symbols:");
    let formatted_plate = "ABC-1234";
    let clean_plate = license_plate::remove_symbols(formatted_plate);
    println!("   Input:  {}", formatted_plate);
    println!("   Output: {}\n", clean_plate);

    // Example 2: Format license plates
    println!("2. Format License Plates:");
    
    // Old format (LLLNNNN) - adds dash
    let old_plate = "ABC1234";
    match license_plate::format_license_plate(old_plate) {
        Some(formatted) => {
            println!("   Old format: {} -> {}", old_plate, formatted);
        }
        None => println!("   Invalid plate: {}", old_plate),
    }
    
    // Mercosul format (LLLNLNN) - uppercase only
    let mercosul_plate = "abc1d23";
    match license_plate::format_license_plate(mercosul_plate) {
        Some(formatted) => {
            println!("   Mercosul format: {} -> {}", mercosul_plate, formatted);
        }
        None => println!("   Invalid plate: {}", mercosul_plate),
    }
    println!();

    // Example 3: Validate license plates
    println!("3. Validate License Plates:");
    
    let valid_plates = vec![
        ("ABC1234", None, "Any format"),
        ("ABC1D23", None, "Any format"),
        ("XYZ9876", Some("old_format"), "Old format only"),
        ("DEF5G89", Some("mercosul"), "Mercosul only"),
    ];
    
    for (plate, format, description) in valid_plates {
        let is_valid = license_plate::is_valid(plate, format);
        println!(
            "   {} ({}) -> {}",
            plate,
            description,
            if is_valid { "✓ Valid" } else { "✗ Invalid" }
        );
    }
    
    println!("\n   Invalid examples:");
    let invalid_plates = vec![
        ("ABC123", "Too short"),
        ("ABCD1234", "Too long"),
        ("ABC12D3", "Invalid Mercosul pattern"),
        ("12ABC34", "Numbers before letters"),
    ];
    
    for (plate, reason) in invalid_plates {
        let is_valid = license_plate::is_valid(plate, None);
        println!(
            "   {} ({}) -> {}",
            plate,
            reason,
            if is_valid { "✓ Valid" } else { "✗ Invalid" }
        );
    }
    println!();

    // Example 4: Get format
    println!("4. Get License Plate Format:");
    let plates = vec!["ABC1234", "ABC1D23", "xyz9876", "def5g89"];
    
    for plate in plates {
        match license_plate::get_format(plate) {
            Some(format) => println!("   {} -> {}", plate, format),
            None => println!("   {} -> Invalid format", plate),
        }
    }
    println!();

    // Example 5: Convert old format to Mercosul
    println!("5. Convert Old Format to Mercosul:");
    let old_plates = vec!["ABC1234", "XYZ9876", "DEF0000", "GHI4567"];
    
    for plate in old_plates {
        match license_plate::convert_to_mercosul(plate) {
            Some(mercosul) => {
                println!("   {} (old) -> {} (Mercosul)", plate, mercosul);
            }
            None => println!("   {} -> Cannot convert", plate),
        }
    }
    println!();

    // Example 6: Generate random license plates
    println!("6. Generate Random License Plates:");
    
    // Generate Mercosul format (default)
    println!("   Mercosul format (default):");
    for _ in 0..3 {
        if let Some(plate) = license_plate::generate(None) {
            let formatted = license_plate::format_license_plate(&plate).unwrap();
            println!("      {}", formatted);
        }
    }
    
    // Generate old format
    println!("\n   Old format:");
    for _ in 0..3 {
        if let Some(plate) = license_plate::generate(Some("LLLNNNN")) {
            let formatted = license_plate::format_license_plate(&plate).unwrap();
            println!("      {}", formatted);
        }
    }
    println!();

    // Example 7: Understanding the formats
    println!("7. License Plate Formats:");
    println!("   Old Format (before 2018):");
    println!("   ┌─────────────────┐");
    println!("   │   ABC-1234      │  LLLNNNN (3 letters + 4 numbers)");
    println!("   └─────────────────┘");
    println!();
    println!("   Mercosul Format (since 2018):");
    println!("   ┌─────────────────┐");
    println!("   │   ABC1D23       │  LLLNLNN (3 letters + 1 number + 1 letter + 2 numbers)");
    println!("   └─────────────────┘");
    println!();
    println!("   Conversion example:");
    println!("   ABC-1234 (old) -> ABC1C34 (Mercosul)");
    println!("                       ↑");
    println!("   2nd digit '2' becomes letter 'C' (0->A, 1->B, 2->C, ...)");
}
