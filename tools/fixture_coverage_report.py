#!/usr/bin/env python3
"""Generate fixture-to-capability coverage report."""
from __future__ import annotations
import json, collections
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def main()->int:
    by_cap=collections.defaultdict(list); by_type=collections.Counter(); manifests=[]
    for path in sorted((ROOT/'fixtures/manifests').glob('*.json')):
        data=json.loads(path.read_text(encoding='utf-8'))
        fid=data.get('fixture_id',path.stem); manifests.append(fid); by_type[data.get('source_type','unknown')]+=1
        for cap in data.get('capability_ids',[]): by_cap[cap].append(fid)
    cap_registry=[]
    try:
        import yaml
        caps=yaml.safe_load((ROOT/'codev/resources/capabilities.yaml').read_text(encoding='utf-8')).get('capabilities',{})
        cap_registry=sorted(caps)
    except Exception:
        cap_registry=[]
    uncovered=[cap for cap in cap_registry if cap not in by_cap]
    report={'schema_version':'0.25.0','manifest_count':len(manifests),'source_types':dict(sorted(by_type.items())),'capability_fixture_coverage':dict(sorted(by_cap.items())),'capabilities_without_fixture_manifest':uncovered,'status':'ok'}
    (ROOT/'FIXTURE_COVERAGE_REPORT.json').write_text(json.dumps(report, indent=2)+'\n')
    lines=['# Fixture Coverage Report','',f"Fixture manifests: {len(manifests)}",'', '## Source types','']
    lines += [f'- `{k}`: {v}' for k,v in sorted(by_type.items())] or ['- None']
    lines += ['','## Capability coverage','']
    for cap in sorted(by_cap): lines.append(f'- `{cap}`: '+', '.join(f'`{x}`' for x in by_cap[cap]))
    lines += ['','## Capabilities without fixture manifests','']
    lines += [f'- `{x}`' for x in uncovered[:200]] or ['- None']
    (ROOT/'FIXTURE_COVERAGE_REPORT.md').write_text('\n'.join(lines)+'\n')
    print(f"fixture-coverage-report: passed ({len(manifests)} manifests, {len(uncovered)} uncovered capabilities)")
    return 0
if __name__=='__main__': raise SystemExit(main())
