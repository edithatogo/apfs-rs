# Review 0002: GPT Probe Implementation Review

Document version: 0.2.0  
Status: Draft review  
Codev phase: Review

## What changed

The implementation now includes a second inspect path:

1. Probe block zero for `NXSB`.
2. If not direct APFS, read GPT header at LBA 1.
3. Parse GPT partition entries.
4. Find APFS partition type GUID.
5. Probe APFS container superblock at the partition first LBA.

## Safety result

All work remains image-only and read-only. Raw physical-device access and write support were not added.

## Known limitations

- GPT CRCs are not validated yet.
- GPT sector size is assumed to be 512 bytes.
- Synthetic fixtures are not macOS-generated APFS filesystems.
- APFS object checksum validation is still missing.

## Next implementation work

1. Add real Fletcher checksum validation.
2. Generate macOS APFS image fixture.
3. Add checkpoint descriptor parsing.
4. Add object map root parsing.
