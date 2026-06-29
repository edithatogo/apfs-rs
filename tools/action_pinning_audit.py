#!/usr/bin/env python3
"""Report action pinning readiness without failing current scaffold."""
from __future__ import annotations
import json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
items=[]
for path in sorted((ROOT/'.github/workflows').glob('*.yml'))+sorted((ROOT/'.github/workflows').glob('*.yaml')):
    text=path.read_text(encoding='utf-8')
    for m in re.finditer(r'uses:\s*([^\s]+)', text):
        uses=m.group(1).strip().strip('"\'')
        pinned=bool(re.search(r'@[0-9a-fA-F]{40}\b',uses)) or uses.startswith('docker://')
        items.append({'workflow':path.relative_to(ROOT).as_posix(),'uses':uses,'pinned_to_sha':pinned})
report={'schema_version':'0.29.0','uses_count':len(items),'unpinned_count':sum(not i['pinned_to_sha'] for i in items),'items':items}
(ROOT/'ACTION_PINNING_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
md=['# Action Pinning Audit','',f"Action uses: {report['uses_count']}",f"Not SHA-pinned yet: {report['unpinned_count']}",'','These are informational until release hardening; public release should pin third-party actions to immutable SHAs.']
(ROOT/'ACTION_PINNING_AUDIT.md').write_text('\n'.join(md)+"\n")
print('action-pinning-audit: passed')
