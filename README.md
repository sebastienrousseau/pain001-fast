<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/pain001/v1/logos/pain001.svg" alt="pain001-fast logo" width="128" />
</p>

<h1 align="center">pain001-fast</h1>

<p align="center">
  High-throughput native validation and serialization core for the pain001 ISO 20022 ecosystem.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/pain001-fast/actions"><img src="https://github.com/sebastienrousseau/pain001-fast/workflows/ci/badge.svg?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://pypi.org/project/pain001-fast/"><img src="https://img.shields.io/pypi/v/pain001-fast?style=for-the-badge&color=fc8d62&logo=python" alt="Registry" /></a>
  <a href="docs/index.md"><img src="https://img.shields.io/badge/docs-source?style=for-the-badge&labelColor=555555&logo=readthedocs" alt="Docs" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/pain001-fast"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/pain001-fast?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" /></a>
  <a href="https://github.com/sebastienrousseau/pain001-fast/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/Python-3.10%2B-93450a.svg?style=for-the-badge&logo=python" alt="Python 3.10 or newer" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — PyPI and source installation
- [Requirements](#requirements) — toolchain floor, platforms
- [Quick Start](#quick-start) — high-speed financial validation in Python

**The pain001-fast ecosystem**

- [The pain001-fast ecosystem](#the-pain001-fast-ecosystem) — core engine and companion accelerators

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
pip install pain001-fast
```

For source builds, ensure Rust 1.89+ and `maturin` are installed:

```bash
maturin develop --release
```

---

## Requirements

- Python 3.10 or newer.
- Rust 1.89 or newer (for source compilation), as declared by
  `rust-version` in `Cargo.toml`.
- Supported platforms: Linux (x86_64, aarch64, musl), macOS (Apple Silicon, Intel), Windows (x86_64).

---

## Quick Start

```python
from pain001_fast import validate_iban, validate_bic, validate_payment_batch

# Zero-allocation IBAN mod-97 check (< 25 ns)
assert validate_iban("DE89370400440532013000") is True

# Structural BIC verification
assert validate_bic("DEUTDEFF500") is True

# Multi-core batch validation (50,000+ tx/s)
results = validate_payment_batch([
    {
        "creditor_iban": "DE89370400440532013000",
        "creditor_bic": "DEUTDEFF",
        "amount": "1250.50",
        "currency": "EUR",
        "execution_date": "2026-10-01",
    }
])
assert results["valid"] is True
```

`pain001-fast` accelerates payment batch workflows transparently without changes to your business logic.

---

## The pain001-fast ecosystem

`pain001-fast` operates as an optional native accelerator for `pain001`, offloading CPU-intensive validation loops to multi-threaded Rust.

| Component | Purpose | Use case |
| :--- | :--- | :--- |
| [pain001](https://github.com/sebastienrousseau/pain001) | Primary ISO 20022 generator | Core XML synthesis, CLI, and streaming |
| [pain001-fast](https://github.com/sebastienrousseau/pain001-fast) | Native Rust validation engine | 50,000+ tx/s high-throughput pipelines |
| [pain001-mcp](https://github.com/sebastienrousseau/pain001-mcp) | Model Context Protocol server | Agentic orchestration and simulation |

---

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| Checksum Engine | Zero-allocation ISO 7064 mod-97 IBAN validator | Production Ready |
| Routing Engine | Structural ISO 9362 BIC validator with ISO 3166-1 verification | Production Ready |
| Charset Sanitization | NFKD transliteration to SWIFT-X restricted Latin character set | Production Ready |
| Batch Processing | Multi-threaded Rayon validation and duplicate fingerprinting | Production Ready |

---

## Ecosystem comparison

This matrix describes the repository's native performance characteristics.

| Project | Native Rust Backend | Zero-Allocation Mod-97 | Multi-Core Rayon Batching |
| :--- | :---: | :---: | :---: |
| **pain001-fast** | Yes (PyO3) | Yes | Yes |
| standard pain001 | Pure Python | No (BigInt Python) | Single-threaded |

See [`docs/COMPARISON.md`](docs/COMPARISON.md) for the evidence and complete matrix.

---

## Benchmarks

Micro-benchmarked on Apple Silicon M-series hardware.

| Scenario | Result | Environment |
| :--- | ---: | :--- |
| IBAN Checksum (Mod-97) | 22 ns / tx | Apple Silicon, release build |
| BIC Structural Check | 14 ns / tx | Apple Silicon, release build |
| Batch Validation (10,000 records) | 165 ms (> 60,000 tx/s) | 8 Rayon worker threads |

See [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) for methodology and full results.

---

## Features

- **Zero-Allocation IBAN Validation**: Streaming modulo-97 calculation in CPU registers without intermediate big integer heap allocations.
- **Structural BIC Validation**: Regex-free byte-level parser checking 8 and 11 character ISO 9362 codes against ISO 3166-1 country tables.
- **SWIFT-X Charset Enforcement**: Fast lookup-table validation and NFKD diacritic transliteration.
- **Parallel Multi-Core Batch Analyzer**: Rayon parallel iterator computing currency control sums, detecting duplicate transactions, and collecting errors in a single pass.

---

## Configuration

`pain001-fast` requires zero configuration. When installed, `pain001` core automatically detects and promotes validation routines to native speed.

---

## Examples

Run the benchmark and integration suite:

```bash
cargo test --no-default-features
pytest tests/
```

---

## When not to use pain001-fast

Do not use `pain001-fast` in pure WebAssembly or micro-Python environments where native CPython C extensions are unsupported. Standard `pain001` pure-Python fallback should be used in those environments.

---

## Development

```bash
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
maturin develop --release
pytest tests/
```

Quality gates enforce 100% test coverage and zero clippy warnings.

---

## Security

Input buffers are bounded to prevent allocation denial-of-service. Memory safety is guaranteed by Rust invariants.

Report vulnerabilities according to [`SECURITY.md`](SECURITY.md).

---

## Documentation

- [User manual](docs/index.md)
- [API reference](docs/index.md)
- [Developer guide](DEVELOPMENT.md)
- [Family map](https://github.com/sebastienrousseau/pain001#the-pain001-ecosystem)

---

## Stability guarantees

Follows Semantic Versioning strictly. Initial version starts at `v0.0.1`.

---

## License

Dual-licensed under [Apache-2.0](LICENSE-APACHE) OR [MIT](LICENSE-MIT), at your option. See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
