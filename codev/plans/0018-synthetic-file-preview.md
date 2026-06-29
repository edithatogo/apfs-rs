# Plan 0018: Synthetic File Preview

Document version: 0.15.0  
Status: Implemented scaffold  
Codev phase: Plan

1. Reuse the synthetic directory listing.
2. Match a directory entry by synthetic name.
3. Read the direct physical block pointer in a bounded read-only way.
4. Return UTF-8 and hex previews only.
