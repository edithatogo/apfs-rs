# M-044: Release signing SBOM provenance installer scaffold

Document version: 0.20.0
Status: Implemented/scaffolded

## Goal

Release signing SBOM provenance installer scaffold.

## Safety

Read-only/default-safe scaffold. No APFS media writes, encryption bypass, repair, format, or unreviewed mount lifecycle.

## Plan

1. Add scaffold/tooling/docs.
2. Add validation hook.
3. Update Conductor and remaining ledger.
