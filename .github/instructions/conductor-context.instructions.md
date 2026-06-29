# Conductor Context Instructions

Applies to: `conductor/**`, `codev/**`, `AGENTS.md`, `GEMINI.md`, `.claude/skills/**`, `.gemini/skills/**`

- Keep Conductor product/tech/workflow/track files synchronized with Codev specs/plans/reviews.
- Every implemented capability should have both a Codev spec/plan/review and a Conductor track or track update.
- Do not mark a Conductor plan task complete unless matching code/context exists.
- Preserve safety gates: no physical writes, no encryption bypass, no unreviewed unsafe code.


## Conductor history requirement

When continuing development, read `conductor/history.md` and keep `conductor/tracks.md` plus `conductor/tracks/<track_id>/` synchronized with Codev specs/plans/reviews. The full development history from M-001 through the current track must remain represented as Conductor tracks.
