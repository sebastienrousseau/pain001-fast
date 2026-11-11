// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! High-throughput native acceleration core for ISO 20022 financial validations.
//!
//! Provides IBAN checksum mod-97 and structural BIC validation on stack
//! buffers, restricted character set transliteration, and parallel
//! multi-core batch analysis. The `*_fast` functions are the Rust API;
//! the others are the Python bindings exported by the `_pain001_fast`
//! module, and each is a thin call into the Rust API.

#![warn(missing_docs)]

use pyo3::prelude::*;

mod batch;
mod bic;
mod charset;
mod iban;
#[cfg(any(test, feature = "fuzzing"))]
mod invariants;

pub use batch::validate_payment_batch;
pub use bic::validate_bic_detailed_fast;
pub use charset::{find_invalid_characters_fast, sanitize_to_charset_fast};
pub use iban::validate_iban_detailed_fast;
#[cfg(any(test, feature = "fuzzing"))]
#[doc(hidden)]
pub use invariants::check_invariants;

/// Copy `input` into an `N`-byte buffer without spaces or hyphens,
/// uppercasing ASCII letters. Returns the buffer and the length used,
/// or `Err(N + 1)` as soon as an `N + 1`th byte would not fit.
pub(crate) fn compact<const N: usize>(input: &str) -> Result<([u8; N], usize), usize> {
    let mut buffer = [0u8; N];
    let mut len = 0;
    for byte in input.bytes().filter(|&b| b != b' ' && b != b'-') {
        if len == N {
            return Err(N + 1);
        }
        buffer[len] = byte.to_ascii_uppercase();
        len += 1;
    }
    Ok((buffer, len))
}

/// Turn a check result into the `(is_valid, message)` pair the
/// bindings return.
pub(crate) fn verdict(result: Result<(), String>) -> (bool, String) {
    match result {
        Ok(()) => (true, String::new()),
        Err(message) => (false, message),
    }
}

/// Whether `iban` is a valid IBAN; see [`validate_iban_detailed_fast`].
#[pyfunction]
#[pyo3(signature = (iban))]
pub fn validate_iban(iban: &str) -> bool {
    validate_iban_detailed_fast(iban).0
}

/// Validate an IBAN, returning `(True, "")` or `(False, reason)`; see
/// [`validate_iban_detailed_fast`].
#[pyfunction]
#[pyo3(signature = (iban))]
pub fn validate_iban_detailed(iban: &str) -> (bool, String) {
    validate_iban_detailed_fast(iban)
}

/// Whether `bic` is a valid BIC; see [`validate_bic_detailed_fast`].
#[pyfunction]
#[pyo3(signature = (bic))]
pub fn validate_bic(bic: &str) -> bool {
    validate_bic_detailed_fast(bic).0
}

/// Validate a BIC, returning `(True, "")` or `(False, reason)`; see
/// [`validate_bic_detailed_fast`].
#[pyfunction]
#[pyo3(signature = (bic))]
pub fn validate_bic_detailed(bic: &str) -> (bool, String) {
    validate_bic_detailed_fast(bic)
}

/// Whether every character of `value` is in the ISO 20022 restricted
/// Latin set: ASCII letters, digits, space and `/-?:().,'+`.
#[pyfunction]
#[pyo3(signature = (value))]
pub fn is_valid_charset(value: &str) -> bool {
    value.chars().all(charset::is_allowed_char)
}

/// Transliterate `value` into the restricted Latin set; see
/// [`sanitize_to_charset_fast`].
#[pyfunction]
#[pyo3(signature = (value, replacement = " "))]
pub fn sanitize_to_charset(value: &str, replacement: &str) -> String {
    sanitize_to_charset_fast(value, Some(replacement))
}

/// The distinct characters of `value` outside the restricted Latin
/// set, sorted; see [`find_invalid_characters_fast`].
#[pyfunction]
#[pyo3(signature = (value))]
pub fn find_invalid_characters(value: &str) -> Vec<String> {
    find_invalid_characters_fast(value)
}

/// The `pain001_fast._pain001_fast` extension module: the bindings
/// above, under the same names.
#[pymodule]
mod _pain001_fast {
    #[pymodule_export]
    use super::{
        find_invalid_characters, is_valid_charset, sanitize_to_charset, validate_bic,
        validate_bic_detailed, validate_iban, validate_iban_detailed, validate_payment_batch,
    };
}

#[cfg(test)]
mod tests;
