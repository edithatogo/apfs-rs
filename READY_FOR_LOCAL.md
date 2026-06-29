# Ready For Local Checklist

Version: 0.20.0

## Ready in this package

- Codev history is present.
- Conductor track history is present.
- Requirements and design files are present.
- Cargoless static validation exists.
- Synthetic fixtures and oracles exist.
- Local handoff and cargo triage tooling exist.
- Platform setup docs exist.
- Release/provenance scaffolding exists.
- Non-Windows adapter readiness crates exist.
- Software-encryption and image-only write-lab readiness crates exist.

## Still requires local execution

- Cargo compile/lint/test.
- macOS APFS fixture generation.
- Parser correction against real APFS metadata.
- Windows WinFsp mount implementation and smoke tests.

## Move locally when

You are ready to run the first Cargo build and generate a macOS APFS sparse-image fixture.

## v0.21.0 local-handoff additions

The package now includes reproducible-tooling configs and a local environment doctor:

```bash
python3 tools/config_sanity_check.py
python3 tools/local_env_doctor.py --json target/local-env-doctor.json
python3 tools/handoff_status.py --write
python3 tools/repo_manifest.py --write
```

The repository is ready to move locally when all cargoless checks pass and you are ready to run the first Rust compile/test loop.

## Current handoff version

`0.25.0`
