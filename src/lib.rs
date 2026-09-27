// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! High-throughput native acceleration core for ISO 20022 financial validations.
//!
//! Provides zero-allocation IBAN checksum mod-97, structural BIC validation,
//! restricted character set transliteration, and parallel multi-core batch analysis.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use unicode_normalization::UnicodeNormalization;

/// Permitted ASCII character set table for ISO 20022 restricted Latin text.
const ALLOWED_ASCII: [bool; 128] = {
    let mut table = [false; 128];
    let allowed = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+ ";
    let mut i = 0;
    while i < allowed.len() {
        table[allowed[i] as usize] = true;
        i += 1;
    }
    table
};

#[inline(always)]
fn is_allowed_char(ch: char) -> bool {
    let val = ch as u32;
    if val < 128 {
        ALLOWED_ASCII[val as usize]
    } else {
        false
    }
}

#[inline(always)]
fn is_combining_mark(c: char) -> bool {
    matches!(
        c as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F
    )
}

/// Country-specific IBAN length lookup (ISO 13616).
#[inline]
fn get_expected_iban_length(country: &[u8; 2]) -> Option<usize> {
    match country {
        b"AD" => Some(24),
        b"AE" => Some(23),
        b"AL" => Some(28),
        b"AT" => Some(20),
        b"AZ" => Some(28),
        b"BA" => Some(20),
        b"BE" => Some(16),
        b"BG" => Some(22),
        b"BH" => Some(22),
        b"BR" => Some(29),
        b"BY" => Some(28),
        b"CH" => Some(21),
        b"CR" => Some(22),
        b"CY" => Some(28),
        b"CZ" => Some(24),
        b"DE" => Some(22),
        b"DK" => Some(18),
        b"DO" => Some(28),
        b"EE" => Some(20),
        b"EG" => Some(29),
        b"ES" => Some(24),
        b"FI" => Some(18),
        b"FO" => Some(18),
        b"FR" => Some(27),
        b"GB" => Some(22),
        b"GE" => Some(22),
        b"GI" => Some(23),
        b"GL" => Some(18),
        b"GR" => Some(27),
        b"GT" => Some(28),
        b"HR" => Some(21),
        b"HU" => Some(28),
        b"IE" => Some(22),
        b"IL" => Some(23),
        b"IQ" => Some(23),
        b"IS" => Some(26),
        b"IT" => Some(27),
        b"JO" => Some(30),
        b"KW" => Some(30),
        b"KZ" => Some(20),
        b"LB" => Some(28),
        b"LC" => Some(32),
        b"LI" => Some(21),
        b"LT" => Some(20),
        b"LU" => Some(20),
        b"LV" => Some(21),
        b"MC" => Some(27),
        b"MD" => Some(24),
        b"ME" => Some(22),
        b"MK" => Some(19),
        b"MR" => Some(27),
        b"MT" => Some(31),
        b"MU" => Some(30),
        b"NL" => Some(18),
        b"NO" => Some(15),
        b"PK" => Some(24),
        b"PL" => Some(28),
        b"PS" => Some(29),
        b"PT" => Some(25),
        b"QA" => Some(29),
        b"RO" => Some(24),
        b"RS" => Some(22),
        b"SA" => Some(24),
        b"SE" => Some(24),
        b"SI" => Some(19),
        b"SK" => Some(24),
        b"SM" => Some(27),
        b"TN" => Some(24),
        b"TR" => Some(26),
        b"UA" => Some(29),
        b"VA" => Some(22),
        b"VG" => Some(24),
        b"XK" => Some(20),
        _ => None,
    }
}

