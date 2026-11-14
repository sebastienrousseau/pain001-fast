<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Security Policy

## Supported versions

This package follows the [`pain001`](https://github.com/sebastienrousseau/pain001)
suite cadence. While pre-`1.0`, the latest released 0.0.x and the
immediately prior 0.0.x receive security fixes; older versions do not.

| Version | Status | Receives security fixes? |
| :--- | :--- | :--- |
| `0.0.88` (latest) | Current | ✅ Yes |
| `0.0.87` | Prior | ✅ Yes |
| `≤ 0.0.86` | Old | ❌ No, upgrade |

## Reporting a vulnerability

**Do not open a public issue for security reports.**

Use one of the following private channels, in this order of
preference:

1. **GitHub Private Vulnerability Reporting**
   <https://github.com/sebastienrousseau/pain001-fast/security/advisories/new>
2. **Email**: `security@pain001.com`

Please include a description and its impact, a minimal reproducer
(preferably a failing test or a fuzz input), and the affected
version (`python -c "import pain001_fast; print(pain001_fast.__version__)"`).

These are goals, not guarantees: the project is maintained in a
personal capacity, with no SLA and no out-of-hours cover.
**Acknowledgement**: the goal is within 5 working days. **Fix
windows** (best effort, from triage): critical 7 days, high 30 days,
medium 60 days, low best-effort. Fixes are published through GitHub
Security Advisories, and reporters are credited unless they ask to
remain anonymous.

## Security posture

- **Scope.** The extension validates strings it is given. It reads no
  files, opens no network connections, and holds no secrets.
- **Memory safety.** The crate contains no `unsafe` code of its own;
  memory safety rests on Rust and on PyO3's bindings.
- **Bounded input handling.** IBAN and BIC checks copy at most 34 and
  11 bytes into stack buffers and stop at the first byte past that
  bound, whatever the input length. The character-set functions and
  the batch validator allocate in proportion to their input.
- **Panics.** The release profile sets `panic = "abort"`. A fuzz target
  (`fuzz/`) runs every entry point on arbitrary text in CI, and the
  committed corpus is replayed on every `cargo test`.
- **Dependencies.** `pyo3`, `rayon` and `unicode-normalization`, locked
  in `Cargo.lock` and audited by `cargo audit` in CI and by Dependabot.
- **Supply chain.** GitHub Actions are pinned by commit SHA; release
  tags are signed.
