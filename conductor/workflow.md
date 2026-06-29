# APFS-RS Conductor Workflow

Conductor version: 0.13.0

## Lifecycle

1. Read `conductor/product.md`, `conductor/tech-stack.md`, `conductor/tracks.md`, and `conductor/history.md`.
2. Select the active track.
3. Read the track `spec.md` and `plan.md`.
4. Cross-check Codev spec/plan/review files.
5. Implement the smallest read-only vertical slice.
6. Update tests, fixtures, Codev, and Conductor track status.
7. Run local checks when Rust is available.
8. Record review notes.

## Required checks

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo xtask registry-check
cargo xtask conductor-check
cargo xtask safety-check
```

## Fixture readiness checks

On macOS, once ready:

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
```

## Safety gates

- No raw physical device writes.
- No encryption bypass.
- No `unsafe` code without accepted review.
- No dependencies without registry/policy update.
- No implemented capability without Codev and Conductor context.
- No real APFS compatibility claim without fixture/oracle evidence.

## TDD preference

For parser work:

1. Add/extend synthetic or macOS-generated fixture.
2. Add/extend parser test expectations.
3. Implement parser.
4. Add CLI/report output.
5. Update context and review.

## Manual verification

Because this package was created in an environment without Rust/Cargo, the first manual verification on a development computer is compilation and unit test execution.


## Current ledger note

The Conductor history now spans M-001 through M-119. Keep Codev and Conductor synchronized for every new slice, and preserve the distinction between configured scaffolds and locally or CI-executed evidence.
