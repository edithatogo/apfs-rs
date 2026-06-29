# Real fixture feedback promotion

Capability: `M-122`  
Status: `implemented`
Category: `fixture-readiness`

## Goal

Turn real APFS evidence into reviewed parser tasks with redacted artifacts.

## Scope

Run the real-fixture feedback loop and promote parser mismatches into scoped implementation tracks.

## Safety

Read manifest and inspect JSON only; do not mutate APFS media or expose personal paths.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
