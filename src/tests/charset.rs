// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{
    find_invalid_characters, find_invalid_characters_fast, is_valid_charset, sanitize_to_charset,
    sanitize_to_charset_fast,
};

const PERMITTED: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+ ";

#[test]
fn the_permitted_set_is_exactly_the_restricted_latin_set() {
    assert!(is_valid_charset(PERMITTED));
    for code in 0u32..0x250 {
        let ch = char::from_u32(code).unwrap();
        let text = ch.to_string();
        assert_eq!(is_valid_charset(&text), PERMITTED.contains(ch), "{code:#x}");
    }
}

#[test]
fn invalid_characters_are_unique_and_sorted() {
    assert_eq!(find_invalid_characters_fast("A & B @ C &"), ["&", "@"]);
    assert_eq!(find_invalid_characters("Caf\u{e9}"), ["\u{e9}"]);
    assert!(find_invalid_characters("Clean string").is_empty());
}

#[test]
fn sanitising_strips_accents_and_replaces_the_rest() {
    assert_eq!(
        sanitize_to_charset_fast("Caf\u{e9} M\u{fc}nch\u{eb}n & Co", Some(" ")),
        "Cafe Munchen   Co"
    );
    assert_eq!(sanitize_to_charset("A & B", "and"), "A and B");
}

#[test]
fn sanitising_defaults_to_a_space() {
    assert_eq!(sanitize_to_charset_fast("A&B", None), "A B");
}

#[test]
fn every_combining_block_is_stripped() {
    for mark in ['\u{0301}', '\u{1AB0}', '\u{1DC0}', '\u{20D0}', '\u{FE20}'] {
        let text = format!("e{mark}");
        assert_eq!(sanitize_to_charset_fast(&text, Some("_")), "e", "{mark:?}");
    }
}
