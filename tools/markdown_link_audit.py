#!/usr/bin/env python3
"""Audit Markdown links without requiring network access.

This is a cargoless, best-effort internal-link audit. It records broken-looking
relative links but exits successfully so it can run in constrained environments
without blocking on false positives from external documentation examples.
"""
from __future__ import annotations
import json, re
from pathlib import Path
from urllib.parse import unquote
ROOT = Path(__file__).resolve().parents[1]
SKIP_PARTS = {'.git','target','__pycache__'}
LINK_RE = re.compile(r'!?\[[^\]]*\]\(([^)]+)\)')

def is_external(target: str) -> bool:
    return target.startswith(('http://','https://','mailto:','tel:','data:'))

def normalize_target(raw: str) -> str:
    target = raw.strip().split()[0] if raw.strip() else ''
    if target.startswith('<') and target.endswith('>'):
        target = target[1:-1]
    return unquote(target.split('#',1)[0])

def main() -> int:
    checked = 0; external = 0; anchors = 0; missing = []
    for md in sorted(ROOT.rglob('*.md')):
        if any(part in SKIP_PARTS for part in md.relative_to(ROOT).parts):
            continue
        text = md.read_text(encoding='utf-8', errors='ignore')
        for line_no, line in enumerate(text.splitlines(), 1):
            for m in LINK_RE.finditer(line):
                raw = m.group(1).strip()
                if not raw:
                    continue
                if raw.startswith('#'):
                    anchors += 1; continue
                if is_external(raw):
                    external += 1; continue
                target = normalize_target(raw)
                if not target or target.startswith('#'):
                    anchors += 1; continue
                checked += 1
                base = ROOT if target.startswith('/') else md.parent
                rel = target[1:] if target.startswith('/') else target
                path = (base / rel).resolve()
                try:
                    path.relative_to(ROOT.resolve())
                except ValueError:
                    missing.append({'file': str(md.relative_to(ROOT)), 'line': line_no, 'target': raw, 'reason': 'outside_repo'})
                    continue
                if not path.exists():
                    missing.append({'file': str(md.relative_to(ROOT)), 'line': line_no, 'target': raw, 'reason': 'missing'})
    report = {'schema_version':'0.25.0','checked_relative_links':checked,'external_links':external,'anchor_links':anchors,'missing_or_suspicious':missing,'status':'ok' if not missing else 'warnings'}
    (ROOT/'MARKDOWN_LINK_AUDIT.json').write_text(json.dumps(report, indent=2)+'\n')
    lines = ['# Markdown Link Audit','',f"Status: `{report['status']}`",'',f"Checked relative links: {checked}",f"External links skipped: {external}",f"Anchor-only links skipped: {anchors}",'','## Missing or suspicious links','']
    if not missing:
        lines.append('- None')
    else:
        for item in missing[:200]:
            lines.append(f"- `{item['file']}:{item['line']}` -> `{item['target']}` ({item['reason']})")
        if len(missing)>200:
            lines.append(f"- ... {len(missing)-200} additional items truncated in Markdown view; see JSON.")
    (ROOT/'MARKDOWN_LINK_AUDIT.md').write_text('\n'.join(lines)+'\n')
    print(f"markdown-link-audit: {report['status']} ({checked} relative links checked, {len(missing)} warnings)")
    return 0
if __name__ == '__main__':
    raise SystemExit(main())
