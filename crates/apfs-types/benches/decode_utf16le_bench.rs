#![forbid(unsafe_code)]

use criterion::{criterion_group, criterion_main, Criterion};

fn decode_utf16le_name(bytes: &[u8]) -> String {
    let mut code_units = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
        if code_unit == 0 {
            break;
        }
        code_units.push(code_unit);
    }
    String::from_utf16_lossy(&code_units)
}

fn bench_decode_utf16le_name(c: &mut Criterion) {
    let mut name = String::from("This is a somewhat long name for a volume in an APFS partition, maybe it could be even longer just in case").encode_utf16().flat_map(|u| u.to_le_bytes()).collect::<Vec<u8>>();
    name.push(0);
    name.push(0);

    c.bench_function("decode_utf16le_name", |b| {
        b.iter(|| decode_utf16le_name(&name))
    });
}

criterion_group!(benches, bench_decode_utf16le_name);
criterion_main!(benches);
