# Spec 0019: M-018 Synthetic File Preview

Document version: 0.15.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Expose a read-only `apfs cat --name <file>` preview path for synthetic directory entries with direct block payloads.

## Non-goals

Production APFS file extent resolution, compressed/sparse/clone handling, extraction to disk, or write support.
