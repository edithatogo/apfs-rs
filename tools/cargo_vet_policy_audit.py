#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
required=['supply-chain/config.toml','supply-chain/audits.toml','supply-chain/imports.lock','SUPPLY_CHAIN_POLICY.md','deny.toml']
missing=[p for p in required if not (ROOT/p).exists()]
if not missing:
    audits_text = (ROOT / "supply-chain/audits.toml").read_text(encoding="utf-8")
    config_text = (ROOT / "supply-chain/config.toml").read_text(encoding="utf-8")
    imports_lock_text = (ROOT / "supply-chain/imports.lock").read_text(encoding="utf-8") if (ROOT / "supply-chain/imports.lock").exists() else ""
    placeholder_markers = [
        "placeholder",
        "Fill this file",
    ]
    if any(marker in audits_text for marker in placeholder_markers):
        missing.append("supply-chain/audits.toml must contain real audits and not a placeholder")
    if "[audits]" not in audits_text:
        missing.append("supply-chain/audits.toml must define an [audits] table")
    if "[cargo-vet]" not in config_text:
        missing.append("supply-chain/config.toml must define a [cargo-vet] table")
    if "imports lock" in imports_lock_text.lower() and "placeholder" in imports_lock_text.lower():
        missing.append("supply-chain/imports.lock must be a real cargo-vet lockfile")
report={'schema_version':'0.29.0','required':required,'missing':missing,'status':'passed' if not missing else 'failed'}
(ROOT/'CARGO_VET_POLICY_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'CARGO_VET_POLICY_AUDIT.md').write_text('# cargo-vet Policy Audit\n\n' + ('Passed.\n' if not missing else 'Missing: '+', '.join(missing)+'\n'))
if missing: raise SystemExit('cargo-vet-policy-audit: missing '+', '.join(missing))
print('cargo-vet-policy-audit: passed')
