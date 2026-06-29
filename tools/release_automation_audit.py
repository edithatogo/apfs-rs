#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
required=['release-plz.toml','dist-workspace.toml','RELEASE_AUTOMATION.md','.github/workflows/release-automation.yml']
missing=[p for p in required if not (ROOT/p).exists()]
report={'schema_version':'0.29.0','missing':missing,'passed':not missing}
(ROOT/'RELEASE_AUTOMATION_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'RELEASE_AUTOMATION_AUDIT.md').write_text('# Release Automation Audit\n\n'+('Passed.\n' if not missing else 'Missing: '+', '.join(missing)+'\n'))
if missing: raise SystemExit('release-automation-audit: failed')
print('release-automation-audit: passed')
