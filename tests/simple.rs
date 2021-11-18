extern crate reed_solomon_32;

use reed_solomon_32::encode;
use reed_solomon_32::Decoder;

const ECC_LEN: usize = 8;

#[test]
fn helloworld() {
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Create encoder and decoder
    let dec = Decoder::new(ECC_LEN);

    // Encode data
    let encoded = encode(&data[..], ECC_LEN).unwrap();

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let recovered = dec.correct(&mut corrupted, None).unwrap();

    assert_eq!(data, recovered.data());
}

#[test]
fn with_erasures() {
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Create encoder and decoder
    let dec = Decoder::new(ECC_LEN);

    // Encode data
    let encoded = encode(&data[..], ECC_LEN).unwrap();

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0, 1, 2];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    assert_eq!(data, recovered.data());
}
