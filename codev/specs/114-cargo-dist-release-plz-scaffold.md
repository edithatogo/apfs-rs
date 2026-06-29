# Spec M-114: cargo-dist and release-plz automation scaffold

Document version: 0.29.0  
Status: Implemented as current-environment scaffold

## Goal

Implement a safe, current-environment-auditable scaffold for cargo-dist and release-plz automation scaffold.

## Acceptance

- Policy/config/tool exists.
- Cargoless audit can run here where applicable.
- Conductor track exists.
- No production APFS media writes or mount lifecycle is introduced.
