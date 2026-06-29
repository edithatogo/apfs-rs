# APFS-RS Agent Instructions

## Prime directive

This is filesystem software. Prefer safe refusal over best-effort behaviour. Never add, enable, or simulate physical-device write support unless the task explicitly links to an accepted write-safety spec, an accepted plan, and the image-only write-lab evidence gate.

## Context systems

This repository is set up for both Codev and Conductor context management.

Before changing code, read:

1. `conductor/product.md`
2. `conductor/tech-stack.md`
3. `conductor/workflow.md`
4. `conductor/tracks.md`
5. The active Conductor track `spec.md` and `plan.md`
6. The linked Codev spec and plan under `codev/`
7. `codev/resources/capabilities.yaml`
8. `codev/resources/safety-gates.yaml`
9. Any path-specific instructions in `.github/instructions/`

## Forbidden without explicit maintainer approval

- Raw-disk or physical-device writes.
- New low-level memory-risk code.
- New production dependencies.
- New cryptography dependencies.
- Key extraction, password recovery, password cracking, or access-control bypass.
- Write support for encrypted, sealed, damaged, Fusion/multi-device, or unknown-feature APFS states.

## Standard checks

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo xtask registry-check
cargo xtask safety-check
cargo xtask precompile-check
```

## Conductor skill

Use `conductor/skills/conductor-context-management/SKILL.md` or the mirrored `.claude/skills/conductor-context-management/SKILL.md` / `.gemini/skills/conductor-context-management/SKILL.md` / `.agents/skills/conductor-context-management/SKILL.md` whenever asked to continue, plan, review, or implement APFS-RS work.


## Conductor history requirement

When continuing development, read `conductor/history.md` and keep `conductor/tracks.md` plus `conductor/tracks/<track_id>/` synchronized with Codev specs/plans/reviews. The full development history from M-001 through the current track must remain represented as Conductor tracks.


## Current history span

The current Conductor/Codev history spans M-001 through M-118.
