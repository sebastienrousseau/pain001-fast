<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Development

The single entry point for working on `pain001-fast`: toolchain, how to
reproduce every CI gate locally, where tests live, and how a release
happens. [CONTRIBUTING.md](CONTRIBUTING.md) covers the etiquette (DCO,
commit format, review); this file covers the mechanics.

## Toolchain

| Tool | Version | Where it is pinned |
| :--- | :--- | :--- |
| Rust | 1.89 is the floor; CI also tests stable | `rust-version` in `Cargo.toml`, the `ci.yml` matrix, [toolchain policy](docs/POLICIES.md) |
| Python | 3.11 is the floor; CI tests 3.11 to 3.14 | `pyproject.toml` `requires-python`, the `ci.yml` matrix |
| uv | any recent release | resolves the dev environment from `uv.lock` |
| maturin | 1.16.0 | `pyproject.toml` `[build-system]` and the `dev` extra |
| cargo-llvm-cov | 0.9.1 (with `llvm-tools-preview`) | `ci.yml` Rust coverage job |
| cargo-fuzz | 0.13.2, on a nightly toolchain | `fuzz.yml`; only for `make fuzz` |
| Node | any current LTS | only for `markdownlint-cli2` in the docs-lint job |
| pre-commit | any | optional; hooks pinned by commit SHA in `.pre-commit-config.yaml` |

```bash
git clone https://github.com/sebastienrousseau/pain001-fast
cd pain001-fast
uv sync --extra dev --locked --python 3.12
rustup component add clippy rustfmt llvm-tools-preview
cargo install cargo-llvm-cov --locked --version 0.9.1
pre-commit install        # optional: ruff, codespell, cargo fmt, hygiene
PATH="$PWD/.venv/bin:$PATH" make check PYTHON="$PWD/.venv/bin/python"
```

`uv sync` builds the extension into `.venv` with maturin. After a change
to `src/`, rebuild it with
`uv sync --extra dev --locked --python 3.12 --reinstall-package pain001-fast`
before running the Python tests.

The Rust tests link libpython (PyO3 without `extension-module`), so the
Makefile passes the interpreter named by `PYTHON` to PyO3 and sets an
rpath to its library directory. Use an interpreter that ships a shared
libpython: uv's own CPython builds do. On macOS, an older Rust toolchain
may also need `SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"`.

The [devcontainer](.devcontainer/devcontainer.json) installs the same
toolchain and finishes by running `make check`.

## Reproducing every CI gate

| Gate | Local command | CI workflow and job |
| :--- | :--- | :--- |
| rustfmt, clippy (warnings denied, complexity lints), ruff, ruff format | `make lint` | `ci.yml` Rust, Python |
| Complexity: cognitive 15 and 60 lines per Rust function (`clippy.toml`), cyclomatic 10 per Python function (ruff C901) | `make lint` | `ci.yml` Rust, Python |
| Rust API docs, warnings denied; `missing_docs` | `make doc` | `ci.yml` Rust |
| mypy `--strict` | `make type` | `ci.yml` Python |
| Rust tests, then Python tests with the 100% line and branch floor | `make test` | `ci.yml` Rust, Python |
| Rust line and function coverage at 100% | `make coverage-rust` | `ci.yml` Rust coverage |
| Examples and the README Quick Start run | `make test` (`tests/test_examples.py`, `tests/test_readme.py`) | `ci.yml` Python |
| Release versions agree; release notes present | `make versions` | `ci.yml` Python |
| Advisory audit | `make sec` (needs `cargo-audit`) | `ci.yml` Dependency audit |
| Docs lint (codespell, markdownlint) | `make docs-lint` (needs `requirements/docs-lint.txt` and Node) | `docs-lint.yml` |
| Fuzzing | `make fuzz FUZZ_SECONDS=60` (needs nightly and cargo-fuzz) | `fuzz.yml` |
| Signed-off-by on every commit | `git commit -s` | `dco.yml` |
| Suite versions agree across the family | none locally (network) | `suite-consistency.yml` |
| All of the above that run offline without extra tools | `make check` | |

## Coverage

Python: 100% of statements and branches of `python/pain001_fast`,
stated once in `pyproject.toml` `[tool.coverage.report]`, including the
pure-Python fallback, which `tests/test_fallback.py` loads with the
extension blocked.

Rust: 100% of lines and functions of `src/`, measured by
`cargo llvm-cov` over the unit tests in `src/tests/`. Rust branch
coverage is not measured: it needs a nightly compiler
(`-Zcoverage-options=branch`), and the gate runs on stable. Line
coverage is the floor instead; the error paths that branch coverage
would add are the `?` exits of PyDict and PyList calls in `batch.rs`,
which no test provokes (regions stand at 96%).

## Complexity

Every function is held to cyclomatic 10, cognitive 15, Halstead
difficulty 30 and 60 lines, and every file to 500 lines. Clippy
enforces cognitive complexity and function length for Rust; ruff C901
enforces cyclomatic complexity for Python. Clippy has no cyclomatic or
Halstead lint, so those are measured, not gated:
`rust-code-analysis-cli -m -O json -p src/` reports a maximum of
cyclomatic 10 and Halstead difficulty 24.9 today. There is no baseline
of existing offenders; none remain.

## Test layout

```text
src/tests/
  iban.rs, bic.rs        every refusal message in full; country tables
  charset.rs             the permitted set, sanitising, combining marks
  batch.rs, module.rs    the batch validator and module, in an embedded interpreter
  corpus.rs              replays fuzz/corpus through the fuzz checks on stable
src/invariants.rs        the contract checks shared with the fuzz target
tests/
  test_fast.py           the extension from Python
  test_fallback.py       the pure-Python fallback, against the extension
  test_api_docs.py       every public name has a docstring
  test_examples.py       runs every examples/[0-9]*.py
  test_readme.py         README section order, tokens, links; Quick Start runs
  test_release_notes.py  scripts/release_notes.py; vendored from pain001, do not edit
examples/                numbered, self-checking scripts
fuzz/                    cargo-fuzz harness and its seed corpus
```

## Fuzzing

`fuzz/` holds one cargo-fuzz target, `validators`, which runs every
entry point on arbitrary UTF-8 and checks the contract in
`src/invariants.rs`. `make fuzz` runs it for `FUZZ_SECONDS`, writing
new inputs to `fuzz/target/corpus` (ignored). A crash is fixed with
its input added to `fuzz/corpus/validators`, which `cargo test`
replays on every run.

## Release model

The version is `0.0.x`, moves in `0.0.1` steps, and ships in lockstep
with the other pain001 suite members (adopted at 0.0.82; the earlier
0.0.1 and 0.0.2 tags are kept). Work for the next version happens on
`feat/v<next-version>`, the only pull request into `main`. The version
is restated in `Cargo.toml`, `pyproject.toml`,
`python/pain001_fast/__init__.py`, `CHANGELOG.md`, the README and
`releases/v<version>.md`; `make versions` checks they agree. The
maintainer opens and cuts releases; the tag-triggered `release.yml`
builds the wheels and the source distribution and attaches them to the
GitHub release. The package is not published to PyPI.
