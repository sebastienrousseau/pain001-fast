// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Arbitrary text through every Rust entry point. Beyond "does not
//! panic", each call is checked against the invariants the bindings
//! promise; the same checks replay the committed corpus in
//! `src/tests/corpus.rs` on stable Rust.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        pain001_fast::check_invariants(text);
    }
});
