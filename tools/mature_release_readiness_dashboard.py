#!/usr/bin/env python3
"""Generate a mature release readiness dashboard and release-train summary."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_VERSION = (ROOT / "VERSION").read_text(encoding="utf-8").strip()

PASS_STATUSES = {
    "ok",
    "pass",
    "passed",
    "ready",
    "reviewed",
    "complete",
    "completed",
}
FAIL_STATUSES = {
    "blocked",
    "broken",
    "error",
    "fail",
    "failed",
}


def path(rel: str) -> Path:
    return ROOT / rel


def text_exists(rel: str) -> bool:
    file = path(rel)
    return file.exists() and file.read_text(encoding="utf-8", errors="ignore").strip() != ""


def report_state(rel: str) -> tuple[str, str]:
    file = path(rel)
    if not file.exists():
        return "configured", rel

    if file.suffix.lower() == ".json":
        try:
            payload = json.loads(file.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            return "failed", f"{rel} is not valid JSON"

        status = str(payload.get("status") or "").strip().lower()
        if payload.get("passed") is False or status in FAIL_STATUSES:
            return "failed", f"status={status or 'failed'}"
        return "executed", f"status={status or 'present'}"

    contents = file.read_text(encoding="utf-8", errors="ignore")
    if "Status: `failed`" in contents or "Status: **failed**" in contents:
        return "failed", "status=failed"
    return "executed", "present"


def artifact_state(paths: list[str]) -> tuple[str, str]:
    existing = [rel for rel in paths if text_exists(rel)]
    if existing:
        return "executed", ", ".join(existing)
    return "manual", paths[0] if paths else "missing evidence"


def planned_roadmap_tracks() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    tracks_root = ROOT / "conductor" / "tracks"
    for track_dir in sorted(path for path in tracks_root.iterdir() if path.is_dir()):
        metadata_path = track_dir / "metadata.json"
        if not metadata_path.exists():
            continue
        try:
            metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        if metadata.get("status") != "planned_roadmap":
            continue
        rows.append(
            {
                "track_id": str(metadata.get("track_id") or track_dir.name),
                "capability": str(metadata.get("capability") or metadata.get("capability_id") or ""),
                "purpose": str(metadata.get("purpose") or metadata.get("title") or ""),
            }
        )
    return rows


def build_entries() -> list[dict[str, object]]:
    return [
        {
            "domain": "CI",
            "name": "M-140 dashboard artifact",
            "required": True,
            "kind": "generated_artifact",
            "paths": [
                "MATURE_RELEASE_READINESS_DASHBOARD.md",
                "MATURE_RELEASE_READINESS_DASHBOARD.json",
            ],
            "notes": "generated release-readiness evidence",
        },
        {
            "domain": "CI",
            "name": "Quality gate audit",
            "required": True,
            "kind": "report",
            "paths": [
                "QUALITY_GATE_CHECK.json",
                "QUALITY_GATE_CHECK.md",
            ],
            "notes": ".github/workflows/quality-gates.yml and the strict-quality workflow remain the configured gate surface.",
        },
        {
            "domain": "CI",
            "name": "Testing infrastructure report",
            "required": True,
            "kind": "report",
            "paths": [
                "TESTING_INFRASTRUCTURE_REPORT.json",
                "TESTING_INFRASTRUCTURE_REPORT.md",
            ],
            "notes": "documents the test matrix and the configured coverage/profiling scaffolds.",
        },
        {
            "domain": "CI",
            "name": "Current-environment self-test",
            "required": True,
            "kind": "report",
            "paths": [
                "CURRENT_ENV_SELFTEST.json",
                "CURRENT_ENV_SELFTEST.md",
            ],
            "notes": "confirms the local shell-side evidence bundle can still be generated.",
        },
        {
            "domain": "CI",
            "name": "Release-readiness workflow",
            "required": False,
            "kind": "config",
            "paths": [".github/workflows/release-readiness.yml"],
            "notes": "uses `schedule` and `workflow_dispatch` to refresh the dashboard and preflight bundle.",
        },
        {
            "domain": "Fixture",
            "name": "Real macOS APFS fixture",
            "required": True,
            "kind": "artifact",
            "paths": [
                "fixtures/real/macos-minimal-apfs-001/manifest.json",
                "fixtures/real/macos-minimal-apfs-001/file-hashes.sha256",
                "fixtures/real/macos-minimal-apfs-001/macos-oracle.redacted.txt",
            ],
            "notes": "executed macOS fixture evidence already captured in the repository.",
        },
        {
            "domain": "Fixture",
            "name": "Real fixture feedback promotion",
            "required": False,
            "kind": "artifact",
            "paths": [
                "target/real-fixture-feedback/real-fixture-feedback.json",
                "target/real-fixture-feedback/real-fixture-feedback.md",
                "conductor/archive/0122-real-fixture-feedback-promotion/review.md",
            ],
            "notes": "feedback promotion evidence exists, but the generated target artefacts are local-run outputs.",
        },
        {
            "domain": "Supply-chain",
            "name": "Dependency policy audit",
            "required": True,
            "kind": "report",
            "paths": [
                "DEPENDENCY_POLICY_AUDIT.json",
                "DEPENDENCY_POLICY_AUDIT.md",
            ],
            "notes": "covers dependency licensing and policy enforcement scaffolding.",
        },
        {
            "domain": "Supply-chain",
            "name": "cargo-vet policy audit",
            "required": True,
            "kind": "report",
            "paths": [
                "CARGO_VET_POLICY_AUDIT.json",
                "CARGO_VET_POLICY_AUDIT.md",
            ],
            "notes": "verifies the local cargo-vet governance scaffolding and imports lock.",
        },
        {
            "domain": "Supply-chain",
            "name": "Release automation audit",
            "required": True,
            "kind": "report",
            "paths": [
                "RELEASE_AUTOMATION_AUDIT.json",
                "RELEASE_AUTOMATION_AUDIT.md",
            ],
            "notes": "checks the cargo-dist and release-plz automation surface.",
        },
        {
            "domain": "Supply-chain",
            "name": "Bleeding-edge repo audit",
            "required": True,
            "kind": "report",
            "paths": [
                "BLEEDING_EDGE_REPO_AUDIT.json",
                "BLEEDING_EDGE_REPO_AUDIT.md",
            ],
            "notes": "aggregates the repo hardening audits, including this dashboard generator.",
        },
        {
            "domain": "Docs",
            "name": "Docs site check",
            "required": True,
            "kind": "report",
            "paths": [
                "DOCS_SITE_CHECK.json",
                "DOCS_SITE_CHECK.md",
            ],
            "notes": "checks the Astro docs site and its configured build path.",
        },
        {
            "domain": "Docs",
            "name": "Docs package audit",
            "required": True,
            "kind": "report",
            "paths": [
                "DOCS_PACKAGE_AUDIT.json",
                "DOCS_PACKAGE_AUDIT.md",
            ],
            "notes": "verifies the docs package surface and package metadata.",
        },
        {
            "domain": "Docs",
            "name": "Documentation index audit",
            "required": True,
            "kind": "report",
            "paths": [
                "DOCUMENTATION_INDEX_AUDIT.json",
                "DOCUMENTATION_INDEX_AUDIT.md",
            ],
            "notes": "keeps the root documentation map consistent.",
        },
        {
            "domain": "Security",
            "name": "Action pinning audit",
            "required": True,
            "kind": "report",
            "paths": [
                "ACTION_PINNING_AUDIT.json",
                "ACTION_PINNING_AUDIT.md",
            ],
            "notes": "confirms pinned GitHub Actions usage.",
        },
        {
            "domain": "Security",
            "name": "GitHub workflow policy audit",
            "required": True,
            "kind": "report",
            "paths": [
                "GITHUB_WORKFLOW_POLICY_AUDIT.json",
                "GITHUB_WORKFLOW_POLICY_AUDIT.md",
            ],
            "notes": "checks repository workflow policy invariants.",
        },
        {
            "domain": "Security",
            "name": "Scorecard dependency review audit",
            "required": True,
            "kind": "report",
            "paths": [
                "SCORECARD_DEPENDENCY_REVIEW_AUDIT.json",
                "SCORECARD_DEPENDENCY_REVIEW_AUDIT.md",
            ],
            "notes": "tracks dependency-review and OpenSSF scorecard readiness.",
        },
        {
            "domain": "Security",
            "name": "Provenance verification audit",
            "required": True,
            "kind": "report",
            "paths": [
                "PROVENANCE_VERIFICATION_AUDIT.json",
                "PROVENANCE_VERIFICATION_AUDIT.md",
            ],
            "notes": "documents attestation/provenance verification readiness.",
        },
        {
            "domain": "Release",
            "name": "Version consistency",
            "required": True,
            "kind": "report",
            "paths": [
                "VERSION_CONSISTENCY.json",
                "VERSION_CONSISTENCY.md",
            ],
            "notes": "keeps the dynamic versioning path aligned with the repo metadata.",
        },
        {
            "domain": "Release",
            "name": "Production gap report",
            "required": True,
            "kind": "report",
            "paths": [
                "PRODUCTION_GAP_REPORT.json",
                "PRODUCTION_GAP_REPORT.md",
            ],
            "notes": "summarizes the remaining production APFS gaps and local blockers.",
        },
        {
            "domain": "Release",
            "name": "Release-train workflow",
            "required": False,
            "kind": "config",
            "paths": [".github/workflows/release-readiness.yml"],
            "notes": "scheduled refresh path for the dashboard and preflight bundle.",
        },
    ]


def render_entry(entry: dict[str, object]) -> dict[str, str]:
    kind = str(entry["kind"])
    paths = [str(path) for path in entry["paths"]]  # type: ignore[index]
    if kind == "report":
        state = "configured"
        evidence = paths[0]
        for rel in paths:
            current_state, evidence = report_state(rel)
            if current_state == "failed":
                state = "failed"
                break
            if current_state == "executed":
                state = "executed"
                break
        else:
            state = "configured"
            evidence = paths[0]
    elif kind == "artifact":
        state, evidence = artifact_state(paths)
    elif kind == "generated_artifact":
        state = "executed"
        evidence = ", ".join(paths)
    else:
        state = "configured" if any(text_exists(rel) for rel in paths) else "configured"
        evidence = paths[0]

    return {
        "domain": str(entry["domain"]),
        "name": str(entry["name"]),
        "required": "yes" if entry["required"] else "no",
        "state": state,
        "evidence": evidence,
        "notes": str(entry.get("notes") or ""),
    }


def main() -> int:
    entries = [render_entry(entry) for entry in build_entries()]
    required_entries = [entry for entry in entries if entry["required"] == "yes"]
    counts: dict[str, int] = {"executed": 0, "configured": 0, "failed": 0, "manual": 0, "skipped": 0}
    for entry in required_entries:
        state = entry["state"]
        counts[state] = counts.get(state, 0) + 1
    roadmap_tracks = planned_roadmap_tracks()
    go_no_go = (
        "go"
        if counts["failed"] == 0
        and counts["configured"] == 0
        and counts["manual"] == 0
        and counts["skipped"] == 0
        and not roadmap_tracks
        else "no-go"
    )
    report = {
        "schema_version": SCHEMA_VERSION,
        "track": "M-140",
        "status": "executed_scaffold",
        "go_no_go": go_no_go,
        "required_summary": counts,
        "required_count": len(required_entries),
        "total_entry_count": len(entries),
        "entries": entries,
        "open_roadmap_tracks": roadmap_tracks,
        "release_train": [
            {
                "name": "Refresh evidence",
                "state": "configured",
                "evidence": ".github/workflows/release-readiness.yml",
                "notes": "`schedule` and `workflow_dispatch` keep the release-readiness dashboard fresh.",
            },
            {
                "name": "Run preflight",
                "state": "configured",
                "evidence": "cargo xtask release-preflight",
                "notes": "preflight should remain the downstream gate after the dashboard refresh.",
            },
            {
                "name": "Clear remaining roadmap tracks",
                "state": "blocked" if roadmap_tracks else "executed",
                "evidence": ", ".join(track["track_id"] for track in roadmap_tracks) if roadmap_tracks else "none",
                "notes": "release is still gated by the remaining roadmap work.",
            },
        ],
    }

    json_path = ROOT / "MATURE_RELEASE_READINESS_DASHBOARD.json"
    md_path = ROOT / "MATURE_RELEASE_READINESS_DASHBOARD.md"
    json_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    lines = [
        "# APFS-RS Mature Release Readiness Dashboard",
        "",
        "Generated by `tools/mature_release_readiness_dashboard.py`.",
        "",
        "Release slug: `mature-release-readiness-dashboard`.",
        "",
        f"Status: `{report['status']}`.",
        f"Go/no-go: `{go_no_go}`.",
        "",
        "## Required evidence summary",
        "",
        f"- Required entries: **{len(required_entries)}**",
        f"- Executed: **{counts['executed']}**",
        f"- Configured only: **{counts['configured']}**",
        f"- Failed: **{counts['failed']}**",
        f"- Manual: **{counts['manual']}**",
        f"- Skipped: **{counts['skipped']}**",
        "",
        "| Domain | Item | Required | State | Evidence | Notes |",
        "|---|---|---:|---|---|---|",
    ]
    for entry in entries:
        lines.append(
            f"| {entry['domain']} | {entry['name']} | {entry['required']} | {entry['state']} | `{entry['evidence']}` | {entry['notes']} |"
        )

    lines += [
        "",
        "## Release train",
        "",
    ]
    for step in report["release_train"]:
        lines.append(
            f"- **{step['name']}**: `{step['state']}` - {step['evidence']} ({step['notes']})"
        )

    lines += [
        "",
        "## Open roadmap tracks",
        "",
    ]
    if roadmap_tracks:
        for row in roadmap_tracks:
            lines.append(
                f"- `{row['track_id']}` ({row['capability']}): {row['purpose']}"
            )
    else:
        lines.append("- None")

    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(
        "mature-release-readiness-dashboard: wrote "
        f"{json_path.name} and {md_path.name}; go/no-go={go_no_go}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
