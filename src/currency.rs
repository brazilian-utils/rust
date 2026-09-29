/// Currency formatting utilities for Brazilian Real (BRL).
const ONES: &[&str] = &[
    "", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove",
];

const TENS_TEENS: &[&str] = &[
    "",
    "onze",
    "doze",
    "treze",
    "quatorze",
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

        // Lei 14.822/2024 (LOA), art. 1o: no comma between groups; "e" only
        // joins the last group when it is < 100 or a round hundred (e.g.
        // "mil e quatrocentos reais", "mil quinhentos e dezoito reais").
        let connector = if remainder < 100 || remainder % 100 == 0 {
            " e "
        } else {
            " "
        };
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

        // Lei 14.822/2024 (LOA), art. 1o: no comma between groups; "e" only
        // joins the last group when it is < 100 or a round hundred (e.g.
        // "mil e quatrocentos reais", "mil quinhentos e dezoito reais").
        let connector = if remainder < 100 || remainder % 100 == 0 {
            " e "
        } else {
            " "
        };
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

        // Lei 14.822/2024 (LOA), art. 1o: no comma between groups; "e" only
        // joins the last group when it is < 100 or a round hundred (e.g.
        // "mil e quatrocentos reais", "mil quinhentos e dezoito reais").
        let connector = if remainder < 100 || remainder % 100 == 0 {
            " e "
        } else {
            " "
        };
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

        // Lei 14.822/2024 (LOA), art. 1o: no comma between groups; "e" only
        // joins the last group when it is < 100 or a round hundred (e.g.
        // "mil e quatrocentos reais", "mil quinhentos e dezoito reais").
        let connector = if remainder < 100 || remainder % 100 == 0 {
            " e "
        } else {
            " "
        };
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

        // Use "e" for round hundreds (100, 200, 300, etc.) or remainders < 100;
        // no comma between groups (Lei 14.822/2024).
        let connector = if remainder % 100 == 0 || remainder < 100 {
            " e "
        } else {
            " "
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

/// Format a numeric value as Brazilian currency (BRL), without a currency symbol.
///
/// This function takes a numeric value and formats it according to Brazilian
/// currency standards:
/// - No currency symbol is added by default (matching the contract's reference)
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
/// A formatted currency string (e.g., "1.234,56") or None if the value cannot be formatted.
///
/// # Examples
///
/// ```
/// use brazilian_utils::currency::format_currency;
///
/// assert_eq!(format_currency(1234.56), Some("1.234,56".to_string()));
/// assert_eq!(format_currency(0.0), Some("0,00".to_string()));
/// assert_eq!(format_currency(-9876.54), Some("-9.876,54".to_string()));
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

    // The contract's reference implementation adds no currency symbol by
    // default (an opt-in `options.symbol` would add ""); no negative sign
    // duplication.
    if negative {
        Some(format!("-{}", formatted))
    } else {
        Some(formatted)
    }
}

/// Parses a Brazilian currency (BRL) amount string into a number.
///
/// - The last `.` or `,` followed by 1 to 2 digits is read as the decimal
///   separator; every other `.` or `,` is a thousands separator.
/// - A value with no separator at all is read as cents (divided by 100).
/// - A leading `-` is preserved; an empty (or blank) string parses as `0.0`.
///
/// # Arguments
///
/// * `value` - The BRL amount string to parse, e.g. `"R$ 1.234,56"`.
///
/// # Returns
///
/// The parsed value as `f64`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::currency::parse;
///
/// assert_eq!(parse("R$ 1.234,56"), 1234.56);
/// assert_eq!(parse("1234"), 12.34);
/// assert_eq!(parse(""), 0.0);
/// ```
pub fn parse(value: &str) -> f64 {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return 0.0;
    }

    let negative = trimmed.starts_with('-');

    // Keep only digits and the two possible separators; everything else
    // (currency symbol, spaces, letters) is noise.
    let cleaned: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .collect();

    if cleaned.is_empty() {
        return 0.0;
    }

    let last_sep_idx = cleaned.rfind(|c| c == '.' || c == ',');

    let (int_part, frac_part): (String, String) = match last_sep_idx {
        Some(idx) => {
            let after = &cleaned[idx + 1..];
            if !after.is_empty() && after.len() <= 2 && after.chars().all(|c| c.is_ascii_digit())
            {
                let before = &cleaned[..idx];
                let int_digits: String = before.chars().filter(|c| c.is_ascii_digit()).collect();
                (int_digits, after.to_string())
            } else {
                let digits: String = cleaned.chars().filter(|c| c.is_ascii_digit()).collect();
                (digits, String::new())
            }
        }
        None => {
            // No separator at all: the whole value is read as cents.
            let digits: String = cleaned.chars().filter(|c| c.is_ascii_digit()).collect();
            let cents: i64 = digits.parse().unwrap_or(0);
            let result = cents as f64 / 100.0;
            return if negative { -result } else { result };
        }
    };

    let int_val: f64 = if int_part.is_empty() {
        0.0
    } else {
        int_part.parse().unwrap_or(0.0)
    };
    let frac_val: f64 = if frac_part.is_empty() {
        0.0
    } else {
        let frac_digits = frac_part.len() as i32;
        let numer: f64 = frac_part.parse().unwrap_or(0.0);
        numer / 10f64.powi(frac_digits)
    };

    let result = int_val + frac_val;
    if negative {
        -result
    } else {
        result
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
        return "zero reais".to_string();
    }

    let is_negative = value < 0.0;
    let abs_value = value.abs();

    // Truncate (do not round) to 2 decimal places. A tiny epsilon corrects
    // for binary floating point representation error (e.g. 0.29 * 100 ==
    // 28.999999999999996) without masking a genuine 3rd-decimal digit, which
    // contributes at least 0.1 to the fractional part here.
    let total_cents = (abs_value * 100.0 + 1e-9).floor() as i64;
    let reais = total_cents / 100;
    let centavos = total_cents % 100;

    let mut result = String::new();

    if is_negative {
        result.push_str("menos ");
    }

    if reais > 0 {
        let reais_text = number_to_words(reais);

        // "de reais"/"de real" is only used when the spoken number ends
        // exactly on a bare "milhão"/"milhões" (or higher multiple of it)
        // with nothing smaller following, e.g. "um milhão de reais" but
        // "um milhão duzentos e trinta reais" (no "de").
        let use_de = reais >= 1_000_000 && reais % 1_000_000 == 0;

        let currency_name = if reais == 1 {
            "real".to_string()
        } else if use_de {
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
        assert_eq!(format_currency(1234.56), Some("1.234,56".to_string()));
        assert_eq!(
            format_currency(123236.70),
            Some("123.236,70".to_string())
        );
        assert_eq!(format_currency(1259.03), Some("1.259,03".to_string()));
    }

    #[test]
    fn test_format_currency_zero() {
        assert_eq!(format_currency(0.0), Some("0,00".to_string()));
    }

    #[test]
    fn test_format_currency_negative_values() {
        assert_eq!(
            format_currency(-123236.70),
            Some("-123.236,70".to_string())
        );
        assert_eq!(format_currency(-9876.54), Some("-9.876,54".to_string()));
    }

    #[test]
    fn test_format_currency_rounding() {
        // Test decimal rounding
        assert_eq!(
            format_currency(123236.7676),
            Some("123.236,77".to_string())
        );
        assert_eq!(format_currency(10.555), Some("10,56".to_string()));
    }

    #[test]
    fn test_format_currency_small_values() {
        assert_eq!(format_currency(0.01), Some("0,01".to_string()));
        assert_eq!(format_currency(0.99), Some("0,99".to_string()));
        assert_eq!(format_currency(5.50), Some("5,50".to_string()));
    }

    #[test]
    fn test_format_currency_large_values() {
        assert_eq!(
            format_currency(1_000_000.00),
            Some("1.000.000,00".to_string())
        );
        assert_eq!(
            format_currency(999_999_999.99),
            Some("999.999.999,99".to_string())
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
        assert_eq!(format_currency(0.001), Some("0,00".to_string()));

        // Very small negative value - rounds to 0 but we treat as positive zero
        assert_eq!(format_currency(-0.001), Some("0,00".to_string()));

        // Values that need rounding
        assert_eq!(format_currency(1.234), Some("1,23".to_string()));
        assert_eq!(format_currency(1.235), Some("1,24".to_string()));
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
        assert_eq!(number_to_words(1111), "mil cento e onze");
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
        assert_eq!(convert_real_to_text(0.0), "zero reais");
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
            "mil cento e onze reais e onze centavos"
        );
        assert_eq!(convert_real_to_text(123456.78), "cento e vinte e três mil quatrocentos e cinquenta e seis reais e setenta e oito centavos");
    }

    #[test]
    fn test_convert_real_to_text_million_and_more_no_de() {
        // "de reais" only applies when nothing follows the bare million(s);
        // a million-plus-remainder amount does not get it.
        assert_eq!(
            convert_real_to_text(1000230.0),
            "um milhão duzentos e trinta reais"
        );
    }

    #[test]
    fn test_convert_real_to_text_truncates_sub_cent() {
        assert_eq!(
            convert_real_to_text(1.999),
            "um real e noventa e nove centavos"
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("R$ 1.234,56"), 1234.56);
        assert_eq!(parse("1234,56"), 1234.56);
        assert_eq!(parse("R$ 0,50"), 0.5);
        assert_eq!(parse("1.000.000,50"), 1000000.5);
        assert_eq!(parse("-1.234,56"), -1234.56);
        assert_eq!(parse("0,00"), 0.0);
        assert_eq!(parse("1234"), 12.34);
        assert_eq!(parse(""), 0.0);
    }

    #[test]
    fn test_format_currency_no_symbol() {
        assert_eq!(format_currency(1000.01), Some("1.000,01".to_string()));
        assert_eq!(format_currency(0.01), Some("0,01".to_string()));
        assert_eq!(format_currency(1.0), Some("1,00".to_string()));
        assert_eq!(format_currency(1000000.01), Some("1.000.000,01".to_string()));
        assert_eq!(format_currency(-10.1), Some("-10,10".to_string()));
    }
}
