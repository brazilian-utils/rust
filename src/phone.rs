/// Phone (telefone) utilities for Brazilian phone numbers.
///
/// Supports both mobile and landline phone numbers.
use rand::Rng;

/// The 67 area codes (DDD) of the Anatel Plano Geral de Numeração.
///
/// Source: Resolução Anatel nº 749/2022. Codes outside this list (e.g. 23,
/// 25, 26, 29, 36, 39, 52, 56-59, 72, 76, 78) do not exist, even though they
/// pass a naive "two digits 1-9" check.
const VALID_DDDS: &[&str] = &[
    "11", "12", "13", "14", "15", "16", "17", "18", "19", // SP
    "21", "22", "24", // RJ
    "27", "28", // ES
    "31", "32", "33", "34", "35", "37", "38", // MG
    "41", "42", "43", "44", "45", "46", // PR
    "47", "48", "49", // SC
    "51", "53", "54", "55", // RS
    "61", // DF
    "62", "64", // GO
    "63", // TO
    "65", "66", // MT
    "67", // MS
    "68", // AC
    "69", // RO
    "71", "73", "74", "75", "77", // BA
    "79", // SE
    "81", "87", // PE
    "82", // AL
    "83", // PB
    "84", // RN
    "85", "88", // CE
    "86", "89", // PI
    "91", "93", "94", // PA
    "92", "97", // AM
    "95", // RR
    "96", // AP
    "98", "99", // MA
];

/// Removes common symbols from a Brazilian phone number string.
///
/// # Arguments
///
/// * `phone_number` - The phone number to remove symbols from.
///
/// # Returns
///
/// A new string with the specified symbols removed.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::remove_symbols;
///
/// assert_eq!(remove_symbols("(11)99402-9275"), "11994029275");
/// assert_eq!(remove_symbols("+55 11 9 9402-9275"), "5511994029275");
/// assert_eq!(remove_symbols("16 3501-4415"), "1635014415");
/// ```
pub fn remove_symbols(phone_number: &str) -> String {
    phone_number
        .replace("(", "")
        .replace(")", "")
        .replace("-", "")
        .replace("+", "")
        .replace(" ", "")
}

/// Removes phone formatting and keeps only digits, capped to 11 digits.
///
/// A country code (`+55`, `0055` or a bare `55`) is stripped first, but only
/// when 10 or 11 digits are left afterwards — so a DDD of `55` is never
/// mistaken for the country code.
///
/// # Arguments
///
/// * `value` - The phone number to parse, with or without formatting.
///
/// # Returns
///
/// The unformatted phone number, capped to 11 digits.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::parse;
///
/// assert_eq!(parse("(11) 98888-7777"), "11988887777");
/// assert_eq!(parse("+55 11 98888-7777"), "11988887777");
/// assert_eq!(parse("55988887777"), "55988887777");
/// ```
pub fn parse(value: &str) -> String {
    strip_country_code_digits(value).chars().take(11).collect()
}

/// Digit-only normalization shared by [`parse`] and the `is_valid*`
/// functions: strips a `+55`/`0055`/bare `55` country code prefix (only when
/// 10 or 11 digits remain afterwards), but does **not** cap the length —
/// unlike [`parse`], callers that need to reject an over-long number must
/// see its true length.
fn strip_country_code_digits(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.starts_with("0055") && matches!(digits.len() as i64 - 4, 10 | 11) {
        digits[4..].to_string()
    } else if digits.starts_with("55") && matches!(digits.len() as i64 - 2, 10 | 11) {
        digits[2..].to_string()
    } else {
        digits
    }
}

/// Checks if a phone number string matches the mobile format.
///
/// Mobile format: DDD + [7-9] + XXXXXXXX (11 digits)
/// - First 2 digits: DDD (area code), must be one of the 67 real Anatel codes
/// - Third digit: 7, 8 or 9 (SMP mobile identifier, Res. Anatel 749/2022,
///   art. 12, I, "a" — only 9 is currently assigned in practice, but 7 and 8
///   are reserved by the same rule)
/// - Remaining 8 digits: Any digit 0-9
///
/// # Arguments
///
/// * `phone_number` - The phone number string to validate.
///
/// # Returns
///
/// `true` if the phone matches mobile format, `false` otherwise.
fn matches_mobile_format(phone_number: &str) -> bool {
    if phone_number.len() != 11 {
        return false;
    }

    let chars: Vec<char> = phone_number.chars().collect();

    // Check if all characters are digits
    if !chars.iter().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // First two digits (DDD) must be a real Anatel area code
    if !VALID_DDDS.contains(&&phone_number[0..2]) {
        return false;
    }

    // Third digit must be 7, 8 or 9 (mobile identifier)
    if !('7'..='9').contains(&chars[2]) {
        return false;
    }

    true
}

