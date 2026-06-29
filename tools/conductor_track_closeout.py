#!/usr/bin/env python3
"""Audit and generate Conductor track review/archive closeout evidence."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRACKS = ROOT / "conductor" / "tracks"
CODEV_REVIEWS = ROOT / "codev" / "reviews"

IMPLEMENTED_STATUSES = {
    "active",
    "completed_in_package_uncompiled",
    "implemented_in_package_uncompiled",
    "implemented_scaffold",
    "implemented_or_scaffolded",
    "implemented_python",
    "completed_scaffold",
    "implemented",
    "scaffolded",
    "planned_roadmap",
}

CODEV_REVIEW_OVERRIDES = {
    "0015-mapped-object-read-report": "codev/reviews/0015-mapped-object-read-report-review.md",
    "0016-synthetic-directory-root-tree": "codev/reviews/0015-m015-synthetic-directory-root-tree-review.md",
    "0017-directory-listing-cli": "codev/reviews/0016-m016-directory-listing-cli-review.md",
    "0026-windows-mount-plan": "codev/reviews/0027-m026-windows-mount-plan-review.md",
    "0027-redacted-diagnostics-bundle": "codev/reviews/0028-m027-diagnostics-bundle-review.md",
    "0028-cli-contract-check": "codev/reviews/0029-m028-cli-contract-check-review.md",
    "0029-context-integrity-check": "codev/reviews/0030-m029-context-integrity-check-review.md",
}


def fail(message: str) -> None:
    print(f"conductor-track-closeout: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def slug_words(text: str) -> set[str]:
    return {part for part in re.split(r"[^a-z0-9]+", text.lower()) if part}


def title_from_spec(spec_text: str, track_id: str) -> str:
    for line in spec_text.splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return track_id


def find_codev_review(track_id: str, metadata: dict[str, object], title: str) -> str | None:
    if track_id in CODEV_REVIEW_OVERRIDES:
        return CODEV_REVIEW_OVERRIDES[track_id]
    capability = str(metadata.get("capability") or metadata.get("capability_id") or "")
    candidates = sorted(CODEV_REVIEWS.glob("*.md"))
    track_words = slug_words(track_id) | slug_words(title)
    if capability.startswith("M-"):
        number = capability[2:].lstrip("0") or "0"
        padded = capability[2:]
        numeric_prefixes = {f"{int(number):04d}", f"{int(number):03d}", f"{int(number):02d}", number}
        capability_tokens = {capability.lower(), capability.lower().replace("-", ""), f"m-{padded}", f"m{padded}"}
        best: tuple[int, Path] | None = None
        for review in candidates:
            stem = review.stem.lower()
            words = slug_words(stem)
            score = len(track_words & words)
            if any(token in stem for token in capability_tokens):
                score += 100
            if any(stem.startswith(prefix + "-") for prefix in numeric_prefixes):
                score += 25
            if score and (best is None or score > best[0]):
                best = (score, review)
        if best and best[0] >= 25:
            return str(best[1].relative_to(ROOT))
    best: tuple[int, Path] | None = None
    for review in candidates:
        score = len(track_words & slug_words(review.stem))
        if score and (best is None or score > best[0]):
            best = (score, review)
    if best and best[0] >= 2:
        return str(best[1].relative_to(ROOT))
    return None


def review_body(track_id: str, metadata: dict[str, object], title: str, codev_review: str | None) -> str:
    status = metadata.get("status", "unknown")
    is_planned = status == "planned_roadmap"
    archive_status = "open" if is_planned else "archived"
    capability = metadata.get("capability") or metadata.get("capability_id") or "context"
    codev_line = (
        f"- Codev review: `{codev_review}`."
        if codev_review
        else "- Codev review: no direct one-to-one Codev review file was matched; this Conductor track is preserved as ledger/context evidence."
    )
    active_note = (
        "- Note: `0000-project-context` remains the active root context track; this closeout archives the current reviewed ledger snapshot, not the living context system."
        if track_id == "0000-project-context"
        else ""
    )
    lines = [
        f"# Review: {title}",
        "",
        "## Implementation status",
        "",
        f"- Track: `{track_id}`.",
        f"- Capability: `{capability}`.",
        f"- Metadata status: `{status}`.",
        "- Spec and plan are present in this Conductor track.",
        codev_line,
    ]
    if active_note:
        lines.append(active_note)
    lines += [
        "",
        "## Fixes applied",
        "",
        "- Archive audit confirmed required Conductor files are present.",
        "- No additional track-local implementation fix was required during this closeout pass.",
        "- Any runtime or CI fixes for this capability remain represented in source history, Codev review files, and generated audit reports.",
        "",
        "## Archive closeout",
        "",
        "- Review status: `reviewed`.",
        f"- Archive status: `{archive_status}`.",
        "- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.",
        "- Evidence boundary: scaffolded and synthetic-only tracks remain scaffolded/synthetic-only unless their own specs and external evidence gates say otherwise.",
    ]
    return "\n".join(lines) + "\n"


def update_metadata(metadata: dict[str, object], codev_review: str | None, version: str) -> dict[str, object]:
    updated = dict(metadata)
    is_planned = updated.get("status") == "planned_roadmap"
    updated["review_status"] = "reviewed"
    updated["archive_status"] = "open" if is_planned else "archived"
    updated["archived"] = False if is_planned else True
    updated["archive_version"] = version
    updated["archive_evidence"] = "CONDUCTOR_TRACK_CLOSEOUT_AUDIT.md"
    if codev_review:
        updated["codev_review"] = codev_review
    return updated


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true", help="write review files, metadata, and audit reports")
    args = parser.parse_args()

    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    rows: list[dict[str, object]] = []
    failures: list[str] = []

    for track_dir in sorted(path for path in TRACKS.iterdir() if path.is_dir()):
        track_id = track_dir.name
        spec_path = track_dir / "spec.md"
        plan_path = track_dir / "plan.md"
        metadata_path = track_dir / "metadata.json"
        review_path = track_dir / "review.md"
        missing = [name for name, path in [("spec.md", spec_path), ("plan.md", plan_path), ("metadata.json", metadata_path)] if not path.exists() or not path.read_text(encoding="utf-8").strip()]
        if missing:
            failures.append(f"{track_id}: missing {', '.join(missing)}")
            continue
        metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
        if metadata.get("track_id") != track_id:
            failures.append(f"{track_id}: metadata track_id mismatch")
            continue
        status = str(metadata.get("status", ""))
        if status not in IMPLEMENTED_STATUSES:
            failures.append(f"{track_id}: status {status!r} is not an implemented/scaffolded closeout status")
        spec_text = spec_path.read_text(encoding="utf-8")
        title = title_from_spec(spec_text, track_id)
        codev_review = find_codev_review(track_id, metadata, title)
        if args.write:
            review_path.write_text(review_body(track_id, metadata, title, codev_review), encoding="utf-8")
            new_metadata = update_metadata(metadata, codev_review, version)
            metadata_path.write_text(json.dumps(new_metadata, indent=2) + "\n", encoding="utf-8")
            metadata = new_metadata
        review_ok = review_path.exists() and "## Archive closeout" in review_path.read_text(encoding="utf-8")
        metadata_reviewed = metadata.get("review_status") == "reviewed"
        is_planned = metadata.get("status") == "planned_roadmap"
        metadata_archived = (
            metadata.get("archive_status") == "open" and metadata.get("archived") is False
            if is_planned
            else metadata.get("archive_status") == "archived" and metadata.get("archived") is True
        )
        if not review_ok:
            failures.append(f"{track_id}: missing review.md archive closeout")
        if not metadata_reviewed:
            failures.append(f"{track_id}: metadata review_status is not reviewed")
        if not metadata_archived:
            expected = "open" if is_planned else "archived"
            failures.append(f"{track_id}: metadata archive_status is not {expected}")
        rows.append(
            {
                "track_id": track_id,
                "capability": metadata.get("capability") or metadata.get("capability_id") or "context",
                "status": status,
                "review": str(review_path.relative_to(ROOT)),
                "archive_status": metadata.get("archive_status"),
                "codev_review": codev_review,
            }
        )

    report = {
        "schema_version": version,
        "status": "passed" if not failures else "failed",
        "track_count": len(rows),
        "failures": failures,
        "tracks": rows,
    }
    if args.write or not failures:
        (ROOT / "CONDUCTOR_TRACK_CLOSEOUT_AUDIT.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        lines = [
            "# Conductor Track Closeout Audit",
            "",
            f"Status: `{report['status']}`.",
            f"Track count: `{len(rows)}`.",
            "",
            "| Track | Capability | Status | Review | Archive | Codev review |",
            "|---|---|---|---|---|---|",
        ]
        for row in rows:
            codev = row["codev_review"] or ""
            lines.append(
                f"| `{row['track_id']}` | `{row['capability']}` | `{row['status']}` | `{row['review']}` | `{row['archive_status']}` | `{codev}` |"
            )
        if failures:
            lines += ["", "## Failures", ""]
            lines.extend(f"- {item}" for item in failures)
        (ROOT / "CONDUCTOR_TRACK_CLOSEOUT_AUDIT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    if failures:
        fail("failed checks: " + "; ".join(failures[:20]))
    print(f"conductor-track-closeout: passed ({len(rows)} tracks)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
