#![forbid(unsafe_code)]


use apfs_types::parse_nx_superblock;
use criterion::{criterion_group, criterion_main, Criterion};

fn synthetic_nxsb() -> Vec<u8> {
    let mut block = vec![0u8; 4096];
    block[32..36].copy_from_slice(b"NXSB");
    block[36..40].copy_from_slice(&4096u32.to_le_bytes());
    block[40..48].copy_from_slice(&16u64.to_le_bytes());
    block[180..184].copy_from_slice(&1u32.to_le_bytes());
    block
}

fn bench_parse_nxsb(c: &mut Criterion) {
    let block = synthetic_nxsb();
    c.bench_function("parse_nx_superblock_synthetic", |b| b.iter(|| parse_nx_superblock(&block)));
}

criterion_group!(benches, bench_parse_nxsb);
criterion_main!(benches);
