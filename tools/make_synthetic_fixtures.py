#!/usr/bin/env python3
"""Generate synthetic parser-development fixtures.

These are not complete APFS filesystems. They contain enough APFS/GPT/checkpoint
structure to exercise read-only parser code.
"""
from __future__ import annotations

import binascii
import hashlib
import json
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures"
MANIFESTS = FIXTURES / "manifests"
SCHEMA_VERSION = "0.15.0"
SOURCE_TYPE = "synthetic_apfs_parser_development_image"
BLOCK_SIZE = 4096
GPT_SECTOR_SIZE = 512
APFS_TYPE_GUID = bytes.fromhex("ef57347c0000aa11aa1100306543ecac")
OBJ_EPHEMERAL = 0x80000000
OBJECT_TYPE_NX_SUPERBLOCK = 0x0001
OBJECT_TYPE_BTREE_NODE = 0x0003
OBJECT_TYPE_OMAP = 0x000B
OBJECT_TYPE_CHECKPOINT_MAP = 0x000C
OBJECT_TYPE_FS = 0x000D
CHECKPOINT_MAP_LAST = 0x00000001
BTREE_NODE_ROOT = 0x0001
BTREE_NODE_LEAF = 0x0002


def apfs_fletcher64(block: bytes) -> int:
    if len(block) < 12 or (len(block) - 8) % 4 != 0:
        raise ValueError("APFS checksum input must be 4-byte aligned after checksum field")
    checksum_input = bytearray(block)
    checksum_input[0:8] = b"\x00" * 8
    c0 = 0
    c1 = 0
    modulus = 0xFFFFFFFF
    for offset in range(0, len(checksum_input), 4):
        value = struct.unpack_from("<I", checksum_input, offset)[0]
        c0 = (c0 + value) % modulus
        c1 = (c1 + c0) % modulus
    checksum_lower = (modulus - ((c0 + c1) % modulus)) % modulus
    checksum_upper = (modulus - ((c0 + checksum_lower) % modulus)) % modulus
    return (checksum_upper << 32) | checksum_lower


def put_u32(buf: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<I", buf, offset, value)


def put_u64(buf: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<Q", buf, offset, value)


def sign_apfs_object(block: bytearray) -> None:
    block[0:8] = b"\x00" * 8
    put_u64(block, 0, apfs_fletcher64(bytes(block)))


def make_nxsb(*, xid: int, desc_base: int = 0, desc_len: int = 0, data_base: int = 0, data_len: int = 0, fs_oid: int = 42, block_count: int = 64, omap_oid: int = 12) -> bytes:
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, 1)  # OID_NX_SUPERBLOCK
    put_u64(block, 16, xid)
    put_u32(block, 24, OBJ_EPHEMERAL | OBJECT_TYPE_NX_SUPERBLOCK)
    block[32:36] = b"NXSB"
    put_u32(block, 36, BLOCK_SIZE)
    put_u64(block, 40, block_count)
    block[72:88] = bytes.fromhex("00112233445566778899aabbccddeeff")
    put_u64(block, 88, 1024)
    put_u64(block, 96, xid + 1)
    put_u32(block, 104, desc_len)
    put_u32(block, 108, data_len)
    put_u64(block, 112, desc_base)
    put_u64(block, 120, data_base)
    put_u32(block, 128, 0)
    put_u32(block, 132, 0)
    put_u32(block, 136, 0)
    put_u32(block, 140, desc_len)
    put_u32(block, 144, 0)
    put_u32(block, 148, data_len)
    put_u64(block, 152, 11)
    put_u64(block, 160, omap_oid)
    put_u64(block, 168, 13)
    put_u32(block, 176, 0)
    put_u32(block, 180, 1)
    put_u64(block, 184, fs_oid)
    sign_apfs_object(block)
    return bytes(block)


def make_checkpoint_map(*, xid: int, oid: int, mappings: list[tuple[int, int, int, int, int]]) -> bytes:
    """Create a checkpoint_map_phys_t block.

    mappings are tuples of (object_type, subtype, size_bytes, oid, paddr).
    """
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, OBJ_EPHEMERAL | OBJECT_TYPE_CHECKPOINT_MAP)
    put_u32(block, 32, CHECKPOINT_MAP_LAST)
    put_u32(block, 36, len(mappings))
    offset = 40
    for object_type, subtype, size_bytes, object_oid, paddr in mappings:
        put_u32(block, offset + 0, OBJ_EPHEMERAL | object_type)
        put_u32(block, offset + 4, subtype)
        put_u32(block, offset + 8, size_bytes)
        put_u32(block, offset + 12, 0)
        put_u64(block, offset + 16, 0)  # container-scoped mapping
        put_u64(block, offset + 24, object_oid)
        put_u64(block, offset + 32, paddr)
        offset += 40
    sign_apfs_object(block)
    return bytes(block)


