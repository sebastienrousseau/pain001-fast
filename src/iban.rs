// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! IBAN validation: ISO 13616 structure and the ISO 7064 mod 97-10
//! check, on a stack buffer with no heap allocation on the valid path.

use crate::{compact, verdict};

/// The shortest and longest IBAN any country issues.
const MIN_LEN: usize = 15;
const MAX_LEN: usize = 34;

/// IBAN length per country (ISO 13616), sorted by country code so
/// [`expected_length`] can binary-search it. A country missing here
/// is not refused; only its length goes unchecked.
pub(crate) const IBAN_LENGTHS: [([u8; 2], usize); 73] = [
    (*b"AD", 24),
    (*b"AE", 23),
    (*b"AL", 28),
    (*b"AT", 20),
    (*b"AZ", 28),
    (*b"BA", 20),
    (*b"BE", 16),
    (*b"BG", 22),
    (*b"BH", 22),
    (*b"BR", 29),
    (*b"BY", 28),
    (*b"CH", 21),
    (*b"CR", 22),
    (*b"CY", 28),
    (*b"CZ", 24),
    (*b"DE", 22),
    (*b"DK", 18),
    (*b"DO", 28),
    (*b"EE", 20),
    (*b"EG", 29),
    (*b"ES", 24),
    (*b"FI", 18),
    (*b"FO", 18),
    (*b"FR", 27),
    (*b"GB", 22),
    (*b"GE", 22),
    (*b"GI", 23),
    (*b"GL", 18),
    (*b"GR", 27),
    (*b"GT", 28),
    (*b"HR", 21),
    (*b"HU", 28),
    (*b"IE", 22),
    (*b"IL", 23),
    (*b"IQ", 23),
    (*b"IS", 26),
    (*b"IT", 27),
    (*b"JO", 30),
    (*b"KW", 30),
    (*b"KZ", 20),
    (*b"LB", 28),
    (*b"LC", 32),
    (*b"LI", 21),
    (*b"LT", 20),
    (*b"LU", 20),
    (*b"LV", 21),
    (*b"MC", 27),
    (*b"MD", 24),
    (*b"ME", 22),
    (*b"MK", 19),
    (*b"MR", 27),
    (*b"MT", 31),
    (*b"MU", 30),
    (*b"NL", 18),
    (*b"NO", 15),
    (*b"PK", 24),
    (*b"PL", 28),
    (*b"PS", 29),
    (*b"PT", 25),
    (*b"QA", 29),
    (*b"RO", 24),
    (*b"RS", 22),
    (*b"SA", 24),
    (*b"SE", 24),
    (*b"SI", 19),
    (*b"SK", 24),
    (*b"SM", 27),
    (*b"TN", 24),
    (*b"TR", 26),
    (*b"UA", 29),
    (*b"VA", 22),
    (*b"VG", 24),
    (*b"XK", 20),
];

/// The registered IBAN length for `country`, if it is in the table.
fn expected_length(country: &[u8]) -> Option<usize> {
    IBAN_LENGTHS
        .binary_search_by(|(code, _)| code.as_slice().cmp(country))
        .ok()
        .map(|index| IBAN_LENGTHS[index].1)
}

/// Validate an IBAN, returning `(true, "")` or `(false, reason)`.
///
/// Spaces and hyphens are ignored and letters are uppercased, so
/// `de89 3704-0044 0532 0130 00` is accepted. Checks run in order and
/// stop at the first that fails: not empty, 15 to 34 characters,
/// country letters with check digits and an alphanumeric remainder
/// (every fault of this step is reported together), the country's
/// registered length, then mod 97.
#[inline]
pub fn validate_iban_detailed_fast(iban: &str) -> (bool, String) {
    verdict(check(iban))
}

fn check(iban: &str) -> Result<(), String> {
    if iban.is_empty() {
        return Err("IBAN cannot be empty".to_string());
    }
    let (buffer, len) = compact::<MAX_LEN>(iban).map_err(length_error)?;
    if len < MIN_LEN {
        return Err(length_error(len));
    }
    let clean = &buffer[..len];
    check_format(clean)?;
    check_country_length(clean)?;
    check_mod97(clean)
}

fn length_error(len: usize) -> String {
    format!("IBAN length must be 15-34 characters (got {len})")
}

fn check_format(clean: &[u8]) -> Result<(), String> {
    let faults: Vec<&str> = [
        (
            clean[..2].iter().all(u8::is_ascii_alphabetic),
            "must start with 2-letter country code",
        ),
        (
            clean[2..4].iter().all(u8::is_ascii_digit),
            "characters 3-4 must be check digits (00-99)",
        ),
        (
            clean[4..].iter().all(u8::is_ascii_alphanumeric),
            "must contain only alphanumeric characters",
        ),
    ]
    .into_iter()
    .filter(|(ok, _)| !ok)
    .map(|(_, fault)| fault)
    .collect();
    if faults.is_empty() {
        Ok(())
    } else {
        Err(format!("IBAN format invalid: {}", faults.join("; ")))
    }
}

fn check_country_length(clean: &[u8]) -> Result<(), String> {
    match expected_length(&clean[..2]) {
        Some(expected) if expected != clean.len() => Err(format!(
            "Invalid IBAN length for {}{}: {} characters (expected {expected})",
            char::from(clean[0]),
            char::from(clean[1]),
            clean.len()
        )),
        _ => Ok(()),
    }
}

/// ISO 7064 mod 97-10 over the IBAN with its first four characters
/// moved to the end, letters counting as 10 to 35. `clean` has passed
/// [`check_format`], so every byte is an ASCII digit or capital.
fn check_mod97(clean: &[u8]) -> Result<(), String> {
    let remainder = clean[4..]
        .iter()
        .chain(&clean[..4])
        .fold(0u64, |acc, &byte| {
            if byte.is_ascii_digit() {
                (acc * 10 + u64::from(byte - b'0')) % 97
            } else {
                (acc * 100 + u64::from(byte - b'A' + 10)) % 97
            }
        });
    if remainder == 1 {
        Ok(())
    } else {
        Err(format!(
            "IBAN checksum validation failed (mod 97 = {remainder}, expected 1)"
        ))
    }
}
