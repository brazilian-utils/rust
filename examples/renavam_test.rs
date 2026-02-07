use brazilian_utils::renavam::calculate_checksum;

fn main() {
    println!("=== RENAVAM Checksum Verification ===\n");

    // Test the known valid RENAVAM from Python
    let test_cases = vec![
        ("8676959730", "86769597308"),
        ("0123456789", "01234567891"),
        ("9876543210", "98765432101"),
    ];

    for (base, full) in test_cases {
        let calculated_check = calculate_checksum(base);
        let expected_full = format!("{}{}", base, calculated_check);
        let original_check = full.chars().last().unwrap();

        println!("Base: {}", base);
        println!("  Calculated check digit: {}", calculated_check);
        println!("  Expected full RENAVAM: {}", full);
        println!("  Generated full RENAVAM: {}", expected_full);
        println!("  Original check digit: {}", original_check);
        println!("  Match: {}\n", expected_full == full);
    }

    // Manually calculate for "0123456789"
    println!("=== Manual calculation for 0123456789 ===");
    let base = "0123456789";
    let weights = [2, 3, 4, 5, 6, 7, 8, 9, 2, 3];
    let digits: Vec<u32> = base.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    
    println!("Digits (reversed): {:?}", digits);
    println!("Weights: {:?}", weights);
    
    let mut sum = 0;
    for (i, (d, w)) in digits.iter().zip(weights.iter()).enumerate() {
        let product = d * w;
        println!("  Position {}: {} * {} = {}", i, d, w, product);
        sum += product;
    }
    
    println!("Sum: {}", sum);
    println!("Sum % 11: {}", sum % 11);
    println!("11 - (Sum % 11): {}", 11 - (sum % 11));
    let check = if 11 - (sum % 11) >= 10 { 0 } else { 11 - (sum % 11) };
    println!("Check digit (adjusted): {}", check);
}