def make_omap(*, xid: int, oid: int = 12, tree_oid: int = 99) -> bytes:
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, OBJ_EPHEMERAL | OBJECT_TYPE_OMAP)
    put_u32(block, 32, 1)  # OMAP_MANUALLY_MANAGED for container omap in this synthetic fixture
    put_u32(block, 36, 0)
    put_u32(block, 40, 0x80000003)  # synthetic B-tree node storage/type marker
    put_u32(block, 44, 0)
    put_u64(block, 48, tree_oid)
    put_u64(block, 56, 0)
    put_u64(block, 64, 0)
    put_u64(block, 72, 0)
    put_u64(block, 80, 0)
    sign_apfs_object(block)
    return bytes(block)


def make_btree_root_node(*, xid: int, oid: int = 99, key_count: int = 2) -> bytes:
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | OBJECT_TYPE_BTREE_NODE)
    struct.pack_into("<H", block, 32, BTREE_NODE_ROOT | BTREE_NODE_LEAF)
    struct.pack_into("<H", block, 34, 0)  # leaf level
    put_u32(block, 36, key_count)
    struct.pack_into("<H", block, 40, 0)
    struct.pack_into("<H", block, 42, key_count * 4)
    # Two synthetic TOC entries. The parser intentionally reports offsets only;
    # it does not yet interpret OMAP keys/values.
    for index in range(key_count):
        key_offset = 128 + index * 32
        value_offset = 256 + index * 32
        base = 56 + index * 4
        struct.pack_into("<H", block, base + 0, key_offset)
        struct.pack_into("<H", block, base + 2, value_offset)
        key_base = 56 + key_offset
        value_base = 56 + value_offset
        put_u64(block, key_base + 0, 500 + index)
        put_u64(block, key_base + 8, xid)
        put_u32(block, value_base + 0, 0)
        put_u32(block, value_base + 4, BLOCK_SIZE)
        put_u64(block, value_base + 8, 20 + index)
    sign_apfs_object(block)
    return bytes(block)


def make_direct_nxsb() -> bytes:
    return make_nxsb(xid=10, desc_base=0, desc_len=0)


FIXTURE_CAPABILITIES: dict[str, list[str]] = {
    "synthetic-nxsb-block0.bin": ["M-001", "M-003"],
    "synthetic-gpt-apfs.img": ["M-002"],
    "synthetic-checkpoint-ring.img": ["M-004"],
    "synthetic-checkpoint-map-omap.img": ["M-004"],
    "synthetic-omap-btree-root.img": ["M-005"],
    "synthetic-omap-lookup.img": ["M-006"],
    "synthetic-omap-multinode-lookup.img": ["M-007"],
    "synthetic-btree-traversal.img": ["M-008"],
    "synthetic-resolver-facade.img": ["M-009"],
    "synthetic-btree-cursor.img": ["M-010"],
    "synthetic-volume-superblock.img": ["M-014"],
    "synthetic-mapped-object-read.img": ["M-015"],
    "synthetic-directory-listing.img": ["M-016", "M-017", "M-019"],
    "synthetic-file-preview.img": ["M-018", "M-020"],
}


