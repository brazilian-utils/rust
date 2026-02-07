/// Currency formatting utilities for Brazilian Real (BRL).
const ONES: &[&str] = &[
    "", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove",
];

const TENS_TEENS: &[&str] = &[
    "",
    "onze",
    "doze",
    "treze",
    "catorze",
    "quinze",
    "dezesseis",
    "dezessete",
    "dezoito",
    "dezenove",
];

const TENS: &[&str] = &[
    "",
    "dez",
    "vinte",
    "trinta",
    "quarenta",
    "cinquenta",
    "sessenta",
    "setenta",
    "oitenta",
    "noventa",
];

const HUNDREDS: &[&str] = &[
    "",
    "cento",
    "duzentos",
    "trezentos",
    "quatrocentos",
    "quinhentos",
    "seiscentos",
    "setecentos",
    "oitocentos",
    "novecentos",
];

/// Convert an integer to its Portuguese (Brazilian) word representation.
///
/// # Arguments
///
/// * `n` - The number to convert (0-999,999,999,999,999,999)
///
/// # Returns
///
/// The number written in words in Brazilian Portuguese.
///
/// # Examples
///
/// ```
/// use brazilian_utils::currency::number_to_words;
///
/// assert_eq!(number_to_words(123), "cento e vinte e três");
/// assert_eq!(number_to_words(1000), "mil");
/// ```
pub fn number_to_words(n: i64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    if n < 0 {
        return format!("menos {}", number_to_words(-n));
    }

    // Handle quadrillions (10^15)
    if n >= 1_000_000_000_000_000 {
        let quadrillions = n / 1_000_000_000_000_000;
        let remainder = n % 1_000_000_000_000_000;

        let quadrillion_text = if quadrillions == 1 {
            "um quatrilhão".to_string()
        } else {
            format!("{} quatrilhões", number_to_words(quadrillions))
        };

        if remainder == 0 {
            return quadrillion_text;
        }

        let connector = if remainder < 100 { " e " } else { ", " };
        return format!(
            "{}{}{}",
            quadrillion_text,
            connector,
            number_to_words(remainder)
        );
    }

    // Handle trillions (10^12)
    if n >= 1_000_000_000_000 {
        let trillions = n / 1_000_000_000_000;
        let remainder = n % 1_000_000_000_000;

        let trillion_text = if trillions == 1 {
            "um trilhão".to_string()
        } else {
            format!("{} trilhões", number_to_words(trillions))
        };

        if remainder == 0 {
            return trillion_text;
        }

        let connector = if remainder < 100 { " e " } else { ", " };
        return format!(
            "{}{}{}",
            trillion_text,
            connector,
            number_to_words(remainder)
        );
    }

    // Handle billions (10^9)
    if n >= 1_000_000_000 {
        let billions = n / 1_000_000_000;
        let remainder = n % 1_000_000_000;

        let billion_text = if billions == 1 {
            "um bilhão".to_string()
        } else {
            format!("{} bilhões", number_to_words(billions))
        };

        if remainder == 0 {
            return billion_text;
        }

        let connector = if remainder < 100 { " e " } else { ", " };
        return format!(
            "{}{}{}",
            billion_text,
            connector,
            number_to_words(remainder)
        );
    }

    // Handle millions (10^6)
    if n >= 1_000_000 {
        let millions = n / 1_000_000;
        let remainder = n % 1_000_000;

        let million_text = if millions == 1 {
            "um milhão".to_string()
        } else {
            format!("{} milhões", number_to_words(millions))
        };

        if remainder == 0 {
            return million_text;
        }

        let connector = if remainder < 100 { " e " } else { ", " };
        return format!(
            "{}{}{}",
            million_text,
            connector,
            number_to_words(remainder)
        );
    }

    // Handle thousands (10^3)
    if n >= 1000 {
        let thousands = n / 1000;
        let remainder = n % 1000;

        let thousand_text = if thousands == 1 {
            "mil".to_string()
        } else {
            format!("{} mil", number_to_words(thousands))
        };

        if remainder == 0 {
            return thousand_text;
        }

        // Use "e" for round hundreds (100, 200, 300, etc.), otherwise use ","
        let connector = if remainder % 100 == 0 || remainder < 100 {
            " e "
        } else {
            ", "
        };
        return format!(
            "{}{}{}",
            thousand_text,
            connector,
            number_to_words(remainder)
        );
    }

    // Handle hundreds (100-999)
    if n >= 100 {
        let hundreds_digit = (n / 100) as usize;
        let remainder = n % 100;

        let hundred_text = if n == 100 {
            "cem".to_string()
        } else {
            HUNDREDS[hundreds_digit].to_string()
        };

        if remainder == 0 {
            return hundred_text;
        }

        return format!("{} e {}", hundred_text, number_to_words(remainder));
    }

    // Handle numbers from 20-99
    if n >= 20 {
        let tens_digit = (n / 10) as usize;
        let ones_digit = (n % 10) as usize;

        if ones_digit == 0 {
            return TENS[tens_digit].to_string();
        }

        return format!("{} e {}", TENS[tens_digit], ONES[ones_digit]);
    }

    // Handle teens (11-19)
    if n >= 11 {
        let index = (n - 10) as usize;
        return TENS_TEENS[index].to_string();
    }

    // Handle 1-10
    if n == 10 {
        return "dez".to_string();
    }

    ONES[n as usize].to_string()
}

