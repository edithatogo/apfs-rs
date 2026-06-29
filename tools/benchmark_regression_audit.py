#!/usr/bin/env python3
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
checks={'plan_exists':(ROOT/'BENCHMARK_REGRESSION_PLAN.md').exists(),'benches_dir':(ROOT/'benches').exists(),'profiling_dir':(ROOT/'profiling').exists()}
report={'schema_version':'0.29.0','checks':checks,'passed':all(checks.values())}
(ROOT/'BENCHMARK_REGRESSION_AUDIT.json').write_text(json.dumps(report,indent=2)+"\n")
(ROOT/'BENCHMARK_REGRESSION_AUDIT.md').write_text('# Benchmark Regression Audit\n\n'+'\n'.join(f'- {k}: {v}' for k,v in checks.items())+'\n')
if not all(checks.values()): raise SystemExit('benchmark-regression-audit: failed')
print('benchmark-regression-audit: passed')
