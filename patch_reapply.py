import re

with open('crates/apfs-types/src/lib.rs', 'r') as f:
    content = f.read()

tests = """

    #[test]
    fn validates_gpt_entries_checksum_correct() {
        let entries = b"some mock partition entries";
        let expected_crc32 = crc32_ieee(entries);

        let header = GptHeader {
            revision: 0,
            header_size: 0,
            stored_header_crc32: 0,
            computed_header_crc32: 0,
            header_crc32_valid: true,
            current_lba: 0,
            backup_lba: 0,
            first_usable_lba: 0,
            last_usable_lba: 0,
            disk_guid: String::new(),
            partition_entry_lba: 0,
            number_of_partition_entries: 0,
            size_of_partition_entry: 0,
            stored_partition_entries_crc32: expected_crc32,
        };

        let result = validate_gpt_entries_checksum(entries, &header);

        assert!(result.valid);
        assert_eq!(result.stored_crc32, expected_crc32);
        assert_eq!(result.computed_crc32, expected_crc32);
        assert_eq!(result.checked_bytes, entries.len());
    }

    #[test]
    fn validates_gpt_entries_checksum_incorrect() {
        let entries = b"some mock partition entries";
        let expected_crc32 = crc32_ieee(entries);
        let incorrect_crc32 = expected_crc32.wrapping_add(1);

        let header = GptHeader {
            revision: 0,
            header_size: 0,
            stored_header_crc32: 0,
            computed_header_crc32: 0,
            header_crc32_valid: true,
            current_lba: 0,
            backup_lba: 0,
            first_usable_lba: 0,
            last_usable_lba: 0,
            disk_guid: String::new(),
            partition_entry_lba: 0,
            number_of_partition_entries: 0,
            size_of_partition_entry: 0,
            stored_partition_entries_crc32: incorrect_crc32,
        };

        let result = validate_gpt_entries_checksum(entries, &header);

        assert!(!result.valid);
        assert_eq!(result.stored_crc32, incorrect_crc32);
        assert_eq!(result.computed_crc32, expected_crc32);
        assert_eq!(result.checked_bytes, entries.len());
    }
}"""

content = re.sub(r'}\n}$', tests, content)

with open('crates/apfs-types/src/lib.rs', 'w') as f:
    f.write(content)
