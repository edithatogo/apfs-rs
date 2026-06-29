#!/usr/bin/env python3
"""Cargoless quality-gate configuration check."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
REQ_FILES=[
    'QUALITY_GATES.md','TEST_STRATEGY.md','PROFILING.md',
    '.github/workflows/quality-gates.yml','.github/workflows/profiling.yml',
    'crates/apfs-types/tests/property_nx_superblock.rs',
    'crates/apfs-core/tests/integration_inspect.rs',
    'crates/apfs-cli/tests/e2e_cli.rs',
    'crates/apfs-core/benches/inspect_synthetic.rs',
    'mutants.toml',
    # CI/CD quality gates
    '.github/workflows/ci.yml',
    '.github/workflows/coverage.yml',
    '.github/workflows/fuzz.yml',
    '.github/workflows/mutation.yml',
    '.github/workflows/strict-quality.yml',
    '.github/workflows/supply-chain.yml',
    '.github/workflows/release.yml',
    '.github/workflows/release-automation.yml',
    '.github/workflows/provenance-verify.yml',
    '.github/workflows/workflow-security.yml',
    '.github/workflows/docs-site.yml',
    '.github/workflows/docs-quality.yml',
    '.github/workflows/local-handoff.yml',
    '.github/workflows/python-property.yml',
    'RELEASE_AUTOMATION.md',
    'RELEASE_SCAFFOLD.md',
    'QUALITY_GATE_EVIDENCE.md',
    'QUALITY_GATE_REPORT.md',
]
REQ_TERMS={
    '.github/workflows/quality-gates.yml':['cargo nextest','cargo fmt','cargo clippy','precompile-check'],
    'QUALITY_GATES.md':['>= 90%','Unit tests','Mutation testing','Profiling'],
    'TEST_STRATEGY.md':['proptest','Hypothesis','End-to-end','Fuzzing'],
    'PROFILING.md':['Criterion','cargo bench'],
    '.github/workflows/ci.yml':['cargo fmt','cargo clippy','cargo test','registry-check','conductor-check','safety-check'],
    '.github/workflows/coverage.yml':['cargo llvm-cov','workflow_dispatch','schedule'],
    '.github/workflows/fuzz.yml':['fuzz run','workflow_dispatch','schedule'],
    '.github/workflows/mutation.yml':['cargo mutants','workflow_dispatch','schedule'],
    '.github/workflows/strict-quality.yml':['fail-under-lines 90','cargo nextest','cargo deny','cargo audit','cargo mutants'],
    '.github/workflows/supply-chain.yml':['dependency-review','scorecard','cargo-vet'],
    '.github/workflows/release.yml':['release-preflight','attest-build-provenance'],
    '.github/workflows/release-automation.yml':['cargo-dist','release-plz'],
    '.github/workflows/provenance-verify.yml':['attest-build-provenance'],
    '.github/workflows/docs-site.yml':['npm ci','npm run build'],
    '.github/workflows/local-handoff.yml':['cargoless','precompile_static_check','handoff_status','repo_manifest'],
    'QUALITY_GATE_EVIDENCE.md':['>=90% coverage','mutation','fuzz smoke','Astro 7 docs'],
    'QUALITY_GATE_REPORT.md':['coverage_fail_under_90_configured','cargo_nextest_configured','cargo_mutants_configured'],
}
def main():
    issues=[]
    for rel in REQ_FILES:
        p=ROOT/rel
        if not p.exists() or not p.read_text(encoding='utf-8', errors='ignore').strip():
            issues.append(f'missing or empty {rel}')
    for rel, terms in REQ_TERMS.items():
        text=(ROOT/rel).read_text(encoding='utf-8', errors='ignore') if (ROOT/rel).exists() else ''
        for term in terms:
            if term not in text:
                issues.append(f'{rel} missing term {term!r}')
    payload={'schema_version':(ROOT/'VERSION').read_text().strip(),'status':'passed' if not issues else 'failed','issues':issues}
    (ROOT/'QUALITY_GATE_CHECK.json').write_text(json.dumps(payload,indent=2)+'\n')
    lines=['# Quality Gate Check','',f"Status: `{payload['status']}`",'', '## Issues']
    lines += [f'- {i}' for i in issues] if issues else ['- none']
    (ROOT/'QUALITY_GATE_CHECK.md').write_text('\n'.join(lines)+'\n')
    if issues:
        print('quality-gate-check: failed', file=sys.stderr)
        return 1
    print('quality-gate-check: passed')
    return 0
if __name__=='__main__': raise SystemExit(main())
