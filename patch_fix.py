with open('crates/apfs-types/src/lib.rs', 'r') as f:
    content = f.read()

content = content.replace("assert_eq!(selected.selected_child_oid, Some(101));\n    \n\n    #[test]", "assert_eq!(selected.selected_child_oid, Some(101));\n    }\n\n    #[test]")

with open('crates/apfs-types/src/lib.rs', 'w') as f:
    f.write(content)