/// ISO 3166-1 alpha-2 country code validity for BIC.
#[inline]
fn is_valid_bic_country(country: &[u8; 2]) -> bool {
    matches!(
        country,
        b"AD"
            | b"AE"
            | b"AL"
            | b"AR"
            | b"AT"
            | b"AU"
            | b"AZ"
            | b"BA"
            | b"BE"
            | b"BG"
            | b"BH"
            | b"BR"
            | b"BY"
            | b"CA"
            | b"CH"
            | b"CL"
            | b"CN"
            | b"CO"
            | b"CR"
            | b"CY"
            | b"CZ"
            | b"DE"
            | b"DK"
            | b"DO"
            | b"EE"
            | b"EG"
            | b"ES"
            | b"FI"
            | b"FO"
            | b"FR"
            | b"GB"
            | b"GE"
            | b"GI"
            | b"GL"
            | b"GR"
            | b"GT"
            | b"HK"
            | b"HR"
            | b"HU"
            | b"ID"
            | b"IE"
            | b"IL"
            | b"IN"
            | b"IQ"
            | b"IS"
            | b"IT"
            | b"JO"
            | b"JP"
            | b"KR"
            | b"KW"
            | b"KZ"
            | b"LB"
            | b"LC"
            | b"LI"
            | b"LT"
            | b"LU"
            | b"LV"
            | b"MC"
            | b"MD"
            | b"ME"
            | b"MK"
            | b"MR"
            | b"MT"
            | b"MU"
            | b"MX"
            | b"MY"
            | b"NL"
            | b"NO"
            | b"NZ"
            | b"PH"
            | b"PK"
            | b"PL"
            | b"PS"
            | b"PT"
            | b"QA"
            | b"RO"
            | b"RS"
            | b"RU"
            | b"SA"
            | b"SE"
            | b"SG"
            | b"SI"
            | b"SK"
            | b"SM"
            | b"TH"
            | b"TN"
            | b"TR"
            | b"TW"
            | b"UA"
            | b"US"
            | b"VA"
            | b"VG"
            | b"XK"
            | b"ZA"
    )
}

/// Zero-allocation, high-throughput IBAN validation returning (is_valid, error_message).
#[inline]
pub fn validate_iban_detailed_fast(iban: &str) -> (bool, String) {
    if iban.is_empty() {
        return (false, "IBAN cannot be empty".to_string());
    }

    // Clean bytes buffer on stack (max IBAN length is 34, filter spaces/hyphens)
    let mut clean = [0u8; 34];
    let mut clean_len = 0;

    for b in iban.bytes() {
        if b == b' ' || b == b'-' {
            continue;
        }
        if clean_len >= 34 {
            return (
                false,
                format!(
                    "IBAN length must be 15-34 characters (got {})",
                    clean_len + 1
                ),
            );
        }
        clean[clean_len] = b.to_ascii_uppercase();
        clean_len += 1;
    }

    if clean_len < 15 {
        return (
            false,
            format!("IBAN length must be 15-34 characters (got {clean_len})"),
        );
    }

    let country = [clean[0], clean[1]];
    let country_ok = clean[0].is_ascii_alphabetic() && clean[1].is_ascii_alphabetic();
    let checkdigit_ok = clean[2].is_ascii_digit() && clean[3].is_ascii_digit();

    let mut bban_ok = true;
    for &b in &clean[4..clean_len] {
        if !b.is_ascii_alphanumeric() {
            bban_ok = false;
            break;
        }
    }

    if !country_ok || !checkdigit_ok || !bban_ok {
        let mut errors = Vec::new();
        if !country_ok {
            errors.push("must start with 2-letter country code");
        }
        if !checkdigit_ok {
            errors.push("characters 3-4 must be check digits (00-99)");
        }
        if !bban_ok {
            errors.push("must contain only alphanumeric characters");
        }
        return (false, format!("IBAN format invalid: {}", errors.join("; ")));
    }

    if let Some(expected_len) = get_expected_iban_length(&country) {
        if clean_len != expected_len {
            let c_str = std::str::from_utf8(&country).unwrap_or("??");
            return (
                false,
                format!("Invalid IBAN length for {c_str}: {clean_len} characters (expected {expected_len})"),
            );
        }
    }

    // Modulo 97 calculation (zero allocation, streaming accumulator)
    // Rearrange: clean[4..clean_len] then clean[0..4]
    let mut acc: u64 = 0;

    // Process clean[4..clean_len]
    for &b in &clean[4..clean_len] {
        if b.is_ascii_digit() {
            acc = (acc * 10 + (b - b'0') as u64) % 97;
        } else if b.is_ascii_alphabetic() {
            let val = (b - b'A' + 10) as u64;
            acc = (acc * 100 + val) % 97;
        } else {
            return (false, format!("Invalid character in IBAN: {}", b as char));
        }
    }

    // Process clean[0..4]
    for &b in &clean[0..4] {
        if b.is_ascii_digit() {
            acc = (acc * 10 + (b - b'0') as u64) % 97;
        } else if b.is_ascii_alphabetic() {
            let val = (b - b'A' + 10) as u64;
            acc = (acc * 100 + val) % 97;
        } else {
            return (false, format!("Invalid character in IBAN: {}", b as char));
        }
    }

    if acc != 1 {
        return (
            false,
            format!("IBAN checksum validation failed (mod 97 = {acc}, expected 1)"),
        );
    }

    (true, String::new())
}

