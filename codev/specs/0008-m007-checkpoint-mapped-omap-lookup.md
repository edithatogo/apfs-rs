# Spec 0008: M-007 Checkpoint-Map-Backed Multi-Node OMAP Lookup

Document version: 0.8.0  
Status: Implementing  
Codev phase: Specify

## Goal

Extend the synthetic object-map lookup path so it can aggregate decoded OMAP records from multiple B-tree leaf nodes that are mapped by valid checkpoint-map entries.

## Why

The v0.6 lookup path could resolve records from one synthetic root/leaf node only. Real APFS object maps are B-tree-backed. Before implementing full B-tree traversal, the next safe step is to decode multiple mapped leaf nodes and perform a bounded aggregate lookup across their decoded records.

## Non-goals

- General APFS B-tree traversal.
- Production object-map lookup against arbitrary macOS images.
- Volume enumeration.
- File extraction.
- Windows mounting.
- Write support.

## Acceptance

- `inspect --json` reports additional checkpoint-map-mapped OMAP leaf nodes.
- `lookup-object --json` searches root records plus additional decoded leaf-node records.
- Synthetic multi-node fixture resolves records across at least two mapped leaves.
- Lookup remains read-only and bounded.
- No raw-device access or write path is added.
