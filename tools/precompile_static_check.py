#!/usr/bin/env python3
"""Pre-compile static checks for APFS-RS.

This intentionally does not replace `cargo check` or `cargo test`. It catches
repository-shape, context, fixture, and obvious Rust-source hazards in
sandboxed environments where Rust/Cargo is unavailable.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from pathlib import Path

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None

ROOT = Path(__file__).resolve().parents[1]

FORBIDDEN_RUST_TERMS = [
    "GENERIC_WRITE",
    "FILE_WRITE_DATA",
    "raw_write",
    "write_at_device",
    "exclusive_write",
    "CreateFileW",
]

REQUIRED_CONDUCTOR_TRACKS: list[str] = []


def fail(message: str) -> None:
    print(f"precompile-static-check: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_yaml(path: Path):
    if yaml is None:
        fail("PyYAML is not installed; cannot parse YAML registries")
    with path.open("r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def check_cargo_workspace() -> None:
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    members = manifest.get("workspace", {}).get("members", [])
    if not members:
        fail("Cargo workspace has no members")
    for member in members:
        member_path = ROOT / member
        if not (member_path / "Cargo.toml").exists():
            fail(f"workspace member {member} has no Cargo.toml")
        if member != "xtask" and not (member_path / "src").exists():
            fail(f"workspace member {member} has no src directory")
    print(f"precompile-static-check: workspace members ok ({len(members)})")


def strip_rust_comments(text: str) -> str:
    # Simple conservative stripping; enough for duplicate-symbol and forbidden-term checks.
    text = re.sub(r"//.*", "", text)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return text


def strip_rust_strings(text: str) -> str:
    # Replace normal quoted strings and character literals with empty placeholders.
    # This intentionally does not attempt to fully parse Rust raw strings; it only
    # avoids common false positives in brace counting for ordinary strings.
    text = re.sub(r'"(?:\\.|[^"\\])*"', '""', text)
    text = re.sub(r"'(?:\\.|[^'\\])'", "''", text)
    return text


def check_rust_sources() -> None:
    for path in sorted((ROOT / "crates").rglob("*.rs")) + sorted((ROOT / "xtask").rglob("*.rs")):
        text = path.read_text(encoding="utf-8")
        stripped = strip_rust_comments(text)
        shape_text = strip_rust_strings(stripped)
        if shape_text.count("{") != shape_text.count("}"):
            fail(f"unbalanced braces in {path.relative_to(ROOT)}")
        if "#![forbid(unsafe_code)]" not in text and "src/main.rs" not in path.as_posix():
            fail(f"missing #![forbid(unsafe_code)] in {path.relative_to(ROOT)}")
        unsafe_hits = [m.start() for m in re.finditer(r"\bunsafe\b", stripped)]
        if unsafe_hits and "forbid(unsafe_code)" not in stripped:
            fail(f"unsafe-looking token in {path.relative_to(ROOT)}")
        symbols: dict[str, int] = {}
        for match in re.finditer(r"^(?:pub\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", stripped, flags=re.M):
            name = match.group(1)
            symbols[name] = symbols.get(name, 0) + 1
        duplicates = sorted(name for name, count in symbols.items() if count > 1)
        if duplicates:
            fail(f"duplicate function names in {path.relative_to(ROOT)}: {', '.join(duplicates)}")
        forbidden_scan_text = strip_rust_strings(stripped)
        for term in FORBIDDEN_RUST_TERMS:
            if term in forbidden_scan_text:
                fail(f"forbidden raw-write term `{term}` in {path.relative_to(ROOT)}")
    print("precompile-static-check: Rust source shape ok")


def check_registries() -> None:
    capabilities = load_yaml(ROOT / "codev/resources/capabilities.yaml")
    gates = load_yaml(ROOT / "codev/resources/safety-gates.yaml")
    cap_map = capabilities.get("capabilities", {})
    gate_map = gates.get("gates", {})
    if not cap_map:
        fail("capabilities.yaml contains no capabilities")
    for cap_id, cap in cap_map.items():
        safety_gates = cap.get("safety_gates")
        if not isinstance(safety_gates, list) or not safety_gates:
            fail(f"capability {cap_id} has no safety_gates")
        for gate in safety_gates:
            if gate not in gate_map:
                fail(f"capability {cap_id} references undefined safety gate {gate}")
    print(f"precompile-static-check: registry links ok ({len(cap_map)} capabilities)")


def check_json_files() -> None:
    count = 0
    for path in sorted(ROOT.rglob("*.json")):
        if any(part in {"target", "node_modules", "dist"} for part in path.parts):
            continue
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except Exception as exc:
            fail(f"invalid JSON in {path.relative_to(ROOT)}: {exc}")
        count += 1
    print(f"precompile-static-check: JSON files parse ok ({count})")


def check_conductor_tracks() -> None:
    for root_file in [
        "conductor/product.md",
        "conductor/product-guidelines.md",
        "conductor/tech-stack.md",
        "conductor/workflow.md",
        "conductor/tracks.md",
        "conductor/history.md",
    ]:
        path = ROOT / root_file
        if not path.exists() or not path.read_text(encoding="utf-8").strip():
            fail(f"missing or empty Conductor root file {root_file}")
    track_dirs = sorted(path.name for path in (ROOT / "conductor/tracks").iterdir() if path.is_dir())
    if not track_dirs:
        fail("no Conductor track directories found")
    for track_id in track_dirs:
        base = ROOT / "conductor/tracks" / track_id
        for name in ["metadata.json", "spec.md", "plan.md", "review.md"]:
            path = base / name
            if not path.exists() or not path.read_text(encoding="utf-8").strip():
                fail(f"missing or empty Conductor file {path.relative_to(ROOT)}")
        metadata = json.loads((base / "metadata.json").read_text(encoding="utf-8"))
        if metadata.get("track_id") != track_id:
            fail(f"Conductor track_id mismatch in {base.relative_to(ROOT)}")
        if metadata.get("review_status") != "reviewed":
            fail(f"Conductor track {track_id} is not reviewed")
        if metadata.get("archive_status") != "archived" or metadata.get("archived") is not True:
            fail(f"Conductor track {track_id} is not archived")
        if "## Archive closeout" not in (base / "review.md").read_text(encoding="utf-8"):
            fail(f"Conductor track {track_id} review.md missing Archive closeout")
    tracks_text = (ROOT / "conductor/tracks.md").read_text(encoding="utf-8")
    missing_in_index = [track for track in track_dirs if track not in tracks_text]
    if missing_in_index:
        fail(f"Conductor tracks missing from conductor/tracks.md: {missing_in_index}")
    print(f"precompile-static-check: Conductor tracks ok ({len(track_dirs)})")


def check_fixture_manifests() -> None:
    count = 0
    for path in sorted((ROOT / "fixtures/manifests").glob("*.json")):
        manifest = json.loads(path.read_text(encoding="utf-8"))
        for field in ["schema_version", "fixture_id", "source_type", "apfs_features", "capability_ids", "redaction"]:
            if field not in manifest:
                fail(f"fixture manifest {path.name} missing {field}")
        redaction = manifest.get("redaction", {})
        if redaction.get("contains_personal_data") is not False:
            fail(f"fixture manifest {path.name} is not explicitly personal-data-free")
        if redaction.get("contains_secret_material") is not False:
            fail(f"fixture manifest {path.name} is not explicitly secret-free")
        count += 1
    print(f"precompile-static-check: fixture manifests ok ({count})")


def check_cli_contract() -> None:
    text = (ROOT / "crates/apfs-cli/src/main.rs").read_text(encoding="utf-8")
    expected_variants = [
        "Inspect",
        "CompatibilityReport",
        "Doctor",
        "DiagnosticsExport",
        "LookupObject",
        "Volumes",
        "ResolverReport",
        "BtreeCursorReport",
        "ReadObject",
        "Ls",
        "Cat",
        "Stat",
        "Extract",
    ]
    for name in expected_variants:
        if f"{name} {{" not in text and f"{name}," not in text:
            fail(f"CLI command variant {name} is missing from clap enum")
    print("precompile-static-check: CLI contract spot-check ok")


def check_tooling_files() -> None:
    required = [
        "tools/diagnostics_bundle.py",
        "tools/rust_api_map.py",
        "tools/next_loop_plan.py",
        "tools/windows_readiness_check.py",
        "tools/cargo_error_to_tracks.py",
        "tools/handoff_readiness_check.py",
        "tools/release_scaffold_check.py",
        "tools/cli_contract_snapshot.py",
        "tools/api_surface_snapshot.py",
        "tools/source_metrics.py",
        "tools/safety_case_check.py",
        "fuzz/Cargo.toml",
        "fuzz/fuzz_targets/object_header.rs",
        "fuzz/fuzz_targets/nx_superblock.rs",
    ]
    for rel in required:
        path = ROOT / rel
        if not path.exists() or not path.read_text(encoding="utf-8").strip():
            fail(f"missing required tooling file {rel}")
    print("precompile-static-check: v0.18 tooling files ok")


def main() -> int:
    global ROOT
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    ROOT = args.root.resolve()
    check_cargo_workspace()
    check_rust_sources()
    check_registries()
    check_json_files()
    check_conductor_tracks()
    check_fixture_manifests()
    check_cli_contract()
    check_tooling_files()
    print("precompile-static-check: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
