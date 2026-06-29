# APFS-RS Gemini/Conductor Instructions

This repository is prepared for Conductor-style context-driven development.

Read in order:

1. `conductor/product.md`
2. `conductor/tech-stack.md`
3. `conductor/workflow.md`
4. `conductor/tracks.md`
5. Active track spec/plan under `conductor/tracks/`
6. Matching Codev spec/plan/review under `codev/`
7. `AGENTS.md`

Current active implementation track: `conductor/tracks/0010-btree-cursor/`.

Do not implement writes, mount adapters, encryption, or unsafe code unless a future accepted Conductor track and Codev spec explicitly authorize it.


## Conductor history requirement

When continuing development, read `conductor/history.md` and keep `conductor/tracks.md` plus `conductor/tracks/<track_id>/` synchronized with Codev specs/plans/reviews. The full development history from M-001 through the current track must remain represented as Conductor tracks.


## Current history span

The current Conductor/Codev history spans M-001 through M-021.
