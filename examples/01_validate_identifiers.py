# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Check IBANs and BICs, and read why one is refused.

Run with ``python examples/01_validate_identifiers.py``. The script
asserts what it prints, so the test suite runs it as a check.
"""

from pain001_fast import (
    validate_bic,
    validate_bic_detailed,
    validate_iban,
    validate_iban_detailed,
)

# Spaces and hyphens are ignored and letters are uppercased.
assert validate_iban("de89 3704-0044 0532 0130 00")
assert validate_bic("DEUTDEFF500")

# The detailed forms return (is_valid, reason); the reason is empty
# when the code is valid.
ok, reason = validate_iban_detailed("DE00370400440532013000")
assert not ok
assert reason == "IBAN checksum validation failed (mod 97 = 9, expected 1)"
print("An IBAN with a wrong check digit is refused with its mod 97 reason.")

ok, reason = validate_bic_detailed("DEUT99FF")
print(f"BIC with digits in the country code: {reason}")
assert not ok
assert reason == (
    "BIC format invalid: country code (chars 5-6) must be letters (got '99')"
)
