# Plan 0008: Bounded Synthetic B-tree Traversal

Document version: 0.8.0  
Status: Implemented in package  
Codev phase: Plan

## Tasks

1. Add `BTreeIndexRecord` and child-selection result types.
2. Add parser for synthetic index records from non-leaf B-tree nodes.
3. Add bounded child-selection helper.
4. Add traversal report type to lookup JSON.
5. Change lookup to prefer bounded traversal when root index records are available.
6. Add `synthetic-btree-traversal.img` fixture.
7. Update README, runbook, implementation status, capabilities, safety gates, changelog, and review.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `synthetic_btree_traversal_limit`.

## Acceptance commands

```bash
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 1500 --xid 70
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 2500 --xid 70
```
