# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

.PHONY: all build check test coverage-rust lint type versions sec format clean

all: build test

build:
	uv sync --extra dev --locked

check: lint type test coverage-rust versions

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

format:
	cargo fmt
	ruff format python scripts tests examples

clean:
	cargo clean
	rm -rf target/ dist/ build/ *.so *.dylib *.egg-info
