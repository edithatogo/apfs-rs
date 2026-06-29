#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
wf=ROOT/'.github/workflows/supply-chain.yml'
text=wf.read_text(encoding='utf-8') if wf.exists() else ''
checks={'workflow_exists':wf.exists(),'dependency_review':'dependency-review-action' in text,'scorecard':'scorecard-action' in text}
report={'schema_version':'0.29.0','checks':checks,'passed':all(checks.values())}
(ROOT/'SCORECARD_DEPENDENCY_REVIEW_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'SCORECARD_DEPENDENCY_REVIEW_AUDIT.md').write_text('# Scorecard and Dependency Review Audit\n\n'+'\n'.join(f'- {k}: {v}' for k,v in checks.items())+'\n')
if not all(checks.values()): raise SystemExit('scorecard-dependency-review-audit: failed')
print('scorecard-dependency-review-audit: passed')
