// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The ISO 20022 restricted Latin character set: membership, the
//! characters outside it, and transliteration into it.

use std::collections::BTreeSet;

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

/// Whether `ch` is in the restricted Latin set.
#[inline(always)]
pub(crate) fn is_allowed_char(ch: char) -> bool {
    ch.is_ascii() && ALLOWED_ASCII[ch as usize]
}

/// Whether `c` is in one of the Unicode combining-mark blocks that
/// NFKD decomposition leaves after a base letter.
#[inline(always)]
fn is_combining_mark(c: char) -> bool {
    matches!(
        c as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE20..=0xFE2F
    )
}

/// Transliterate `value` into the restricted Latin set.
///
/// The text is NFKD-decomposed, combining marks are dropped (so `é`
/// becomes `e`), and every remaining character outside the set is
/// replaced by `replacement`, a single space when it is `None`.
#[inline]
pub fn sanitize_to_charset_fast(value: &str, replacement: Option<&str>) -> String {
    let rep = replacement.unwrap_or(" ");
    let mut out = String::with_capacity(value.len());
    for ch in value.nfkd().filter(|&ch| !is_combining_mark(ch)) {
        if is_allowed_char(ch) {
            out.push(ch);
        } else {
            out.push_str(rep);
        }
    }
    out
}

/// The distinct characters of `value` outside the restricted Latin
/// set, each as a one-character string, in code-point order.
#[inline]
pub fn find_invalid_characters_fast(value: &str) -> Vec<String> {
    let invalid: BTreeSet<char> = value.chars().filter(|&ch| !is_allowed_char(ch)).collect();
    invalid.into_iter().map(String::from).collect()
}
