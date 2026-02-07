use brazilian_utils::currency::{format_currency, convert_real_to_text, number_to_words};

fn main() {
    println!("=== Demonstração do Módulo Currency ===\n");
    
    // Formatação de moeda
    println!("1. Formatação de moeda:");
    println!("   R$ 1.234,56: {:?}", format_currency(1234.56));
    println!("   R$ 1.000.000,00: {:?}", format_currency(1_000_000.00));
    println!("   R$ -9.876,54: {:?}\n", format_currency(-9876.54));
    
    // Conversão de números para palavras
    println!("2. Números por extenso:");
    println!("   123: {}", number_to_words(123));
    println!("   1000: {}", number_to_words(1000));
    println!("   1111: {}", number_to_words(1111));
    println!("   1.000.000: {}\n", number_to_words(1_000_000));
    
    // Conversão de valores em reais para texto
    println!("3. Valores em reais por extenso:");
    println!("   R$ 1,00: {}", convert_real_to_text(1.0));
    println!("   R$ 2,50: {}", convert_real_to_text(2.50));
    println!("   R$ 100,00: {}", convert_real_to_text(100.0));
    println!("   R$ 1.111,11: {}", convert_real_to_text(1111.11));
    println!("   R$ 1.000.000,00: {}", convert_real_to_text(1_000_000.0));
    println!("   R$ 1.000.000,50: {}", convert_real_to_text(1_000_000.50));
    println!("   R$ -50,00: {}", convert_real_to_text(-50.0));
}
