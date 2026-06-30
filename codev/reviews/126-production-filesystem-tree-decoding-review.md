# Review M-126: Production filesystem tree decoding and metadata mapping

## Status

`implemented`.

## Notes

Filesystem metadata mapping is now implemented for decoded directory entries: the `stat` command emits a structured metadata object derived from the directory record, and the regression coverage proves the mapping helper and CLI JSON contract. This remains bounded read-side reporting rather than full production APFS inode/stat decoding.
