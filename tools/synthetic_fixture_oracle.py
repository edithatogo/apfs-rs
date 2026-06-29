#!/usr/bin/env python3
"""Cargoless synthetic fixture oracle for APFS-RS.

This checks parser-development fixture structure directly from bytes. It is not a
substitute for cargo test or real APFS fixture validation, but it gives this repo a
meaningful pre-Rust quality loop in constrained environments.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import struct
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BLOCK = 4096
NXSB = b"NXSB"
APSB = b"APSB"
OBJECT_TYPE_BTREE_NODE = 0x0003
OBJECT_TYPE_OMAP = 0x000B
OBJECT_TYPE_CHECKPOINT_MAP = 0x000C
OBJECT_TYPE_FS = 0x000D


def fail(message: str) -> None:
    print(f"synthetic-fixture-oracle: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def u16(buf: bytes, off: int) -> int:
    return struct.unpack_from("<H", buf, off)[0]


def u32(buf: bytes, off: int) -> int:
    return struct.unpack_from("<I", buf, off)[0]


def u64(buf: bytes, off: int) -> int:
    return struct.unpack_from("<Q", buf, off)[0]


def block(data: bytes, idx: int) -> bytes:
    start = idx * BLOCK
    end = start + BLOCK
    if end > len(data):
        fail(f"block {idx} outside image size {len(data)}")
    return data[start:end]


def apfs_fletcher64(obj: bytes) -> int:
    if len(obj) < 12 or (len(obj) - 8) % 4:
        fail("object length is not APFS Fletcher-compatible")
    checksum_input = bytearray(obj)
    checksum_input[0:8] = b"\x00" * 8
    c0 = 0
    c1 = 0
    mod = 0xFFFFFFFF
    for off in range(0, len(checksum_input), 4):
        c0 = (c0 + u32(checksum_input, off)) % mod
        c1 = (c1 + c0) % mod
    checksum_lower = (mod - ((c0 + c1) % mod)) % mod
    checksum_upper = (mod - ((c0 + checksum_lower) % mod)) % mod
    return (checksum_upper << 32) | checksum_lower


def assert_signed(obj: bytes, label: str) -> None:
    found = u64(obj, 0)
    candidate = bytearray(obj)
    candidate[0:8] = b"\x00" * 8
    expected = apfs_fletcher64(bytes(candidate))
    if found != expected:
        fail(f"{label} checksum mismatch: found 0x{found:016x}, expected 0x{expected:016x}")


@dataclass(frozen=True)
class Mapping:
    object_type: int
    oid: int
    paddr: int


def parse_nxsb(data: bytes) -> dict:
    b0 = block(data, 0)
    if b0[32:36] != NXSB:
        fail("block zero is not synthetic NXSB")
    assert_signed(b0, "block0 NXSB")
    return {
        "xid": u64(b0, 16),
        "block_size": u32(b0, 36),
        "block_count": u64(b0, 40),
        "desc_base": u64(b0, 112),
        "desc_len": u32(b0, 140),
        "data_base": u64(b0, 120),
        "data_len": u32(b0, 148),
        "omap_oid": u64(b0, 160),
        "fs_oids": [u64(b0, 184)],
    }


def parse_checkpoint_mappings(data: bytes, nx: dict) -> list[Mapping]:
    out: list[Mapping] = []
    for idx in range(nx["desc_base"], nx["desc_base"] + min(nx["desc_len"], 8)):
        blk = block(data, idx)
        if (u32(blk, 24) & 0xFFFF) != OBJECT_TYPE_CHECKPOINT_MAP:
            continue
        assert_signed(blk, f"checkpoint map block {idx}")
        count = u32(blk, 36)
        off = 40
        for _ in range(count):
            raw_type = u32(blk, off) & 0xFFFF
            oid = u64(blk, off + 24)
            paddr = u64(blk, off + 32)
            out.append(Mapping(raw_type, oid, paddr))
            off += 40
    return out


def parse_btree_records(data: bytes, paddr: int) -> tuple[list[tuple[int,int,int]], list[tuple[int,int,int]]]:
    blk = block(data, paddr)
    if (u32(blk, 24) & 0xFFFF) != OBJECT_TYPE_BTREE_NODE:
        fail(f"block {paddr} is not a synthetic B-tree node")
    assert_signed(blk, f"B-tree block {paddr}")
    count = u32(blk, 36)
    leaf_records: list[tuple[int,int,int]] = []
    index_records: list[tuple[int,int,int]] = []
    for i in range(count):
        toc = 56 + i * 4
        key_base = 56 + u16(blk, toc)
        val_base = 56 + u16(blk, toc + 2)
        key_oid = u64(blk, key_base)
        key_xid = u64(blk, key_base + 8)
        # Synthetic leaf values have size at +4 and paddr at +8. Synthetic index values store child_oid at +0.
        value_size = u32(blk, val_base + 4)
        value_paddr = u64(blk, val_base + 8)
        child_oid = u64(blk, val_base)
        if value_size == BLOCK and value_paddr != 0:
            leaf_records.append((key_oid, key_xid, value_paddr))
        elif child_oid != 0:
            index_records.append((key_oid, key_xid, child_oid))
    return leaf_records, index_records


def lookup(records: list[tuple[int,int,int]], oid: int, xid: int) -> int | None:
    candidates = [(rxid, paddr) for roid, rxid, paddr in records if roid == oid and rxid <= xid]
    if not candidates:
        return None
    candidates.sort()
    return candidates[-1][1]


def parse_directory_records(data: bytes, paddr: int) -> dict[str, dict]:
    blk = block(data, paddr)
    assert_signed(blk, f"directory block {paddr}")
    count = u32(blk, 36)
    out: dict[str, dict] = {}
    for i in range(count):
        toc = 56 + i * 4
        val_base = 56 + u16(blk, toc + 2)
        object_id = u64(blk, val_base)
        kind = u16(blk, val_base + 8)
        name_len = u16(blk, val_base + 10)
        logical_size = u64(blk, val_base + 12)
        physical_block = u64(blk, val_base + 20)
        name = blk[val_base + 28:val_base + 28 + name_len].decode("utf-8")
        out[name] = {
            "object_id": object_id,
            "kind": kind,
            "logical_size": logical_size,
            "physical_block": physical_block,
        }
    return out


def check_manifest_hashes() -> None:
    count = 0
    for manifest_path in sorted((ROOT / "fixtures/manifests").glob("synthetic-*.json")):
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        fixture = ROOT / "fixtures" / manifest["fixture_id"]
        if not fixture.exists():
            fail(f"manifest {manifest_path.name} points to missing fixture {fixture.name}")
        digest = hashlib.sha256(fixture.read_bytes()).hexdigest()
        if manifest.get("sha256") and manifest["sha256"] != digest:
            fail(f"manifest {manifest_path.name} sha256 mismatch")
        count += 1
    print(f"synthetic-fixture-oracle: manifest hashes ok ({count})")


def check_lookup_fixtures() -> None:
    expectations = [
        ("synthetic-omap-lookup.img", 500, 50, 20),
        ("synthetic-omap-lookup.img", 500, 49, 22),
        ("synthetic-omap-multinode-lookup.img", 700, 60, 25),
        ("synthetic-omap-multinode-lookup.img", 800, 59, 27),
        ("synthetic-btree-traversal.img", 1500, 70, 33),
        ("synthetic-btree-traversal.img", 2500, 70, 35),
    ]
    for name, oid, xid, expected in expectations:
        data = (ROOT / "fixtures" / name).read_bytes()
        nx = parse_nxsb(data)
        mappings = parse_checkpoint_mappings(data, nx)
        btree_paddrs = [m.paddr for m in mappings if m.object_type == OBJECT_TYPE_BTREE_NODE]
        records: list[tuple[int,int,int]] = []
        for paddr in btree_paddrs:
            leaf, _index = parse_btree_records(data, paddr)
            records.extend(leaf)
        observed = lookup(records, oid, xid)
        if observed != expected:
            fail(f"{name} lookup oid={oid} xid={xid}: expected {expected}, observed {observed}")
    print(f"synthetic-fixture-oracle: lookup fixtures ok ({len(expectations)})")


def check_volume_and_directory() -> None:
    data = (ROOT / "fixtures/synthetic-directory-listing.img").read_bytes()
    nx = parse_nxsb(data)
    mappings = parse_checkpoint_mappings(data, nx)
    btree_paddrs = [m.paddr for m in mappings if m.object_type == OBJECT_TYPE_BTREE_NODE]
    records: list[tuple[int,int,int]] = []
    for paddr in btree_paddrs:
        leaf, _index = parse_btree_records(data, paddr)
        records.extend(leaf)
    volume_paddr = lookup(records, 1000, nx["xid"])
    if volume_paddr is None:
        fail("synthetic directory fixture does not resolve volume oid 1000")
    volume_block = block(data, volume_paddr)
    if volume_block[32:36] != APSB:
        fail("synthetic volume block is not APSB")
    assert_signed(volume_block, "synthetic volume superblock")
    root_tree_oid = u64(volume_block, 112)
    root_paddr = lookup(records, root_tree_oid, nx["xid"])
    if root_paddr is None:
        fail(f"synthetic directory fixture does not resolve root tree oid {root_tree_oid}")
    entries = parse_directory_records(data, root_paddr)
    if "hello.txt" not in entries or "Documents" not in entries:
        fail("synthetic directory listing missing expected entries")
    hello = entries["hello.txt"]
    content = block(data, hello["physical_block"])[:hello["logical_size"]]
    if content != b"Hello from APFS-RS synthetic file preview!\n":
        fail("synthetic hello.txt content mismatch")
    print("synthetic-fixture-oracle: volume/directory/file fixtures ok")


def main() -> int:
    global ROOT
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    ROOT = args.root.resolve()
    check_manifest_hashes()
    check_lookup_fixtures()
    check_volume_and_directory()
    print("synthetic-fixture-oracle: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
