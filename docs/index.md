<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# pain001-fast Documentation

`pain001-fast` is a native Rust acceleration core for the `pain001` ecosystem.

## Modules

- **IBAN Validation**: `validate_iban`, `validate_iban_detailed`
- **BIC Validation**: `validate_bic`, `validate_bic_detailed`
- **Charset Sanitization**: `is_valid_charset`, `sanitize_to_charset`, `find_invalid_characters`
- **Batch Processing**: `validate_payment_batch`

## Further reading

- [Architecture](ARCHITECTURE.md): the layers, the modules and their
  contracts
- [Policies](POLICIES.md): the minimum toolchain
- [Benchmarks](BENCHMARKS.md) and [comparison](COMPARISON.md)
- [Development](../DEVELOPMENT.md): toolchain and every CI gate
