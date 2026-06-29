# Plan 0009: Object-Map Resolver Facade

Document version: 0.9.0  
Status: Implemented in synthetic/read-only form  
Codev phase: Plan

## Tasks

1. Add resolver mode/status model types.
2. Add resolver report envelope.
3. Add resolver facade around current synthetic traversal and aggregate lookup paths.
4. Update lookup result JSON to include resolver metadata.
5. Add CLI `resolver-report` command.
6. Add synthetic resolver-facade fixture and manifest.
7. Update Codev registries, runbook, and review.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `object_map_resolver_facade_boundary`.
