#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Runs once when the devcontainer is created: the toolchain that
# DEVELOPMENT.md lists, then `make check`, so a container that finishes
# creating is one where every local gate has passed.
set -euo pipefail

python3 -m pip install --user uv==0.13.6
export PATH="$HOME/.local/bin:$PATH"

rustup component add clippy rustfmt llvm-tools-preview
cargo install cargo-llvm-cov --locked --version 0.9.1

# uv's own CPython build ships libpython, which `make test` links the
# Rust tests against.
uv sync --extra dev --locked --python 3.12 --managed-python
python3 -m pip install --user --require-hashes -r requirements/docs-lint.txt

PATH="$PWD/.venv/bin:$PATH" make check PYTHON="$PWD/.venv/bin/python"