/// Zero-allocation, high-throughput BIC validation returning (is_valid, error_message).
#[inline]
pub fn validate_bic_detailed_fast(bic: &str) -> (bool, String) {
    if bic.is_empty() {
        return (false, "BIC cannot be empty".to_string());
    }

    let mut clean = [0u8; 11];
    let mut clean_len = 0;

    for b in bic.bytes() {
        if b == b' ' || b == b'-' {
            continue;
        }
        if clean_len >= 11 {
            return (
                false,
                format!(
                    "BIC must be 8 or 11 characters (got {}): '{bic}'",
                    clean_len + 1
                ),
            );
        }
        clean[clean_len] = b.to_ascii_uppercase();
        clean_len += 1;
    }

    if clean_len != 8 && clean_len != 11 {
        let clean_str = std::str::from_utf8(&clean[..clean_len]).unwrap_or("");
        return (
            false,
            format!("BIC must be 8 or 11 characters (got {clean_len}: '{clean_str}')"),
        );
    }

    let bank_ok = clean[0].is_ascii_alphabetic()
        && clean[1].is_ascii_alphabetic()
        && clean[2].is_ascii_alphabetic()
        && clean[3].is_ascii_alphabetic();

    let country = [clean[4], clean[5]];
    let country_ok = clean[4].is_ascii_alphabetic() && clean[5].is_ascii_alphabetic();
    let location_ok = clean[6].is_ascii_alphanumeric() && clean[7].is_ascii_alphanumeric();

    let branch_ok = if clean_len == 11 {
        clean[8].is_ascii_alphanumeric()
            && clean[9].is_ascii_alphanumeric()
            && clean[10].is_ascii_alphanumeric()
    } else {
        true
    };

    if !bank_ok || !country_ok || !location_ok || !branch_ok {
        let mut errors = Vec::new();
        if !bank_ok {
            let bank_str = std::str::from_utf8(&clean[0..4]).unwrap_or("");
            errors.push(format!(
                "bank code (first 4 chars) must be letters (got '{bank_str}')"
            ));
        }
        if !country_ok {
            let country_str = std::str::from_utf8(&clean[4..6]).unwrap_or("");
            errors.push(format!(
                "country code (chars 5-6) must be letters (got '{country_str}')"
            ));
        }
        if !location_ok {
            let loc_str = std::str::from_utf8(&clean[6..8]).unwrap_or("");
            errors.push(format!(
                "location code (chars 7-8) must be alphanumeric (got '{loc_str}')"
            ));
        }
        if !branch_ok {
            let br_str = std::str::from_utf8(&clean[8..11]).unwrap_or("");
            errors.push(format!(
                "branch code (chars 9-11) must be alphanumeric (got '{br_str}')"
            ));
        }
        return (false, format!("BIC format invalid: {}", errors.join("; ")));
    }

    if !is_valid_bic_country(&country) {
        let c_str = std::str::from_utf8(&country).unwrap_or("??");
        return (
            false,
            format!("BIC country code '{c_str}' is not a valid ISO 3166-1 alpha-2 code"),
        );
    }

    (true, String::new())
}

/// Transliterate string into ISO 20022 restricted Latin character set.
#[inline]
pub fn sanitize_to_charset_fast(value: &str, replacement: Option<&str>) -> String {
    let rep = replacement.unwrap_or(" ");
    let mut out = String::with_capacity(value.len());

    for ch in value.nfkd() {
        if is_combining_mark(ch) {
            continue;
        }
        if is_allowed_char(ch) {
            out.push(ch);
        } else {
            out.push_str(rep);
        }
    }
    out
}

/// Find unique disallowed characters in value.
#[inline]
pub fn find_invalid_characters_fast(value: &str) -> Vec<String> {
    let mut invalid = HashSet::new();
    for ch in value.chars() {
        if !is_allowed_char(ch) {
            invalid.insert(ch.to_string());
        }
    }
    let mut sorted: Vec<String> = invalid.into_iter().collect();
    sorted.sort();
    sorted
}

/// PyO3 Python bindings
#[pyfunction]
#[pyo3(signature = (iban))]
pub fn validate_iban(iban: &str) -> bool {
    validate_iban_detailed_fast(iban).0
}

