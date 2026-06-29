# Review 0034: Unicode and case-sensitivity policy CLI

Document version: 0.19.0  
Status: Implemented scaffold review

## What changed

Adds apfs path-policy --json and a host-facing Unicode/case-sensitivity policy scaffold.

## Safety

Read-only diagnostic/readiness scaffolding only. No APFS media mutation, raw device access, encryption bypass, repair, format, or mount lifecycle was added.

## Remaining production work

Real macOS fixtures, production parser support, and feature-specific byte/metadata validation remain required.
