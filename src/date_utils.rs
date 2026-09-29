use crate::currency::number_to_words;
/// Date utilities for Brazilian dates.
use chrono::{Datelike, NaiveDate, Weekday};

/// Get the month name in Brazilian Portuguese.
///
/// # Arguments
///
/// * `month` - The month number (1-12)
///
/// # Returns
///
/// The month name in Portuguese.
fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "janeiro",
        2 => "fevereiro",
        3 => "março",
        4 => "abril",
        5 => "maio",
        6 => "junho",
        7 => "julho",
        8 => "agosto",
        9 => "setembro",
        10 => "outubro",
        11 => "novembro",
        12 => "dezembro",
        _ => "",
    }
}

/// Converts a date string in Brazilian format (dd/mm/yyyy) to its textual representation.
///
/// This function takes a date as a string in the format dd/mm/yyyy and converts it
/// to a string with the date written out in Brazilian Portuguese, including the full
/// month name and the year.
///
/// # Arguments
///
/// * `date` - The date to be converted into text. Expected format: dd/mm/yyyy.
///
/// # Returns
///
/// A string with the date written out in Brazilian Portuguese, or None if the date is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::convert_date_to_text;
///
/// assert_eq!(
///     convert_date_to_text("01/01/2024"),
///     Some("Primeiro de janeiro de dois mil e vinte e quatro".to_string())
/// );
/// assert_eq!(
///     convert_date_to_text("15/08/1990"),
///     Some("Quinze de agosto de mil novecentos e noventa".to_string())
/// );
/// assert_eq!(convert_date_to_text("invalid"), None);
/// ```
pub fn convert_date_to_text(date: &str) -> Option<String> {
    // Check format with regex
    let parts: Vec<&str> = date.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let year: i32 = parts[2].parse().ok()?;

    // Validate the date
    NaiveDate::from_ymd_opt(year, month, day)?;

    // Convert day to text
    let day_str = if day == 1 {
        "Primeiro".to_string()
    } else {
        let mut text = number_to_words(day as i64);
        // Capitalize first letter
        if let Some(first_char) = text.chars().next() {
            text = first_char.to_uppercase().to_string() + &text[first_char.len_utf8()..];
        }
        text
    };

    let month_name = get_month_name(month);
    let year_str = number_to_words(year as i64);

    Some(format!("{} de {} de {}", day_str, month_name, year_str))
}

/// Calculates the date of Easter Sunday for a given year using the Anonymous Gregorian algorithm.
///
/// # Arguments
///
/// * `year` - The year for which to calculate Easter
///
/// # Returns
///
/// The date of Easter Sunday
fn calculate_easter(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;

    NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}

/// Checks if the given date is a national or state holiday in Brazil.
///
/// This function takes a date and an optional UF (Unidade Federativa),
/// returning a boolean value indicating whether the date is a holiday or None
/// if the UF is invalid.
///
/// The method does not handle municipal holidays.
///
/// # Arguments
///
/// * `target_date` - The date to be checked.
/// * `uf` - The state abbreviation (UF) to check for state holidays.
///   If not provided, only national holidays will be considered.
///
/// # Returns
///
/// Returns `Some(true)` if the date is a holiday, `Some(false)` if it is not,
/// or `None` if the UF is invalid.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::{is_holiday, IsHolidayParams};
/// use chrono::NaiveDate;
///
/// // New Year's Day
/// assert_eq!(is_holiday(Some(IsHolidayParams {
///     date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
///     uf: None,
/// })), Some(true));
///
/// // Regular day
/// assert_eq!(is_holiday(Some(IsHolidayParams {
///     date: Some(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()),
///     uf: None,
/// })), Some(false));
///
/// // Independence Day (Bahia state holiday)
/// assert_eq!(is_holiday(Some(IsHolidayParams {
///     date: Some(NaiveDate::from_ymd_opt(2024, 7, 2).unwrap()),
///     uf: Some("BA".to_string()),
/// })), Some(true));
///
/// // Invalid UF
/// assert_eq!(is_holiday(Some(IsHolidayParams {
///     date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
///     uf: Some("XX".to_string()),
/// })), None);
/// ```
/// Parameters for [`is_holiday`].
#[derive(Debug, Clone, Default)]
pub struct IsHolidayParams {
    /// The date to check. Defaults to today (local date) when omitted.
    pub date: Option<NaiveDate>,
    /// The state abbreviation (UF) whose state holidays also count. An
    /// unknown code makes `is_holiday` return `None` (mirrors the previous
    /// two-argument behavior); omitted, only national holidays are checked.
    pub uf: Option<String>,
}

