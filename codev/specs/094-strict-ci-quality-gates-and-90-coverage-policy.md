# Spec M-094: Strict CI quality gates and 90% coverage policy

Document version: 0.27.0  
Status: implemented scaffold  
Codev phase: Specify

## Goal

Add strict ci quality gates and 90% coverage policy as a current-environment-completable quality/handoff layer.

## Non-goals

- Do not claim Rust gates have passed in this environment.
- Do not add APFS media writes, raw physical-device access, repair, format, or mount lifecycle.
