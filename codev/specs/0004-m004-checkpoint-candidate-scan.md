# Spec 0004: M-004 Checkpoint Candidate Scan

Document version: 0.3.0  
Status: Scaffold implemented  
Codev phase: Specify

## Goal

Use the block-zero container superblock to locate the checkpoint descriptor area and scan for valid `nx_superblock_t` candidates.

## Acceptance

- Use `nx_xp_desc_base` and `nx_xp_desc_len` from the container superblock.
- Scan a bounded number of descriptor blocks.
- Parse `NXSB` candidates.
- Validate candidate checksums.
- Report latest valid candidate transaction ID and block index.

## Non-goals

- Parsing `checkpoint_map_phys_t`.
- Reconstructing ephemeral object state.
- Full mount checkpoint selection.
- B-tree or object-map parsing.
