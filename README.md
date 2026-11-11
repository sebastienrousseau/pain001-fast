<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/pain001/v1/logos/pain001.svg" alt="pain001-fast logo" width="128" />
</p>

<h1 align="center">pain001-fast</h1>

<p align="center">
  The optional Rust extension behind pain001's IBAN, BIC and character-set checks, with a parallel batch validator.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/pain001-fast/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/pain001-fast/ci.yml?style=for-the-badge&logo=github" alt="Build" height="28" /></a>
  <a href="https://github.com/sebastienrousseau/pain001-fast/releases"><img src="https://img.shields.io/github/v/release/sebastienrousseau/pain001-fast?style=for-the-badge&color=fc8d62&logo=python" alt="Registry" height="28" /></a>
  <a href="docs/index.md"><img src="https://img.shields.io/badge/docs-source-blue?style=for-the-badge&labelColor=555555&logo=readthedocs" alt="Docs" height="28" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/pain001-fast"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/pain001-fast?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" height="28" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" height="28" /></a>
  <a href="https://github.com/sebastienrousseau/pain001-fast/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/Python-3.11%2B-93450a.svg?style=for-the-badge&logo=python" alt="Python 3.11 or newer" height="28" /></a>
</p>

<p align="center">
  <img src=".github/demo.gif" alt="pain001-fast demo: the identifier example prints the refusal messages for a bad IBAN checksum and a bad BIC country code, then the batch example prints the IBAN and amount faults it finds in four records" width="100%" />
</p>

---

## Contents

**Getting started**

