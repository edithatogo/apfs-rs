# Review M-127: Production file extent resolution and extraction

## Status

`implemented`.

## Notes

This roadmap track now has fixture-backed read-only extraction evidence. The CLI writes only to the requested host destination directory, resolves synthetic extent records for the extraction fixture, and refuses unsafe traversal names. No APFS media writes are introduced, and production APFS compatibility claims remain bounded to the specific fixture and tests executed here.
