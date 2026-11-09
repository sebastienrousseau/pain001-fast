<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Changelog

All notable changes to `pain001-fast` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.84] - 2026-11-09

### Added

- Audit all nine suite repositories and their publication channels daily and after publishing workflows, using the shared checker pinned to a core commit.

### Fixed

- Describe the core's automatic native validation helpers accurately and link to the current source and wheel release.
- `make test` passes `PYO3_PYTHON` with the rpath, so the Rust tests
  link the same libpython the rpath names; with a venv on a different
  Python first on `PATH` they aborted with "Library not loaded".

### Changed

- Release 0.0.84 with the suite. Native validation and the Python API retain their existing behavior.

## [0.0.83] - 2026-11-09

### Changed

- Release 0.0.83 in lockstep with pain001 0.0.83. Nothing in the
  extension changed beyond the version: the validation algorithms and
  the Python API are the same as in 0.0.82, and the extension does not
  depend on the core.

## [0.0.82] - 2026-11-09

### Changed

- Join the pain001 suite at 0.0.82 by explicit maintainer decision. Preserve
  the 0.0.1 and 0.0.2 tags; subsequent releases advance one step together.
- Build CPython stable-ABI wheels for Python 3.11 and newer, and test the
  native extension on Python 3.11 through 3.14.
- Document GitHub as the distribution channel and explicit imports as the
  integration model; no PyPI publication or automatic core activation is claimed.

### Added

- A tag-driven release workflow with platform wheels, a source distribution,
  build provenance, complete asset checksums and generated release notes.
- A release-version gate across Rust, Python, lockfiles and documentation,
  plus a single `make check` gate and `make sec` dependency audit.

## [0.0.2] - 2026-10-27

### Changed

- **Breaking: the MSRV is now declared as Rust 1.89**, as `rust-version` in
  `Cargo.toml`, one floor shared with the maintainer's other Rust
  crates. The README and DEVELOPMENT.md said "Rust 1.70+", which
  nothing checked; they now name 1.89.
- **Breaking: Python 3.11 is now the minimum.** `requires-python` is
  `>=3.11,<4`, the 3.10 classifier and CI job are gone, and ruff and
  mypy target 3.11. Python 3.10 reaches end of life in October 2026,
  and the pain001 family drops it together.
- Dependencies: `pyo3` 0.29.2 to 0.30.1, `unicode-normalization`
  0.1.25 to 0.1.26, and the dev-only `criterion` 0.5.1 to 0.8.2. CI
  moves to `actions/setup-python` 7.1.0 and `Swatinem/rust-cache`
  2.9.3.

### Added

- CI on every pull request and push to `main`: rustfmt, clippy with
  warnings denied and the Rust tests on Rust 1.89 and stable, then ruff,
  mypy and pytest (100% line and branch coverage) against the built
  extension on Python 3.11 to 3.13. A DCO check and Dependabot for
  cargo and GitHub Actions come with it.

### Fixed

- `make test` linked the Rust tests against one machine's Python
  install path; it now uses the library directory of the active
  `python3`.

## [0.0.1] - 2026-09-27

### Added

- Initial release of `pain001-fast` high-throughput native acceleration core.
- Zero-allocation ISO 7064 mod-97 IBAN validator with country length checks.
- Structural ISO 9362 BIC validator with ISO 3166-1 country code verification.
- ISO 20022 restricted Latin (SWIFT-X) character set validator and NFKD transliteration.
- Parallel multi-core batch validation engine using Rayon for duplicate detection and control sums.
- PyO3 native extension bindings with complete `.pyi` type stubs.