- [Install](#install) — release wheels from GitHub, or a source build
- [Requirements](#requirements) — toolchain floor, platforms
- [Quick Start](#quick-start) — check identifiers and a batch from Python

**The pain001-fast ecosystem**

- [The pain001-fast ecosystem](#the-pain001-fast-ecosystem) — the core library that uses it, and the suite map

**Library reference**

- [Capabilities at a glance](#capabilities-at-a-glance) — the current surface by theme
- [Ecosystem comparison](#ecosystem-comparison) — short matrix; full table at [`docs/COMPARISON.md`](docs/COMPARISON.md)
- [Benchmarks](#benchmarks) — headline numbers; full table at [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md)
- [Features](#features) — module-level capability list
- [Configuration](#configuration) — core options
- [Examples](#examples) — runnable example index

**Operational**

- [When not to use pain001-fast](#when-not-to-use-pain001-fast) — limitations
- [Development](#development) — make targets, fuzzing, CI
- [Security](#security) — guarantees and compliance
- [Documentation](#documentation) — all reference docs
- [Stability guarantees](#stability-guarantees) — SemVer axis, output stability, minimum toolchain discipline
- [License](#license)

---

## Install

### As a Python library

```bash
pip install "git+https://github.com/sebastienrousseau/pain001-fast.git@v0.0.85"
```

This package is distributed through
[GitHub Releases](https://github.com/sebastienrousseau/pain001-fast/releases/tag/v0.0.85),
not PyPI. The command above builds from the signed source tag and needs
Rust 1.89 or newer; to skip the build, download the wheel for your
platform from the release and install it with `pip install <wheel>`.

From a checkout, `uv sync --extra dev --locked` builds the extension
into a virtual environment with maturin.

---

## Requirements

- Python 3.11 or newer.
- Rust 1.89 or newer for a source build, as declared by `rust-version`
  in `Cargo.toml`.
- Release wheels are built for Linux (x86_64, aarch64), macOS (arm64,
  x86_64) and Windows (x86_64). Other platforms need a source build.

---

## Quick Start

```python
from pain001_fast import validate_bic, validate_iban, validate_payment_batch

assert validate_iban("DE89370400440532013000") is True
assert validate_bic("DEUTDEFF500") is True

results = validate_payment_batch(
    [
        {
            "creditor_iban": "DE89370400440532013000",
            "creditor_bic": "DEUTDEFF",
            "amount": "1250.50",
            "currency": "EUR",
            "execution_date": "2026-10-01",
        }
    ]
)
assert results["valid"] is True
assert results["control_sums"] == {"EUR": 1250.5}
```

pain001 calls the IBAN, BIC and character-set checks itself when the
extension is installed; see [Configuration](#configuration). The batch
validator is only reached by calling it, as above.

---

## The pain001-fast ecosystem

`pain001-fast` is an optional accelerator for `pain001`: installing it
moves pain001's IBAN, BIC and character-set checks into Rust, and
removing it puts pain001 back on its own Python validators.

| Component | Purpose | Use case |
| :--- | :--- | :--- |
| [pain001](https://github.com/sebastienrousseau/pain001) | ISO 20022 payment file generator and validator | The library, CLI and REST API that use this extension |
| [pain001-fast](https://github.com/sebastienrousseau/pain001-fast) | Native Rust validation core | IBAN, BIC and character-set checks in pain001; batch validation from Python |
| [pain001 suite map](https://github.com/sebastienrousseau/pain001#the-pain001-ecosystem) | Every member of the suite | The MCP server, language server, loaders and the rest |

---

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| IBAN | ISO 13616 structure, per-country length, ISO 7064 mod 97-10 | Shipped; API experimental |
| BIC | ISO 9362 structure and the country list pain001 core uses | Shipped; API experimental |
| Character set | ISO 20022 restricted Latin check, refused-character list, NFKD transliteration | Shipped; API experimental |
| Batch | Parallel per-record validation, control sums per currency, duplicate detection | Shipped; API experimental |

---

## Ecosystem comparison

How the extension differs from pain001's built-in Python validators,
which run when it is not installed. The two are meant to give the same
verdicts; an input on which they differ is a bug.

| Project | Language | mod 97 arithmetic | Batch validation call |
| :--- | :---: | :---: | :---: |
| **pain001-fast** | Rust (PyO3) | Running remainder over the bytes | `validate_payment_batch`, parallel |
| pain001 without it | Python | `int()` of the whole digit string | None |

See [`docs/COMPARISON.md`](docs/COMPARISON.md) for the evidence and complete matrix.

---

## Benchmarks

No benchmark in this repository measures the extension yet, so no
figures are published. Earlier releases quoted per-call times and batch
throughput with no committed benchmark behind them; those figures are
withdrawn until one reproduces them.

| Scenario | Result | Environment |
| :--- | ---: | :--- |
| IBAN, BIC and batch validation | Not yet measured | No benchmark in the repository |

See [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) for methodology and full results.

---

## Features

- **IBAN validation** on a 34-byte stack buffer: separators removed,
  letters uppercased, structure and the country's registered length
  checked, then mod 97-10 as a running remainder. A valid IBAN costs no
  heap allocation; a refusal allocates its message.
- **BIC validation** on an 11-byte stack buffer: bank, country,
  location and branch checked byte by byte, every faulty part named in
  one message, then the country code.
- **Restricted character set**: a 128-entry lookup table for the check,
  the distinct refused characters in code-point order, and NFKD
  transliteration that drops accents and replaces the rest.
- **Batch validation**: records read under the GIL, validated in
  parallel with Rayon, then control sums per currency, the first fault
  of each record, and records repeated in full.
- **Pure-Python fallback** for every binding, used only when the
  compiled module cannot be imported, held to the same verdicts by the
  test suite.

---

## Configuration

`pain001-fast` has no configuration. When it is importable, pain001's
`validate_iban_safe`, `validate_bic_safe` and `is_valid_charset` use
its checks automatically. Installing it does not route XML generation
or pain001's detailed validation through it; call
`validate_payment_batch` directly for batch validation. Rayon sizes
its thread pool from the CPU count, or from `RAYON_NUM_THREADS` when
set.

---

## Examples

Each script asserts what it prints, and `tests/test_examples.py` runs
them all in CI.

| Script | Shows |
| :--- | :--- |
| [`examples/01_validate_identifiers.py`](examples/01_validate_identifiers.py) | IBAN and BIC checks, and reading why one is refused |
| [`examples/02_restricted_charset.py`](examples/02_restricted_charset.py) | Finding and replacing characters outside the restricted set |
| [`examples/03_validate_batch.py`](examples/03_validate_batch.py) | A batch with a duplicate, a bad IBAN and an amount that does not parse |

---

## When not to use pain001-fast

- On a platform with no release wheel and no Rust toolchain: pain001
  works without it.
- Where native CPython extensions cannot load (WebAssembly,
  MicroPython).
- When you need a stable API: the bindings are experimental, and
  pain001's own validators are the supported interface.
- For BICs from countries outside the list pain001 core uses: both
  refuse them.

---

## Development

```bash
uv sync --extra dev --locked --python 3.12
PATH="$PWD/.venv/bin:$PATH" make check PYTHON="$PWD/.venv/bin/python"
make docs-lint
make fuzz FUZZ_SECONDS=60
```

`make check` runs clippy and rustfmt, ruff and mypy, rustdoc with
warnings denied, the Rust and Python tests, 100% Rust line and function
coverage, 100% Python line and branch coverage, and the release
version checks. [DEVELOPMENT.md](DEVELOPMENT.md) maps every CI job to
its local command.

---

## Security

Report vulnerabilities privately through GitHub Private Vulnerability
Reporting or `security@pain001.com`, never a public issue. The crate
has no `unsafe` code of its own and reads no files and no network.
IBAN and BIC checks copy at most 34 and 11 bytes into stack buffers
whatever the input length. A cargo-fuzz target runs every entry point
on arbitrary text in CI, and `cargo test` replays its committed corpus
on every run. OSS-Fuzz is not set up.

Report vulnerabilities according to [`SECURITY.md`](SECURITY.md).

---

## Documentation

- User manual: [docs/index.md](docs/index.md) (no rendered manual yet)
- API reference: the Rust doc comments, built with `make doc`; they are
  also the Python docstrings
- Developer docs: [DEVELOPMENT.md](DEVELOPMENT.md) and
  [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- Family map: [the pain001 ecosystem](https://github.com/sebastienrousseau/pain001#the-pain001-ecosystem)

---

## Stability guarantees

Version 0.0.85, in lockstep with the pain001 suite, which this extension joined at 0.0.82 by maintainer decision. Earlier 0.0.1 and 0.0.2 tags are preserved. Releases advance together by one 0.0.1 step. This native extension remains experimental; no stable public API is promised.

Output is part of the interface: a change to what a validator accepts,
or to the text of a message, is a behaviour change for pain001's users
even when no signature moves, and is agreed in the core first. The
minimum Rust and Python versions are stated in
[docs/POLICIES.md](docs/POLICIES.md) and tested in CI.

---

## License

Dual-licensed under [Apache-2.0](LICENSE-APACHE) OR [MIT](LICENSE-MIT), at your option. See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