#[pyfunction]
#[pyo3(signature = (iban))]
pub fn validate_iban_detailed(iban: &str) -> (bool, String) {
    validate_iban_detailed_fast(iban)
}

#[pyfunction]
#[pyo3(signature = (bic))]
pub fn validate_bic(bic: &str) -> bool {
    validate_bic_detailed_fast(bic).0
}

#[pyfunction]
#[pyo3(signature = (bic))]
pub fn validate_bic_detailed(bic: &str) -> (bool, String) {
    validate_bic_detailed_fast(bic)
}

#[pyfunction]
#[pyo3(signature = (value))]
pub fn is_valid_charset(value: &str) -> bool {
    for ch in value.chars() {
        if !is_allowed_char(ch) {
            return false;
        }
    }
    true
}

#[pyfunction]
#[pyo3(signature = (value, replacement = " "))]
pub fn sanitize_to_charset(value: &str, replacement: &str) -> String {
    sanitize_to_charset_fast(value, Some(replacement))
}

#[pyfunction]
#[pyo3(signature = (value))]
pub fn find_invalid_characters(value: &str) -> Vec<String> {
    find_invalid_characters_fast(value)
}

struct RecordData {
    row: usize,
    creditor_iban: String,
    creditor_bic: String,
    amount_str: String,
    currency: String,
    execution_date: String,
}

struct ValidatedRecord {
    row: usize,
    error: Option<(String, String)>,
    parsed_amount: Option<f64>,
    currency: String,
    fingerprint: String,
}

#[pyfunction]
#[pyo3(signature = (records, message_type = "pain.001.001.03"))]
pub fn validate_payment_batch<'py>(
    py: Python<'py>,
    records: &Bound<'py, PyList>,
    message_type: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let start_time = Instant::now();
    let num_records = records.len();

    let mut parsed_inputs = Vec::with_capacity(num_records);

    for (idx, item) in records.iter().enumerate() {
        if let Ok(dict) = item.cast::<PyDict>() {
            let iban = dict
                .get_item("creditor_iban")?
                .or(dict.get_item("cdtr_iban")?)
                .map(|v| v.to_string())
                .unwrap_or_default();

            let bic = dict
                .get_item("creditor_bic")?
                .or(dict.get_item("cdtr_bic")?)
                .map(|v| v.to_string())
                .unwrap_or_default();

            let amount_str = dict
                .get_item("amount")?
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0.0".to_string());

            let currency = dict
                .get_item("currency")?
                .map(|v| v.to_string().to_uppercase())
                .unwrap_or_else(|| "EUR".to_string());

            let execution_date = dict
                .get_item("execution_date")?
                .map(|v| v.to_string())
                .unwrap_or_default();

            parsed_inputs.push(RecordData {
                row: idx,
                creditor_iban: iban,
                creditor_bic: bic,
                amount_str,
                currency,
                execution_date,
            });
        }
    }

    // Parallel validation over all CPU cores using Rayon
    let validation_results: Vec<ValidatedRecord> = parsed_inputs
        .par_iter()
        .map(|rec| {
            let mut err: Option<(String, String)> = None;

            if !rec.creditor_iban.is_empty() {
                let (valid, err_msg) = validate_iban_detailed_fast(&rec.creditor_iban);
                if !valid {
                    err = Some(("creditor_iban".to_string(), err_msg));
                }
            }

            if err.is_none() && !rec.creditor_bic.is_empty() {
                let (valid, err_msg) = validate_bic_detailed_fast(&rec.creditor_bic);
                if !valid {
                    err = Some(("creditor_bic".to_string(), err_msg));
                }
            }

            let parsed_amt = rec.amount_str.trim().parse::<f64>().ok();
            if err.is_none() && parsed_amt.is_none() {
                err = Some(("amount".to_string(), "Invalid amount format".to_string()));
            }

            let fingerprint = format!(
                "{}:{}:{}:{}:{}",
                rec.creditor_iban,
                rec.creditor_bic,
                rec.amount_str,
                rec.currency,
                rec.execution_date
            );

            ValidatedRecord {
                row: rec.row,
                error: err,
                parsed_amount: parsed_amt,
                currency: rec.currency.clone(),
                fingerprint,
            }
        })
        .collect();

    // Aggregate control sums, collect errors and detect duplicates
    let mut control_sums: HashMap<String, f64> = HashMap::new();
    let errors_list = PyList::empty(py);
    let duplicates_list = PyList::empty(py);
    let mut seen_fingerprints = HashSet::new();

    for item in validation_results {
        if let Some((field, err_msg)) = item.error {
            let err_dict = PyDict::new(py);
            err_dict.set_item("row", item.row)?;
            err_dict.set_item("field", field)?;
            err_dict.set_item("error", err_msg)?;
            errors_list.append(err_dict)?;
        }

        if let Some(amt) = item.parsed_amount {
            let entry = control_sums.entry(item.currency).or_insert(0.0);
            *entry += amt;
        }

        if !item.fingerprint.is_empty() {
            if seen_fingerprints.contains(&item.fingerprint) {
                let dup_dict = PyDict::new(py);
                dup_dict.set_item("row", item.row)?;
                dup_dict.set_item("fingerprint", &item.fingerprint)?;
                duplicates_list.append(dup_dict)?;
            } else {
                seen_fingerprints.insert(item.fingerprint);
            }
        }
    }

    let elapsed = start_time.elapsed().as_micros();
    let is_valid = errors_list.is_empty();

    let result = PyDict::new(py);
    result.set_item("valid", is_valid)?;
    result.set_item("message_type", message_type)?;
    result.set_item("transaction_count", num_records)?;
    result.set_item("control_sums", control_sums)?;
    result.set_item("errors", errors_list)?;
    result.set_item("duplicates", duplicates_list)?;
    result.set_item("duration_micros", elapsed)?;

    Ok(result)
}

