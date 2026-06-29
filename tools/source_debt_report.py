#!/usr/bin/env python3
"""Cargoless source-debt report.

Scans Rust, Python, markdown, and config files for explicit debt markers and
source risks that should be triaged locally after Cargo is available. This is
not a linter; it is a handoff visibility tool.
"""
from __future__ import annotations
import json, re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
EXCLUDE_PARTS = {".git", "target", "__pycache__"}
PATTERNS = {
    "todo": re.compile(r"\bTODO\b|\bFIXME\b|\bHACK\b", re.I),
    "scaffold": re.compile(r"scaffold|placeholder|synthetic|not implemented|readiness", re.I),
    "panic_risk": re.compile(r"\bunwrap\s*\(|\bexpect\s*\(|\bpanic!\s*\(", re.I),
    "write_risk_term": re.compile(r"raw[_ -]?write|physical[_ -]?write|GENERIC_WRITE|FILE_WRITE_DATA", re.I),
}
FILE_SUFFIXES = {".rs", ".py", ".md", ".toml", ".yaml", ".yml", ".json", ".sh"}
def iter_files():
    for path in ROOT.rglob("*"):
        if path.is_file() and not EXCLUDE_PARTS.intersection(path.parts) and path.suffix in FILE_SUFFIXES:
            yield path
def main() -> int:
    findings=[]
    for path in iter_files():
        try: text=path.read_text(encoding='utf-8')
        except UnicodeDecodeError: continue
        rel=str(path.relative_to(ROOT))
        for lineno,line in enumerate(text.splitlines(),1):
            for kind,pat in PATTERNS.items():
                if pat.search(line):
                    findings.append({"kind":kind,"path":rel,"line":lineno,"text":line.strip()[:220]})
    by_kind={}
    for f in findings: by_kind[f['kind']]=by_kind.get(f['kind'],0)+1
    report={"schema_version":"0.27.0","status":"informational","finding_count":len(findings),"by_kind":by_kind,"findings":findings[:1000]}
    (ROOT/'SOURCE_DEBT_REPORT.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    md=['# Source Debt Report','',f"Findings: `{len(findings)}`",'', '## Counts by kind']
    for k,v in sorted(by_kind.items()): md.append(f"- `{k}`: {v}")
    md += ['', '## First findings']
    for f in findings[:200]: md.append(f"- `{f['kind']}` `{f['path']}:{f['line']}` — {f['text']}")
    if not findings: md.append('- None')
    (ROOT/'SOURCE_DEBT_REPORT.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print(f"source-debt-report: informational ({len(findings)} findings)")
    return 0
if __name__=='__main__': raise SystemExit(main())