/// Format a numeric value as Brazilian currency (R$).
///
/// This function takes a numeric value and formats it according to Brazilian
/// currency standards:
/// - Adds "R$" prefix
/// - Uses comma (,) as decimal separator
/// - Uses period (.) as thousands separator
/// - Always shows 2 decimal places
///
/// # Arguments
///
/// * `value` - The numeric value to format (can be positive, negative, or zero).
///
/// # Returns
///
/// A formatted currency string (e.g., "R$ 1.234,56") or None if the value cannot be formatted.
///
/// # Examples
///
/// ```
/// use brazilian_utils::currency::format_currency;
///
/// assert_eq!(format_currency(1234.56), Some("R$ 1.234,56".to_string()));
/// assert_eq!(format_currency(0.0), Some("R$ 0,00".to_string()));
/// assert_eq!(format_currency(-9876.54), Some("R$ -9.876,54".to_string()));
/// ```
pub fn format_currency(value: f64) -> Option<String> {
    if !value.is_finite() {
        return None;
    }

    // Round to 2 decimal places
    let rounded = (value * 100.0).round() / 100.0;

    // Separate integer and decimal parts
    let abs_value = rounded.abs();
    let integer_part = abs_value.floor() as i64;
    let decimal_part = ((abs_value - abs_value.floor()) * 100.0).round() as i64;

    // Check if the value is effectively zero after rounding
    let is_zero = integer_part == 0 && decimal_part == 0;

    // Handle negative sign (but treat -0.00 as 0.00)
    let negative = rounded < 0.0 && !is_zero;

    // Format integer part with thousands separator
    let integer_str = format_with_thousands_separator(integer_part);

    // Format with 2 decimal places
    let formatted = format!("{},{:02}", integer_str, decimal_part);

    // Add currency symbol and negative sign if needed
    if negative {
        Some(format!("R$ -{}", formatted))
    } else {
        Some(format!("R$ {}", formatted))
    }
}

/// Helper function to format an integer with thousands separator (period).
fn format_with_thousands_separator(mut num: i64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let mut count = 0;

    while num > 0 {
        if count == 3 {
            result.push('.');
            count = 0;
        }
        result.push(char::from_digit((num % 10) as u32, 10).unwrap());
        num /= 10;
        count += 1;
    }

    result.reverse();
    result.into_iter().collect()
}

