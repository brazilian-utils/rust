use brazilian_utils::boleto;

fn main() {
    println!("=== Boleto Validation Demo ===\n");

    // Example 1: Valid boleto with numbers only
    let valid_boleto = "00190000090114971860168524522114675860000102656";
    println!("Example 1: Validating '{}'", valid_boleto);
    println!("is_valid: {}", boleto::is_valid(valid_boleto));
    println!("validate: {}\n", boleto::validate(valid_boleto));

    // Example 2: Valid boleto with formatting (spaces and dots)
    let formatted_boleto = "0019000009 01149.718601 68524.522114 6 75860000102656";
    println!("Example 2: Validating '{}'", formatted_boleto);
    println!("is_valid: {}", boleto::is_valid(formatted_boleto));
    println!("validate: {}\n", boleto::validate(formatted_boleto));

    // Example 3: Invalid boleto (wrong first check digit)
    let invalid_boleto1 = "00190000020114971860168524522114675860000102656";
    println!("Example 3: Validating '{}'", invalid_boleto1);
    println!("is_valid: {}", boleto::is_valid(invalid_boleto1));
    println!("validate: {}\n", boleto::validate(invalid_boleto1));

    // Example 4: Invalid boleto (wrong main check digit)
    let invalid_boleto2 = "00190000090114971860168524522114975860000102656";
    println!("Example 4: Validating '{}'", invalid_boleto2);
    println!("is_valid: {}", boleto::is_valid(invalid_boleto2));
    println!("validate: {}\n", boleto::validate(invalid_boleto2));

    // Example 5: Invalid boleto (too short)
    let short_boleto = "000111";
    println!("Example 5: Validating '{}'", short_boleto);
    println!("is_valid: {}", boleto::is_valid(short_boleto));
    println!("validate: {}\n", boleto::validate(short_boleto));

    // Example 6: Empty string
    let empty_boleto = "";
    println!("Example 6: Validating empty string");
    println!("is_valid: {}", boleto::is_valid(empty_boleto));
    println!("validate: {}\n", boleto::validate(empty_boleto));

    println!("=== Demo Complete ===");
}
