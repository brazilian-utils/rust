use brazilian_utils::pis;

fn main() {
    println!("Testing PIS checksum calculation:");
    
    let test_cases = vec![
        "1234567890",
        "8217853746",
        "5555020775",
        "1702435475",
        "9876543210",
    ];
    
    for base in test_cases {
        let check_digit = pis::checksum(base);
        let full_pis = format!("{}{}", base, check_digit);
        let is_valid = pis::is_valid(&full_pis);
        println!("Base: {} -> Check digit: {} -> Full PIS: {} -> Valid: {}", 
                 base, check_digit, full_pis, is_valid);
    }
    
    println!("\nKnown valid PIS numbers:");
    let valid_pis = vec![
        "82178537464",
        "55550207753",
        "17024354757",
        "12345678909",
    ];
    
    for pis_num in valid_pis {
        let is_valid = pis::is_valid(pis_num);
        let base = &pis_num[..10];
        let check_digit = pis::checksum(base);
        let actual_digit = &pis_num[10..11];
        println!("PIS: {} -> Valid: {} -> Calculated check: {} -> Actual: {}", 
                 pis_num, is_valid, check_digit, actual_digit);
    }
    
    println!("\nGenerating new valid PIS numbers:");
    for _ in 0..5 {
        let pis_num = pis::generate();
        let formatted = pis::format_pis(&pis_num);
        println!("Generated: {} -> Formatted: {:?}", pis_num, formatted);
    }
}
