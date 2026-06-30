# Review: APFS format governance and refusal model

## Implementation status

- Track: `0135-apfs-format-governance`.
- Capability: `M-135`.
- Metadata status: `implemented_scaffold`.
- Spec and plan are present in this Conductor track.
- Codev review: `codev/reviews/135-apfs-format-governance-review.md`.

## Fixes applied

- Added an APFS format governance report and refusal model in `xtask` that stays blocked until accepted destructive-test evidence exists.
- Added an `xtask format-governance-audit` command that writes JSON and Markdown evidence into `target/format-governance`.
- Added unit coverage proving the format governance report remains blocked and still refuses format, mkfs, erase, and initialization paths.

## Archive closeout

- Review status: `reviewed`.
- Archive status: `archived`.
- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.
- Evidence boundary: scaffolded and synthetic-only tracks remain scaffolded/synthetic-only unless their own specs and external evidence gates say otherwise.