def make_gpt_apfs_image() -> bytes:
    total_sectors = 160
    image = bytearray(total_sectors * GPT_SECTOR_SIZE)
    apfs_first_lba = 40
    apfs_last_lba = total_sectors - 1

    image[510:512] = b"\x55\xaa"

    entries = bytearray(128 * 128)
    entries[0:16] = APFS_TYPE_GUID
    entries[16:32] = bytes.fromhex("1032547698badcfe0011223344556677")
    put_u64(entries, 32, apfs_first_lba)
    put_u64(entries, 40, apfs_last_lba)
    name = "APFS-RS Synthetic".encode("utf-16le")
    entries[56:56 + len(name)] = name
    entries_crc = binascii.crc32(entries) & 0xFFFFFFFF
    image[2 * GPT_SECTOR_SIZE:2 * GPT_SECTOR_SIZE + len(entries)] = entries

    header = bytearray(GPT_SECTOR_SIZE)
    header[0:8] = b"EFI PART"
    put_u32(header, 8, 0x00010000)
    put_u32(header, 12, 92)
    put_u64(header, 24, 1)
    put_u64(header, 32, total_sectors - 1)
    put_u64(header, 40, 4)
    put_u64(header, 48, total_sectors - 2)
    header[56:72] = bytes.fromhex("8899aabbccddeeff0011223344556677")
    put_u64(header, 72, 2)
    put_u32(header, 80, 128)
    put_u32(header, 84, 128)
    put_u32(header, 88, entries_crc)
    header_crc = binascii.crc32(header[:92]) & 0xFFFFFFFF
    put_u32(header, 16, header_crc)
    image[GPT_SECTOR_SIZE:2 * GPT_SECTOR_SIZE] = header

    nxsb = make_nxsb(xid=20, desc_base=0, desc_len=0, block_count=16)
    apfs_offset = apfs_first_lba * GPT_SECTOR_SIZE
    image[apfs_offset:apfs_offset + len(nxsb)] = nxsb
    return bytes(image)




def make_btree_root_node_records(*, xid: int, oid: int = 99, records: list[tuple[int, int, int]]) -> bytes:
    """Create a synthetic OMAP B-tree root/leaf node from (oid, xid, paddr) records."""
    block = bytearray(BLOCK_SIZE)
    key_count = len(records)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | OBJECT_TYPE_BTREE_NODE)
    struct.pack_into("<H", block, 32, BTREE_NODE_ROOT | BTREE_NODE_LEAF)
    struct.pack_into("<H", block, 34, 0)
    put_u32(block, 36, key_count)
    struct.pack_into("<H", block, 40, 0)
    struct.pack_into("<H", block, 42, key_count * 4)
    for index, (record_oid, record_xid, paddr) in enumerate(records):
        key_offset = 128 + index * 32
        value_offset = 512 + index * 32
        base = 56 + index * 4
        struct.pack_into("<H", block, base + 0, key_offset)
        struct.pack_into("<H", block, base + 2, value_offset)
        key_base = 56 + key_offset
        value_base = 56 + value_offset
        put_u64(block, key_base + 0, record_oid)
        put_u64(block, key_base + 8, record_xid)
        put_u32(block, value_base + 0, 0)
        put_u32(block, value_base + 4, BLOCK_SIZE)
        put_u64(block, value_base + 8, paddr)
    sign_apfs_object(block)
    return bytes(block)

def make_checkpoint_ring_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(16)]
    blocks[0][:] = make_nxsb(xid=10, desc_base=2, desc_len=4, block_count=16)
    blocks[2][:] = make_nxsb(xid=11, desc_base=2, desc_len=4, block_count=16)
    blocks[3][:] = make_nxsb(xid=15, desc_base=2, desc_len=4, block_count=16)
    blocks[4][32:36] = b"NOPE"
    return b"".join(blocks)


def make_checkpoint_map_omap_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(24)]
    blocks[0][:] = make_nxsb(xid=30, desc_base=2, desc_len=4, data_base=10, data_len=4, block_count=24, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=30,
        oid=200,
        mappings=[(OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10)],
    )
    blocks[3][:] = make_nxsb(xid=31, desc_base=2, desc_len=4, data_base=10, data_len=4, block_count=24, omap_oid=12)
    blocks[4][32:36] = b"NOPE"
    blocks[10][:] = make_omap(xid=30, oid=12, tree_oid=99)
    return b"".join(blocks)




