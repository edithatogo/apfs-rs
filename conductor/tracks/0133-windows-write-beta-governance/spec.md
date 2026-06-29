# Windows write beta governance

Capability: `M-133`  
Status: `planned_roadmap`  
Category: `write-beta`

## Goal

Define governance, test matrix, rollback, and refusal boundaries for any future write beta.

## Scope

Plan and gate a Windows write beta only after image-only write-lab evidence is accepted.

## Safety

No implementation before accepted write-lab evidence, maintainer approval, and image-only gates.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
