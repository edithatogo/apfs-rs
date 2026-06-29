# Windows WinFsp read-only mount adapter and packaging

Capability: `M-128`  
Status: `planned_roadmap`  
Category: `windows-mvp`

## Goal

Deliver Windows-first read-only mount experience over the validated VFS facade.

## Scope

Implement and package the Windows user-mode read-only WinFsp adapter with smoke tests.

## Safety

Read-only callbacks only; all write-like operations refused; no kernel driver.

## Evidence required

- Codev spec, plan, and review stay synchronized with this Conductor track.
- Local and CI evidence is recorded before status changes from planned.
- Any APFS compatibility claim names the fixture, platform, and exact covered behavior.
