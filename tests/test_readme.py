# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Keep the README canonical and its runnable claims true.

The README follows the portfolio's canonical layout. These tests hold
its ``##`` sections to that order, refuse unresolved ``{{TOKEN}}``
placeholders and repository links to missing files, and run the
Python Quick Start, so a change to a binding fails here rather than in
a reader's terminal.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = (ROOT / "README.md").read_text(encoding="utf-8")

# The second-level headings of ~/Code/README-TEMPLATE.md, in order.
CANONICAL_SECTIONS = [
    "Contents",
    "Install",
    "Requirements",
    "Quick Start",
    "The pain001-fast ecosystem",
    "Capabilities at a glance",
    "Ecosystem comparison",
    "Benchmarks",
    "Features",
    "Configuration",
    "Examples",
    "When not to use pain001-fast",
    "Development",
    "Security",
    "Documentation",
    "Stability guarantees",
    "License",
]


def _section(title: str) -> str:
    """Return the body of the ``## title`` section of the README."""
    return README.split(f"\n## {title}\n", 1)[1].split("\n---", 1)[0]


def test_sections_follow_the_canonical_order() -> None:
    headings = re.findall(r"^## (.+)$", README, re.MULTILINE)
    assert headings == CANONICAL_SECTIONS


def test_no_unresolved_template_tokens() -> None:
    assert not re.findall(r"\{\{[A-Z0-9_]+\}\}", README)


def test_repository_links_resolve() -> None:
    targets = re.findall(r'(?:\]\(|href=")([^)"#]+)', README)
    local = [t for t in targets if not re.match(r"[a-z]+:", t)]
    assert local
    missing = [t for t in local if not (ROOT / t).exists()]
    assert not missing


def test_quick_start_runs() -> None:
    block = re.search(
        r"```python\n(.*?)\n```", _section("Quick Start"), re.DOTALL
    )
    assert block is not None, "Quick Start has no python block"
    subprocess.run([sys.executable, "-c", block[1]], check=True, timeout=60)
