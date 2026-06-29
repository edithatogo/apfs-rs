#!/usr/bin/env python3
"""Cargoless GitHub Actions hardening audit."""
from __future__ import annotations
import json, re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
WF=ROOT/'.github/workflows'
findings=[]
for path in sorted(WF.glob('*.yml'))+sorted(WF.glob('*.yaml')):
    text=path.read_text(encoding='utf-8')
    rel=path.relative_to(ROOT).as_posix()
    if 'permissions:' not in text:
        findings.append({'workflow':rel,'severity':'medium','kind':'missing_permissions','message':'workflow lacks explicit permissions block'})
    if 'pull_request_target:' in text:
        findings.append({'workflow':rel,'severity':'high','kind':'pull_request_target','message':'pull_request_target requires dedicated review'})
    for m in re.finditer(r'uses:\s*([^\s]+)', text):
        uses=m.group(1).strip().strip('"\'')
        if '@' in uses and not re.search(r'@[0-9a-fA-F]{40}\b', uses) and not uses.startswith('docker://'):
            findings.append({'workflow':rel,'severity':'info','kind':'unpinned_action','uses':uses,'message':'pin action to commit SHA before public release'})
report={'schema_version':'0.29.0','workflow_count':len(list(WF.glob('*.yml'))+list(WF.glob('*.yaml'))),'findings':findings,'blocking_findings':[f for f in findings if f['severity']=='high']}
(ROOT/'GITHUB_ACTIONS_HARDENING.json').write_text(json.dumps(report,indent=2)+"\n")
md=['# GitHub Actions Hardening Audit','',f"Workflows scanned: {report['workflow_count']}",f"Findings: {len(findings)}"]
for f in findings[:200]: md.append(f"- {f['severity']} `{f['kind']}` in `{f['workflow']}`: {f.get('message','')}")
(ROOT/'GITHUB_ACTIONS_HARDENING_AUDIT.md').write_text('\n'.join(md)+"\n")
print(f"github-actions-hardening-audit: passed ({len(findings)} findings, {len(report['blocking_findings'])} high)")
