// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Replay the committed fuzz corpus through the fuzz target's checks
//! on every `cargo test`, so a past finding stays fixed without a
//! nightly compiler or cargo-fuzz.

use std::path::Path;

#[test]
fn the_fuzz_corpus_keeps_every_invariant() {
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("fuzz/corpus/validators");
    let mut replayed = 0;
    for entry in std::fs::read_dir(&corpus).unwrap() {
        let bytes = std::fs::read(entry.unwrap().path()).unwrap();
        // The target skips input that is not UTF-8, as the bindings
        // only ever receive Python strings.
        if let Ok(text) = std::str::from_utf8(&bytes) {
            crate::check_invariants(text);
            replayed += 1;
        }
    }
    assert!(replayed > 0, "no corpus under {}", corpus.display());
}
