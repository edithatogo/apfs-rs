# Plan 0011: Real APFS Fixture Readiness Harness

Document version: 0.11.0  
Status: Implemented scaffold, pending macOS execution  
Codev phase: Plan

## Tasks

- [x] Add macOS fixture-generation script.
- [x] Add real fixture README.
- [x] Add manifest template.
- [x] Add inspect/manifest comparison script.
- [x] Add `apfs-test` fixture manifest structs.
- [x] Add `cargo xtask fixture-manifest-check`.
- [x] Add Conductor historical track for this capability.
- [ ] Run fixture generation on macOS.
- [ ] Run `apfs inspect --json` against the generated sparse image.
- [ ] Compare inspect JSON with manifest.
- [ ] Use the first real APFS image to correct parser assumptions.

## Safety gates

- `read_only_default`.
- `macos_fixture_no_personal_data`.
- `oracle_manifest_required`.
- `real_fixture_not_production_claim`.
