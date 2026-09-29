/// Text utilities: capitalization and accent removal.
use unicode_normalization::UnicodeNormalization;

/// Removes diacritical marks (accents, tildes, cedillas) from a string.
///
/// Every character is decomposed (Unicode NFD) and every combining mark
/// (general category M) is dropped, so accents from any script go.
///
/// # Arguments
///
/// * `value` - The string to strip accents from.
///
/// # Returns
///
/// `value` with every combining mark removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::text::remove_accents;
///
/// assert_eq!(remove_accents("São Paulo"), "Sao Paulo");
/// assert_eq!(remove_accents("Açaí"), "Acai");
/// assert_eq!(remove_accents("Piauí, SP - 2024!"), "Piaui, SP - 2024!");
/// ```
pub fn remove_accents(value: &str) -> String {
    value
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect()
}

/// The default prepositions/articles kept lower case between two words.
const DEFAULT_PREPOSITIONS: &[&str] = &[
    "de", "da", "do", "das", "dos", "e", "em", "a", "o", "as", "os", "com", "para", "por",
];

/// The default company designations and abbreviations kept upper case.
const DEFAULT_UPPERCASE_WORDS: &[&str] = &["ltda", "me", "epp", "mei", "eireli", "cnpj", "cpf"];

/// Options for [`capitalize`].
#[derive(Debug, Clone, Default)]
pub struct CapitalizeOptions {
    /// Replaces the default preposition/article list.
    pub prepositions: Option<Vec<String>>,
    /// Replaces the default company-designation/abbreviation list.
    pub uppercase_words: Option<Vec<String>>,
}

fn is_roman_numeral(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| matches!(c, 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm'))
}

enum Segment {
    Word(String),
    Sep(String),
}

/// Collapses runs of whitespace into a single space and trims the ends.
fn collapse_whitespace(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_space = false;
    for c in s.trim().chars() {
        if c.is_whitespace() {
            if !last_was_space {
                result.push(' ');
            }
            last_was_space = true;
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    result
}

/// Splits a string into alternating word / separator segments. A "word" is
/// a maximal run of alphanumeric characters; everything else (whitespace,
/// `-`, `/`, apostrophes, punctuation) is a separator segment.
fn segment(s: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut current_is_word: Option<bool> = None;

    for c in s.chars() {
        let is_word = c.is_alphanumeric();
        match current_is_word {
            Some(w) if w == is_word => current.push(c),
            _ => {
                if !current.is_empty() {
                    segments.push(if current_is_word.unwrap() {
                        Segment::Word(std::mem::take(&mut current))
                    } else {
                        Segment::Sep(std::mem::take(&mut current))
                    });
                }
                current.push(c);
                current_is_word = Some(is_word);
            }
        }
    }
    if !current.is_empty() {
        segments.push(if current_is_word.unwrap() {
            Segment::Word(current)
        } else {
            Segment::Sep(current)
        });
    }

    segments
}

/// Capitalizes the first letter of each word the way a Brazilian name,
/// company name or address is written.
///
/// Prepositions and articles (`de`, `da`, `do`, `e`, …) stay lower case
/// between two words; company designations and abbreviations (`LTDA`,
/// `ME`, `CNPJ`, …) and Roman numerals are upper-cased. `options` can
/// replace either list.
///
/// Words split at whitespace, `-`, `/`, apostrophes and adjoining
/// punctuation; whitespace runs collapse into one space. A lower-case word
/// that is first, last or followed by punctuation keeps its capital.
///
/// # Examples
///
/// ```
/// use brazilian_utils::text::capitalize;
///
/// assert_eq!(capitalize("esponja vegetal", None), "Esponja Vegetal");
/// assert_eq!(capitalize("fulano de tal", None), "Fulano de Tal");
/// assert_eq!(capitalize("empresa ltda", None), "Empresa LTDA");
/// ```
pub fn capitalize(value: &str, options: Option<CapitalizeOptions>) -> String {
    let opts = options.unwrap_or_default();
    let prepositions: Vec<String> = opts
        .prepositions
        .unwrap_or_else(|| DEFAULT_PREPOSITIONS.iter().map(|s| s.to_string()).collect());
    let uppercase_words: Vec<String> = opts.uppercase_words.unwrap_or_else(|| {
        DEFAULT_UPPERCASE_WORDS
            .iter()
            .map(|s| s.to_string())
            .collect()
    });

    let collapsed = collapse_whitespace(value);
    if collapsed.is_empty() {
        return String::new();
    }

    let segments = segment(&collapsed);
    let word_indices: Vec<usize> = segments
        .iter()
        .enumerate()
        .filter_map(|(i, s)| matches!(s, Segment::Word(_)).then_some(i))
        .collect();

    if word_indices.is_empty() {
        return collapsed;
    }

    let first_word_idx = *word_indices.first().unwrap();
    let last_word_idx = *word_indices.last().unwrap();

    let mut result = String::new();
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Sep(s) => result.push_str(s),
            Segment::Word(w) => {
                let lower = w.to_lowercase();

                let followed_by_punct = matches!(
                    segments.get(i + 1),
                    Some(Segment::Sep(s)) if s.chars().any(|c| !c.is_whitespace())
                );

                let is_first = i == first_word_idx;
                let is_last = i == last_word_idx;

                if uppercase_words.iter().any(|u| u.eq_ignore_ascii_case(&lower))
                    || is_roman_numeral(&lower)
                {
                    result.push_str(&lower.to_uppercase());
                } else if prepositions.iter().any(|p| p.eq_ignore_ascii_case(&lower))
                    && !is_first
                    && !is_last
                    && !followed_by_punct
                {
                    result.push_str(&lower);
                } else {
                    let mut chars = lower.chars();
                    match chars.next() {
                        Some(f) => {
                            result.extend(f.to_uppercase());
                            result.push_str(chars.as_str());
                        }
                        None => {}
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("esponja vegetal", None), "Esponja Vegetal");
        assert_eq!(capitalize("JOAQUIM JOSÉ", None), "Joaquim José");
        assert_eq!(capitalize("fulano de tal", None), "Fulano de Tal");
        assert_eq!(
            capitalize("esponja de    aço 60G", None),
            "Esponja de Aço 60g"
        );
        assert_eq!(capitalize("de", None), "De");
        assert_eq!(capitalize("", None), "");
    }

    #[test]
    fn test_capitalize_company_designation() {
        assert_eq!(capitalize("empresa ltda", None), "Empresa LTDA");
    }

    #[test]
    fn test_remove_accents() {
        assert_eq!(remove_accents("São Paulo"), "Sao Paulo");
        assert_eq!(remove_accents("Açaí"), "Acai");
        assert_eq!(remove_accents("Piauí"), "Piaui");
        assert_eq!(remove_accents("Brasil"), "Brasil");
        assert_eq!(
            remove_accents("São Paulo, SP - 2024!"),
            "Sao Paulo, SP - 2024!"
        );
        assert_eq!(remove_accents(""), "");
    }
}
