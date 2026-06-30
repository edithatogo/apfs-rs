# Production software-encryption read support

Capability: `M-129`
Status: `implemented_scaffold`
Category: `encryption`

## Goal

Support legitimate user-supplied unlock material for read-only encrypted APFS images.

## Scope

Implement the smallest safe software-encryption read readiness scaffold after accepted dependency and safety review.

## Safety

No password recovery, cracking, bypass, key extraction, or hardware-bound unlock.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
