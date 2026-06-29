# cargo-vet and dependency policy enforcement maturation

Capability: `M-139`  
Status: `planned_roadmap`  
Category: `supply-chain`

## Goal

Make dependency additions and transitive risk changes fail closed before release.

## Scope

Promote cargo-vet/cargo-deny dependency policy from audit scaffolds to enforced review gates.

## Safety

No new production, crypto, or low-level dependencies without explicit review and policy evidence.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
