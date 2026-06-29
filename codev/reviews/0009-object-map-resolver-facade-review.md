# Review 0009: Object-Map Resolver Facade

Document version: 0.9.0  
Status: Review  
Codev phase: Review

## What changed

- Added resolver mode and resolver status types.
- Added `ObjectMapResolverReport` and envelope output.
- Added `ResolvedObjectLookup` internal result model.
- Updated `lookup-object` to go through the resolver facade.
- Added `resolver-report` CLI command.
- Added `synthetic-resolver-facade.img` fixture and manifest.

## Safety result

The resolver facade is read-only and does not add raw-device access, mounting, encryption, compression, repair, format, or writes.

## Known limitations

- The resolver currently wraps synthetic two-level traversal and aggregate decoded-record fallback only.
- Production APFS B-tree traversal remains future work.
- Rust/Cargo compilation still needs to be run on a Rust-enabled computer.

## Next work

Start replacing synthetic traversal internals with a production-shaped B-tree cursor and node reader once a real macOS-generated APFS fixture is available.
