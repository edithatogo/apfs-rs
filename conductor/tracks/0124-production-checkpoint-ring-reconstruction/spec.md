# Production checkpoint ring reconstruction

Capability: `M-124`  
Status: `planned_roadmap`  
Category: `core-read`

## Goal

Replace synthetic checkpoint assumptions with production read-only checkpoint selection.

## Scope

Implement full APFS checkpoint ring reconstruction from validated checkpoint descriptors and maps.

## Safety

Bounded scans, checksum refusal, no repair, no mutation.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
