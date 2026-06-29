#![forbid(unsafe_code)]

use apfs_core::inspect_bytes;
use criterion::{criterion_group, criterion_main, Criterion};

fn synthetic_nxsb() -> Vec<u8> {
    let mut block = vec![0u8; 4096];
    block[32..36].copy_from_slice(b"NXSB");
    block[36..40].copy_from_slice(&4096u32.to_le_bytes());
    block[40..48].copy_from_slice(&1024u64.to_le_bytes());
    block[180..184].copy_from_slice(&1u32.to_le_bytes());
    block
}

fn bench_inspect_synthetic(c: &mut Criterion) {
    let fixture = synthetic_nxsb();
    c.bench_function("inspect_synthetic_nxsb", |b| {
        b.iter(|| inspect_bytes(&fixture))
    });
}

criterion_group!(benches, bench_inspect_synthetic);
criterion_main!(benches);
