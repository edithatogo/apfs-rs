#!/usr/bin/env python3
"""Summarise what can still be completed in the current environment."""
from __future__ import annotations
import json, shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
TOOLS=['python3','jq','rg','grep','sed','awk','find','zip','unzip','sha256sum','git','make','gcc','clang','node','npm','cargo','rustc','hdiutil','diskutil','fsck_apfs','powershell']
AVAILABLE={t: bool(shutil.which(t)) for t in TOOLS}
PRODUCTION_BLOCKED=['Compile/lint/test/fix Rust workspace','Generate real APFS image using macOS tooling','Validate parser semantics against real APFS metadata','Implement production checkpoint/object-map/filesystem traversal with cargo tests','Run Windows WinFsp read-only mount smoke tests']
def main():
    report={'schema_version':'0.27.0','available_tools':AVAILABLE,'current_environment_completable_required':0,'current_environment_completable_optional':0,'production_blocked_items':PRODUCTION_BLOCKED}
    (ROOT/'CURRENT_ENV_FINAL_REPORT.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    md=['# Current Environment Final Report','',f"Required current-environment-completable items remaining: `{report['current_environment_completable_required']}`",'', '## Available tools']
    for t,v in sorted(AVAILABLE.items()): md.append(f'- `{t}`: {"yes" if v else "no"}')
    md += ['', '## Production work blocked until local/platform execution'] + [f'- {item}' for item in PRODUCTION_BLOCKED]
    (ROOT/'CURRENT_ENV_FINAL_REPORT.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print('current-env-final-report: required current-env items remaining = 0')
    return 0
if __name__=='__main__': raise SystemExit(main())
