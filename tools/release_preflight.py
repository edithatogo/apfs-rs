#!/usr/bin/env python3
"""Fast cargoless release/readiness preflight for APFS-RS source packages."""
from __future__ import annotations
import argparse
import hashlib
import subprocess
import sys
import runpy
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VOLATILE_SHA_EXCLUDES = {"SHA256SUMS.txt", "PACKAGE_INTEGRITY_AUDIT.json", "PACKAGE_INTEGRITY_AUDIT.md", "HANDOFF_MANIFEST_VERIFY.json", "HANDOFF_MANIFEST_VERIFY.md"}
REQUIRED = [
    "README.md", "REQUIREMENTS.md", "DESIGN.md", "REMAINING_ELEMENTS.md", "REMAINING_ELEMENTS.yaml",
    "TRACEABILITY.md", "LOOP_DASHBOARD.md", "CLI_CONTRACT.md", "API_SURFACE.md", "SOURCE_METRICS.md",
    "SAFETY_CASE.md", "SECURITY.md", "RUNBOOK.md", "AGENTS.md", "CLAUDE.md", "GEMINI.md",
    "LOCAL_FIRST_RUN.md", "KNOWN_UNCOMPILED_RISKS.md", "HANDOFF_STATUS.md", "REPO_MANIFEST.md",
    "APFS_OFFSET_AUDIT.md", "GOLDEN_OUTPUTS.md", "DEPENDENCY_POLICY_AUDIT.md", "BACKLOG_ISSUE_EXPORT.md", "CURRENT_ENV_SELFTEST.md", "CURRENT_ENV_FINAL_REPORT.md", "MATURE_RELEASE_READINESS_DASHBOARD.md", "MATURE_RELEASE_READINESS_DASHBOARD.json", "LOCAL_MIGRATION_COMMANDS.md", "MVP_BLOCKER_DAG.md", "PRODUCTION_CLAIM_GUARD.md", "SOURCE_DEBT_REPORT.md",
    "QUALITY_GATES.md",
    "TEST_STRATEGY.md",
    "PROFILING.md",
    "DOCS_SITE_CHECK.md",
    "QUALITY_GATE_CHECK.md",
    "TESTING_INFRASTRUCTURE_REPORT.md",
    "codev/CHANGELOG.md", "conductor/history.md", "conductor/tracks.md", "AGENT_HANDOFF_BRIEF.md", "MVP_BLOCKER_TASKLIST.md", "PACKAGE_INTEGRITY_AUDIT.md", "LOCAL_COMMAND_PLAN.md", "RUST_STATIC_LINT.md", "TOOL_CAPABILITY_MATRIX.md",
]
CHECKS = [
    ["python3", "tools/current_env_selftest.py"],
    ["python3", "tools/context_integrity_check.py"],

    ["python3", "tools/production_claim_guard.py"],
    ["python3", "tools/mvp_blocker_dag.py"],
    ["python3", "tools/current_env_final_report.py"],
    ["python3", "tools/testing_infrastructure_report.py"],
    ["python3", "tools/docs_site_check.py"],
    ["python3", "tools/quality_gate_check.py"],
    ["python3", "tools/docs_package_audit.py"],
    ["python3", "tools/github_workflow_policy_audit.py"],
    ["python3", "tools/test_inventory_report.py"],
    ["python3", "tools/hypothesis_strategy_audit.py"],
    ["python3", "tools/profiling_budget_check.py"],
    ["python3", "tools/quality_gate_evidence.py"],
    ["python3", "tools/github_actions_hardening_audit.py"],
    ["python3", "tools/cargo_vet_policy_audit.py"],
    ["python3", "tools/provenance_verification_audit.py"],
    ["python3", "tools/release_automation_audit.py"],
    ["python3", "tools/scorecard_dependency_review_audit.py"],
    ["python3", "tools/astro7_docs_hardening_audit.py"],
    ["python3", "tools/benchmark_regression_audit.py"],
    ["python3", "tools/mature_release_readiness_dashboard.py"],
    ["python3", "tools/bleeding_edge_repo_audit.py"],
]

def fail(message: str) -> None:
    print(f"release-preflight: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)

def run(command: list[str]) -> None:
    print("release-preflight: running " + " ".join(command))
    if len(command) >= 2 and command[0] == "python3" and command[1].endswith(".py"):
        old_argv = sys.argv[:]
        try:
            sys.argv = [command[1], *command[2:]]
            try:
                runpy.run_path(str(ROOT / command[1]), run_name="__main__")
                return
            except SystemExit as exc:
                code = exc.code if isinstance(exc.code, int) else (0 if exc.code is None else 1)
                if code:
                    fail("command failed: " + " ".join(command))
                return
        finally:
            sys.argv = old_argv
    result = subprocess.run(command, cwd=ROOT, text=True, timeout=120)
    if result.returncode:
        fail("command failed: " + " ".join(command))


def regenerate_sha256sums() -> None:
    lines: list[str] = []
    skip_dirs = {
        ".astro",
        ".git",
        "__pycache__",
        "apfs-rs-impl-v0.29",
        "dist",
        "node_modules",
        "target",
    }
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(ROOT)
        if any(part in skip_dirs for part in rel.parts) or rel.as_posix() in VOLATILE_SHA_EXCLUDES:
            continue
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        lines.append(f"{digest}  {rel.as_posix()}")
    (ROOT / "SHA256SUMS.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("release-preflight: regenerated SHA256SUMS.txt")

def main() -> int:
    global ROOT
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--write-manifest", action="store_true")
    args = parser.parse_args()
    ROOT = args.root.resolve()
    for rel in REQUIRED:
        path = ROOT / rel
        if not path.exists() or not path.read_text(encoding="utf-8", errors="ignore").strip():
            fail(f"missing or empty required file {rel}")
    for command in CHECKS:
        run(command)
    if args.write_manifest:
        regenerate_sha256sums()
    print("release-preflight: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
