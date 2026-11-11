# SPDX-License-Identifier: Apache-2.0 OR MIT
"""Verify that the native release has one version in every package format."""

import os
import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Refuse a tag or package whose version differs from Cargo's source."""
    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]
    version = cargo["version"]
    project = tomllib.loads((ROOT / "pyproject.toml").read_text())
    assert project["project"]["version"] == version
    for name in ("Cargo.lock", "uv.lock"):
        packages = tomllib.loads((ROOT / name).read_text())["package"]
        own = next(p for p in packages if p["name"] == "pain001-fast")
        assert own["version"] == version, name
    module = (ROOT / "python/pain001_fast/__init__.py").read_text()
    assert re.search(
        rf'^__version__ = "{re.escape(version)}"$', module, re.MULTILINE
    )
    assert f"## [{version}] - " in (ROOT / "CHANGELOG.md").read_text()
    readme = (ROOT / "README.md").read_text()
    assert f"Version {version}, in lockstep" in readme
    # The install snippet and the release link name the current tag.
    assert f"pain001-fast.git@v{version}" in readme
    assert f"releases/tag/v{version}" in readme
    assert (ROOT / "releases" / f"v{version}.md").is_file()
    if os.environ.get("GITHUB_REF_TYPE") == "tag":
        assert os.environ["GITHUB_REF_NAME"] == f"v{version}"
    print(f"Rust, Python, lockfiles and documentation agree on {version}")


if __name__ == "__main__":
    main()
