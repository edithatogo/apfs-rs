#!/usr/bin/env python3
"""Validate the Astro documentation site scaffold without npm."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
REQ = [
    'docs-site/package.json',
    'docs-site/astro.config.mjs',
    'docs-site/tsconfig.json',
    'docs-site/src/pages/index.astro',
    'docs-site/src/pages/quality.astro',
    'docs-site/src/pages/conductor.astro',
    'docs-site/src/pages/handoff/index.astro',
]

def main() -> int:
    issues = []
    for rel in REQ:
        p = ROOT / rel
        if not p.exists() or not p.read_text(encoding='utf-8', errors='ignore').strip():
            issues.append(f'missing or empty {rel}')
    pkg = json.loads((ROOT / 'docs-site/package.json').read_text()) if (ROOT / 'docs-site/package.json').exists() else {}
    deps = {**pkg.get('dependencies', {}), **pkg.get('devDependencies', {})}
    astro = str(deps.get('astro', ''))
    if not (astro == '7.0.2' or astro.startswith('7.') or astro.startswith('^7')):
        issues.append('docs-site package.json does not request Astro 7')
    if 'Node' not in (ROOT / 'docs-site/README.md').read_text(encoding='utf-8', errors='ignore') and 'node' not in json.dumps(pkg):
        issues.append('docs-site lacks Node engine/readme note')
    payload = {'schema_version': (ROOT / 'VERSION').read_text().strip(), 'status': 'passed' if not issues else 'failed', 'issues': issues, 'astro_dependency': astro}
    (ROOT / 'DOCS_SITE_CHECK.json').write_text(json.dumps(payload, indent=2) + '\n')
    lines = ['# Docs Site Check', '', f"Status: `{payload['status']}`", f"Astro dependency: `{astro}`", '', '## Issues']
    lines += [f'- {i}' for i in issues] if issues else ['- none']
    (ROOT / 'DOCS_SITE_CHECK.md').write_text('\n'.join(lines) + '\n')
    if issues:
        print('docs-site-check: failed', file=sys.stderr)
        return 1
    print('docs-site-check: passed')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
