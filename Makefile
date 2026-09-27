# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

.PHONY: all build check test lint format clean

all: build test

build:
	uv tool run maturin develop --release

test:
	RUSTFLAGS="-C link-arg=-Wl,-rpath,/Users/seb/.local/share/mise/installs/python/3.12.14/lib" cargo test --no-default-features
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
