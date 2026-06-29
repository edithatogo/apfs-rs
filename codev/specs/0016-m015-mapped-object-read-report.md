# Spec 0015: M-014 Resolver-Backed Mapped Object Read Report

Document version: 0.15.0  
Status: Implemented in package, uncompiled  
Date: 2026-06-24  
Codev phase: Specify

## Goal

Add a read-only command that resolves an object ID through the current object-map resolver facade, reads the mapped physical block, validates its APFS object checksum, and reports the generic object header.

## Non-goals

- Decoding object payload semantics.
- Directory listing.
- File extraction.
- Writing, repairing, formatting, mounting, encryption, or compression.
- Treating synthetic object reads as production APFS object-map support.

## Acceptance

- `apfs read-object --json fixtures/synthetic-mapped-object-read.img --oid 1500 --xid 70` exists.
- The report includes resolver metadata, lookup metadata, physical block index, object header, checksum status, and a short redacted hex prefix.
- Missing objects produce structured `not_found` output.
- Payload decoding remains explicitly unsupported.
