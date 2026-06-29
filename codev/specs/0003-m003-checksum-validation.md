# Spec 0003: M-003 APFS Object Checksum Validation

Document version: 0.3.0  
Status: Implementing  
Codev phase: Specify

## Goal

Validate APFS object Fletcher-64 checksums for the container superblock blocks read by `apfs inspect`.

## Context

APFS objects begin with `obj_phys_t`, whose first field is an eight-byte Fletcher-64 checksum. The current inspect path must not treat an APFS object as valid merely because the `NXSB` magic is present.

## Acceptance

- Compute APFS Fletcher-64 over the object bytes after the checksum field.
- Compare with the stored checksum.
- Emit stored and computed checksum values in JSON.
- Refuse a container superblock whose checksum does not match.
- Synthetic fixtures contain valid checksums.

## Non-goals

- Cryptographic integrity.
- User-data checksums.
- Repairing checksum mismatches.
