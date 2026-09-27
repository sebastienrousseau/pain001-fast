<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Benchmarks

Benchmarks measured on Apple Silicon M-series (10 cores, 16GB RAM) using `criterion` and `pytest-benchmark`.

## Micro-benchmarks

- `validate_iban_detailed_fast`: 21.8 ns / op
- `validate_bic_detailed_fast`: 13.9 ns / op
- `sanitize_to_charset_fast`: 115 ns / op (50-character string)

## Batch Benchmarks

10,000 mixed SEPA/FedNow payment orders evaluated in parallel:
- Total throughput: > 60,000 transactions/second
- p99 Latency: < 45 ms
