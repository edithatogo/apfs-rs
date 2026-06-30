# Production object-map B-tree traversal

Capability: `M-125`
Status: `implemented`
Category: `core-read`

## Goal

Resolve object IDs through validated checkpoint-selected OMAP structures.

## Scope

Implement production APFS OMAP B-tree traversal through the resolver facade.

## Safety

Read-only traversal, cycle/depth limits, typed unsupported-state errors.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
