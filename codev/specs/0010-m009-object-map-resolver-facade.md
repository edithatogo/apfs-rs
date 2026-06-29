# Spec 0010: M-009 Object-Map Resolver Facade

Document version: 0.9.0  
Status: Implemented in synthetic/read-only form  
Codev phase: Specify

## Goal

Introduce a stable object-map resolver facade around the current synthetic OMAP traversal and aggregate-record lookup paths. Later production APFS B-tree traversal should replace the internals without changing CLI or higher-level callers.

## Non-goals

- Production APFS B-tree traversal.
- Multi-level recursive traversal beyond the bounded synthetic fixture.
- Volume enumeration.
- File extraction.
- Write support.

## Acceptance

- `apfs resolver-report --json <source>` reports whether the current object-map resolver is available.
- `apfs lookup-object --json` includes resolver mode and lookup strategy.
- Resolver mode is explicit: bounded synthetic two-level traversal, aggregate decoded records, or unavailable.
- The resolver remains read-only and diagnostic.
