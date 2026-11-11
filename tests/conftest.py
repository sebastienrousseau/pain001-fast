# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Shared fixtures: the package loaded with its extension blocked."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from types import ModuleType
from typing import Any

import pain001_fast
import pytest

INIT = Path(pain001_fast.__file__)


@pytest.fixture(scope="session")
def fallback() -> Any:
    """Load the package's ``__init__`` with the extension unimportable."""
    blocked = "pain001_fast._pain001_fast"
    saved = sys.modules.get(blocked)
    sys.modules[blocked] = None  # type: ignore[assignment]
    try:
        spec = importlib.util.spec_from_file_location("_fallback", INIT)
        assert spec is not None and spec.loader is not None
        module: ModuleType = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
    finally:
        if saved is None:
            del sys.modules[blocked]
        else:
            sys.modules[blocked] = saved
    assert module.validate_iban is not pain001_fast.validate_iban
    return module
