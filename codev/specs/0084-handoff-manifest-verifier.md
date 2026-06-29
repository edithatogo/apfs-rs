# Spec 0084: Handoff manifest verifier

Document version: 0.26.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add handoff manifest verifier as part of the current-environment handoff closure layer.

## Non-goals

- Production APFS parsing beyond existing synthetic/readiness paths.
- APFS media writes.
- Raw physical-device access.
- Encryption bypass, repair, format, or live mount lifecycle.

## Acceptance

- Tooling/report exists.
- Codev and Conductor history are updated.
- Cargoless validation can run in this environment.