pub fn is_holiday(options: Option<IsHolidayParams>) -> Option<bool> {
    const VALID_UFS: &[&str] = &[
        "AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB",
        "PE", "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO",
    ];

    let opts = options.unwrap_or_default();
    let target_date = opts
        .date
        .unwrap_or_else(|| chrono::Local::now().date_naive());
    let uf = opts.uf.as_deref();

    // Check if UF is valid
    if let Some(state) = uf {
        if !VALID_UFS.contains(&state) {
            return None;
        }
    }

    let year = target_date.year();
    let month = target_date.month();
    let day = target_date.day();

    // Check national holidays
    if is_national_holiday(year, month, day, target_date) {
        return Some(true);
    }

    // Check state holidays if UF is provided
    if let Some(state) = uf {
        if is_state_holiday(year, month, day, target_date, state) {
            return Some(true);
        }
    }

    Some(false)
}

/// Checks if a date is a national holiday in Brazil.
fn is_national_holiday(year: i32, month: u32, day: u32, date: NaiveDate) -> bool {
    // Check fixed national holidays
    let is_fixed_holiday = match (month, day) {
        (1, 1) => true, // New Year's Day - Confraternização Universal
        (4, 21) if year != 1931 && year != 1932 => true, // Tiradentes' Day
        (5, 1) if year >= 1925 => true, // Labor Day
        (9, 7) if year >= 1890 => true, // Independence Day
        (10, 12) if year <= 1930 || year >= 1980 => true, // Our Lady of Aparecida
        (11, 2) => true, // All Souls' Day - Finados
        (11, 15) => true, // Republic Proclamation Day
        (11, 20) if year >= 2024 => true, // Black Awareness Day (national, Lei 14.759/2023)
        (12, 25) if year >= 1922 => true, // Christmas Day
        _ => false,
    };

    if is_fixed_holiday {
        return true;
    }

    // Check movable holidays (Easter-based)
    let easter = calculate_easter(year);

    // Good Friday (2 days before Easter)
    let good_friday = easter - chrono::Duration::days(2);
    date == good_friday
}

