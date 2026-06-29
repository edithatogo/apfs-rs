#![forbid(unsafe_code)]

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = apfs_types::parse_nx_superblock_with_checksum(data);
});
