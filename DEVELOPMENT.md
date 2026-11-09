<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Development Guide

## Prerequisites

- Rust 1.89+ (`rust-version` in `Cargo.toml`)
- Python 3.11+
- The `maturin` version pinned in `pyproject.toml`

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

The repository ships the pain001 suite version in lockstep. Run the release
version gate before publishing; preserve existing tags. The 0.0.82 release
records the maintainer-authorized adoption of the suite version.