/// Checks if a date is a state holiday in Brazil.
fn is_state_holiday(year: i32, month: u32, day: u32, date: NaiveDate, uf: &str) -> bool {
    if year < 1996 {
        // Lei n. 9.093, de 12.09.1995 - state holidays only from 1996 onwards
        return false;
    }

    match uf {
        "AC" => {
            match (month, day) {
                (1, 23) if year >= 2005 => true, // Evangelical Day
                (3, 8) if year >= 2002 => true,  // International Women's Day
                (6, 15) => true,                 // Founding of Acre
                (9, 5) if year >= 2004 => true,  // Amazonia Day
                (11, 17) => true,                // Signing of the Petropolis Treaty
                _ => false,
            }
        }
        "AL" => {
            matches!(
                (month, day),
                (6, 24) |   // Saint John's Day
                (6, 29) |   // Saint Peter's Day
                (9, 16) |   // Political Emancipation of Alagoas
                (11, 20) // Black Awareness Day
            ) || (month == 11 && day == 30 && year >= 2013) // Evangelical Day
        }
        "AM" => {
            (month == 9 && day == 5) ||  // Elevation of Amazonas to province
            (month == 11 && day == 20 && year >= 2010) // Black Awareness Day
        }
        "AP" => {
            match (month, day) {
                (3, 19) if year >= 2003 => true,  // Saint Joseph's Day
                (7, 25) if year >= 2012 => true,  // Saint James' Day
                (9, 13) => true,                  // Creation of the Federal Territory
                (11, 20) if year >= 2008 => true, // Black Awareness Day
                _ => false,
            }
        }
        "BA" => {
            matches!((month, day), (7, 2)) // Bahia Independence Day
        }
        "CE" => {
            matches!(
                (month, day),
                (3, 19) |   // Saint Joseph's Day
                (3, 25) // Abolition of slavery in Ceará
            ) || (month == 8 && day == 15 && year >= 2004) // Our Lady of Assumption
        }
        "DF" => {
            matches!(
                (month, day),
                (4, 21) |   // Founding of Brasilia
                (11, 30) // Evangelical Day
            )
        }
        "ES" => {
            // Our Lady of Penha (Easter Sunday + 8 days)
            if year >= 2020 {
                let easter = calculate_easter(year);
                let penha = easter + chrono::Duration::days(8);
                date == penha
            } else {
                false
            }
        }
        "GO" => {
            matches!(
                (month, day),
                (7, 26) |   // Foundation of Goiás city
                (10, 24) // Foundation of Goiânia
            )
        }
        "MA" => {
            matches!((month, day), (7, 28)) // Maranhão joining to independence
        }
        "MG" => {
            matches!((month, day), (4, 21)) // Tiradentes' Execution
        }
        "MS" => {
            matches!((month, day), (10, 11)) // State Creation Day
        }
        "MT" => {
            month == 11 && day == 20 && year >= 2003 // Black Awareness Day
        }
        "PA" => {
            matches!((month, day), (8, 15)) // Grão-Pará joining to independence
        }
        "PB" => {
            matches!((month, day), (8, 5)) // State Founding Day
        }
        "PE" => {
            // Pernambuco Revolution (1st Sunday of March)
            if year >= 2008 && month == 3 {
                let first_day = NaiveDate::from_ymd_opt(year, 3, 1).unwrap();
                let first_sunday = match first_day.weekday() {
                    Weekday::Sun => first_day,
                    _ => {
                        let days_until_sunday = 7 - first_day.weekday().num_days_from_sunday();
                        first_day + chrono::Duration::days(days_until_sunday as i64)
                    }
                };
                date == first_sunday
            } else {
                false
            }
        }
        "PI" => {
            matches!((month, day), (10, 19)) // Piauí Day
        }
        "PR" => {
            matches!((month, day), (12, 19)) // Emancipation of Paraná
        }
        "RJ" => {
            match (month, day) {
                (4, 23) if year >= 2008 => true,  // Saint George's Day
                (11, 20) if year >= 2002 => true, // Black Awareness Day
                _ => false,
            }
        }
        "RN" => {
            match (month, day) {
                (8, 7) if year >= 2000 => true,  // Rio Grande do Norte Day
                (10, 3) if year >= 2007 => true, // Uruaçú and Cunhaú Martyrs Day
                _ => false,
            }
        }
        "RO" => {
            (month == 1 && day == 4) ||  // State Creation Day
            (month == 6 && day == 18 && year >= 2002) // Evangelical Day
        }
        "RR" => {
            matches!((month, day), (10, 5)) // State Creation Day
        }
        "RS" => {
            matches!((month, day), (9, 20)) // Gaucho Day
        }
        "SC" => {
            // Santa Catarina State Day (1st Sunday from Aug 11, if >= 2005)
            let is_sc_state_day = if year >= 2005 {
                if month == 8 && day >= 11 {
                    let target = NaiveDate::from_ymd_opt(year, 8, 11).unwrap();
                    let first_sunday = match target.weekday() {
                        Weekday::Sun => target,
                        _ => {
                            let days_until_sunday = 7 - target.weekday().num_days_from_sunday();
                            target + chrono::Duration::days(days_until_sunday as i64)
                        }
                    };
                    date == first_sunday
                } else {
                    false
                }
            } else if year == 2004 {
                month == 8 && day == 11
            } else {
                false
            };

            // Saint Catherine of Alexandria Day (Nov 25 or 1st Sunday from Nov 25)
            let is_saint_catherine = if (1999..=2030).contains(&year) && year != 2004 {
                if month == 11 && day >= 25 {
                    let target = NaiveDate::from_ymd_opt(year, 11, 25).unwrap();
                    let first_sunday = match target.weekday() {
                        Weekday::Sun => target,
                        _ => {
                            let days_until_sunday = 7 - target.weekday().num_days_from_sunday();
                            target + chrono::Duration::days(days_until_sunday as i64)
                        }
                    };
                    date == first_sunday
                } else {
                    false
                }
            } else {
                month == 11 && day == 25
            };

            is_sc_state_day || is_saint_catherine
        }
        "SE" => {
            matches!((month, day), (7, 8)) // Sergipe Political Emancipation Day
        }
        "SP" => {
            month == 7 && day == 9 && year >= 1997 // Constitutionalist Revolution
        }
        "TO" => {
            match (month, day) {
                (3, 18) if year >= 1998 => true, // Autonomy Day
                (9, 8) => true,                  // Our Lady of Nativity
                (10, 5) => true,                 // State Creation Day
                _ => false,
            }
        }
        _ => false,
    }
}

