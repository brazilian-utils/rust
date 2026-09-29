/// IBAN (International Bank Account Number) utilities for the Brazilian
/// IBAN layout.

/// Removes IBAN formatting, keeps letters and digits upper-cased, capped to
/// 29 characters.
///
/// # Examples
///
/// ```
/// use brazilian_utils::iban::parse;
///
/// assert_eq!(parse("BR15 0000 0000 0000 1093 2840 814P 2"), "BR1500000000000010932840814P2");
/// assert_eq!(parse("br15-0000.0000/0000 1093 2840 814p-2"), "BR1500000000000010932840814P2");
/// ```
pub fn parse(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .take(29)
        .collect()
}

/// Formats an IBAN in the ISO 13616 print grouping: blocks of 4 characters
/// separated by spaces, upper-cased. Does not validate (use [`is_valid`]).
///
/// # Examples
///
/// ```
/// use brazilian_utils::iban::format;
///
/// assert_eq!(format("BR1500000000000010932840814P2"), "BR15 0000 0000 0000 1093 2840 814P 2");
/// assert_eq!(format("BR150"), "BR15 0");
/// assert_eq!(format(""), "");
/// ```
pub fn format(value: &str) -> String {
    let compact = parse(value);
    let mut result = String::new();
    for (i, chunk) in compact.as_bytes().chunks(4).enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(std::str::from_utf8(chunk).unwrap());
    }
    result
}

/// Normalizes a possibly-masked IBAN into its 29-character compact form,
/// rejecting a value with a separator inside a group.
fn to_compact(value: &str) -> Option<String> {
    let has_sep = value
        .chars()
        .any(|c| c.is_whitespace() || c == '.' || c == '-' || c == '/');

    if has_sep {
        let groups: Vec<&str> = value
            .split(|c: char| c.is_whitespace() || c == '.' || c == '-' || c == '/')
            .filter(|s| !s.is_empty())
            .collect();
        if groups.is_empty() {
            return None;
        }
        for g in &groups[..groups.len() - 1] {
            if g.len() != 4 || !g.chars().all(|c| c.is_ascii_alphanumeric()) {
                return None;
            }
        }
        let last = groups.last().unwrap();
        if last.is_empty() || last.len() > 4 || !last.chars().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }
        Some(groups.concat().to_uppercase())
    } else {
        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }
        Some(value.to_uppercase())
    }
}

/// Checks the ISO 7064 MOD 97-10 IBAN check digits.
fn mod97_check(compact: &str) -> bool {
    let rearranged = format!("{}{}", &compact[4..], &compact[0..4]);
    let mut remainder: u64 = 0;

    for ch in rearranged.chars() {
        let val: u64 = if ch.is_ascii_digit() {
            ch.to_digit(10).unwrap() as u64
        } else if ch.is_ascii_uppercase() {
            (ch as u64 - 'A' as u64) + 10
        } else {
            return false;
        };

        for d in val.to_string().chars() {
            remainder = (remainder * 10 + d.to_digit(10).unwrap() as u64) % 97;
        }
    }

    remainder == 1
}

/// Validates a Brazilian IBAN; any other country is invalid.
///
/// Layout, 29 characters: `BR`, 2 check digits (ISO 7064 MOD 97-10), 8-digit
/// ISPB, 5-digit branch, 10-digit account, 1-letter account type, 1 owner
/// indicator (`1` to `9`, then `A` to `Z`).
///
/// # Examples
///
/// ```
/// use brazilian_utils::iban::is_valid;
///
/// assert!(is_valid("BR1500000000000010932840814P2"));
/// assert!(is_valid("BR15 0000 0000 0000 1093 2840 814P 2"));
/// assert!(!is_valid("BR1500000000000010932840814P3"));
/// assert!(!is_valid("DE89370400440532013000"));
/// ```
pub fn is_valid(value: &str) -> bool {
    let compact = match to_compact(value) {
        Some(c) => c,
        None => return false,
    };

    if compact.len() != 29 || !compact.starts_with("BR") {
        return false;
    }

    mod97_check(&compact)
}

/// Parsed fields of a Brazilian IBAN, as returned by [`get_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct IbanInfo {
    pub country_code: String,
    pub check_digits: String,
    pub bank_ispb: String,
    pub branch: String,
    pub account: String,
    pub account_type: String,
    pub owner: String,
}

/// Parses a Brazilian IBAN into its fields; returns `None` whenever
/// [`is_valid`] would return `false`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::iban::get_info;
///
/// let info = get_info("BR1500000000000010932840814P2").unwrap();
/// assert_eq!(info.bank_ispb, "00000000");
/// assert_eq!(info.branch, "00001");
/// ```
pub fn get_info(value: &str) -> Option<IbanInfo> {
    if !is_valid(value) {
        return None;
    }

    let compact = to_compact(value)?;

    Some(IbanInfo {
        country_code: compact[0..2].to_string(),
        check_digits: compact[2..4].to_string(),
        bank_ispb: compact[4..12].to_string(),
        branch: compact[12..17].to_string(),
        account: compact[17..27].to_string(),
        account_type: compact[27..28].to_string(),
        owner: compact[28..29].to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(
            format("BR1500000000000010932840814P2"),
            "BR15 0000 0000 0000 1093 2840 814P 2"
        );
        assert_eq!(
            format("br1500000000000010932840814p2"),
            "BR15 0000 0000 0000 1093 2840 814P 2"
        );
        assert_eq!(format("BR150"), "BR15 0");
        assert_eq!(format(""), "");
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("BR1500000000000010932840814P2"));
        assert!(is_valid("BR15 0000 0000 0000 1093 2840 814P 2"));
        assert!(is_valid("br1500000000000010932840814p2"));
        assert!(!is_valid("BR1500000000000010932840814P3"));
        assert!(!is_valid("DE89370400440532013000"));
        assert!(!is_valid("BR15000000000000109328408"));
        assert!(!is_valid(""));
    }

    #[test]
    fn test_get_info() {
        let info = get_info("BR1500000000000010932840814P2").unwrap();
        assert_eq!(info.country_code, "BR");
        assert_eq!(info.check_digits, "15");
        assert_eq!(info.bank_ispb, "00000000");
        assert_eq!(info.branch, "00001");
        assert_eq!(info.account, "0932840814");
        assert_eq!(info.account_type, "P");
        assert_eq!(info.owner, "2");

        let info2 = get_info("BR3860701190000010000012345C1").unwrap();
        assert_eq!(info2.bank_ispb, "60701190");
        assert_eq!(info2.account, "0000012345");
        assert_eq!(info2.account_type, "C");

        assert_eq!(get_info("BR1500000000000010932840814P3"), None);
        assert_eq!(get_info("DE89370400440532013000"), None);
        assert_eq!(get_info(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("BR15 0000 0000 0000 1093 2840 814P 2"),
            "BR1500000000000010932840814P2"
        );
        assert_eq!(
            parse("br15-0000.0000/0000 1093 2840 814p-2"),
            "BR1500000000000010932840814P2"
        );
        assert_eq!(
            parse("BR1500000000000010932840814P2EXTRA"),
            "BR1500000000000010932840814P2"
        );
        assert_eq!(parse(""), "");
    }
}