/// Checks if a phone number string matches the landline format.
///
/// Landline format: DDD + [2-6] + XXXXXXX (10 digits)
/// - First 2 digits: DDD (area code), must be one of the 67 real Anatel codes
/// - Third digit: 2 to 6 (STFC/SCM landline identifier, Res. Anatel 749/2022,
///   art. 11, I, "a")
/// - Remaining 7 digits: Any digit 0-9
///
/// # Arguments
///
/// * `phone_number` - The phone number string to validate.
///
/// # Returns
///
/// `true` if the phone matches landline format, `false` otherwise.
fn matches_landline_format(phone_number: &str) -> bool {
    if phone_number.len() != 10 {
        return false;
    }

    let chars: Vec<char> = phone_number.chars().collect();

    // Check if all characters are digits
    if !chars.iter().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // First two digits (DDD) must be a real Anatel area code
    if !VALID_DDDS.contains(&&phone_number[0..2]) {
        return false;
    }

    // Third digit must be 2-6 (landline identifier)
    if !('2'..='6').contains(&chars[2]) {
        return false;
    }

    true
}

/// Options for [`is_valid`].
#[derive(Debug, Clone, Default)]
pub struct IsValidPhoneOptions {
    /// Restricts the accepted kind: `"mobile"`, `"landline"` or `"service"`.
    /// Omitted, both mobile and landline are accepted (never service).
    pub kind: Option<String>,
    /// The mobile numbering rule `is_valid_mobile` uses (1 or 2). Defaults to 1.
    pub mobile_version: Option<u8>,
}

/// Returns if a Brazilian phone number is valid.
///
/// It does not verify if the number actually exists. A country code (`+55`,
/// `0055` or a bare `55`) is accepted and removed first, as [`parse`] does.
///
/// # Arguments
///
/// * `phone_number` - The phone number to validate.
/// * `options` - Optionally restricts the accepted kind and picks the mobile
///   numbering rule; see [`IsValidPhoneOptions`].
///
/// # Returns
///
/// `true` if the phone number is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::is_valid;
///
/// // Valid mobile
/// assert!(is_valid("11994029275", None));
///
/// // Valid landline
/// assert!(is_valid("1635014415", None));
///
/// // Invalid
/// assert!(!is_valid("123", None));
/// ```
pub fn is_valid(phone_number: &str, options: Option<IsValidPhoneOptions>) -> bool {
    let opts = options.unwrap_or_default();
    let cleaned = strip_country_code_digits(phone_number);
    let version = opts.mobile_version.unwrap_or(1);

    match opts.kind.as_deref() {
        Some("mobile") => matches_mobile_versioned(&cleaned, version),
        Some("landline") => matches_landline_format(&cleaned),
        Some("service") => is_valid_service(&cleaned),
        _ => matches_mobile_versioned(&cleaned, version) || matches_landline_format(&cleaned),
    }
}

fn matches_mobile_versioned(cleaned: &str, _version: u8) -> bool {
    // Both versions currently share the same 7-8-9 rule; version 2 (Res.
    // Anatel 749/2022) additionally excludes the reserved 700 (satellite)
    // series, which is not implemented for lack of a verifiable published
    // list of exactly which numbers that covers.
    matches_mobile_format(cleaned)
}

/// Validates a Brazilian mobile phone number: DDD plus 9 digits.
///
/// A country code (`+55`, `0055` or a bare `55`) is accepted and removed
/// first, as [`parse`] does.
///
/// `version` picks the numbering rule (1, the default, or 2); see
/// [`is_valid`]'s [`IsValidPhoneOptions::mobile_version`] for details.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::is_valid_mobile;
///
/// assert!(is_valid_mobile("(11) 98765-4321", None));
/// assert!(is_valid_mobile("+55 11 98765-4321", None));
/// assert!(!is_valid_mobile("1130000000", None));
/// ```
pub fn is_valid_mobile(phone_number: &str, version: Option<u8>) -> bool {
    let cleaned = strip_country_code_digits(phone_number);
    matches_mobile_versioned(&cleaned, version.unwrap_or(1))
}

