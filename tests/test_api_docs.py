# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Every public name has a docstring, from Rust or from the fallback.

The extension's docstrings are the Rust doc comments, which clippy's
``missing_docs`` lint and ``make doc`` already require; this checks
they reach Python, and that the fallback documents the same names.
"""

from __future__ import annotations

from typing import Any

import pain001_fast
import pytest

FUNCTIONS = [name for name in pain001_fast.__all__ if name != "__version__"]


@pytest.mark.parametrize("name", FUNCTIONS)
def test_extension_function_is_documented(name: str) -> None:
    assert (getattr(pain001_fast, name).__doc__ or "").strip()


@pytest.mark.parametrize("name", FUNCTIONS)
def test_fallback_function_is_documented(fallback: Any, name: str) -> None:
    assert (getattr(fallback, name).__doc__ or "").strip()
