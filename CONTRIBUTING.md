<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Contributing to pain001-fast

Thank you for contributing to `pain001-fast`, the native Rust extension
behind pain001's IBAN, BIC and character-set checks.

## Setup

[DEVELOPMENT.md](DEVELOPMENT.md) lists the toolchain (Rust 1.89+,
Python 3.11+, uv, cargo-llvm-cov) and maps every CI job to the local
command that reproduces it. In short:

```bash
git clone git@github.com:sebastienrousseau/pain001-fast.git
cd pain001-fast
uv sync --extra dev --locked --python 3.12
PATH="$PWD/.venv/bin:$PATH" make check PYTHON="$PWD/.venv/bin/python"
```

## Workflow

1. Fork the repository and branch from `main`.
2. Make the change, with its tests in the same commit.
3. Run `make check`: clippy, rustfmt, ruff and mypy; rustdoc with
   warnings denied; the Rust and Python tests; 100% Rust line and
   function coverage; 100% Python line and branch coverage; the
   version checks.
4. Commit with a sign-off and a signature:
   `git commit -s -S -m "fix: describe the change"`.
5. Push and open a pull request; the template lists what review
   expects.

## Standards

- **Commits** follow [Conventional Commits](https://www.conventionalcommits.org/)
  (`feat:`, `fix:`, `test:`, `docs:`, `chore:` and so on) with a body
  that says why. Every commit carries a `Signed-off-by` trailer
  (Developer Certificate of Origin, checked by `dco.yml`).
- **Behaviour.** pain001's results must not depend on whether this
  extension is installed. A change to what a validator accepts, or to
  the text of a message, starts as an issue on the core. Tests compare
  every message in full for this reason.
- **The fallback.** `python/pain001_fast/__init__.py` holds a
  pure-Python version of every binding; keep its verdicts equal to the
  extension's (`tests/test_fallback.py`).
- **Complexity.** Functions stay within cyclomatic 10, cognitive 15
  and 60 lines; files within 500 lines. Clippy and ruff enforce it.
- **Documentation.** Every public Rust item has a doc comment; it is
  also the Python docstring.
- **Licensing.** Every source file starts with the SPDX header
  `Apache-2.0 OR MIT`. No real account numbers or personal data.

## Reporting

Bugs and feature requests go to the issue tracker; see
[SUPPORT.md](SUPPORT.md). Vulnerabilities go through
[SECURITY.md](SECURITY.md), never a public issue.