/// Validates a Brazilian landline phone number: DDD plus 8 digits.
///
/// A country code (`+55`, `0055` or a bare `55`) is accepted and removed
/// first, as [`parse`] does.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::is_valid_landline;
///
/// assert!(is_valid_landline("(11) 3000-0000"));
/// assert!(!is_valid_landline("11987654321"));
/// ```
pub fn is_valid_landline(phone_number: &str) -> bool {
    let cleaned = strip_country_code_digits(phone_number);
    matches_landline_format(&cleaned)
}

/// Validates a Brazilian service number (dialed without a DDD).
///
/// Only the structure is checked:
/// - Códigos Não Geográficos `0300`, `0303`, `0500`, `0800` and `0900`
///   followed by 7 digits (11 digits total).
/// - The abbreviated `300X`/`400X` numbers (8 digits total).
/// - A small set of well-known 3-digit public-utility codes (e.g. `190`,
///   `192`). This list is best-effort, not exhaustive.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::is_valid_service;
///
/// assert!(is_valid_service("0800 123 4567"));
/// assert!(is_valid_service("40041234"));
/// assert!(!is_valid_service("911"));
/// ```
pub fn is_valid_service(phone_number: &str) -> bool {
    let digits: String = phone_number.chars().filter(|c| c.is_ascii_digit()).collect();

    const CNG_PREFIXES: &[&str] = &["0300", "0303", "0500", "0800", "0900"];
    if digits.len() == 11 && CNG_PREFIXES.contains(&&digits[0..4]) {
        return true;
    }

    if digits.len() == 8 && (digits.starts_with("300") || digits.starts_with("400")) {
        return true;
    }

    // Anatel-designated public-utility short codes. Best-effort list; `112`
    // and `911` are deliberately excluded (not Brazilian short codes).
    const UTILITY_CODES: &[&str] = &[
        "100", "180", "181", "190", "191", "192", "193", "197", "198", "199",
    ];
    if digits.len() == 3 && UTILITY_CODES.contains(&digits.as_str()) {
        return true;
    }

    false
}

/// Function responsible for formatting a telephone number under the
/// "subscriber number" mask (the contract's default): the first 9 digits of
/// the (symbol-stripped) input, split as `XXXXX-XXXX`. It does not validate
/// the input first — unlike a DDD-aware mask, this mask does not need to
/// know whether the value carries an area code.
///
/// # Arguments
///
/// * `phone` - The phone number to format.
///
/// # Returns
///
/// The formatted phone number. Returns an empty string when there is no
/// digit at all (mirroring the contract's reference implementation, which
/// never returns null here).
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::format_phone;
///
/// assert_eq!(format_phone("988887777"), "98888-7777");
/// assert_eq!(format_phone("98888777"), "98888-777");
/// assert_eq!(format_phone(""), "");
///
/// // The default mask ignores any DDD and keeps only the first 9 digits.
/// assert_eq!(format_phone("1130000000"), "11300-0000");
/// ```
pub fn format_phone(phone: &str) -> String {
    let cleaned = remove_symbols(phone);
    let truncated: String = cleaned.chars().filter(|c| c.is_ascii_digit()).take(9).collect();
    let len = truncated.len();

    if len == 0 {
        return String::new();
    }

    let split = len.min(5);
    let prefix = &truncated[0..split];
    let suffix = &truncated[split..];

    if suffix.is_empty() {
        prefix.to_string()
    } else {
        format!("{}-{}", prefix, suffix)
    }
}

/// Generate a valid DDD (area code) number.
///
/// # Returns
///
/// A real Anatel DDD, picked at random from [`VALID_DDDS`].
fn generate_ddd_number() -> String {
    let mut rng = rand::thread_rng();
    VALID_DDDS[rng.gen_range(0..VALID_DDDS.len())].to_string()
}

/// Generate a valid and random mobile phone number.
///
/// # Returns
///
/// An 11-digit mobile phone number string.
fn generate_mobile_phone() -> String {
    let mut rng = rand::thread_rng();
    let ddd = generate_ddd_number();
    let client_number: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();

    format!("{}9{}", ddd, client_number)
}

/// Generate a valid and random landline phone number.
///
/// # Returns
///
/// A 10-digit landline phone number string.
fn generate_landline_phone() -> String {
    let mut rng = rand::thread_rng();
    let ddd = generate_ddd_number();
    let first_digit = rng.gen_range(2..=6);
    let remaining = format!("{:07}", rng.gen_range(0..=9999999));

    format!("{}{}{}", ddd, first_digit, remaining)
}

