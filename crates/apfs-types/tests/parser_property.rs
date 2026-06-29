#![forbid(unsafe_code)]

use apfs_types::{parse_nx_superblock, parse_object_header};
use proptest::prelude::*;

proptest! {
    #[test]
    fn object_header_parser_never_panics(data in proptest::collection::vec(any::<u8>(), 0..8192)) {
        let _ = parse_object_header(&data);
    }

    #[test]
    fn nx_superblock_parser_never_panics(data in proptest::collection::vec(any::<u8>(), 0..8192)) {
        let _ = parse_nx_superblock(&data);
    }
}