/// Checks if a date is an "optional" (facultative) Brazilian holiday:
/// Carnaval (Monday and Tuesday before Ash Wednesday) and Corpus Christi.
/// Neither is fixed by federal law, but both are conventionally observed
/// and are counted by default by the business-day functions.
fn is_optional_holiday(date: NaiveDate) -> bool {
    let easter = calculate_easter(date.year());
    date == easter - chrono::Duration::days(48) // Carnaval Monday
        || date == easter - chrono::Duration::days(47) // Carnaval Tuesday
        || date == easter + chrono::Duration::days(60) // Corpus Christi
}

/// A single Brazilian holiday, as returned by [`get_holidays`].
#[derive(Debug, Clone, PartialEq)]
pub struct Holiday {
    /// The holiday's name, in Brazilian Portuguese.
    pub name: String,
    /// The calendar date of the holiday for the requested year.
    pub date: NaiveDate,
    /// One of `"national"`, `"state"`, `"optional"` or `"religious"`.
    pub holiday_type: String,
}

/// Returns the national Brazilian holidays of a year, sorted by date.
///
/// This includes the fixed and movable national holidays (`"national"`),
/// Good Friday (`"religious"`) and the optional/facultative Carnaval and
/// Corpus Christi holidays (`"optional"`). It does not include state
/// holidays (see [`is_holiday`]'s `uf` option for those).
///
/// # Arguments
///
/// * `year` - The year to list holidays for.
///
/// # Returns
///
/// The year's holidays sorted by date, or an empty list when `year` is
/// outside 1900 to 2099.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::get_holidays;
///
/// let holidays = get_holidays(2024);
/// assert!(holidays.iter().any(|h| h.name == "Confraternização Universal"));
/// assert!(get_holidays(1800).is_empty());
/// ```
pub fn get_holidays(year: i32) -> Vec<Holiday> {
    if !(1900..=2099).contains(&year) {
        return Vec::new();
    }

    let mut holidays = Vec::new();
    let mut push = |name: &str, date: Option<NaiveDate>, holiday_type: &str| {
        if let Some(date) = date {
            holidays.push(Holiday {
                name: name.to_string(),
                date,
                holiday_type: holiday_type.to_string(),
            });
        }
    };

    push(
        "Confraternização Universal",
        NaiveDate::from_ymd_opt(year, 1, 1),
        "national",
    );
    if year != 1931 && year != 1932 {
        push("Tiradentes", NaiveDate::from_ymd_opt(year, 4, 21), "national");
    }
    if year >= 1925 {
        push("Dia do Trabalho", NaiveDate::from_ymd_opt(year, 5, 1), "national");
    }
    if year >= 1890 {
        push(
            "Independência do Brasil",
            NaiveDate::from_ymd_opt(year, 9, 7),
            "national",
        );
    }
    if year <= 1930 || year >= 1980 {
        push(
            "Nossa Senhora Aparecida",
            NaiveDate::from_ymd_opt(year, 10, 12),
            "national",
        );
    }
    push("Finados", NaiveDate::from_ymd_opt(year, 11, 2), "national");
    push(
        "Proclamação da República",
        NaiveDate::from_ymd_opt(year, 11, 15),
        "national",
    );
    if year >= 2024 {
        push(
            "Dia da Consciência Negra",
            NaiveDate::from_ymd_opt(year, 11, 20),
            "national",
        );
    }
    if year >= 1922 {
        push("Natal", NaiveDate::from_ymd_opt(year, 12, 25), "national");
    }

    let easter = calculate_easter(year);
    push(
        "Carnaval (segunda-feira)",
        Some(easter - chrono::Duration::days(48)),
        "optional",
    );
    push(
        "Carnaval (terça-feira)",
        Some(easter - chrono::Duration::days(47)),
        "optional",
    );
    push(
        "Sexta-feira Santa",
        Some(easter - chrono::Duration::days(2)),
        "religious",
    );
    push(
        "Corpus Christi",
        Some(easter + chrono::Duration::days(60)),
        "optional",
    );

    holidays.sort_by_key(|h| h.date);
    holidays
}

