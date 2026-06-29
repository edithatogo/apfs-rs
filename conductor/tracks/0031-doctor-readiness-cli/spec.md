# Doctor readiness CLI

Package: v0.18.0  
Capability: M-031

## Goal

Adds apfs doctor --json to aggregate inspection, volume, resolver, and directory readiness without mounting or writing APFS media.

## Safety

Read-only with respect to APFS media. No mount, decrypt, repair, format, or APFS write behaviour.
