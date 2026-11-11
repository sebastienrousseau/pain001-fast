# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

.PHONY: all build check test coverage-rust fuzz demo lint doc docs-lint type versions sec format clean

all: build test

build:
	uv sync --extra dev --locked

check: lint doc type test coverage-rust versions

type:
	mypy python/pain001_fast

versions:
	python3 scripts/check_release.py
	python3 scripts/release_notes.py check

sec:
	cargo audit

# The Rust tests link libpython (pyo3 without extension-module), so the
# test binary needs an rpath to the library directory of the Python that
# pyo3 builds against. PYO3_PYTHON names that interpreter explicitly:
# pyo3 caches the one it found on the first build and re-detects only
# when PYO3_PYTHON changes, so with PATH alone a venv on a different
# Python made the rpath point at one libpython and the binary link
# another ("Library not loaded: @rpath/libpython3.12.dylib").
PYTHON ?= python3
PYO3_PYTHON := $(shell command -v $(PYTHON))
PYTHON_LIBDIR := $(shell $(PYO3_PYTHON) -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')

test:
	PYO3_PYTHON="$(PYO3_PYTHON)" RUSTFLAGS="-C link-arg=-Wl,-rpath,$(PYTHON_LIBDIR)" cargo test --no-default-features --locked
	pytest tests/ -v

# Rust line and function coverage of src/, at 100%. Needs cargo-llvm-cov
# (`cargo install cargo-llvm-cov --locked`) and the llvm-tools-preview
# component. Branch coverage needs a nightly compiler, so lines and
# functions are the floor; see DEVELOPMENT.md.
coverage-rust:
	PYO3_PYTHON="$(PYO3_PYTHON)" RUSTFLAGS="-C link-arg=-Wl,-rpath,$(PYTHON_LIBDIR)" cargo llvm-cov --no-default-features --locked --fail-under-lines 100 --fail-under-functions 100

lint:
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check
	ruff check .
	ruff format --check python scripts tests examples

# Rust API docs with every rustdoc warning denied; `#![warn(missing_docs)]`
# in src/lib.rs makes an undocumented public item fail clippy too.
doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features --locked

# Spelling (codespell, from requirements/docs-lint.txt) and Markdown
# style (markdownlint-cli2, needs Node); the docs-lint.yml jobs.
docs-lint:
	codespell
	npx --yes markdownlint-cli2@0.23.3 "**/*.md"

# Fuzz the validators for FUZZ_SECONDS (default 60) with cargo-fuzz on a
# nightly toolchain. New inputs go to a scratch directory; the committed
# seeds in fuzz/corpus/validators are only read. `make test` replays the
# committed corpus on stable.
FUZZ_SECONDS ?= 60
fuzz:
	mkdir -p fuzz/target/corpus
	cd fuzz && PYO3_PYTHON="$(PYO3_PYTHON)" RUSTFLAGS="-C link-arg=-Wl,-rpath,$(PYTHON_LIBDIR)" cargo +nightly fuzz run validators target/corpus corpus/validators -- -max_total_time=$(FUZZ_SECONDS)

# Re-render .github/demo.gif from .github/demo.tape (needs vhs, ttyd and
# ffmpeg), using the extension built into .venv.
demo:
	PATH="$(CURDIR)/.venv/bin:$$PATH" vhs .github/demo.tape

format:
	cargo fmt
	ruff format python scripts tests examples

clean:
	cargo clean
	rm -rf target/ dist/ build/ *.so *.dylib *.egg-info