/// Options shared by the business-day functions.
#[derive(Debug, Clone, Default)]
pub struct BusinessDayOptions {
    /// Whether Carnaval and Corpus Christi also count as non-business days.
    /// Defaults to `true`.
    pub include_optional: Option<bool>,
    /// A state abbreviation (UF) whose state holidays also count.
    pub uf: Option<String>,
}

fn is_business_day_internal(date: NaiveDate, opts: &BusinessDayOptions) -> bool {
    if !(1900..=2099).contains(&date.year()) {
        return false;
    }

    let weekday = date.weekday();
    if weekday == Weekday::Sat || weekday == Weekday::Sun {
        return false;
    }

    if is_national_holiday(date.year(), date.month(), date.day(), date) {
        return false;
    }

    if opts.include_optional.unwrap_or(true) && is_optional_holiday(date) {
        return false;
    }

    if let Some(state) = opts.uf.as_deref() {
        if is_state_holiday(date.year(), date.month(), date.day(), date, state) {
            return false;
        }
    }

    true
}

/// Checks whether a date is a Brazilian business day (dia útil): not a
/// Saturday, a Sunday or a holiday.
///
/// # Arguments
///
/// * `value` - The date to check.
/// * `options` - Optionally excludes the optional holidays from the check
///   and/or adds a state's holidays; see [`BusinessDayOptions`].
///
/// # Returns
///
/// `true` when `value` is a business day, `false` otherwise (including when
/// its year is outside 1900 to 2099).
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::is_business_day;
/// use chrono::NaiveDate;
///
/// assert!(is_business_day(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(), None));
/// assert!(!is_business_day(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), None)); // holiday
/// assert!(!is_business_day(NaiveDate::from_ymd_opt(2024, 1, 6).unwrap(), None)); // Saturday
/// ```
pub fn is_business_day(value: NaiveDate, options: Option<BusinessDayOptions>) -> bool {
    let opts = options.unwrap_or_default();
    is_business_day_internal(value, &opts)
}

