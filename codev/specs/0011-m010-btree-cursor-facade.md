# Spec 0011: M-010 Production-Shaped B-tree Cursor Facade

Document version: 0.10.0
Status: Implemented in package, uncompiled
Codev phase: Specify

## Goal

Introduce a reusable B-tree cursor API boundary that can later host real APFS B-tree traversal while currently wrapping the bounded synthetic OMAP traversal implementation.

## Requirements

- Provide a cursor report API with status, mode, root, steps, and lookup result.
- Keep general APFS B-tree traversal explicitly unsupported.
- Support synthetic two-level OMAP fixtures.
- Add CLI command `btree-cursor-report`.
- Add Conductor track context for the same work.

## Non-goals

- Production APFS B-tree traversal.
- APFS filesystem tree traversal.
- Volume enumeration.
- Directory listing.
- File extraction.
- Write support.

## Acceptance

```bash
cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 1500 --xid 70
```

Expected: available cursor report using synthetic OMAP two-level mode and resolving to physical block 33 in the synthetic fixture.
