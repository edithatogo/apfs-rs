#!/usr/bin/env python3
"""Static docs-site check compatible with the Astro 7 handoff scaffold."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    pkg = ROOT / 'docs-site/package.json'
    config = ROOT / 'docs-site/astro.config.mjs'
    pages = [
        ROOT / 'docs-site/src/pages/index.astro',
        ROOT / 'docs-site/src/pages/quality.astro',
        ROOT / 'docs-site/src/pages/handoff/index.astro',
    ]
    issues = []
    if not pkg.exists(): issues.append('missing docs-site/package.json')
    if not config.exists(): issues.append('missing docs-site/astro.config.mjs')
    for page in pages:
        if not page.exists(): issues.append(f'missing {page.relative_to(ROOT)}')
    package = json.loads(pkg.read_text()) if pkg.exists() else {}
    astro = {**package.get('dependencies', {}), **package.get('devDependencies', {})}.get('astro', '')
    if not (str(astro) == '7.0.2' or str(astro).startswith('7.') or str(astro).startswith('^7')):
        issues.append('Astro 7 dependency not configured')
    report = {'schema_version': (ROOT/'VERSION').read_text().strip(), 'status': 'passed' if not issues else 'failed', 'issues': issues, 'astro_dependency': astro}
    (ROOT/'DOCS_SITE_STATIC_CHECK.json').write_text(json.dumps(report, indent=2)+'\n')
    (ROOT/'DOCS_SITE_STATIC_CHECK.md').write_text('# Docs Site Static Check\n\nStatus: `'+report['status']+'`\n')
    if issues:
        print('docs-site-static-check: failed', file=sys.stderr)
        return 1
    print('docs-site-static-check: passed')
    return 0
if __name__ == '__main__':
    raise SystemExit(main())