/// Adds a number of Brazilian business days to a date.
///
/// # Arguments
///
/// * `date` - The starting date.
/// * `amount` - The number of business days to add; negative walks backwards,
///   `0` returns `date` unchanged (even when it is not itself a business day).
/// * `options` - See [`BusinessDayOptions`].
///
/// # Returns
///
/// The resulting date, or `None` if `date` or the result falls outside the
/// years 1900 to 2099.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::add_business_days;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // holiday (Monday)
/// let result = add_business_days(start, 1, None);
/// assert_eq!(result, NaiveDate::from_ymd_opt(2024, 1, 2));
/// ```
pub fn add_business_days(
    date: NaiveDate,
    amount: i64,
    options: Option<BusinessDayOptions>,
) -> Option<NaiveDate> {
    if !(1900..=2099).contains(&date.year()) {
        return None;
    }

    if amount == 0 {
        return Some(date);
    }

    let opts = options.unwrap_or_default();
    let step: i64 = if amount > 0 { 1 } else { -1 };
    let mut remaining = amount.abs();
    let mut current = date;

    while remaining > 0 {
        current += chrono::Duration::days(step);
        if !(1900..=2099).contains(&current.year()) {
            return None;
        }
        if is_business_day_internal(current, &opts) {
            remaining -= 1;
        }
    }

    Some(current)
}

/// Subtracts a number of Brazilian business days from a date.
///
/// Equivalent to [`add_business_days`] with the opposite `amount`.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::sub_business_days;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
/// let result = sub_business_days(start, 1, None);
/// assert_eq!(result, NaiveDate::from_ymd_opt(2023, 12, 29));
/// ```
pub fn sub_business_days(
    date: NaiveDate,
    amount: i64,
    options: Option<BusinessDayOptions>,
) -> Option<NaiveDate> {
    add_business_days(date, -amount, options)
}

/// Counts the Brazilian business days between two dates.
///
/// Counts `earlier_date` when it is a business day, and every business day
/// strictly between the two; `later_date` is never counted. Negative when
/// `later_date` is before `earlier_date`; `0` on the same calendar day.
///
/// # Arguments
///
/// * `later_date` - The later of the two dates (by convention).
/// * `earlier_date` - The earlier of the two dates (by convention).
/// * `options` - See [`BusinessDayOptions`].
///
/// # Returns
///
/// The signed count of business days, or `None` when either date is outside
/// the years 1900 to 2099.
pub fn difference_in_business_days(
    later_date: NaiveDate,
    earlier_date: NaiveDate,
    options: Option<BusinessDayOptions>,
) -> Option<i64> {
    if !(1900..=2099).contains(&later_date.year()) || !(1900..=2099).contains(&earlier_date.year())
    {
        return None;
    }

    if later_date == earlier_date {
        return Some(0);
    }

    let opts = options.unwrap_or_default();
    let (start, end, sign) = if later_date > earlier_date {
        (earlier_date, later_date, 1)
    } else {
        (later_date, earlier_date, -1)
    };

    let mut count: i64 = 0;
    let mut current = start;
    while current < end {
        if is_business_day_internal(current, &opts) {
            count += 1;
        }
        current += chrono::Duration::days(1);
    }

    Some(count * sign)
}

/// Writes a date in Brazilian Portuguese words ("por extenso"), all lower
/// case, e.g. `"primeiro de janeiro de dois mil e vinte e quatro"`.
///
/// Unlike [`convert_date_to_text`], this accepts both the `dd/mm/yyyy` and
/// the ISO `yyyy-mm-dd` formats, and returns an empty string (never `None`)
/// for invalid input, matching the contract's reference implementation.
///
/// # Arguments
///
/// * `value` - The date to convert, as `dd/mm/yyyy` or `yyyy-mm-dd`.
///
/// # Returns
///
/// The date written out in lower-case Brazilian Portuguese, or `""` if
/// `value` is not a valid date in one of the two accepted formats.
///
/// # Examples
///
/// ```
/// use brazilian_utils::date_utils::convert_to_words;
///
/// assert_eq!(convert_to_words("01/01/2024"), "primeiro de janeiro de dois mil e vinte e quatro");
/// assert_eq!(convert_to_words("2024-12-25"), "vinte e cinco de dezembro de dois mil e vinte e quatro");
/// assert_eq!(convert_to_words("31/04/2024"), "");
/// assert_eq!(convert_to_words("not a date"), "");
/// ```
pub fn convert_to_words(value: &str) -> String {
    let parsed = parse_br_date(value).or_else(|| parse_iso_date(value));

    let (day, month, year) = match parsed {
        Some(parts) => parts,
        None => return String::new(),
    };

    if year < 1 {
        return String::new();
    }

    if NaiveDate::from_ymd_opt(year, month, day).is_none() {
        return String::new();
    }

    let day_str = if day == 1 {
        "primeiro".to_string()
    } else {
        number_to_words(day as i64)
    };

    let month_name = get_month_name(month);
    let year_str = number_to_words(year as i64);

    format!("{} de {} de {}", day_str, month_name, year_str)
}

