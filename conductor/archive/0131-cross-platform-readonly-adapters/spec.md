# Linux macOS ChromeOS Android read-only adapters

Capability: `M-131`  
Status: `implemented_scaffold`
Category: `cross-platform`

## Goal

Expand validated read-only access beyond Windows after core parser maturity.

## Scope

Implement the shared Linux, macOS, ChromeOS, and Android read-only adapter readiness scaffold with platform-oriented tests and explicit refusal boundaries.

## Safety

Adapters consume read-only VFS only; no unverified mount lifecycle or write operations.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
