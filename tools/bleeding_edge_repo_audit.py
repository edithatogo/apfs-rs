#!/usr/bin/env python3
"""Aggregate current bleeding-edge repo-hardening checks."""
from __future__ import annotations
import runpy, sys, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
TOOLS=['github_actions_hardening_audit.py','action_pinning_audit.py','cargo_vet_policy_audit.py','provenance_verification_audit.py','release_automation_audit.py','mature_release_readiness_dashboard.py','scorecard_dependency_review_audit.py','astro7_docs_hardening_audit.py','benchmark_regression_audit.py']
results=[]
for tool in TOOLS:
    old=sys.argv[:]
    try:
        sys.argv=[str(ROOT/'tools'/tool)]
        try:
            runpy.run_path(str(ROOT/'tools'/tool),run_name='__main__')
            results.append({'tool':tool,'passed':True})
        except SystemExit as exc:
            code=exc.code if isinstance(exc.code,int) else 1
            results.append({'tool':tool,'passed':code==0,'code':code})
    finally:
        sys.argv=old
report={'schema_version':'0.29.0','results':results,'passed':all(r['passed'] for r in results)}
(ROOT/'BLEEDING_EDGE_REPO_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'BLEEDING_EDGE_REPO_AUDIT.md').write_text('# Bleeding Edge Repo Audit\n\n'+'\n'.join(f"- {r['tool']}: {r['passed']}" for r in results)+'\n')
if not report['passed']: raise SystemExit('bleeding-edge-repo-audit: failed')
print('bleeding-edge-repo-audit: passed')
