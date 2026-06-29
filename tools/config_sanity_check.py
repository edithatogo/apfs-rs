#!/usr/bin/env python3
"""Cargoless sanity checks for local-handoff tool/config files."""
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FILES = [
    "rust-toolchain.toml",
    ".cargo/config.toml",
    "deny.toml",
    ".config/nextest.toml",
    ".config/cargo-llvm-cov.toml",
    ".devcontainer/devcontainer.json",
    ".devcontainer/Dockerfile",
    ".pre-commit-config.yaml",
    ".markdownlint.yaml",
    "_typos.toml",
    "taplo.toml",
    ".editorconfig",
    "renovate.json",
    ".github/workflows/local-handoff.yml",
]


def fail(message: str) -> None:
    print(f"config-sanity-check: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_toml(path: str) -> dict:
    return tomllib.loads((ROOT / path).read_text(encoding="utf-8"))


def main() -> int:
    missing = [path for path in REQUIRED_FILES if not (ROOT / path).exists()]
    if missing:
        fail("missing required config files: " + ", ".join(missing))

    toolchain = read_toml("rust-toolchain.toml")
    channel = toolchain.get("toolchain", {}).get("channel", "")
    if not channel:
        fail("rust-toolchain.toml has no toolchain.channel")

    cargo_config = read_toml(".cargo/config.toml")
    rustflags = cargo_config.get("build", {}).get("rustflags", [])
    if "warnings" not in " ".join(rustflags):
        fail(".cargo/config.toml should deny or surface warnings")

    deny = read_toml("deny.toml")
    allowed = deny.get("licenses", {}).get("allow", [])
    for license_name in ["MIT", "Apache-2.0"]:
        if license_name not in allowed:
            fail(f"deny.toml does not allow expected license {license_name}")

    devcontainer = json.loads((ROOT / ".devcontainer/devcontainer.json").read_text(encoding="utf-8"))
    if "APFS-RS" not in devcontainer.get("name", ""):
        fail("devcontainer name should identify APFS-RS")

    print("config-sanity-check: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
