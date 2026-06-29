#!/usr/bin/env python3
"""Create a real-fixture feedback report from apfs inspect JSON and a fixture manifest.

This tool is intentionally shallow and read-only. It does not open APFS images. It compares
already-generated JSON artifacts and produces task-oriented feedback for Codev/Conductor.
"""
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any


@dataclass
class FeedbackIssue:
    severity: str
    field: str
    message: str
    suggested_track: str


@dataclass
class FeedbackReport:
    schema_version: str
    status: str
    inspect_json: str
    manifest_json: str
    fixture_id: str | None
    matched_fields: list[str]
    issues: list[FeedbackIssue]
    next_recommended_tracks: list[str]
    safety_note: str


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise SystemExit(f"{path} must contain a JSON object")
    return value


def nested_get(value: dict[str, Any], path: str) -> Any:
    current: Any = value
    for part in path.split("."):
        if not isinstance(current, dict) or part not in current:
            return None
        current = current[part]
    return current


def compare(inspect: dict[str, Any], manifest: dict[str, Any], inspect_path: Path, manifest_path: Path) -> FeedbackReport:
    fixture_id = manifest.get("fixture_id") if isinstance(manifest.get("fixture_id"), str) else None
    matched: list[str] = []
    issues: list[FeedbackIssue] = []

    redaction = manifest.get("redaction") if isinstance(manifest.get("redaction"), dict) else {}
    if redaction.get("contains_personal_data") is not False:
        issues.append(FeedbackIssue(
            severity="blocker",
            field="manifest.redaction.contains_personal_data",
            message="manifest must explicitly state that it contains no personal data",
            suggested_track="0012-real-fixture-feedback-loop",
        ))
    if redaction.get("contains_secret_material") is not False:
        issues.append(FeedbackIssue(
            severity="blocker",
            field="manifest.redaction.contains_secret_material",
            message="manifest must explicitly state that it contains no secret material",
            suggested_track="0012-real-fixture-feedback-loop",
        ))

    # Generic inspect contract checks.
    for field in ["schema_version", "status", "safety"]:
        if inspect.get(field) is None:
            issues.append(FeedbackIssue("error", field, "inspect JSON is missing required field", "0012-real-fixture-feedback-loop"))
        else:
            matched.append(field)

    if nested_get(inspect, "safety.read_only") is not True:
        issues.append(FeedbackIssue(
            severity="blocker",
            field="inspect.safety.read_only",
            message="inspect output must report read_only=true for this feedback loop",
            suggested_track="0012-real-fixture-feedback-loop",
        ))

    # Expected APFS features from the manifest.
    features = manifest.get("apfs_features") if isinstance(manifest.get("apfs_features"), dict) else {}
    if features.get("encrypted") is True and inspect.get("status") == "apfs_container_detected":
        issues.append(FeedbackIssue(
            severity="warning",
            field="apfs_features.encrypted",
            message="encrypted fixture detected; current implementation should not claim encrypted read support",
            suggested_track="future-software-encryption-read",
        ))

    # Optional expected_fields section allows manifests to specify concrete expected inspect paths.
    expected_fields = manifest.get("expected_inspect_fields")
    if isinstance(expected_fields, dict):
        for path, expected in expected_fields.items():
            observed = nested_get(inspect, path)
            if observed == expected:
                matched.append(path)
            else:
                issues.append(FeedbackIssue(
                    severity="error",
                    field=path,
                    message=f"expected {expected!r}, observed {observed!r}",
                    suggested_track=suggest_track_for_field(path),
                ))
    else:
        issues.append(FeedbackIssue(
            severity="info",
            field="manifest.expected_inspect_fields",
            message="manifest has no expected_inspect_fields section; feedback is limited to generic contract checks",
            suggested_track="0012-real-fixture-feedback-loop",
        ))

    status = "blocked" if any(issue.severity == "blocker" for issue in issues) else "needs_tasks" if issues else "matched"
    next_tracks = sorted({issue.suggested_track for issue in issues if issue.severity in {"blocker", "error", "warning"}})
    return FeedbackReport(
        schema_version="0.13.0",
        status=status,
        inspect_json=str(inspect_path),
        manifest_json=str(manifest_path),
        fixture_id=fixture_id,
        matched_fields=sorted(set(matched)),
        issues=issues,
        next_recommended_tracks=next_tracks,
        safety_note="This feedback report compares JSON artifacts only. It does not open, mount, decrypt, repair, format, or write APFS media.",
    )


def suggest_track_for_field(path: str) -> str:
    if path.startswith("container") or "block" in path:
        return "0001-m001-container-inspect"
    if path.startswith("gpt"):
        return "0002-m002-gpt-apfs-probe"
    if "checkpoint" in path:
        return "0004-m004-checkpoint-scan"
    if "omap" in path or "resolver" in path:
        return "0009b-m009-object-map-resolver"
    if "btree" in path or "cursor" in path:
        return "0010-btree-cursor"
    return "0012-real-fixture-feedback-loop"


def write_markdown(report: FeedbackReport, path: Path) -> None:
    lines = [
        "# APFS-RS Real Fixture Feedback Task Packet",
        "",
        f"Schema version: {report.schema_version}",
        f"Status: {report.status}",
        f"Fixture: {report.fixture_id or 'unknown'}",
        "",
        "## Safety note",
        "",
        report.safety_note,
        "",
        "## Matched fields",
        "",
    ]
    if report.matched_fields:
        lines.extend(f"- `{field}`" for field in report.matched_fields)
    else:
        lines.append("- None")
    lines.extend(["", "## Issues", ""])
    if report.issues:
        for issue in report.issues:
            lines.extend([
                f"### {issue.severity.upper()}: `{issue.field}`",
                "",
                issue.message,
                "",
                f"Suggested track: `{issue.suggested_track}`",
                "",
            ])
    else:
        lines.append("No issues found by the shallow feedback loop.")
    lines.extend(["", "## Next recommended tracks", ""])
    if report.next_recommended_tracks:
        lines.extend(f"- `{track}`" for track in report.next_recommended_tracks)
    else:
        lines.append("- None")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inspect_json", type=Path)
    parser.add_argument("manifest_json", type=Path)
    parser.add_argument("out_dir", type=Path)
    args = parser.parse_args()

    inspect = load_json(args.inspect_json)
    manifest = load_json(args.manifest_json)
    report = compare(inspect, manifest, args.inspect_json, args.manifest_json)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    report_json = args.out_dir / "real-fixture-feedback.json"
    report_md = args.out_dir / "real-fixture-feedback.md"
    report_json.write_text(json.dumps(asdict(report), indent=2, sort_keys=True) + "\n", encoding="utf-8")
    write_markdown(report, report_md)
    print(f"wrote {report_json}")
    print(f"wrote {report_md}")
    return 2 if report.status == "blocked" else 0


if __name__ == "__main__":
    raise SystemExit(main())
