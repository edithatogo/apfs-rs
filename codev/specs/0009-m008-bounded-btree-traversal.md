# Spec 0009: M-008 Bounded Synthetic B-tree Traversal

Document version: 0.8.0  
Status: Implemented in package  
Codev phase: Specify

## Goal

Add a bounded synthetic OMAP B-tree traversal layer that selects one child leaf from a parsed root/index node before performing object-map lookup.

## Scope

This slice supports a synthetic two-level B-tree shape only:

```text
root/index node -> selected leaf node -> decoded OMAP records
```

The root index keys are interpreted as synthetic `(max_oid, max_xid)` ranges, and the value is interpreted as a synthetic `child_oid`.

## Non-goals

- Production APFS B-tree traversal.
- Recursive B-tree traversal beyond depth two.
- Node sibling links.
- Variable key/value layouts beyond the synthetic fixture convention.
- Volume enumeration.
- File extraction.
- Write support.

## Acceptance

- Parse synthetic root/index records.
- Select the child leaf that covers a requested `(oid, xid)` pair.
- Decode OMAP records from only the selected mapped leaf.
- Include traversal details in `lookup-object --json`.
- Preserve aggregate fallback for older synthetic fixtures.
- Add a synthetic bounded traversal fixture.
