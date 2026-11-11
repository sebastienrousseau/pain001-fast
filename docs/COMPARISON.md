<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Ecosystem Comparison

How `pain001-fast` differs from pain001's built-in Python validators,
which pain001 uses when the extension is not installed. Each row
describes the code, not a measurement; see
[BENCHMARKS.md](BENCHMARKS.md) for why no timings are published.

| Aspect | pain001 without the extension | pain001-fast | Evidence |
| :--- | :--- | :--- | :--- |
| Language | Python | Rust, bound with PyO3 | `src/` |
| IBAN mod 97 | `int()` of the whole digit string, then `% 97` | A running remainder over the bytes, no big integer | `pain001/validation/iban_validator.py` in pain001; `src/iban.rs` |
| IBAN and BIC buffers | Python strings | 34- and 11-byte stack buffers; a valid code allocates nothing on the heap | `src/lib.rs` `compact` |
| Character set | Python | A 128-entry lookup table | `src/charset.rs` |
| Batch validation | No equivalent call | `validate_payment_batch`, records validated in parallel with Rayon; reading the dicts and building the result allocate per record | `src/batch.rs` |
| BIC country list | A subset of ISO 3166-1 | The same list | `src/bic.rs` |
