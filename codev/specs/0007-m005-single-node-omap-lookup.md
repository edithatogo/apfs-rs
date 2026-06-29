# Spec 0007: M-005 Single-Node OMAP Lookup API

Document version: 0.8.0  
Status: Implemented in package  
Codev phase: Specify

## Goal

Add the first read-only object-map lookup API by searching the decoded OMAP records from a single B-tree root/leaf node in the synthetic fixture set.

## Scope

- Decode synthetic OMAP records already exposed by the B-tree TOC parser.
- Resolve a requested object ID and transaction ID to the newest matching record with `record.xid <= requested_xid`.
- Expose lookup through `apfs lookup-object --json`.
- Report clear warnings that this is not yet general APFS B-tree traversal.

## Non-goals

- General B-tree traversal.
- Internal-node walking.
- Real APFS object-map lookup across multiple nodes.
- Volume enumeration.
- File extraction.
- Write support.

## Acceptance

- `lookup_omap_record(records, oid, xid)` returns the highest matching XID not newer than the request.
- `apfs lookup-object --json fixtures/synthetic-omap-lookup.img --oid 500 --xid 50` reports physical block 20 in the synthetic fixture.
- Missing records return a structured not-found report.
- No physical device access or write support is added.
