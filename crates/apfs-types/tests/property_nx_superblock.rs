#![forbid(unsafe_code)]

use apfs_types::parse_nx_superblock;
use proptest::prelude::*;

fn synthetic_nxsb(block_size: u32, block_count: u64) -> Vec<u8> {
    let mut block = vec![0u8; 4096];
    block[32..36].copy_from_slice(b"NXSB");
    block[36..40].copy_from_slice(&block_size.to_le_bytes());
    block[40..48].copy_from_slice(&block_count.to_le_bytes());
    block[180..184].copy_from_slice(&1u32.to_le_bytes());
    block
}

proptest! {
    #[test]
    fn power_of_two_block_sizes_are_accepted(exp in 9u32..=20, count in 1u64..1_000_000u64) {
        let block_size = 1u32 << exp;
        let block = synthetic_nxsb(block_size, count);
        let parsed = parse_nx_superblock(&block).expect("synthetic nxsb should parse");
        prop_assert_eq!(parsed.block_size, block_size);
        prop_assert_eq!(parsed.block_count, count);
    }

    #[test]
    fn non_power_of_two_block_sizes_are_refused(size in 513u32..1_048_576u32) {
        prop_assume!(!size.is_power_of_two());
        let block = synthetic_nxsb(size, 1);
        prop_assert!(parse_nx_superblock(&block).is_err());
    }
}
