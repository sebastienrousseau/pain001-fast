<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Architecture

`pain001-fast` is a Rust crate built by maturin into the Python package
`pain001_fast`. pain001's validators import it when it is installed and
fall back to their own Python code when it is not.

## Layers

```text
pain001 (Python)
   |  import pain001_fast
   v
python/pain001_fast/__init__.py   re-exports the extension, or defines a
   |                              pure-Python fallback if it cannot load
   v
_pain001_fast (PyO3 module)       src/lib.rs: thin #[pyfunction] bindings
   |
   v
Rust core                         src/iban.rs, bic.rs, charset.rs, batch.rs
```

## Modules

| File | Role |
| :--- | :--- |
| `src/lib.rs` | The bindings, the declarative `_pain001_fast` module, and two helpers: `compact` (drop spaces and hyphens, uppercase ASCII, into a bounded stack buffer) and `verdict` (check result to `(bool, str)`). |
| `src/iban.rs` | ISO 13616 structure, a sorted country-length table searched by binary search, and the ISO 7064 mod 97-10 check, folded over the bytes with no big integer. |
| `src/bic.rs` | ISO 9362 structure (bank, country, location, branch) and the country-code list pain001 core uses. |
| `src/charset.rs` | The ISO 20022 restricted Latin set as a 128-entry table, NFKD transliteration and the list of refused characters. |
| `src/batch.rs` | `validate_payment_batch`: reads each dict under the GIL, validates the records in parallel with Rayon, then builds control sums, errors and duplicates. |
| `src/invariants.rs` | The contract checks shared by the fuzz target and the corpus replay test; test and `fuzzing` builds only. |

## Contracts

- **Messages are interface.** pain001 shows the validators' messages to
  its users, so they are pinned in full by the Rust tests and compared
  with the fallback's by the Python tests.
- **Checks stop at the first failing step**, except the structural step
  of each validator, which reports every faulty part, joined by semicolons.
- **Input is bytes.** IBAN and BIC lengths count UTF-8 bytes, and any
  non-ASCII character is refused. The fallback refuses it too, with a
  message that may differ.
- **The batch reports one fault per record**, in the order IBAN, BIC,
  amount; amounts that parse count towards the control sums even when
  the record has another fault.

## Decisions

- The country tables are sorted constant arrays rather than `match`
  expressions, so tests can iterate them and each lookup is a binary
  search.
- The BIC country list is a subset of ISO 3166-1, shared with pain001
  core; BICs from unlisted countries are refused by both. Widening it
  is a suite-wide decision.
- `rayon` runs only the per-record validation; reading the dicts and
  building the result need the GIL and stay sequential.
