# Production file extent resolution and extraction

Capability: `M-127`  
Status: `planned_roadmap`  
Category: `core-read`

## Goal

Promote synthetic preview/extract scaffolds to fixture-backed read-only extraction.

## Scope

Resolve production file extents and implement safe host extraction from APFS images.

## Safety

Host-output-only writes; no APFS media writes; path traversal and sparse/clone handling must be explicit.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
