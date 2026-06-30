# Long-running fuzz property mutation coverage hardening

Capability: `M-136`  
Status: `implemented_scaffold`  
Category: `quality`

## Goal

Move configured hardening from scaffold/evidence runs into sustained release gates.

## Scope

Run and enforce long-running fuzz, property, mutation, coverage, and profiling hardening in CI.

## Safety

No production claims unless the relevant long-running gates actually pass.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