/// PyO3 native extension module definition.
#[pymodule]
fn _pain001_fast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_iban, m)?)?;
    m.add_function(wrap_pyfunction!(validate_iban_detailed, m)?)?;
    m.add_function(wrap_pyfunction!(validate_bic, m)?)?;
    m.add_function(wrap_pyfunction!(validate_bic_detailed, m)?)?;
    m.add_function(wrap_pyfunction!(is_valid_charset, m)?)?;
    m.add_function(wrap_pyfunction!(sanitize_to_charset, m)?)?;
    m.add_function(wrap_pyfunction!(find_invalid_characters, m)?)?;
    m.add_function(wrap_pyfunction!(validate_payment_batch, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_iban_valid() {
        assert!(validate_iban_detailed_fast("DE89370400440532013000").0);
        assert!(validate_iban_detailed_fast("GB82WEST12345698765432").0);
        assert!(validate_iban_detailed_fast("FR1420041010050500013M02606").0);
    }

    #[test]
    fn test_validate_iban_invalid() {
        // Empty
        assert!(!validate_iban_detailed_fast("").0);
        // Too short
        assert!(!validate_iban_detailed_fast("DE89").0);
        // Invalid check digits
        assert!(!validate_iban_detailed_fast("DE00370400440532013000").0);
        // Bad country length
        assert!(!validate_iban_detailed_fast("DE8937040044053201300099").0);
        // Too long
        assert!(!validate_iban_detailed_fast("DE89370400440532013000000000000000000").0);
        // Non-alphanumeric
        assert!(!validate_iban_detailed_fast("DE8937040044053201300!").0);
    }

    #[test]
    fn test_validate_bic_valid() {
        assert!(validate_bic_detailed_fast("DEUTDEFF").0);
        assert!(validate_bic_detailed_fast("DEUTDEFF500").0);
        assert!(validate_bic_detailed_fast("deutdeff").0);
    }

    #[test]
    fn test_validate_bic_invalid() {
        assert!(!validate_bic_detailed_fast("").0);
        assert!(!validate_bic_detailed_fast("DEUTDE").0);
        assert!(!validate_bic_detailed_fast("1234DEFF").0);
        assert!(!validate_bic_detailed_fast("DEUT99FF").0);
        assert!(!validate_bic_detailed_fast("DEUTDE!!").0);
        assert!(!validate_bic_detailed_fast("DEUTDEFF50!").0);
        assert!(!validate_bic_detailed_fast("DEUTZZFF").0); // ZZ is not a valid country
    }

    #[test]
    fn test_charset() {
        assert!(is_allowed_char('A'));
        assert!(is_allowed_char('0'));
        assert!(!is_allowed_char('&'));

        let sanitized = sanitize_to_charset_fast("Café Münchën & Co", Some(" "));
        assert_eq!(sanitized, "Cafe Munchen   Co");

        let invalid = find_invalid_characters_fast("A & B @ C");
        assert_eq!(invalid, vec!["&", "@"]);

        assert!(is_valid_charset("Invoice 12345"));
        assert!(!is_valid_charset("Café"));
    }
}
