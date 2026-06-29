#!/usr/bin/env python3
"""Guard against accidental over-claiming of APFS support."""
from __future__ import annotations
import json, re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
EXCLUDE={'.git','target','__pycache__'}
RISKY=[
    ("full_apfs_claim", re.compile(r"\bfully supports APFS\b|\bcomplete APFS support\b", re.I)),
    ("write_ready_claim", re.compile(r"\bwrite support is ready\b|\bread-write APFS\b|\bphysical disk write support\b", re.I)),
    ("production_ready_claim", re.compile(r"\bproduction[- ]ready\b|\bstable release\b", re.I)),
    ("encryption_bypass_claim", re.compile(r"\bbypass\b.*\bencryption\b|\bcrack\b.*\bpassword\b", re.I)),
]
ALLOW_PATH_FRAGMENTS=['SAFETY_CASE','REMAINING_ELEMENTS','PRODUCTION_GAP_REPORT','CURRENT_ENVIRONMENT_REMAINING','production_claim_guard.py','IMPLEMENTATION_STATUS','FEATURE_READINESS','README.md','SECURITY.md','REQUIREMENTS.md','SOURCE_DEBT_REPORT','PRODUCTION_CLAIM_GUARD','tools/feature_readiness_snapshot.py','crates/apfs-features']
def allowed(path: str, line: str) -> bool:
    l=line.lower()
    cautionary_terms=['not ', 'no ', 'remaining', 'blocker', 'guard', 'scaffold', 'readiness', 'deliberately', 'until', 'currently has no stable', 'production-ready advanced features', 'not production-ready']
    return any(fragment in path for fragment in ALLOW_PATH_FRAGMENTS) and any(term in l for term in cautionary_terms)
def main():
    findings=[]
    for path in ROOT.rglob('*'):
        if not path.is_file() or EXCLUDE.intersection(path.parts) or path.suffix.lower() not in {'.md','.rs','.py','.toml','.yaml','.yml'}: continue
        rel=str(path.relative_to(ROOT))
        try: text=path.read_text(encoding='utf-8')
        except UnicodeDecodeError: continue
        for lineno,line in enumerate(text.splitlines(),1):
            for kind,pat in RISKY:
                if pat.search(line) and not allowed(rel,line):
                    findings.append({'kind':kind,'path':rel,'line':lineno,'text':line.strip()[:220]})
    status='passed' if not findings else 'failed'
    report={'schema_version':'0.27.0','status':status,'finding_count':len(findings),'findings':findings}
    (ROOT/'PRODUCTION_CLAIM_GUARD.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    md=['# Production Claim Guard','',f"Status: `{status}`",'',f"Findings: `{len(findings)}`"]
    if findings:
        md.append('## Findings')
        for f in findings: md.append(f"- `{f['kind']}` `{f['path']}:{f['line']}` — {f['text']}")
    else: md.append('No risky production-support claims detected outside allowed cautionary contexts.')
    (ROOT/'PRODUCTION_CLAIM_GUARD.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print(f'production-claim-guard: {status} ({len(findings)} findings)')
    return 0 if not findings else 1
if __name__=='__main__': raise SystemExit(main())
