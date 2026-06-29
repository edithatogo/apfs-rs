#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
pkg=json.loads((ROOT/'docs-site/package.json').read_text(encoding='utf-8'))
deps=pkg.get('devDependencies',{})
checks={'astro7':str(deps.get('astro','')).startswith('7.'),'starlight_declared':'@astrojs/starlight' in deps,'docs_plan':(ROOT/'DOCS_AUTOMATION.md').exists()}
report={'schema_version':'0.29.0','checks':checks,'astro_version':deps.get('astro'),'passed':all(checks.values())}
(ROOT/'ASTRO7_DOCS_HARDENING_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'ASTRO7_DOCS_HARDENING_AUDIT.md').write_text('# Astro 7 Docs Hardening Audit\n\n'+'\n'.join(f'- {k}: {v}' for k,v in checks.items())+'\n')
if not all(checks.values()): raise SystemExit('astro7-docs-hardening-audit: failed')
print('astro7-docs-hardening-audit: passed')
