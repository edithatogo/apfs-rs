#!/usr/bin/env python3
"""Cargoless shell script safety audit.

This is not shellcheck. It verifies that repository shell scripts follow the
project's image-only/read-only posture and flags high-risk patterns for review.
"""
from __future__ import annotations
import json, re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
DANGEROUS = [r'\bdd\b.*\bof=', r'\brm\s+-rf\s+/', r'\bdiskutil\s+eraseDisk\b', r'\bmkfs\.', r'\bformat\b.*[A-Z]:', r'\bmount\s+[^\n]*-o\s+rw', r'\bWrite-VolumeCache\b']

def main() -> int:
    scripts = sorted([p for p in ROOT.rglob('*.sh') if 'target' not in p.parts and '.git' not in p.parts])
    findings=[]
    for script in scripts:
        text=script.read_text(encoding='utf-8', errors='ignore')
        rel=str(script.relative_to(ROOT))
        if not text.startswith('#!'):
            findings.append({'file':rel,'severity':'warning','message':'missing shebang'})
        if 'set -euo pipefail' not in text:
            findings.append({'file':rel,'severity':'warning','message':'missing set -euo pipefail'})
        for pat in DANGEROUS:
            if re.search(pat, text):
                findings.append({'file':rel,'severity':'review','message':f'dangerous-looking pattern {pat}'})
        if 'hdiutil create' in text and '-type SPARSE' not in text:
            findings.append({'file':rel,'severity':'review','message':'hdiutil create should use sparse image fixtures'})
        if 'diskutil' in text and 'eraseDisk' in text:
            findings.append({'file':rel,'severity':'error','message':'diskutil eraseDisk is not allowed in fixture scripts'})
    report={'schema_version':'0.25.0','scripts_checked':len(scripts),'findings':findings,'status':'ok' if not any(f['severity']=='error' for f in findings) else 'error'}
    (ROOT/'SHELL_SCRIPT_AUDIT.json').write_text(json.dumps(report, indent=2)+'\n')
    lines=['# Shell Script Static Audit','',f"Status: `{report['status']}`",'',f"Scripts checked: {len(scripts)}",'', '## Findings','']
    if not findings:
        lines.append('- None')
    else:
        for f in findings:
            lines.append(f"- `{f['severity']}` `{f['file']}` — {f['message']}")
    (ROOT/'SHELL_SCRIPT_AUDIT.md').write_text('\n'.join(lines)+'\n')
    print(f"shell-script-static-check: {report['status']} ({len(scripts)} scripts, {len(findings)} findings)")
    return 0 if report['status'] != 'error' else 1
if __name__ == '__main__':
    raise SystemExit(main())
