# Spec 0017: M-016 Synthetic Directory Record Parser

Document version: 0.15.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Parse bounded synthetic filesystem directory records from a mapped APFS B-tree node reached through the volume `root_tree_oid` and the current object-map resolver facade.

## Non-goals

Production APFS filesystem record decoding, recursive traversal, Unicode/case policy, extraction, mounting, or write support.
