# Review: APFS repair governance and refusal model

## Implementation status

- Track: `0134-apfs-repair-governance`.
- Capability: `M-134`.
- Metadata status: `implemented_scaffold`.
- Spec and plan are present in this Conductor track.
- Codev review: `codev/reviews/134-apfs-repair-governance-review.md`.

## Fixes applied

- Added an APFS repair governance report and refusal model in `xtask` that stays blocked until accepted destructive-test evidence exists.
- Added an `xtask repair-governance-audit` command that writes JSON and Markdown evidence into `target/repair-governance`.
- Added unit coverage proving the repair governance report remains blocked and still refuses repair, fsck, and physical-device write paths.

## Archive closeout

- Review status: `reviewed`.
- Archive status: `archived`.
- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.
- Evidence boundary: scaffolded and synthetic-only tracks remain scaffolded/synthetic-only unless their own specs and external evidence gates say otherwise.
