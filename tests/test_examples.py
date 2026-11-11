# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Run every numbered example; each asserts its own output."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

import pytest

EXAMPLES = sorted(
    (Path(__file__).resolve().parents[1] / "examples").glob("[0-9]*.py")
)


def test_examples_exist() -> None:
    assert EXAMPLES


@pytest.mark.parametrize("script", EXAMPLES, ids=lambda path: path.name)
def test_example_runs(script: Path) -> None:
    subprocess.run([sys.executable, str(script)], check=True, timeout=60)
