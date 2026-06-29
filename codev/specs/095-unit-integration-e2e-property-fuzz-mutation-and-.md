# Spec M-095: Unit, integration, E2E, property, fuzz, mutation, and profiling test strategy

Document version: 0.27.0  
Status: implemented scaffold  
Codev phase: Specify

## Goal

Add unit, integration, e2e, property, fuzz, mutation, and profiling test strategy as a current-environment-completable quality/handoff layer.

## Non-goals

- Do not claim Rust gates have passed in this environment.
- Do not add APFS media writes, raw physical-device access, repair, format, or mount lifecycle.
