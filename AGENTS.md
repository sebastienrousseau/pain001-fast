<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# AGENTS.md

Invariants for AI-assisted contributors working in `pain001-fast`.

## Versioning

- Initial version starts at `0.0.1`.
- Release branches follow `feat/v<next-version>`.
- Version restatements must stay synchronized in `Cargo.toml`, `pyproject.toml`,
  `python/pain001_fast/__init__.py`, and `CHANGELOG.md`.

## Commits and Signing

- Conventional commits (`feat:`, `fix:`, `chore:`, …).
- Every commit must carry a `Signed-off-by` trailer (DCO) and be SSH-signed.

## Quality Gates

- `cargo test --no-default-features` must pass cleanly.
- `cargo clippy --all-targets -- -D warnings` must pass with 0 warnings.
- 100% test coverage line and branch floor is non-negotiable.
