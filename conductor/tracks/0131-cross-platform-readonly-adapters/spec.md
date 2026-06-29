# Linux macOS ChromeOS Android read-only adapters

Capability: `M-131`  
Status: `planned_roadmap`  
Category: `cross-platform`

## Goal

Expand validated read-only access beyond Windows after core parser maturity.

## Scope

Implement production Linux, macOS, ChromeOS, and Android read-only adapters with platform tests.

## Safety

Adapters consume read-only VFS only; no unverified mount lifecycle or write operations.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
