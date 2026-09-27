<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Ecosystem Comparison

Detailed performance comparison across validation backends.

| Metric | pure-Python (Standard) | pain001-fast (Rust) | Improvement |
| :--- | :--- | :--- | :--- |
| IBAN mod-97 | 1,850 ns | 22 ns | ~84x faster |
| BIC structural check | 1,200 ns | 14 ns | ~85x faster |
| Batch validation (10k tx) | 3,450 ms | 165 ms | ~21x faster |
| Memory allocations / tx | ~12 objects | 0 heap allocations | Zero allocation |