def make_omap_btree_root_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(32)]
    blocks[0][:] = make_nxsb(xid=40, desc_base=2, desc_len=4, data_base=10, data_len=8, block_count=32, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=40,
        oid=201,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
        ],
    )
    blocks[3][:] = make_nxsb(xid=41, desc_base=2, desc_len=4, data_base=10, data_len=8, block_count=32, omap_oid=12)
    blocks[10][:] = make_omap(xid=40, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_root_node(xid=40, oid=99, key_count=2)
    return b"".join(blocks)


def make_omap_lookup_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(36)]
    blocks[0][:] = make_nxsb(xid=50, desc_base=2, desc_len=4, data_base=10, data_len=12, block_count=36, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=50,
        oid=202,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
        ],
    )
    blocks[3][:] = make_nxsb(xid=51, desc_base=2, desc_len=4, data_base=10, data_len=12, block_count=36, omap_oid=12)
    blocks[10][:] = make_omap(xid=50, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_root_node_records(
        xid=50,
        oid=99,
        records=[
            (500, 45, 22),
            (500, 50, 20),
            (501, 50, 21),
        ],
    )
    return b"".join(blocks)


def make_btree_index_root_node(*, xid: int, oid: int = 99, child_ranges: list[tuple[int, int, int]]) -> bytes:
    """Create a synthetic non-leaf OMAP B-tree root node.

    child_ranges are tuples of (highest_oid, highest_xid, child_oid). The current Rust
    implementation reports this node and then uses checkpoint-map-backed leaf scanning;
    it does not yet interpret the child pointers as a real APFS traversal contract.
    """
    block = bytearray(BLOCK_SIZE)
    key_count = len(child_ranges)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | OBJECT_TYPE_BTREE_NODE)
    struct.pack_into("<H", block, 32, BTREE_NODE_ROOT)
    struct.pack_into("<H", block, 34, 1)  # synthetic internal/root level
    put_u32(block, 36, key_count)
    struct.pack_into("<H", block, 40, 0)
    struct.pack_into("<H", block, 42, key_count * 4)
    for index, (record_oid, record_xid, child_oid) in enumerate(child_ranges):
        key_offset = 128 + index * 32
        value_offset = 512 + index * 32
        base = 56 + index * 4
        struct.pack_into("<H", block, base + 0, key_offset)
        struct.pack_into("<H", block, base + 2, value_offset)
        key_base = 56 + key_offset
        value_base = 56 + value_offset
        put_u64(block, key_base + 0, record_oid)
        put_u64(block, key_base + 8, record_xid)
        put_u64(block, value_base + 0, child_oid)
    sign_apfs_object(block)
    return bytes(block)


