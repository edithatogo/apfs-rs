# Real APFS parser semantics correction

Capability: `M-123`  
Status: `implemented`
Category: `core-read`

## Goal

Align NXSB, checkpoint, OMAP, B-tree, and filesystem-record parsing with real fixtures.

## Scope

Correct APFS parser offsets and semantics against real macOS fixture evidence.

## Safety

Bounds-checked read-only parser changes only; corrupt/unsupported inputs must return typed refusal.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
