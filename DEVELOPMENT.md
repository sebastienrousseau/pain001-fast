<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Development Guide

## Prerequisites

- Rust 1.89+ (`rust-version` in `Cargo.toml`)
- Python 3.11+
- `maturin` 1.5+

## Setup & Build

```bash
maturin develop --release
```

## Testing

```bash
# Rust unit tests
cargo test --no-default-features

# Python integration tests
pytest tests/ -v --cov=pain001_fast --cov-branch --cov-report=term-missing
```

## Linting & Formatting

```bash
cargo clippy --all-targets -- -D warnings
cargo fmt --check
ruff check .
mypy python/pain001_fast
```