def make_omap_multinode_lookup_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(48)]
    blocks[0][:] = make_nxsb(xid=60, desc_base=2, desc_len=4, data_base=10, data_len=20, block_count=48, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=60,
        oid=203,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 100, 12),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 101, 13),
        ],
    )
    blocks[3][:] = make_nxsb(xid=61, desc_base=2, desc_len=4, data_base=10, data_len=20, block_count=48, omap_oid=12)
    blocks[10][:] = make_omap(xid=60, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_index_root_node(
        xid=60,
        oid=99,
        child_ranges=[
            (700, 60, 100),
            (800, 60, 101),
        ],
    )
    blocks[12][:] = make_btree_root_node_records(
        xid=60,
        oid=100,
        records=[
            (700, 55, 24),
            (700, 60, 25),
            (701, 60, 26),
        ],
    )
    # This synthetic node is intentionally not marked ROOT by rewriting the flags after creation.
    leaf_b = bytearray(blocks[12])
    struct.pack_into("<H", leaf_b, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf_b)
    blocks[12][:] = leaf_b

    blocks[13][:] = make_btree_root_node_records(
        xid=60,
        oid=101,
        records=[
            (800, 58, 27),
            (800, 60, 28),
        ],
    )
    leaf_c = bytearray(blocks[13])
    struct.pack_into("<H", leaf_c, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf_c)
    blocks[13][:] = leaf_c
    return b"".join(blocks)



def make_bounded_btree_traversal_image() -> bytes:
    """Synthetic two-level OMAP B-tree fixture for bounded traversal.

    The root has three synthetic index records that select one of three mapped leaves.
    Lookup should follow the selected child instead of aggregating unrelated leaves.
    """
    blocks = [bytearray(BLOCK_SIZE) for _ in range(64)]
    blocks[0][:] = make_nxsb(xid=70, desc_base=2, desc_len=4, data_base=10, data_len=28, block_count=64, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=70,
        oid=204,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 110, 12),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 111, 13),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 112, 14),
        ],
    )
    blocks[3][:] = make_nxsb(xid=71, desc_base=2, desc_len=4, data_base=10, data_len=28, block_count=64, omap_oid=12)
    blocks[10][:] = make_omap(xid=70, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_index_root_node(
        xid=70,
        oid=99,
        child_ranges=[
            (1000, 70, 110),
            (2000, 70, 111),
            (3000, 70, 112),
        ],
    )
    blocks[12][:] = make_btree_root_node_records(
        xid=70,
        oid=110,
        records=[
            (900, 70, 30),
            (1000, 70, 31),
        ],
    )
    leaf_a = bytearray(blocks[12])
    struct.pack_into("<H", leaf_a, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf_a)
    blocks[12][:] = leaf_a

    blocks[13][:] = make_btree_root_node_records(
        xid=70,
        oid=111,
        records=[
            (1500, 65, 32),
            (1500, 70, 33),
            (1999, 70, 34),
        ],
    )
    leaf_b = bytearray(blocks[13])
    struct.pack_into("<H", leaf_b, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf_b)
    blocks[13][:] = leaf_b

    blocks[14][:] = make_btree_root_node_records(
        xid=70,
        oid=112,
        records=[
            (2500, 70, 35),
            (3000, 70, 36),
        ],
    )
    leaf_c = bytearray(blocks[14])
    struct.pack_into("<H", leaf_c, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf_c)
    blocks[14][:] = leaf_c
    return b"".join(blocks)


def make_volume_superblock(*, xid: int, oid: int = 1000, name: str = "SyntheticHD", role: int = 0x0040, root_tree_oid: int = 2000) -> bytes:
    """Create a synthetic apfs_superblock_t-like block.

    This is a parser-development volume superblock, not a complete APFS volume.
    The Rust parser intentionally reads a conservative field subset from stable
    synthetic offsets while real macOS fixture validation is pending.
    """
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | OBJECT_TYPE_FS)
    block[32:36] = b"APSB"
    put_u32(block, 36, 0)  # fs index
    put_u64(block, 40, 0)  # features
    put_u64(block, 48, 0)  # readonly-compatible features
    put_u64(block, 56, 0)  # incompatible features
    put_u64(block, 64, 0)  # unmount time
    put_u64(block, 72, 0)  # reserve blocks
    put_u64(block, 80, 0)  # quota blocks
    put_u64(block, 88, 8)  # allocated blocks
    put_u64(block, 112, root_tree_oid)
    put_u64(block, 120, 2001)  # extent ref tree oid
    put_u64(block, 128, 2002)  # snapshot metadata tree oid
    put_u64(block, 152, 3000)  # next object id
    put_u64(block, 160, 3)     # file count
    put_u64(block, 168, 2)     # directory count
    put_u64(block, 176, 1)     # symlink count
    put_u64(block, 184, 0)     # other objects
    put_u64(block, 192, 0)     # snapshots
    put_u64(block, 200, 8)     # blocks alloced
    put_u64(block, 208, 0)     # blocks freed
    block[216:232] = bytes.fromhex("102030405060708090a0b0c0d0e0f000")
    put_u64(block, 232, 0)
    put_u64(block, 240, 0)
    encoded_name = name.encode("utf-8")[:255]
    block[512:512 + len(encoded_name)] = encoded_name
    struct.pack_into("<H", block, 768, role)
    sign_apfs_object(block)
    return bytes(block)


def make_volume_superblock_image() -> bytes:
    """Synthetic image that resolves an NX filesystem OID to an APSB-like volume block."""
    blocks = [bytearray(BLOCK_SIZE) for _ in range(72)]
    blocks[0][:] = make_nxsb(xid=80, desc_base=2, desc_len=4, data_base=10, data_len=40, block_count=72, fs_oid=1000, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=80,
        oid=205,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 110, 12),
        ],
    )
    blocks[3][:] = make_nxsb(xid=81, desc_base=2, desc_len=4, data_base=10, data_len=40, block_count=72, fs_oid=1000, omap_oid=12)
    blocks[10][:] = make_omap(xid=80, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_index_root_node(
        xid=80,
        oid=99,
        child_ranges=[
            (2000, 80, 110),
        ],
    )
    blocks[12][:] = make_btree_root_node_records(
        xid=80,
        oid=110,
        records=[
            (1000, 80, 30),
        ],
    )
    leaf = bytearray(blocks[12])
    struct.pack_into("<H", leaf, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf)
    blocks[12][:] = leaf
    blocks[30][:] = make_volume_superblock(xid=80, oid=1000, name="SyntheticHD", role=0x0040, root_tree_oid=2000)
    return b"".join(blocks)



def make_generic_mapped_object(*, xid: int, oid: int, object_type: int = 0x1234, payload: bytes = b"APFS-RS synthetic mapped object") -> bytes:
    """Create a signed generic APFS-like object block for resolver-backed read tests."""
    block = bytearray(BLOCK_SIZE)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | object_type)
    block[64:64 + len(payload)] = payload
    sign_apfs_object(block)
    return bytes(block)


