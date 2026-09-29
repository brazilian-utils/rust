/// Numbers in words (cardinal "por extenso") utilities.
use crate::currency::number_to_words;

/// Options for [`convert_to_words`].
#[derive(Debug, Clone, Default)]
pub struct ConvertNumberToWordsOptions {
    /// Selects the feminine grammatical gender ("uma", "duas",
    /// "-entas", …). Defaults to masculine. Best-effort: only the most
    /// common gender-agreeing words are transformed.
    pub feminine: Option<bool>,
}

/// Best-effort feminization of a masculine number-words string: "um" ->
/// "uma", "dois" -> "duas", and every "...entos" hundred word -> "...entas".
fn feminize(s: &str) -> String {
    s.split(' ')
        .map(|w| match w {
            "um" => "uma".to_string(),
            "dois" => "duas".to_string(),
            w if w.len() > 2 && w.ends_with("entos") => format!("{}as", &w[..w.len() - 2]),
            other => other.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Writes an integer in Brazilian Portuguese cardinal words ("por
/// extenso"), e.g. `1235` is `"mil duzentos e trinta e cinco"`.
///
/// Accepts integers from -999,999,999,999,999 to 999,999,999,999,999; a
/// non-integer is truncated toward zero; a negative number is prefixed with
/// "menos". Returns an empty string for a value outside that range or not
/// finite.
///
/// # Arguments
///
/// * `value` - The number to convert.
/// * `options` - Optionally selects the grammatical gender; see
///   [`ConvertNumberToWordsOptions`].
///
/// # Returns
///
/// The number written in words, or `""` if out of range / not finite.
///
/// # Examples
///
/// ```
/// use brazilian_utils::number::convert_to_words;
///
/// assert_eq!(convert_to_words(0.0, None), "zero");
/// assert_eq!(convert_to_words(1235.0, None), "mil duzentos e trinta e cinco");
/// assert_eq!(convert_to_words(-3.0, None), "menos três");
/// assert_eq!(convert_to_words(12.9, None), "doze");
/// ```
pub fn convert_to_words(value: f64, options: Option<ConvertNumberToWordsOptions>) -> String {
    if !value.is_finite() {
        return String::new();
    }

    let truncated = value.trunc();
    const MAX: f64 = 999_999_999_999_999.0;
    if !(-MAX..=MAX).contains(&truncated) {
        return String::new();
    }

    let n = truncated as i64;
    let words = number_to_words(n);

    let feminine = options.and_then(|o| o.feminine).unwrap_or(false);
    if feminine {
        feminize(&words)
    } else {
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_words() {
        assert_eq!(convert_to_words(0.0, None), "zero");
        assert_eq!(convert_to_words(1.0, None), "um");
        assert_eq!(convert_to_words(3.0, None), "três");
        assert_eq!(convert_to_words(100.0, None), "cem");
        assert_eq!(convert_to_words(101.0, None), "cento e um");
        assert_eq!(convert_to_words(123.0, None), "cento e vinte e três");
        assert_eq!(convert_to_words(1000.0, None), "mil");
        assert_eq!(convert_to_words(1000000.0, None), "um milhão");
        assert_eq!(convert_to_words(-3.0, None), "menos três");
        assert_eq!(convert_to_words(12.9, None), "doze");
    }

    #[test]
    fn test_convert_to_words_out_of_range() {
        assert_eq!(convert_to_words(f64::NAN, None), "");
        assert_eq!(convert_to_words(f64::INFINITY, None), "");
        assert_eq!(convert_to_words(1e18, None), "");
    }

    #[test]
    fn test_convert_to_words_feminine() {
        assert_eq!(
            convert_to_words(1.0, Some(ConvertNumberToWordsOptions { feminine: Some(true) })),
            "uma"
        );
        assert_eq!(
            convert_to_words(2.0, Some(ConvertNumberToWordsOptions { feminine: Some(true) })),
            "duas"
        );
        assert_eq!(
            convert_to_words(200.0, Some(ConvertNumberToWordsOptions { feminine: Some(true) })),
            "duzentas"
        );
    }
}
