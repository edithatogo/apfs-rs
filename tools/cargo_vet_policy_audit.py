#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
required=['supply-chain/config.toml','supply-chain/audits.toml','SUPPLY_CHAIN_POLICY.md','deny.toml']
missing=[p for p in required if not (ROOT/p).exists()]
report={'schema_version':'0.29.0','required':required,'missing':missing,'status':'passed' if not missing else 'failed'}
(ROOT/'CARGO_VET_POLICY_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'CARGO_VET_POLICY_AUDIT.md').write_text('# cargo-vet Policy Audit\n\n' + ('Passed.\n' if not missing else 'Missing: '+', '.join(missing)+'\n'))
if missing: raise SystemExit('cargo-vet-policy-audit: missing '+', '.join(missing))
print('cargo-vet-policy-audit: passed')
