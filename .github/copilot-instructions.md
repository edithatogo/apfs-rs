# Copilot Instructions for APFS-RS

Follow root `AGENTS.md` first.

This repository is set up for both Codev and Conductor context management. Before coding, read:

- `conductor/product.md`
- `conductor/workflow.md`
- `conductor/tracks.md`
- active Conductor track spec/plan
- matching Codev spec/plan/review under `codev/`

Keep APFS logic in core crates, preserve read-only defaults, and never introduce write, low-level memory-risk, crypto, mount, or raw-device behaviour without an accepted Codev spec and Conductor track.


## Conductor history requirement

When continuing development, read `conductor/history.md` and keep `conductor/tracks.md` plus `conductor/tracks/<track_id>/` synchronized with Codev specs/plans/reviews. The full development history from M-001 through the current track must remain represented as Conductor tracks.
