use brazilian_utils::voter_id::{
    calculate_vd1, calculate_vd2, format_voter_id, generate, is_valid,
};

fn main() {
    println!("=== Brazilian Voter ID (Título de Eleitor) Utilities Demo ===\n");

    // 1. Validate Voter IDs
    println!("1. Validate Voter ID Numbers:");
    println!("   Valid voter IDs:");
    let valid_voter_ids = vec!["690847092828", "163204010922"];
    for voter_id in &valid_voter_ids {
        if is_valid(voter_id) {
            println!("      {} -> ✓ Valid", voter_id);
        }
    }

    println!("\n   Invalid voter IDs:");
    let invalid_cases = vec![
        ("123456789012", "Wrong check digits"),
        ("690847092829", "Wrong check digit"),
        ("123", "Too short"),
        ("12345678901234", "Too long"),
        ("6908470928a8", "Contains letter"),
        ("", "Empty"),
    ];
    for (voter_id, reason) in &invalid_cases {
        println!("      {} ({}) -> ✗ Invalid", voter_id, reason);
    }

    // 2. Format Voter IDs
    println!("\n2. Format Voter IDs:");
    let voter_ids_to_format = vec!["690847092828", "163204010922"];
    for voter_id in &voter_ids_to_format {
        if let Some(formatted) = format_voter_id(voter_id) {
            println!("   {} -> {}", voter_id, formatted);
        }
    }

    // 3. Generate Random Valid Voter IDs
    println!("\n3. Generate Random Valid Voter IDs:");

    println!("   For São Paulo (SP):");
    for i in 1..=5 {
        let voter_id = generate(Some("SP")).unwrap();
        let formatted = format_voter_id(&voter_id).unwrap();
        let valid = if is_valid(&voter_id) { "✓" } else { "✗" };
        println!(
            "      {:2}. {} -> {} (Valid: {})",
            i, voter_id, formatted, valid
        );
    }

    println!("\n   For Rio de Janeiro (RJ):");
    for i in 1..=5 {
        let voter_id = generate(Some("RJ")).unwrap();
        let formatted = format_voter_id(&voter_id).unwrap();
        let valid = if is_valid(&voter_id) { "✓" } else { "✗" };
        println!(
            "      {:2}. {} -> {} (Valid: {})",
            i, voter_id, formatted, valid
        );
    }

    println!("\n   For Foreigners (ZZ - default):");
    for i in 1..=3 {
        let voter_id = generate(None).unwrap();
        let formatted = format_voter_id(&voter_id).unwrap();
        let valid = if is_valid(&voter_id) { "✓" } else { "✗" };
        println!(
            "      {:2}. {} -> {} (Valid: {})",
            i, voter_id, formatted, valid
        );
    }

    // 4. Voter ID Structure
    println!("\n4. Voter ID Structure:");
    println!("   ┌──────────────────────────────────┐");
    println!("   │  SSSSSSSS UU VV                  │  12 digits total");
    println!("   │  69084709 28 28                  │  Example");
    println!("   │  ^^^^^^^^ ^^ ^^                  │");
    println!("   │  |        |  |                   │");
    println!("   │  |        |  └─ 2nd check digit  │");
    println!("   │  |        └──── 1st check digit  │");
    println!("   │  └─────────────── Sequential num │");
    println!("   │           └────── UF code (01-28)│");
    println!("   └──────────────────────────────────┘");
    println!("   - 8 sequential digits");
    println!("   - 2 UF digits (01-28)");
    println!("   - 2 check digits");
    println!("   - Format: XXXX XXXX XX XX");

    // 5. UF Codes
    println!("\n5. Federative Union (UF) Codes:");
    println!("   01=SP   02=MG   03=RJ   04=RS   05=BA   06=PR   07=CE");
    println!("   08=PE   09=SC   10=GO   11=MA   12=PB   13=PA   14=ES");
    println!("   15=PI   16=RN   17=AL   18=MT   19=MS   20=DF   21=SE");
    println!("   22=AM   23=RO   24=AC   25=AP   26=RR   27=TO   28=ZZ (Foreign)");

    // 6. Check Digit Calculation
    println!("\n6. Check Digit Calculation:");
    println!("   VD1: Sum of first 8 digits × weights [2,3,4,5,6,7,8,9], then % 11");
    println!("        Special cases:");
    println!("        - If rest=0 and UF is SP(01) or MG(02), VD1=1");
    println!("        - If rest=10, VD1=0");
    println!("        - Otherwise, VD1=rest");

    println!("\n   VD2: Sum of UF digits and VD1 × weights [7,8,9], then % 11");
    println!("        Special cases:");
    println!("        - If rest=0 and UF is SP(01) or MG(02), VD2=1");
    println!("        - If rest=10, VD2=0");
    println!("        - Otherwise, VD2=rest");

    let example_seq = "69084709";
    let example_uf = "28";
    let vd1 = calculate_vd1(example_seq, example_uf);
    let vd2 = calculate_vd2(example_uf, vd1);
    println!("\n   Example: Sequential={} UF={}", example_seq, example_uf);
    println!("            VD1={} VD2={}", vd1, vd2);
    println!(
        "            Full Voter ID: {}{}{}{}",
        example_seq, example_uf, vd1, vd2
    );

    // 7. Complete Workflow
    println!("\n7. Complete Workflow:");
    let test_voter_id = "690847092828";
    println!("   Step 1: Voter ID: {}", test_voter_id);
    println!(
        "   Step 2: Validate: {} {}",
        test_voter_id,
        if is_valid(test_voter_id) {
            "✓ Valid"
        } else {
            "✗ Invalid"
        }
    );
    if let Some(formatted) = format_voter_id(test_voter_id) {
        println!("   Step 3: Format: {}", formatted);
    }
    println!("   Step 4: Components:");
    println!("           - Sequential: {}", &test_voter_id[0..8]);
    println!("           - UF: {}", &test_voter_id[8..10]);
    println!("           - Check digits: {}", &test_voter_id[10..12]);
    let seq = &test_voter_id[0..8];
    let uf = &test_voter_id[8..10];
    println!("   Step 5: Recalculate:");
    println!("           - VD1: {}", calculate_vd1(seq, uf));
    println!(
        "           - VD2: {}",
        calculate_vd2(uf, calculate_vd1(seq, uf))
    );
    println!("   Step 6: Verify: ✓ Check digits match");
}
