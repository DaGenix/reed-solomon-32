//! Reed-Solomon BCH encoder and decoder suitable for `no_std` environment in GF(2^5).
//!
//! This is a fork of <https://github.com/mersinvald/reed-solomon-rs> - the primary change
//! being that it operates in GF(2^5) instead of GF(2^8).
//!
//! This library implements block encoder and decoder: error correction code is appended to original data.
//!
//! # Example
//! ```rust
//! use reed_solomon_32::encode;
//! use reed_solomon_32::correct;
//!
//! fn main() {
//!     let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
//!
//!     // Length of error correction code
//!     let ecc_len = 8;
//!
//!     // Encode data
//!     let encoded = encode(&data, ecc_len).unwrap();
//!
//!     // Simulate some transmission errors
//!     let mut corrupted = *encoded;
//!     for i in 0..4 {
//!         corrupted[i] = 0x0;
//!     }
//!
//!     // Try to recover data
//!     let known_erasures = [0];
//!     let recovered = correct(&mut corrupted, ecc_len, Some(&known_erasures)).unwrap();
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

#![cfg_attr(not(feature = "std"), no_std)]

const POLYNOMIAL_MAX_LENGTH: usize = 31;

#[macro_use]
mod macros;
mod gf;
mod encoder_impl;
mod decoder_impl;
mod buffer;
mod err;

pub use encoder_impl::encode;
pub use decoder_impl::{correct, correct_err_count, is_corrupted};
pub use err::{UsageError, CorrectionError, UsageErrorMessage};
pub use buffer::Buffer;

pub mod encoder {
    //! This is a specialized module and generally the [`encode`](crate::encode)
    //! function should be preferred.
    //!
    //! The downside of the `encode` function is that it will cause around 500 bytes of
    //! data tables to be included in the final binary - with the majority of that data
    //! only being useful for larger ECC sizes that are unlikely to be used (eg,
    //! the table for ECC size 30 takes up 31 bytes, the table for ECC size 29 takes
    //! up 30 bytes - but neither makes much sense in practical use).
    //!
    //! By using the constants in this module, at LTO pass may be able to remove unused
    //! tables from the final binary. Not all targets will be able to take practical
    //! advantage of this, however.
    pub use crate::encoder_impl::{
        Encoder,
        ENCODER_0,
        ENCODER_1,
        ENCODER_2,
        ENCODER_3,
        ENCODER_4,
        ENCODER_5,
        ENCODER_6,
        ENCODER_7,
        ENCODER_8,
        ENCODER_9,
        ENCODER_10,
        ENCODER_11,
        ENCODER_12,
        ENCODER_13,
        ENCODER_14,
        ENCODER_15,
        ENCODER_16,
        ENCODER_17,
        ENCODER_18,
        ENCODER_19,
        ENCODER_20,
        ENCODER_21,
        ENCODER_22,
        ENCODER_23,
        ENCODER_24,
        ENCODER_25,
        ENCODER_26,
        ENCODER_27,
        ENCODER_28,
        ENCODER_29,
        ENCODER_30,
    };
}

pub mod decoder {
    //! This is a specialized module and generally [`correct_err_count`](crate::correct_err_count),
    //! [`correct`](crate::correct), and [`is_corrupted`](crate::is_corrupted) functions
    //! should be preferred.
    //!
    //! As of the current version of this crate, this module exists for future proofing
    //! and to mirror the encode APIs. In the future, using these functions _may_
    //! help decrease binary size. However, currently they do not significantly do so.
    pub use crate::decoder_impl::{
        Decoder,
        DECODER_0,
        DECODER_1,
        DECODER_2,
        DECODER_3,
        DECODER_4,
        DECODER_5,
        DECODER_6,
        DECODER_7,
        DECODER_8,
        DECODER_9,
        DECODER_10,
        DECODER_11,
        DECODER_12,
        DECODER_13,
        DECODER_14,
        DECODER_15,
        DECODER_16,
        DECODER_17,
        DECODER_18,
        DECODER_19,
        DECODER_20,
        DECODER_21,
        DECODER_22,
        DECODER_23,
        DECODER_24,
        DECODER_25,
        DECODER_26,
        DECODER_27,
        DECODER_28,
        DECODER_29,
        DECODER_30,
    };
}