def make_mapped_object_read_image() -> bytes:
    """Synthetic image where OMAP lookup resolves object 1500 to a signed generic object block."""
    image = bytearray(make_bounded_btree_traversal_image())
    image[33 * BLOCK_SIZE:(33 + 1) * BLOCK_SIZE] = make_generic_mapped_object(xid=70, oid=1500, object_type=0x1234)
    image[35 * BLOCK_SIZE:(35 + 1) * BLOCK_SIZE] = make_generic_mapped_object(xid=70, oid=2500, object_type=0x1234, payload=b"APFS-RS second synthetic mapped object")
    return bytes(image)


def stable_name_hash(name: str) -> int:
    return binascii.crc32(name.encode("utf-8")) & 0xFFFFFFFF


def make_synthetic_directory_node(*, xid: int, oid: int = 2000, entries: list[tuple[int, str, int, int, int, int]]) -> bytes:
    """Create a synthetic filesystem-root B-tree leaf node.

    entries are tuples of (parent_id, name, object_id, item_kind, logical_size, physical_block).
    This is not APFS production record encoding; it gives the Rust parser a stable,
    bounded directory-record facade until real APFS filesystem record decoding is validated.
    """
    block = bytearray(BLOCK_SIZE)
    key_count = len(entries)
    put_u64(block, 8, oid)
    put_u64(block, 16, xid)
    put_u32(block, 24, 0x40000000 | OBJECT_TYPE_BTREE_NODE)
    struct.pack_into("<H", block, 32, BTREE_NODE_ROOT | BTREE_NODE_LEAF)
    struct.pack_into("<H", block, 34, 0)
    put_u32(block, 36, key_count)
    struct.pack_into("<H", block, 40, 0)
    struct.pack_into("<H", block, 42, key_count * 4)
    for index, (parent_id, name, object_id, item_kind, logical_size, physical_block) in enumerate(entries):
        encoded_name = name.encode("utf-8")
        key_offset = 128 + index * 32
        value_offset = 512 + index * 128
        toc_base = 56 + index * 4
        struct.pack_into("<H", block, toc_base + 0, key_offset)
        struct.pack_into("<H", block, toc_base + 2, value_offset)
        key_base = 56 + key_offset
        value_base = 56 + value_offset
        put_u64(block, key_base + 0, parent_id)
        put_u64(block, key_base + 8, stable_name_hash(name))
        put_u64(block, value_base + 0, object_id)
        struct.pack_into("<H", block, value_base + 8, item_kind)
        struct.pack_into("<H", block, value_base + 10, len(encoded_name))
        put_u64(block, value_base + 12, logical_size)
        put_u64(block, value_base + 20, physical_block)
        block[value_base + 28:value_base + 28 + len(encoded_name)] = encoded_name
    sign_apfs_object(block)
    return bytes(block)


def make_directory_listing_image() -> bytes:
    blocks = [bytearray(BLOCK_SIZE) for _ in range(96)]
    blocks[0][:] = make_nxsb(xid=90, desc_base=2, desc_len=4, data_base=10, data_len=48, block_count=96, fs_oid=1000, omap_oid=12)
    blocks[2][:] = make_checkpoint_map(
        xid=90,
        oid=206,
        mappings=[
            (OBJECT_TYPE_OMAP, 0, BLOCK_SIZE, 12, 10),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 99, 11),
            (OBJECT_TYPE_BTREE_NODE, 0, BLOCK_SIZE, 110, 12),
        ],
    )
    blocks[3][:] = make_nxsb(xid=91, desc_base=2, desc_len=4, data_base=10, data_len=48, block_count=96, fs_oid=1000, omap_oid=12)
    blocks[10][:] = make_omap(xid=90, oid=12, tree_oid=99)
    blocks[11][:] = make_btree_index_root_node(xid=90, oid=99, child_ranges=[(3000, 90, 110)])
    blocks[12][:] = make_btree_root_node_records(
        xid=90,
        oid=110,
        records=[
            (1000, 90, 30),
            (2000, 90, 31),
        ],
    )
    leaf = bytearray(blocks[12])
    struct.pack_into("<H", leaf, 32, BTREE_NODE_LEAF)
    sign_apfs_object(leaf)
    blocks[12][:] = leaf
    blocks[30][:] = make_volume_superblock(xid=90, oid=1000, name="SyntheticHD", role=0x0040, root_tree_oid=2000)
    content = b"Hello from APFS-RS synthetic file preview!\n"
    blocks[31][:] = make_synthetic_directory_node(
        xid=90,
        oid=2000,
        entries=[
            (2, "hello.txt", 4000, 8, len(content), 40),
            (2, "Documents", 4001, 4, 0, 0),
        ],
    )
    blocks[40][0:len(content)] = content
    return b"".join(blocks)

