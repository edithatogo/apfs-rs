# Plan 0010: B-tree Cursor Facade

Document version: 0.10.0
Status: Implemented in package, uncompiled
Codev phase: Plan

## Tasks

- [x] Add cursor status/mode/step data structures.
- [x] Add `BTreeCursorEnvelope` and `BTreeCursorReport`.
- [x] Add `btree_cursor_report_in_device` / `btree_cursor_report_in_bytes` APIs.
- [x] Add cursor construction from inspected OMAP B-tree root.
- [x] Add CLI command `btree-cursor-report`.
- [x] Add synthetic cursor fixture alias.
- [x] Add Conductor track and skill context.
- [ ] Compile and test on Rust-enabled machine.
- [ ] Replace synthetic traversal internals with real traversal in future track.

## Safety gates

- read-only default.
- no physical writes.
- bounded traversal depth.
- synthetic fixture only until real APFS fixture exists.
