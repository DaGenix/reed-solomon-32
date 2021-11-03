extern crate reed_solomon_32;

use reed_solomon_32::Encoder;
use reed_solomon_32::Decoder;

fn main() {
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Length of error correction code
    let ecc_len = 8;

    // Create encoder and decoder with 
    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&data).unwrap();

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for x in corrupted.iter_mut().take(4) {
        *x = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];
    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let orig_str = std::str::from_utf8(&data).unwrap();
    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("message:               {:?}", orig_str);
    println!("original data:         {:?}", data);
    println!("error correction code: {:?}", encoded.ecc());
    println!("corrupted:             {:?}", corrupted);
    println!("repaired:              {:?}", recv_str);
}