#![forbid(unsafe_code)]

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = apfs_types::parse_object_header(data);
});
