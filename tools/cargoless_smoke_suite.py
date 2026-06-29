#!/usr/bin/env python3
"""Run a cargoless validation suite without spawning a Rust toolchain.

Most checks are executed in-process with runpy to avoid subprocess quirks in
sandboxed environments. This is not a substitute for cargo test.
"""
from __future__ import annotations
import json, py_compile, runpy, time, traceback
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
PY_COMPILE = [
    'tools/markdown_link_audit.py',
    'tools/shell_script_static_check.py',
    'tools/documentation_index_audit.py',
    'tools/fixture_coverage_report.py',
    'tools/current_env_completion_report.py',
    'tools/source_debt_report.py',
    'tools/production_claim_guard.py',
    'tools/mvp_blocker_dag.py',
    'tools/local_migration_commands.py',
    'tools/current_env_final_report.py',
    'tools/tool_capability_matrix.py',
    'tools/rust_static_lint.py',
    'tools/local_command_plan.py',
    'tools/mvp_blocker_tasklist.py',
    'tools/agent_handoff_brief.py',
    'tools/quality_gate_check.py',
    'tools/ci_quality_gate_audit.py',
    'tools/test_strategy_audit.py',
    'tools/docs_site_audit.py',
    'tools/profiling_plan_audit.py',
    'tools/current_env_tool_inventory_extended.py',
    'tools/docs_package_audit.py',
    'tools/github_workflow_policy_audit.py',
    'tools/test_inventory_report.py',
    'tools/hypothesis_strategy_audit.py',
    'tools/profiling_budget_check.py',
    'tools/quality_gate_evidence.py',
]
SCRIPTS = [
    'tools/precompile_static_check.py',
    'tools/context_integrity_check.py',
    'tools/version_consistency_check.py',
    'tools/synthetic_fixture_oracle.py',
    'tools/cargo_workspace_audit.py',
    'tools/config_sanity_check.py',
    'tools/markdown_link_audit.py',
    'tools/shell_script_static_check.py',
    'tools/documentation_index_audit.py',
    'tools/fixture_coverage_report.py',
    'tools/current_env_completion_report.py',
    'tools/source_debt_report.py',
    'tools/production_claim_guard.py',
    'tools/mvp_blocker_dag.py',
    'tools/local_migration_commands.py',
    'tools/current_env_final_report.py',
    'tools/tool_capability_matrix.py',
    'tools/rust_static_lint.py',
    'tools/local_command_plan.py',
    'tools/mvp_blocker_tasklist.py',
    'tools/agent_handoff_brief.py',
    'tools/quality_gate_check.py',
    'tools/ci_quality_gate_audit.py',
    'tools/test_strategy_audit.py',
    'tools/docs_site_audit.py',
    'tools/profiling_plan_audit.py',
    'tools/current_env_tool_inventory_extended.py',
    'tools/docs_package_audit.py',
    'tools/github_workflow_policy_audit.py',
    'tools/test_inventory_report.py',
    'tools/hypothesis_strategy_audit.py',
    'tools/profiling_budget_check.py',
    'tools/quality_gate_evidence.py',
]

def run_script(path: str) -> tuple[int, str]:
    try:
        runpy.run_path(str(ROOT / path), run_name='__main__')
        return 0, ''
    except SystemExit as e:
        code = e.code if isinstance(e.code, int) else (0 if e.code is None else 1)
        return code, ''
    except Exception:
        return 1, traceback.format_exc()[-2000:]

def main() -> int:
    results = []
    ok = True
    start = time.time()
    t = time.time()
    try:
        for rel in PY_COMPILE:
            py_compile.compile(str(ROOT / rel), doraise=True)
        rc = 0
        err = ''
    except Exception:
        rc = 1
        err = traceback.format_exc()[-2000:]
    results.append({'command': 'py_compile selected tools', 'returncode': rc, 'duration_seconds': round(time.time() - t, 3), 'error_tail': err})
    if rc:
        ok = False
    for script in SCRIPTS:
        t = time.time()
        rc, err = run_script(script)
        results.append({'command': f'runpy {script}', 'returncode': rc, 'duration_seconds': round(time.time() - t, 3), 'error_tail': err})
        if rc:
            ok = False
    report = {'schema_version': (ROOT / 'VERSION').read_text().strip(), 'status': 'passed' if ok else 'failed', 'duration_seconds': round(time.time() - start, 3), 'results': results}
    (ROOT / 'CARGOLESS_SMOKE_REPORT.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    lines = ['# Cargoless Smoke Report', '', f"Status: `{report['status']}`", f"Duration seconds: {report['duration_seconds']}", '', '## Commands', '']
    for r in results:
        lines.append(f"- `{r['command']}` → `{r['returncode']}` in {r['duration_seconds']}s")
    (ROOT / 'CARGOLESS_SMOKE_REPORT.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')
    print(f"cargoless-smoke-suite: {report['status']} ({len(results)} checks)")
    return 0 if ok else 1

if __name__ == '__main__':
    raise SystemExit(main())
