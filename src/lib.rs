//! Reed-Solomon BCH encoder and decoder suitable for `no_std` environment in GF(2^5).
//!
//! This is a fork of <https://github.com/mersinvald/reed-solomon-rs> - the primary change
//! being that it operates in GF(2^5) instead of GF(2^8).
//!
//! This library implements block encoder and decoder: error correction code is appended to original data.
//!
//! # Example
//! ```rust
//! use reed_solomon_32::Encoder;
//! use reed_solomon_32::Decoder;
//!
//! fn main() {
//!     let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
//!
//!     // Length of error correction code
//!     let ecc_len = 8;
//!
//!     // Create encoder and decoder with
//!     let enc = Encoder::new(ecc_len);
//!     let dec = Decoder::new(ecc_len);
//!
//!     // Encode data
//!     let encoded = enc.encode(&data).unwrap();
//!
//!     // Simulate some transmission errors
//!     let mut corrupted = *encoded;
//!     for i in 0..4 {
//!         corrupted[i] = 0x0;
//!     }
//!
//!     // Try to recover data
//!     let known_erasures = [0];
//!     let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();
//!
//!     let orig_str = std::str::from_utf8(&data).unwrap();
//!     let recv_str = std::str::from_utf8(recovered.data()).unwrap();
//!
//!     println!("message:               {:?}", orig_str);
//!     println!("original data:         {:?}", data);
//!     println!("error correction code: {:?}", encoded.ecc());
//!     println!("corrupted:             {:?}", corrupted);
//!     println!("repaired:              {:?}", recv_str);
//! }
//! ```

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![warn(missing_docs, missing_debug_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features)]

#![no_std]

#[cfg(feature = "std")]
extern crate std;

const POLYNOMIAL_MAX_LENGTH: usize = 31;

#[macro_use]
mod macros;
mod gf;
mod encoder;
mod decoder;
mod buffer;

pub use encoder::Encoder;
pub use encoder::EncoderError;
pub use decoder::Decoder;
pub use decoder::DecoderError;
pub use buffer::Buffer;