/// Converts a Real (BRL) value to its written text representation in Brazilian Portuguese.
///
/// # Arguments
///
/// * `value` - The monetary value in Brazilian Reais to convert
///
/// # Returns
///
/// The value written in full in Brazilian Portuguese.
///
/// # Examples
///
/// ```
/// use brazilian_utils::currency::convert_real_to_text;
///
/// assert_eq!(convert_real_to_text(1.00), "um real");
/// assert_eq!(convert_real_to_text(2.50), "dois reais e cinquenta centavos");
/// assert_eq!(convert_real_to_text(1000000.00), "um milhão de reais");
/// ```
pub fn convert_real_to_text(value: f64) -> String {
    if value == 0.0 || value.abs() < 0.005 {
        return "zero real".to_string();
    }

    let is_negative = value < 0.0;
    let abs_value = value.abs();

    // Split into integer and decimal parts
    let reais = abs_value.floor() as i64;
    let centavos = ((abs_value - reais as f64) * 100.0).round() as i64;

    let mut result = String::new();

    if is_negative {
        result.push_str("menos ");
    }

    if reais > 0 {
        let reais_text = number_to_words(reais);

        // Add "de" before "reais" for large numbers (million and above)
        let currency_name = if reais == 1 {
            "real".to_string()
        } else if reais >= 1_000_000 {
            "de reais".to_string()
        } else {
            "reais".to_string()
        };

        result.push_str(&format!("{} {}", reais_text, currency_name));
    }

    if centavos > 0 {
        if reais > 0 {
            result.push_str(" e ");
        }

        let centavos_text = number_to_words(centavos);
        let centavo_text = if centavos == 1 { "centavo" } else { "centavos" };

        result.push_str(&format!("{} {}", centavos_text, centavo_text));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency_positive_values() {
        assert_eq!(format_currency(1234.56), Some("R$ 1.234,56".to_string()));
        assert_eq!(
            format_currency(123236.70),
            Some("R$ 123.236,70".to_string())
        );
        assert_eq!(format_currency(1259.03), Some("R$ 1.259,03".to_string()));
    }

    #[test]
    fn test_format_currency_zero() {
        assert_eq!(format_currency(0.0), Some("R$ 0,00".to_string()));
    }

    #[test]
    fn test_format_currency_negative_values() {
        assert_eq!(
            format_currency(-123236.70),
            Some("R$ -123.236,70".to_string())
        );
        assert_eq!(format_currency(-9876.54), Some("R$ -9.876,54".to_string()));
    }

    #[test]
    fn test_format_currency_rounding() {
        // Test decimal rounding
        assert_eq!(
            format_currency(123236.7676),
            Some("R$ 123.236,77".to_string())
        );
        assert_eq!(format_currency(10.555), Some("R$ 10,56".to_string()));
    }

    #[test]
    fn test_format_currency_small_values() {
        assert_eq!(format_currency(0.01), Some("R$ 0,01".to_string()));
        assert_eq!(format_currency(0.99), Some("R$ 0,99".to_string()));
        assert_eq!(format_currency(5.50), Some("R$ 5,50".to_string()));
    }

    #[test]
    fn test_format_currency_large_values() {
        assert_eq!(
            format_currency(1_000_000.00),
            Some("R$ 1.000.000,00".to_string())
        );
        assert_eq!(
            format_currency(999_999_999.99),
            Some("R$ 999.999.999,99".to_string())
        );
    }

    #[test]
    fn test_format_currency_invalid_values() {
        assert_eq!(format_currency(f64::INFINITY), None);
        assert_eq!(format_currency(f64::NEG_INFINITY), None);
        assert_eq!(format_currency(f64::NAN), None);
    }

    #[test]
    fn test_format_with_thousands_separator() {
        assert_eq!(format_with_thousands_separator(0), "0");
        assert_eq!(format_with_thousands_separator(123), "123");
        assert_eq!(format_with_thousands_separator(1234), "1.234");
        assert_eq!(format_with_thousands_separator(123456), "123.456");
        assert_eq!(format_with_thousands_separator(1234567), "1.234.567");
        assert_eq!(format_with_thousands_separator(999999999), "999.999.999");
    }

    #[test]
    fn test_format_currency_edge_cases() {
        // Very small positive value
        assert_eq!(format_currency(0.001), Some("R$ 0,00".to_string()));

        // Very small negative value - rounds to 0 but we treat as positive zero
        assert_eq!(format_currency(-0.001), Some("R$ 0,00".to_string()));

        // Values that need rounding
        assert_eq!(format_currency(1.234), Some("R$ 1,23".to_string()));
        assert_eq!(format_currency(1.235), Some("R$ 1,24".to_string()));
    }

    #[test]
    fn test_number_to_words_basic() {
        assert_eq!(number_to_words(0), "zero");
        assert_eq!(number_to_words(1), "um");
        assert_eq!(number_to_words(5), "cinco");
        assert_eq!(number_to_words(10), "dez");
        assert_eq!(number_to_words(15), "quinze");
        assert_eq!(number_to_words(20), "vinte");
        assert_eq!(number_to_words(25), "vinte e cinco");
        assert_eq!(number_to_words(99), "noventa e nove");
    }

    #[test]
    fn test_number_to_words_hundreds() {
        assert_eq!(number_to_words(100), "cem");
        assert_eq!(number_to_words(101), "cento e um");
        assert_eq!(number_to_words(200), "duzentos");
        assert_eq!(number_to_words(555), "quinhentos e cinquenta e cinco");
        assert_eq!(number_to_words(999), "novecentos e noventa e nove");
    }

    #[test]
    fn test_number_to_words_thousands() {
        assert_eq!(number_to_words(1000), "mil");
        assert_eq!(number_to_words(1001), "mil e um");
        assert_eq!(number_to_words(1111), "mil, cento e onze");
        assert_eq!(number_to_words(2000), "dois mil");
        assert_eq!(number_to_words(2500), "dois mil e quinhentos");
        assert_eq!(number_to_words(10000), "dez mil");
        assert_eq!(number_to_words(100000), "cem mil");
    }

    #[test]
    fn test_number_to_words_large_numbers() {
        assert_eq!(number_to_words(1_000_000), "um milhão");
        assert_eq!(number_to_words(2_000_000), "dois milhões");
        assert_eq!(number_to_words(1_000_000_000), "um bilhão");
        assert_eq!(number_to_words(2_000_000_000), "dois bilhões");
        assert_eq!(number_to_words(1_000_000_000_000), "um trilhão");
        assert_eq!(number_to_words(1_000_000_000_000_000), "um quatrilhão");
    }

    #[test]
    fn test_number_to_words_negative() {
        assert_eq!(number_to_words(-1), "menos um");
        assert_eq!(number_to_words(-42), "menos quarenta e dois");
        assert_eq!(number_to_words(-1000), "menos mil");
    }

    #[test]
    fn test_convert_real_to_text_basic() {
        assert_eq!(convert_real_to_text(0.0), "zero real");
        assert_eq!(convert_real_to_text(1.0), "um real");
        assert_eq!(convert_real_to_text(2.0), "dois reais");
        assert_eq!(convert_real_to_text(10.0), "dez reais");
        assert_eq!(convert_real_to_text(100.0), "cem reais");
    }

    #[test]
    fn test_convert_real_to_text_with_cents() {
        assert_eq!(convert_real_to_text(0.01), "um centavo");
        assert_eq!(convert_real_to_text(0.50), "cinquenta centavos");
        assert_eq!(convert_real_to_text(1.50), "um real e cinquenta centavos");
        assert_eq!(
            convert_real_to_text(2.50),
            "dois reais e cinquenta centavos"
        );
        assert_eq!(
            convert_real_to_text(10.99),
            "dez reais e noventa e nove centavos"
        );
    }

    #[test]
    fn test_convert_real_to_text_large_values() {
        assert_eq!(convert_real_to_text(1000.0), "mil reais");
        assert_eq!(convert_real_to_text(1000000.0), "um milhão de reais");
        assert_eq!(convert_real_to_text(2000000.0), "dois milhões de reais");
        assert_eq!(convert_real_to_text(1000000000.0), "um bilhão de reais");
        assert_eq!(
            convert_real_to_text(1000000.50),
            "um milhão de reais e cinquenta centavos"
        );
    }

    #[test]
    fn test_convert_real_to_text_negative() {
        assert_eq!(convert_real_to_text(-1.0), "menos um real");
        assert_eq!(
            convert_real_to_text(-2.50),
            "menos dois reais e cinquenta centavos"
        );
        assert_eq!(convert_real_to_text(-100.0), "menos cem reais");
    }

    #[test]
    fn test_convert_real_to_text_complex() {
        assert_eq!(
            convert_real_to_text(1111.11),
            "mil, cento e onze reais e onze centavos"
        );
        assert_eq!(convert_real_to_text(123456.78), "cento e vinte e três mil, quatrocentos e cinquenta e seis reais e setenta e oito centavos");
    }
}
