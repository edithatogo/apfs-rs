# Cargo Error Triage

Version: 0.20.0

`tools/cargo_error_to_tracks.py` converts Cargo output into reviewable Codev and Conductor task stubs.

## Usage

```bash
cargo test --workspace 2>&1 | tee target/cargo-test.log
python3 tools/cargo_error_to_tracks.py target/cargo-test.log target/cargo-triage
```

## Output

```text
target/cargo-triage/
├── cargo-triage.json
├── cargo-triage.md
├── codev/specs/*.md
├── codev/plans/*.md
└── conductor/tracks/*/
    ├── metadata.json
    ├── spec.md
    └── plan.md
```

The generated tasks are intentionally not auto-applied to the repository. Review them first.
