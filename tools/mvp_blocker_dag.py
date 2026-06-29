#!/usr/bin/env python3
"""Generate a dependency DAG for the remaining Windows read-only MVP blockers."""
from __future__ import annotations
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
BLOCKERS=[
 ('B1','Compile/lint/test and fix workspace',[],'local-rust'),
 ('B2','Generate first real macOS APFS image and manifest',[],'macos'),
 ('B3','Run real-fixture feedback and convert mismatches into tasks',['B1','B2'],'local-rust-macos'),
 ('B4','Correct APFS parser offsets/semantics against real APFS data',['B3'],'local-rust-macos'),
 ('B5','Full checkpoint ring reconstruction',['B4'],'local-rust-macos'),
 ('B6','Production APFS object-map B-tree traversal',['B5'],'local-rust-macos'),
 ('B7','Production filesystem tree record decoding and metadata/stat mapping',['B6'],'local-rust-macos'),
 ('B8','Production file extent resolution and extraction',['B7'],'local-rust-macos'),
 ('B9','Windows WinFsp read-only mount adapter, smoke tests, packaging baseline',['B8'],'windows-winfsp'),
]
def main():
    report={'schema_version':'0.27.0','blocker_count':len(BLOCKERS),'blockers':[{'id':i,'title':t,'depends_on':d,'environment':e} for i,t,d,e in BLOCKERS]}
    (ROOT/'MVP_BLOCKER_DAG.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    md=['# MVP Blocker Dependency DAG','',f"Blockers: `{len(BLOCKERS)}`",'', '```mermaid','flowchart TD']
    for i,t,d,e in BLOCKERS:
        md.append(f'    {i}["{i}: {t}<br/>{e}"]')
        for dep in d: md.append(f'    {dep} --> {i}')
    md += ['```','', '## Ordered blockers']
    for i,t,d,e in BLOCKERS: md.append(f'- `{i}` **{t}** — environment: `{e}`; depends on: `{", ".join(d) if d else "none"}`')
    (ROOT/'MVP_BLOCKER_DAG.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print(f'mvp-blocker-dag: wrote {len(BLOCKERS)} blockers')
    return 0
if __name__=='__main__': raise SystemExit(main())
