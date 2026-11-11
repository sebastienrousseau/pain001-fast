// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! BIC validation: ISO 9362 structure and a country-code list, on a
//! stack buffer with no heap allocation on the valid path.

use crate::{compact, verdict};

/// Country codes accepted in a BIC, sorted for binary search. This is
/// the list pain001 core uses; it is a subset of ISO 3166-1, so a BIC
/// from an unlisted country is refused.
pub(crate) const BIC_COUNTRIES: [[u8; 2]; 94] = [
    *b"AD", *b"AE", *b"AL", *b"AR", *b"AT", *b"AU", *b"AZ", *b"BA", *b"BE", *b"BG", *b"BH", *b"BR",
    *b"BY", *b"CA", *b"CH", *b"CL", *b"CN", *b"CO", *b"CR", *b"CY", *b"CZ", *b"DE", *b"DK", *b"DO",
    *b"EE", *b"EG", *b"ES", *b"FI", *b"FO", *b"FR", *b"GB", *b"GE", *b"GI", *b"GL", *b"GR", *b"GT",
    *b"HK", *b"HR", *b"HU", *b"ID", *b"IE", *b"IL", *b"IN", *b"IQ", *b"IS", *b"IT", *b"JO", *b"JP",
    *b"KR", *b"KW", *b"KZ", *b"LB", *b"LC", *b"LI", *b"LT", *b"LU", *b"LV", *b"MC", *b"MD", *b"ME",
    *b"MK", *b"MR", *b"MT", *b"MU", *b"MX", *b"MY", *b"NL", *b"NO", *b"NZ", *b"PH", *b"PK", *b"PL",
    *b"PS", *b"PT", *b"QA", *b"RO", *b"RS", *b"RU", *b"SA", *b"SE", *b"SG", *b"SI", *b"SK", *b"SM",
    *b"TH", *b"TN", *b"TR", *b"TW", *b"UA", *b"US", *b"VA", *b"VG", *b"XK", *b"ZA",
];

/// Validate a BIC, returning `(true, "")` or `(false, reason)`.
///
/// Spaces and hyphens are ignored and letters are uppercased. Checks
/// run in order and stop at the first that fails: not empty, 8 or 11
/// characters (counted in bytes), the four parts' character classes
/// (every fault of this step is reported together), then the country
/// code against the list pain001 core uses, a subset of ISO 3166-1, so
/// a BIC from an unlisted country is refused.
#[inline]
pub fn validate_bic_detailed_fast(bic: &str) -> (bool, String) {
    verdict(check(bic))
}

fn check(bic: &str) -> Result<(), String> {
    if bic.is_empty() {
        return Err("BIC cannot be empty".to_string());
    }
    let (buffer, len) = compact::<11>(bic)
        .map_err(|count| format!("BIC must be 8 or 11 characters (got {count}): '{bic}'"))?;
    let clean = &buffer[..len];
    if len != 8 && len != 11 {
        return Err(format!(
            "BIC must be 8 or 11 characters (got {len}: '{}')",
            String::from_utf8_lossy(clean)
        ));
    }
    check_parts(clean)?;
    let country = &clean[4..6];
    if BIC_COUNTRIES
        .binary_search_by(|code| code.as_slice().cmp(country))
        .is_err()
    {
        return Err(format!(
            "BIC country code '{}{}' is not a valid ISO 3166-1 alpha-2 code",
            char::from(country[0]),
            char::from(country[1])
        ));
    }
    Ok(())
}

/// The four parts of a BIC: name, byte range, and whether digits are
/// allowed. An 8-character BIC has an empty branch, which passes.
const PARTS: [(&str, usize, usize, bool); 4] = [
    ("bank code (first 4 chars)", 0, 4, false),
    ("country code (chars 5-6)", 4, 6, false),
    ("location code (chars 7-8)", 6, 8, true),
    ("branch code (chars 9-11)", 8, 11, true),
];

fn check_parts(clean: &[u8]) -> Result<(), String> {
    let faults: Vec<String> = PARTS
        .iter()
        .filter_map(|&(name, start, end, digits)| part_fault(clean, name, start, end, digits))
        .collect();
    if faults.is_empty() {
        Ok(())
    } else {
        Err(format!("BIC format invalid: {}", faults.join("; ")))
    }
}

fn part_fault(clean: &[u8], name: &str, start: usize, end: usize, digits: bool) -> Option<String> {
    let part = &clean[start.min(clean.len())..end.min(clean.len())];
    let (ok, kind) = if digits {
        (part.iter().all(u8::is_ascii_alphanumeric), "alphanumeric")
    } else {
        (part.iter().all(u8::is_ascii_alphabetic), "letters")
    };
    // A part that cuts a multi-byte character in half is not UTF-8 on
    // its own and is quoted empty.
    let quoted = std::str::from_utf8(part).unwrap_or("");
    (!ok).then(|| format!("{name} must be {kind} (got '{quoted}')"))
}
