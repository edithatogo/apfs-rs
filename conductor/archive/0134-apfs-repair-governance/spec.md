# APFS repair governance and refusal model

Capability: `M-134`  
Status: `implemented_scaffold`  
Category: `repair`

## Goal

Ensure repair is not conflated with read-only inspection or extraction.

## Scope

Define APFS repair scope, refusal model, evidence gates, and future implementation prerequisites.

## Safety

No repair implementation until accepted spec and destructive-test evidence exist.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
