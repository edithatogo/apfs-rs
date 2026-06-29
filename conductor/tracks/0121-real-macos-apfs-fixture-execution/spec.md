# Real macOS APFS fixture execution

Capability: `M-121`  
Status: `planned_roadmap`  
Category: `fixture-readiness`

## Goal

Produce real fixture evidence for container, checkpoint, OMAP, and filesystem parser calibration.

## Scope

Generate the first real Apple-created APFS image fixture, redacted manifest, and oracle output on macOS.

## Safety

Image-only fixture generation; no personal data, no raw physical-device writes, no encrypted-media bypass.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
