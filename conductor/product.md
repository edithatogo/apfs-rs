# APFS-RS Product Context

Conductor version: 0.13.0
Status: active context
Date: 2026-06-24

## Product vision

APFS-RS is a clean-room Rust implementation for APFS inspection, extraction, mounting, and eventually carefully gated write support.

The first product milestone is a Windows-first read-only APFS inspection and extraction tool, followed by a user-mode Windows mount adapter when the read-only core is trustworthy.

## Current implementation state

Implementation is early and read-only. The package currently implements synthetic-fixture-driven APFS container inspection, GPT APFS partition detection, APFS object checksum validation, checkpoint map parsing, early object-map probing, synthetic OMAP B-tree probing, lookup, resolver, a production-shaped B-tree cursor facade, macOS real APFS fixture generation, real-fixture feedback promotion evidence, real-fixture checksum-semantics correction, a policy-only software-encryption readiness scaffold, cross-platform read-only adapter readiness scaffolds, disposable-image write-lab crash-evidence scaffolds, Windows write-beta governance scaffolds, and APFS repair governance scaffolds.

## Development history

The complete implementation history is captured in Conductor tracks and summarised in `conductor/history.md`.

## Non-goals until later accepted tracks

- Physical-device write support.
- APFS repair.
- APFS formatting.
- Encryption bypass or password recovery.
- Windows kernel filesystem driver.
- File extraction before volume and filesystem B-tree traversal are validated.

## Users

- Windows users who need safe read-only access to APFS disks/images.
- Developers building cross-platform APFS tooling.
- Forensic and recovery users who need transparent diagnostics and refusal states.

## Success signal

The project succeeds when every supported APFS claim is backed by fixture evidence, Codev/Conductor context, CI checks, and conservative safety gates.


## Current ledger note

The Conductor history now spans M-001 through M-140, including repository hardening, quality evidence, dynamic versioning, redacted logging, read-only APFS scaffolds, cross-platform adapter readiness scaffolds, disposable-image write-lab crash-evidence scaffolds, executed real macOS fixture evidence, executed real-fixture feedback promotion, executed real-fixture checksum-semantics correction, and planned mature-release roadmap tracks. Production APFS compatibility claims still require the relevant local, CI, macOS, or Windows evidence gates.
