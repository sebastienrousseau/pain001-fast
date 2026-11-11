// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Properties every entry point keeps for any input, shared by the
//! cargo-fuzz harness (`fuzz/`) and the corpus replay test, which runs
//! on stable Rust with `cargo test`.

use crate::charset::is_allowed_char;
use crate::{
    find_invalid_characters_fast, sanitize_to_charset_fast, validate_bic_detailed_fast,
    validate_iban_detailed_fast,
};

/// Panic if any entry point breaks its contract on `text`.
///
/// - The validators never panic, and report a message exactly when
///   they refuse.
/// - A valid IBAN or BIC is ASCII letters and digits once spaces and
///   hyphens are removed.
/// - `find_invalid_characters_fast` is empty exactly when every
///   character is permitted, and lists only refused characters.
/// - `sanitize_to_charset_fast` always returns permitted text.
pub fn check_invariants(text: &str) {
    for (valid, message) in [
        validate_iban_detailed_fast(text),
        validate_bic_detailed_fast(text),
    ] {
        assert_eq!(valid, message.is_empty(), "{text:?}: {message:?}");
        if valid {
            let significant = text.bytes().filter(|&b| b != b' ' && b != b'-');
            assert!(
                significant.clone().all(|b| b.is_ascii_alphanumeric()),
                "{text:?}"
            );
        }
    }
    let invalid = find_invalid_characters_fast(text);
    assert_eq!(
        invalid.is_empty(),
        text.chars().all(is_allowed_char),
        "{text:?}"
    );
    assert!(
        invalid
            .iter()
            .flat_map(|s| s.chars())
            .all(|ch| !is_allowed_char(ch)),
        "{text:?}"
    );
    let clean = sanitize_to_charset_fast(text, None);
    assert!(clean.chars().all(is_allowed_char), "{text:?} -> {clean:?}");
}