/// Generate a valid and random phone number.
///
/// # Arguments
///
/// * `phone_type` - Optional type: "landline" or "mobile".
///   If not specified, generates either type randomly.
///
/// # Returns
///
/// A randomly generated valid phone number.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::generate;
///
/// // Generate any type
/// let phone = generate(None);
/// assert!(phone.len() == 10 || phone.len() == 11);
///
/// // Generate mobile
/// let mobile = generate(Some("mobile"));
/// assert_eq!(mobile.len(), 11);
///
/// // Generate landline
/// let landline = generate(Some("landline"));
/// assert_eq!(landline.len(), 10);
/// ```
pub fn generate(phone_type: Option<&str>) -> String {
    match phone_type {
        Some("mobile") => generate_mobile_phone(),
        Some("landline") => generate_landline_phone(),
        _ => {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                generate_mobile_phone()
            } else {
                generate_landline_phone()
            }
        }
    }
}

/// Function responsible for removing an international dialing code from a phone number.
///
/// # Arguments
///
/// * `phone_number` - The phone number with potential international code.
///
/// # Returns
///
/// The phone number without international code, or the same phone number if no code found.
///
/// # Examples
///
/// ```
/// use brazilian_utils::phone::remove_international_dialing_code;
///
/// assert_eq!(remove_international_dialing_code("5511994029275"), "11994029275");
/// assert_eq!(remove_international_dialing_code("1635014415"), "1635014415");
/// assert_eq!(remove_international_dialing_code("+5511994029275"), "11994029275");
/// ```
pub fn remove_international_dialing_code(phone_number: &str) -> String {
    let cleaned = phone_number.replace(' ', "");
    let digits = cleaned.strip_prefix('+').unwrap_or(&cleaned);

    // Check if starts with 55 and has more than 11 digits (i.e. there's a
    // national number left over after the country code). The leading '+',
    // if any, is dropped along with the country code — it's not part of the
    // national number, so it must not survive in the result.
    if digits.len() > 11 && digits.starts_with("55") {
        return digits[2..].to_string();
    }

    phone_number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(k: &str) -> Option<IsValidPhoneOptions> {
        Some(IsValidPhoneOptions {
            kind: Some(k.to_string()),
            mobile_version: None,
        })
    }

    #[test]
    fn test_remove_symbols() {
        assert_eq!(remove_symbols("(11)99402-9275"), "11994029275");
        assert_eq!(remove_symbols("+55 11 9 9402-9275"), "5511994029275");
        assert_eq!(remove_symbols("16 3501-4415"), "1635014415");
        assert_eq!(remove_symbols("1635014415"), "1635014415");
    }

    #[test]
    fn test_is_valid_mobile() {
        assert!(is_valid("11994029275", kind("mobile")));
        assert!(is_valid("21987654321", kind("mobile")));
        assert!(is_valid("85912345678", kind("mobile")));

        // 7 and 8 are reserved for SMP too (Res. Anatel 749/2022, art. 12, I,
        // "a"), even though only 9 is assigned in practice today.
        assert!(is_valid("11894029275", kind("mobile")));
        assert!(is_valid("11794029275", kind("mobile")));

        assert!(!is_valid("1635014415", kind("mobile")));
        assert!(!is_valid("11694029275", kind("mobile"))); // 6 is a landline identifier
        assert!(!is_valid("1194029275", kind("mobile"))); // Too short
        assert!(!is_valid("119940292751", kind("mobile"))); // Too long
        assert!(!is_valid("23994029275", kind("mobile"))); // 23 is not a real DDD
    }

    #[test]
    fn test_is_valid_landline() {
        assert!(is_valid("1635014415", kind("landline")));
        assert!(is_valid("1133334444", kind("landline")));
        assert!(is_valid("8532221111", kind("landline")));

        // 6 is a valid landline identifier too (Res. Anatel 749/2022, art. 11, I, "a")
        assert!(is_valid("1665014415", kind("landline")));

        assert!(!is_valid("11994029275", kind("landline")));
        assert!(!is_valid("1635014415", kind("mobile")));
        assert!(!is_valid("163501441", kind("landline"))); // Too short
        assert!(!is_valid("16350144151", kind("landline"))); // Too long
        assert!(!is_valid("1615014415", kind("landline"))); // 1 is not a landline identifier
        assert!(!is_valid("2335014415", kind("landline"))); // 23 is not a real DDD
    }

    #[test]
    fn test_is_valid_any_type() {
        assert!(is_valid("11994029275", None));
        assert!(is_valid("1635014415", None));
        assert!(is_valid("21987654321", None));
        assert!(is_valid("1133334444", None));
        assert!(is_valid("1665014415", None));

        assert!(!is_valid("123", None));
        assert!(!is_valid("1615014415", None));
        assert!(!is_valid("2335014415", None)); // 23 is not a real DDD
    }

    #[test]
    fn test_format_phone() {
        assert_eq!(format_phone("988887777"), "98888-7777");
        assert_eq!(format_phone("98888777"), "98888-777");
        assert_eq!(format_phone(""), "");
        assert_eq!(format_phone("1130000000"), "11300-0000");
    }

    #[test]
    fn test_is_valid_mobile_fn() {
        assert!(is_valid_mobile("(11) 98765-4321", None));
        assert!(is_valid_mobile("+55 11 98765-4321", None));
        assert!(is_valid_mobile("5511987654321", None));
        assert!(!is_valid_mobile("1130000000", None));
        assert!(!is_valid_mobile("1198765432", None));
        assert!(!is_valid_mobile("", None));
    }

    #[test]
    fn test_is_valid_landline_fn() {
        assert!(is_valid_landline("(11) 3000-0000"));
        assert!(is_valid_landline("1130000000"));
        assert!(is_valid_landline("+55 11 3000-0000"));
        assert!(!is_valid_landline("11987654321"));
        assert!(!is_valid_landline("113000000"));
        assert!(!is_valid_landline(""));
    }

    #[test]
    fn test_is_valid_service_fn() {
        assert!(is_valid_service("08001234567"));
        assert!(is_valid_service("0800 123 4567"));
        assert!(is_valid_service("40041234"));
        assert!(!is_valid_service("11987654321"));
        assert!(!is_valid_service("0800123456"));
        assert!(!is_valid_service("911"));
        assert!(!is_valid_service(""));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("(11) 98888-7777"), "11988887777");
        assert_eq!(parse("98888-7777"), "988887777");
        assert_eq!(parse("+55 11 98888-7777"), "11988887777");
        assert_eq!(parse("005511988887777"), "11988887777");
        assert_eq!(parse("551130000000"), "1130000000");
        assert_eq!(parse("55988887777"), "55988887777");
        assert_eq!(parse("0800 123 4567"), "08001234567");
        assert_eq!(parse("11988887777123"), "11988887777");
        assert_eq!(parse(""), "");
    }

    #[test]
    fn test_generate_mobile() {
        let mobile = generate(Some("mobile"));
        assert_eq!(mobile.len(), 11);
        assert!(is_valid(&mobile, kind("mobile")));
    }

    #[test]
    fn test_generate_landline() {
        let landline = generate(Some("landline"));
        assert_eq!(landline.len(), 10);
        assert!(is_valid(&landline, kind("landline")));
    }

    #[test]
    fn test_generate_any_type() {
        let phone = generate(None);
        assert!(phone.len() == 10 || phone.len() == 11);
        assert!(is_valid(&phone, None));
    }

    #[test]
    fn test_generate_uniqueness() {
        let phone1 = generate(None);
        let phone2 = generate(None);
        let phone3 = generate(None);

        // Very unlikely to generate the same phone twice
        assert!(phone1 != phone2 || phone2 != phone3);
    }

    #[test]
    fn test_remove_international_dialing_code() {
        assert_eq!(
            remove_international_dialing_code("5511994029275"),
            "11994029275"
        );
        assert_eq!(
            remove_international_dialing_code("551635014415"),
            "1635014415"
        );
        assert_eq!(
            remove_international_dialing_code("+551635014415"),
            "1635014415"
        );
        // The leading '+' must not survive: it's part of the dialing code,
        // not the national number.
        assert_eq!(
            remove_international_dialing_code("+5511994029275"),
            "11994029275"
        );

        // Should not remove if length is 11 or less
        assert_eq!(
            remove_international_dialing_code("11994029275"),
            "11994029275"
        );
        assert_eq!(
            remove_international_dialing_code("1635014415"),
            "1635014415"
        );
    }

    #[test]
    fn test_remove_international_dialing_code_with_spaces() {
        assert_eq!(
            remove_international_dialing_code("55 11 99402 9275"),
            "11994029275"
        );
    }
}
