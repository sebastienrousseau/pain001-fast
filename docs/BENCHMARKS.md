<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Benchmarks

No benchmark in this repository measures `pain001-fast` yet, so this
page publishes no figures.

Earlier releases stated per-call times (about 22 ns per IBAN, 14 ns per
BIC, 115 ns to sanitise 50 characters) and batch throughput (over
60,000 records per second, p99 under 45 ms) measured on Apple Silicon.
No benchmark that produced them was ever committed, so they could not
be reproduced or checked, and they are withdrawn.

## What a figure needs before it returns here

- A benchmark in the repository: `criterion` (already a dev-dependency)
  under `benches/` for the Rust functions, and `pytest-benchmark`
  (already in the `dev` extra) for the bindings and
  `validate_payment_batch` as Python sees them.
- The command that reproduces it, the machine, the toolchain and the
  build profile, next to the number.
- A CI job that keeps the benchmark compiling and running, without
  asserting on timings.

`tests/test_fast.py::test_batch_throughput_benchmark` is a guard, not a
benchmark: it fails if 5,000 records take longer than half a second.