def write_fixture(name: str, data: bytes, description: str) -> None:
    path = FIXTURES / name
    path.write_bytes(data)
    manifest = {
        "fixture_id": name,
        "schema_version": SCHEMA_VERSION,
        "source_type": SOURCE_TYPE,
        "description": description,
        "created_with": {
            "tool": "tools/make_synthetic_fixtures.py",
            "note": "synthetic parser-development fixture",
        },
        "expected_artifacts": [],
        "synthetic": True,
        "not_a_complete_filesystem": True,
        "apfs_features": {
            "synthetic": True,
            "not_a_complete_filesystem": True,
        },
        "capability_ids": FIXTURE_CAPABILITIES[name],
        "redaction": {
            "contains_personal_data": False,
            "contains_secret_material": False,
        },
        "size_bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
    }
    (MANIFESTS / f"{name}.json").write_text(json.dumps(manifest, indent=2) + "\n")


def main() -> None:
    FIXTURES.mkdir(exist_ok=True)
    MANIFESTS.mkdir(exist_ok=True)
    write_fixture("synthetic-nxsb-block0.bin", make_direct_nxsb(), "Direct APFS-like NXSB at block zero with valid APFS checksum")
    write_fixture("synthetic-gpt-apfs.img", make_gpt_apfs_image(), "GPT image with APFS partition type and valid APFS checksum")
    write_fixture("synthetic-checkpoint-ring.img", make_checkpoint_ring_image(), "Direct APFS-like image with checkpoint descriptor NXSB candidates")
    write_fixture("synthetic-checkpoint-map-omap.img", make_checkpoint_map_omap_image(), "Direct APFS-like image with a checkpoint map entry that maps the container OMAP")
    write_fixture("synthetic-omap-btree-root.img", make_omap_btree_root_image(), "Direct APFS-like image with checkpoint mappings for the container OMAP and its B-tree root node")
    write_fixture("synthetic-omap-lookup.img", make_omap_lookup_image(), "Direct APFS-like image with a single-node OMAP lookup fixture containing multiple XIDs")
    write_fixture("synthetic-omap-multinode-lookup.img", make_omap_multinode_lookup_image(), "Direct APFS-like image with checkpoint-map-backed OMAP lookup records across multiple synthetic B-tree leaf nodes")
    traversal = make_bounded_btree_traversal_image()
    write_fixture("synthetic-btree-traversal.img", traversal, "Direct APFS-like image with a bounded synthetic two-level OMAP B-tree traversal fixture")
    write_fixture("synthetic-resolver-facade.img", traversal, "Direct APFS-like image exercising the object-map resolver facade over bounded synthetic B-tree traversal")
    write_fixture("synthetic-btree-cursor.img", traversal, "Direct APFS-like image exercising the production-shaped B-tree cursor API over bounded synthetic traversal")
    write_fixture("synthetic-volume-superblock.img", make_volume_superblock_image(), "Direct APFS-like image resolving a container filesystem OID to a synthetic APSB volume superblock")
    write_fixture("synthetic-mapped-object-read.img", make_mapped_object_read_image(), "Direct APFS-like image resolving synthetic OMAP records to signed generic object blocks for read-object reporting")
    directory_image = make_directory_listing_image()
    write_fixture("synthetic-directory-listing.img", directory_image, "Direct APFS-like image resolving a synthetic APFS volume root tree to directory entries")
    write_fixture("synthetic-file-preview.img", directory_image, "Direct APFS-like image resolving synthetic directory entries and previewing a direct-block file payload")


if __name__ == "__main__":
    main()
