# Plan 0016: Synthetic Directory Record Parser

Document version: 0.15.0  
Status: Implemented scaffold  
Codev phase: Plan

1. Use the volume report to find a synthetic APFS volume.
2. Resolve the volume `root_tree_oid` through the object-map resolver.
3. Read the mapped root tree block read-only.
4. Parse bounded synthetic directory records.
5. Preserve warnings that this is not production APFS filesystem traversal.
