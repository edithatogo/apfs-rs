#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
checks={
 'PROVENANCE_VERIFICATION.md':(ROOT/'PROVENANCE_VERIFICATION.md').exists(),
 'provenance-verify workflow':(ROOT/'.github/workflows/provenance-verify.yml').exists(),
 'attest-build-provenance': 'attest-build-provenance' in (ROOT/'.github/workflows/provenance-verify.yml').read_text(encoding='utf-8') if (ROOT/'.github/workflows/provenance-verify.yml').exists() else False,
}
report={'schema_version':'0.29.0','checks':checks,'passed':all(checks.values())}
(ROOT/'PROVENANCE_VERIFICATION_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'PROVENANCE_VERIFICATION_AUDIT.md').write_text('# Provenance Verification Audit\n\n'+'\n'.join(f'- {k}: {v}' for k,v in checks.items())+'\n')
if not all(checks.values()): raise SystemExit('provenance-verification-audit: failed')
print('provenance-verification-audit: passed')
