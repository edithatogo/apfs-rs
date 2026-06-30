# Image-only write lab crash-injection evidence

Capability: `M-132`  
Status: `implemented_scaffold`
Category: `write-lab`

## Goal

Establish whether any future write support can be made safe in disposable images.

## Scope

Execute image-only write lab experiments with crash-injection evidence under accepted spec.

## Safety

Disposable image-only writes; no physical devices; no encrypted/sealed/damaged/unknown-feature writes.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
