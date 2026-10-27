# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

.PHONY: all build check test lint format clean

all: build test

build:
	uv tool run maturin develop --release

# The Rust tests link libpython (pyo3 without extension-module), so the
# test binary needs an rpath to the library directory of the Python that
# pyo3 builds against: the first python3 on PATH.
PYTHON_LIBDIR := $(shell python3 -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')

test:
	RUSTFLAGS="-C link-arg=-Wl,-rpath,$(PYTHON_LIBDIR)" cargo test --no-default-features --locked
	pytest tests/ -v

lint:
	cargo clippy --all-targets -- -D warnings
	cargo fmt --check
	ruff check .

format:
	cargo fmt
	ruff format .

clean:
	cargo clean
	rm -rf target/ dist/ build/ *.so *.dylib *.egg-info
