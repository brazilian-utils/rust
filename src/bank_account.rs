/// Brazilian bank account (agência/conta) structural utilities.
///
/// This module implements the *generic* fallback check described by the
/// contract (a modulus 10 or modulus 11 check digit over the account
/// number) plus the structural checks (bank code must be a real STR
/// participant, agency/account/digit shapes). It does **not** implement the
/// bank-specific check-digit algorithms of Banco do Brasil, Santander,
/// Banrisul, Caixa, Bradesco, Nubank (Verhoeff), Itaú, HSBC/Kirton or
/// Citibank, nor the "structure only" digit-length rules of Inter, C6,
/// PicPay, Sicoob, Sicredi and similar banks, since those require published
/// per-bank formulas this pass could not verify against a trustworthy
/// source. Accepting or rejecting those banks' accounts here is
/// best-effort only.
use crate::bank;

/// Parameters for [`is_valid`].
#[derive(Debug, Clone, Default)]
pub struct IsValidBankAccountParams {
    /// The 3-digit COMPE bank code.
    pub bank_code: String,
    /// The agency number (1 to 5 digits).
    pub agency: String,
    /// The account number (1 to 13 digits).
    pub account: String,
    /// The check digit (1 or 2 characters, or `X`/`P` for some banks).
    pub digit: String,
}

/// Computes a Luhn-style modulus 10 check digit over `digits`.
fn mod10_digit(digits: &str) -> u32 {
    let sum: u32 = digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap_or(0);
            if i % 2 == 0 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();
    (10 - (sum % 10)) % 10
}

/// Computes a modulus 11 check digit over `digits`, weights cycling 2 to 9
/// from the rightmost digit (the same style used elsewhere in Brazilian
/// document check digits).
fn mod11_digit(digits: &str) -> u32 {
    let mut weight = 2u32;
    let mut sum = 0u32;
    for c in digits.chars().rev() {
        sum += c.to_digit(10).unwrap_or(0) * weight;
        weight = if weight == 9 { 2 } else { weight + 1 };
    }
    let rem = sum % 11;
    if rem < 2 {
        0
    } else {
        11 - rem
    }
}

/// Validates a Brazilian bank account (bank code, agency, account and check
/// digit) using the generic fallback rule: `digit` must match modulus 10 or
/// modulus 11 over `account`; a 2-character `digit` chains modulus 10 then
/// modulus 11.
///
/// The `bank_code` must be a Banco Central STR participant (see
/// [`crate::bank::get_by_code`]).
///
/// # Examples
///
/// ```
/// use brazilian_utils::bank_account::{is_valid, IsValidBankAccountParams};
///
/// let params = IsValidBankAccountParams {
///     bank_code: "999".to_string(),
///     agency: "1234".to_string(),
///     account: "123456".to_string(),
///     digit: "9".to_string(),
/// };
/// assert!(!is_valid(&params)); // 999 is not a real COMPE code
/// ```
pub fn is_valid(params: &IsValidBankAccountParams) -> bool {
    if bank::get_by_code(&params.bank_code).is_none() {
        return false;
    }

    if params.agency.is_empty()
        || params.agency.len() > 5
        || !params.agency.chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }

    if params.account.is_empty()
        || params.account.len() > 13
        || !params.account.chars().all(|c| c.is_ascii_digit())
    {
        return false;
    }

    match params.digit.len() {
        1 => {
            let d = match params.digit.chars().next().and_then(|c| c.to_digit(10)) {
                Some(d) => d,
                None => return false,
            };
            mod10_digit(&params.account) == d || mod11_digit(&params.account) == d
        }
        2 => {
            let chars: Vec<char> = params.digit.chars().collect();
            let (Some(d1), Some(d2)) = (chars[0].to_digit(10), chars[1].to_digit(10)) else {
                return false;
            };
            let m10 = mod10_digit(&params.account);
            if m10 != d1 {
                return false;
            }
            let with_m10 = format!("{}{}", params.account, m10);
            mod11_digit(&with_m10) == d2
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejects_unknown_bank() {
        let params = IsValidBankAccountParams {
            bank_code: "999".to_string(),
            agency: "1234".to_string(),
            account: "123456".to_string(),
            digit: "9".to_string(),
        };
        assert!(!is_valid(&params));
    }

    #[test]
    fn test_generic_fallback_mod10() {
        let account = "123456";
        let d = mod10_digit(account);
        let params = IsValidBankAccountParams {
            bank_code: "001".to_string(),
            agency: "1234".to_string(),
            account: account.to_string(),
            digit: d.to_string(),
        };
        assert!(is_valid(&params));
    }

    #[test]
    fn test_rejects_malformed_fields() {
        let base = IsValidBankAccountParams {
            bank_code: "001".to_string(),
            agency: "1234".to_string(),
            account: "123456".to_string(),
            digit: "0".to_string(),
        };

        let mut bad_agency = base.clone();
        bad_agency.agency = "ABCDE".to_string();
        assert!(!is_valid(&bad_agency));

        let mut bad_account = base.clone();
        bad_account.account = "".to_string();
        assert!(!is_valid(&bad_account));

        let mut bad_digit = base.clone();
        bad_digit.digit = "".to_string();
        assert!(!is_valid(&bad_digit));
    }
}
