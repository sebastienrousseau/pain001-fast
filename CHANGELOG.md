<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Changelog

All notable changes to `pain001-fast` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1] - 2026-09-27

### Added

- Initial release of `pain001-fast` high-throughput native acceleration core.
- Zero-allocation ISO 7064 mod-97 IBAN validator with country length checks.
- Structural ISO 9362 BIC validator with ISO 3166-1 country code verification.
- ISO 20022 restricted Latin (SWIFT-X) character set validator and NFKD transliteration.
- Parallel multi-core batch validation engine using Rayon for duplicate detection and control sums.
- PyO3 native extension bindings with complete `.pyi` type stubs.
