#!/usr/bin/env python3
"""Generate a report describing configured test, mutation, coverage, docs, and profiling infrastructure."""
from __future__ import annotations
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
ITEMS=[
 ('unit_tests','cargo test --workspace','configured'),
 ('integration_tests','crates/apfs-core/tests/integration_inspect.rs','configured'),
 ('end_to_end_tests','crates/apfs-cli/tests/e2e_cli.rs','configured'),
 ('property_tests','Rust proptest + optional Python Hypothesis','configured'),
 ('fuzz_tests','cargo fuzz object_header/nx_superblock','configured'),
 ('mutation_tests','cargo mutants scheduled/manual','configured'),
 ('coverage','cargo llvm-cov nextest --fail-under-lines 90','configured'),
 ('profiling','Criterion bench and profiling workflow','configured'),
 ('docs_site','Astro 7 + Starlight scaffold','configured'),
]
def main():
    payload={'schema_version':(ROOT/'VERSION').read_text().strip(),'items':[{'id':i,'command_or_file':c,'status':s} for i,c,s in ITEMS], 'note':'Configured here; must be executed and fixed on a Rust/Node-enabled machine.'}
    (ROOT/'TESTING_INFRASTRUCTURE_REPORT.json').write_text(json.dumps(payload,indent=2)+'\n')
    lines=['# Testing Infrastructure Report','',payload['note'],'','| Area | Command/file | Status |','|---|---|---|']
    for i,c,s in ITEMS: lines.append(f'| `{i}` | `{c}` | `{s}` |')
    (ROOT/'TESTING_INFRASTRUCTURE_REPORT.md').write_text('\n'.join(lines)+'\n')
    print('testing-infrastructure-report: generated')
    return 0
if __name__=='__main__': raise SystemExit(main())