/// Parses a `dd/mm/yyyy` string into its (day, month, year) parts, without
/// validating that the date actually exists.
fn parse_br_date(value: &str) -> Option<(u32, u32, i32)> {
    let parts: Vec<&str> = value.split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let year: i32 = parts[2].parse().ok()?;
    Some((day, month, year))
}

/// Parses a `yyyy-mm-dd` (ISO) string into its (day, month, year) parts,
/// without validating that the date actually exists.
fn parse_iso_date(value: &str) -> Option<(u32, u32, i32)> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;
    Some((day, month, year))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(date: NaiveDate, uf: Option<&str>) -> Option<bool> {
        is_holiday(Some(IsHolidayParams {
            date: Some(date),
            uf: uf.map(|s| s.to_string()),
        }))
    }

    #[test]
    fn test_convert_date_to_text_basic() {
        assert_eq!(
            convert_date_to_text("01/01/2024"),
            Some("Primeiro de janeiro de dois mil e vinte e quatro".to_string())
        );
        assert_eq!(
            convert_date_to_text("15/08/1990"),
            Some("Quinze de agosto de mil novecentos e noventa".to_string())
        );
        assert_eq!(
            convert_date_to_text("25/12/2000"),
            Some("Vinte e cinco de dezembro de dois mil".to_string())
        );
    }

    #[test]
    fn test_convert_date_to_text_invalid() {
        assert_eq!(convert_date_to_text("invalid"), None);
        assert_eq!(convert_date_to_text("32/01/2024"), None);
        assert_eq!(convert_date_to_text("01/13/2024"), None);
        assert_eq!(convert_date_to_text("29/02/2023"), None); // Not a leap year
    }

    #[test]
    fn test_is_holiday_national() {
        // New Year's Day
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), None),
            Some(true)
        );

        // Regular day
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(), None),
            Some(false)
        );

        // Independence Day
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap(), None),
            Some(true)
        );

        // Christmas
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(), None),
            Some(true)
        );
    }

    #[test]
    fn test_is_holiday_black_awareness_day_national() {
        // Lei 14.759/2023: Black Awareness Day (Nov 20) became a national
        // holiday starting in 2024.
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 11, 20).unwrap(), None),
            Some(true)
        );
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2025, 11, 20).unwrap(), None),
            Some(true)
        );

        // Before the law, it was not a national holiday (only in specific states).
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2023, 11, 20).unwrap(), None),
            Some(false)
        );
    }

    #[test]
    fn test_is_holiday_state() {
        // Bahia Independence Day
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 7, 2).unwrap(), Some("BA")),
            Some(true)
        );

        // Not a holiday in other states
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 7, 2).unwrap(), Some("SP")),
            Some(false)
        );

        // São Paulo state holiday
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 7, 9).unwrap(), Some("SP")),
            Some(true)
        );
    }

    #[test]
    fn test_is_holiday_invalid_uf() {
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), Some("XX")),
            None
        );
    }

    #[test]
    fn test_is_holiday_good_friday() {
        // Good Friday 2024 (March 29)
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2024, 3, 29).unwrap(), None),
            Some(true)
        );

        // Good Friday 2023 (April 7)
        assert_eq!(
            h(NaiveDate::from_ymd_opt(2023, 4, 7).unwrap(), None),
            Some(true)
        );
    }

    #[test]
    fn test_calculate_easter() {
        // Easter 2024 - March 31
        assert_eq!(
            calculate_easter(2024),
            NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
        );

        // Easter 2023 - April 9
        assert_eq!(
            calculate_easter(2023),
            NaiveDate::from_ymd_opt(2023, 4, 9).unwrap()
        );

        // Easter 2025 - April 20
        assert_eq!(
            calculate_easter(2025),
            NaiveDate::from_ymd_opt(2025, 4, 20).unwrap()
        );
    }

    #[test]
    fn test_get_month_name() {
        assert_eq!(get_month_name(1), "janeiro");
        assert_eq!(get_month_name(2), "fevereiro");
        assert_eq!(get_month_name(3), "março");
        assert_eq!(get_month_name(12), "dezembro");
    }

    #[test]
    fn test_convert_to_words() {
        assert_eq!(
            convert_to_words("01/01/2024"),
            "primeiro de janeiro de dois mil e vinte e quatro"
        );
        assert_eq!(
            convert_to_words("02/01/2024"),
            "dois de janeiro de dois mil e vinte e quatro"
        );
        assert_eq!(
            convert_to_words("25/12/2024"),
            "vinte e cinco de dezembro de dois mil e vinte e quatro"
        );
        assert_eq!(
            convert_to_words("2024-12-25"),
            "vinte e cinco de dezembro de dois mil e vinte e quatro"
        );
        assert_eq!(
            convert_to_words("29/02/2000"),
            "vinte e nove de fevereiro de dois mil"
        );
        assert_eq!(convert_to_words("29/02/2023"), "");
        assert_eq!(convert_to_words("31/04/2024"), "");
        assert_eq!(convert_to_words("not a date"), "");
        assert_eq!(convert_to_words(""), "");
    }

    #[test]
    fn test_get_holidays() {
        let holidays_2024 = get_holidays(2024);
        assert!(holidays_2024
            .iter()
            .any(|h| h.date == NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
                && h.holiday_type == "national"));
        assert!(holidays_2024
            .iter()
            .any(|h| h.date == NaiveDate::from_ymd_opt(2024, 3, 29).unwrap()
                && h.holiday_type == "religious"));
        assert!(holidays_2024
            .iter()
            .any(|h| h.holiday_type == "optional"));

        // Sorted by date
        let mut sorted = holidays_2024.clone();
        sorted.sort_by_key(|h| h.date);
        assert_eq!(holidays_2024, sorted);

        assert_eq!(get_holidays(1800), Vec::new());
        assert_eq!(get_holidays(2100), Vec::new());
    }

    #[test]
    fn test_is_business_day() {
        assert!(is_business_day(
            NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            None
        ));
        assert!(!is_business_day(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            None
        )); // holiday
        assert!(!is_business_day(
            NaiveDate::from_ymd_opt(2024, 1, 6).unwrap(),
            None
        )); // Saturday
        assert!(!is_business_day(
            NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            None
        )); // Sunday
    }

    #[test]
    fn test_add_business_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday, holiday
        assert_eq!(
            add_business_days(start, 1, None),
            NaiveDate::from_ymd_opt(2024, 1, 2)
        );
        assert_eq!(add_business_days(start, 0, None), Some(start));

        let friday = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
        assert_eq!(
            add_business_days(friday, 1, None),
            NaiveDate::from_ymd_opt(2024, 1, 8)
        );
    }

    #[test]
    fn test_sub_business_days() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        assert_eq!(
            sub_business_days(start, 1, None),
            NaiveDate::from_ymd_opt(2023, 12, 29)
        );
    }

    #[test]
    fn test_difference_in_business_days() {
        let a = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let b = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        // Business days in [Jan 1, Jan 8): Jan 2,3,4,5 (Jan 1 holiday, Jan 6-7 weekend)
        assert_eq!(difference_in_business_days(b, a, None), Some(4));
        assert_eq!(difference_in_business_days(a, b, None), Some(-4));
        assert_eq!(difference_in_business_days(a, a, None), Some(0));
    }
}
