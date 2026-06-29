#!/usr/bin/env python3
"""Audit that important handoff/context documents are present and discoverable."""
from __future__ import annotations
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
KEY_DOCS=['README.md','REQUIREMENTS.md','DESIGN.md','REMAINING_ELEMENTS.md','LOCAL_HANDOFF.md','LOCAL_FIRST_RUN.md','READY_FOR_LOCAL.md','RUNBOOK.md','HANDOFF_STATUS.md','CURRENT_ENVIRONMENT.md','CURRENT_ENVIRONMENT_REMAINING.md','PRODUCTION_GAP_REPORT.md','TEST_CONTROL_MATRIX.md','SAFETY_CASE.md','CONTEXT.md']
INDEX_DOCS=['README.md','RUNBOOK.md','LOCAL_HANDOFF.md','READY_FOR_LOCAL.md','HANDOFF_STATUS.md']
def main()->int:
    missing=[]; empty=[]; not_indexed=[]
    index_text='\n'.join((ROOT/p).read_text(encoding='utf-8', errors='ignore') for p in INDEX_DOCS if (ROOT/p).exists())
    for doc in KEY_DOCS:
        path=ROOT/doc
        if not path.exists(): missing.append(doc); continue
        if not path.read_text(encoding='utf-8', errors='ignore').strip(): empty.append(doc)
        if doc not in index_text and doc not in INDEX_DOCS:
            not_indexed.append(doc)
    report={'schema_version':'0.25.0','key_docs':KEY_DOCS,'missing':missing,'empty':empty,'not_indexed':not_indexed,'status':'ok' if not missing and not empty else 'error'}
    (ROOT/'DOCUMENTATION_INDEX_AUDIT.json').write_text(json.dumps(report, indent=2)+'\n')
    lines=['# Documentation Index Audit','',f"Status: `{report['status']}`",'', '## Missing','']
    lines += [f'- `{x}`' for x in missing] or ['- None']
    lines += ['','## Empty',''] + ([f'- `{x}`' for x in empty] or ['- None'])
    lines += ['','## Present but not obviously indexed',''] + ([f'- `{x}`' for x in not_indexed] or ['- None'])
    (ROOT/'DOCUMENTATION_INDEX_AUDIT.md').write_text('\n'.join(lines)+'\n')
    print(f"documentation-index-audit: {report['status']} ({len(KEY_DOCS)} docs checked)")
    return 0 if report['status']=='ok' else 1
if __name__=='__main__': raise SystemExit(main())
