# Reed-Solomon BCH
[![Crates.io](https://img.shields.io/crates/v/reed-solomon-32.svg)](https://crates.io/crates/reed-solomon-32)
[![Documentation](https://docs.rs/reed-solomon-32/badge.svg)](https://docs.rs/reed-solomon-32)

Reed-Solomon BCH encoder and decoder implemented in Rust.

This is a port of python implementation from [Wikiversity](https://en.wikiversity.org/wiki/Reed–Solomon_codes_for_coders)


This is a fork of <https://github.com/mersinvald/reed-solomon-rs> - the primary change
being that it operates in GF(2^5) instead of GF(2^8).

## Setup 

```toml
[dependencies]
reed-solomon-32 = "^1.0.0"
```

## Example

```rust
use reed_solomon_32::Encoder;
use reed_solomon_32::Decoder;

fn main() {
    let data = b"Hello World!";

    // Length of error correction code
    let ecc_len = 8;

    // Create encoder and decoder with 
    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let orig_str = std::str::from_utf8(data).unwrap();
    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("message:               {:?}", orig_str);
    println!("original data:         {:?}", data);
    println!("error correction code: {:?}", encoded.ecc());
    println!("corrupted:             {:?}", corrupted);
    println!("repaired:              {:?}", recv_str);
}
```
