# Local Environment Doctor

Capability: `M-055`  
Version: 0.21.0  
Status: implemented/scaffolded

## Goal

Improve local handoff quality without adding APFS media write, mount, decryption, repair, or format behaviour.

## Safety

This track is documentation/config/tooling only. It must not add raw physical-device writes or APFS mutation paths.

## Acceptance

- Relevant files exist in the repository.
- Cargoless validation can check them.
- Codev and Conductor history are updated.
