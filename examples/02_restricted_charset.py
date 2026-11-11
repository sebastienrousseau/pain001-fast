# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Find and replace characters outside the ISO 20022 restricted set.

The set is ASCII letters and digits, space, and ``/-?:().,'+``. Run
with ``python examples/02_restricted_charset.py``.
"""

from pain001_fast import (
    find_invalid_characters,
    is_valid_charset,
    sanitize_to_charset,
)

name = "Müller & Söhne GmbH"
print(f"{name!r} is valid: {is_valid_charset(name)}")
assert not is_valid_charset(name)

# Each offending character once, in code-point order.
assert find_invalid_characters(name) == ["&", "ö", "ü"]

# Accents are stripped; anything else becomes the replacement.
clean = sanitize_to_charset(name, replacement="+")
print(f"sanitised: {clean!r}")
assert clean == "Muller + Sohne GmbH"
assert is_valid_charset(clean)
