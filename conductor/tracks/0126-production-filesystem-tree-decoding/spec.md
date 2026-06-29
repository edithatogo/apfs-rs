# Production filesystem tree decoding and metadata mapping

Capability: `M-126`  
Status: `planned_roadmap`  
Category: `core-read`

## Goal

Promote synthetic ls/stat scaffolds into evidence-backed production read behavior.

## Scope

Decode production APFS filesystem tree records and map file, directory, stat, and metadata fields.

## Safety

No extraction until extent resolution is validated; no xattr/resource-fork claims without fixtures.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
