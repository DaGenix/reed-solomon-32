extern crate reed_solomon_32;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reed_solomon_32::{Buffer, Decoder, Encoder};

struct TestEncode {
    ecc: usize,
    message: [u8; 31],
    message_len: usize,
}

fn test_encode(params: TestEncode) -> Buffer {
    let encoder = Encoder::new(params.ecc);
    encoder.encode(&params.message[..params.message_len]).unwrap()
}

struct TestDecode {
    ecc: usize,
    encoded: [u8; 31],
    encoded_len: usize,
}

fn test_decode(params: TestDecode) -> Buffer {
    let decoder = Decoder::new(params.ecc);
    decoder.correct(&params.encoded[..params.encoded_len], None).unwrap()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encode", |b| b.iter(|| test_encode(black_box(TestEncode {
        ecc: 8,
        message: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        message_len: 20,
    }))));
    c.bench_function("decode-no-errors", |b| b.iter(|| test_decode(black_box(TestDecode {
        ecc: 8,
        encoded: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 14, 24, 6, 5, 1, 8, 24, 0, 0, 0],
        encoded_len: 28,
    }))));
    c.bench_function("decode-1-error", |b| b.iter(|| test_decode(black_box(TestDecode {
        ecc: 8,
        encoded: [31, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 14, 24, 6, 5, 1, 8, 24, 0, 0, 0],
        encoded_len: 28,
    }))));
    c.bench_function("decode-max-error", |b| b.iter(|| test_decode(black_box(TestDecode {
        ecc: 8,
        encoded: [31, 31, 31, 31, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 14, 24, 6, 5, 1, 8, 24, 0, 0, 0],
        encoded_len: 28,
    }))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
