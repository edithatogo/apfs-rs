#!/usr/bin/env python3
"""Generate exact local migration command sequences for first execution."""
from __future__ import annotations
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SEQUENCES={
 'cargoless_preflight':['python3 tools/cargoless_smoke_suite.py','python3 tools/current_environment_inventory.py','python3 tools/handoff_manifest_verify.py','python3 tools/mvp_blocker_dag.py'],
 'first_rust_loop':['cargo fmt --all -- --check','cargo test --workspace','cargo clippy --workspace --all-targets --all-features -- -D warnings','cargo xtask registry-check','cargo xtask conductor-check','cargo xtask safety-check'],
 'cargo_failure_triage':['mkdir -p target','cargo test --workspace 2>&1 | tee target/cargo-test.log','python3 tools/cargo_error_to_tracks.py target/cargo-test.log target/cargo-triage'],
 'macos_fixture_loop':['./tools/macos/create_real_apfs_fixture.sh','cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json','cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > target/inspect-real.json','cargo xtask real-fixture-feedback target/inspect-real.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback','cargo xtask promote-feedback target/real-fixture-feedback/real-fixture-feedback.json target/promoted-feedback-tasks'],
 'windows_readonly_readiness':['cargo run -p apfs-cli -- winfsp-callback-matrix --json','python3 tools/winfsp_callback_matrix.py','python3 tools/windows_readiness_check.py'],
}
def main():
    (ROOT/'LOCAL_MIGRATION_COMMANDS.json').write_text(json.dumps({'schema_version':'0.27.0','sequences':SEQUENCES},indent=2)+'\n',encoding='utf-8')
    md=['# Local Migration Commands','', 'Run these in order after unzipping locally.']
    for name,cmds in SEQUENCES.items(): md += ['', f'## {name}', '', '```bash'] + cmds + ['```']
    (ROOT/'LOCAL_MIGRATION_COMMANDS.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print(f'local-migration-commands: wrote {len(SEQUENCES)} sequences')
    return 0
if __name__=='__main__': raise SystemExit(main())
