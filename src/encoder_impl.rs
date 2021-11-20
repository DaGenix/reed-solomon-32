use crate::gf::poly::Polynom;
use crate::buffer::Buffer;
use crate::err::{invalid_combined_len, invalid_data_len, invalid_ecc, invalid_symbol, UsageError};
use crate::gf;

/// [`Encoder`] for messages with 0 ECC symbols
pub const ENCODER_0: Encoder = Encoder::new(polynom![1]);

/// [`Encoder`] for messages with 1 ECC symbol
pub const ENCODER_1: Encoder = Encoder::new(polynom![1, 1]);

/// [`Encoder`] for messages with 2 ECC symbols
pub const ENCODER_2: Encoder = Encoder::new(polynom![1, 3, 2]);

/// [`Encoder`] for messages with 3 ECC symbols
pub const ENCODER_3: Encoder = Encoder::new(polynom![1, 7, 14, 8]);

/// [`Encoder`] for messages with 4 ECC symbols
pub const ENCODER_4: Encoder = Encoder::new(polynom![1, 15, 19, 23, 10]);

/// [`Encoder`] for messages with 5 ECC symbols
pub const ENCODER_5: Encoder = Encoder::new(polynom![1, 31, 24, 15, 24, 17]);

/// [`Encoder`] for messages with 6 ECC symbols
pub const ENCODER_6: Encoder = Encoder::new(polynom![1, 26, 20, 24, 14, 6, 31]);

/// [`Encoder`] for messages with 7 ECC symbols
pub const ENCODER_7: Encoder = Encoder::new(polynom![1, 16, 11, 4, 5, 5, 6, 24]);

/// [`Encoder`] for messages with 8 ECC symbols
pub const ENCODER_8: Encoder = Encoder::new(polynom![1, 4, 12, 12, 31, 11, 8, 15, 22]);

/// [`Encoder`] for messages with 9 ECC symbols
pub const ENCODER_9: Encoder = Encoder::new(polynom![1, 9, 29, 26, 9, 4, 24, 8, 23, 5]);

/// [`Encoder`] for messages with 10 ECC symbols
pub const ENCODER_10: Encoder = Encoder::new(polynom![1, 19, 9, 21, 10, 16, 31, 26, 25, 21, 29]);

/// [`Encoder`] for messages with 11 ECC symbols
pub const ENCODER_11: Encoder = Encoder::new(polynom![1, 2, 2, 24, 8, 11, 2, 3, 31, 5, 31, 30]);

/// [`Encoder`] for messages with 12 ECC symbols
pub const ENCODER_12: Encoder = Encoder::new(polynom![1, 5, 12, 22, 10, 22, 22, 13, 22, 18, 4, 9, 16]);

/// [`Encoder`] for messages with 13 ECC symbols
pub const ENCODER_13: Encoder = Encoder::new(polynom![1, 11, 31, 20, 16, 21, 12, 23, 26, 8, 3, 20, 1, 27]);

/// [`Encoder`] for messages with 14 ECC symbols
pub const ENCODER_14: Encoder = Encoder::new(polynom![1, 23, 5, 2, 28, 6, 28, 19, 23, 29, 24, 21, 13, 7, 9]);

/// [`Encoder`] for messages with 15 ECC symbols
pub const ENCODER_15: Encoder = Encoder::new(polynom![1, 10, 31, 4, 3, 13, 24, 24, 22, 7, 14, 5, 8, 18, 16, 14]);

/// [`Encoder`] for messages with 16 ECC symbols
pub const ENCODER_16: Encoder = Encoder::new(polynom![1, 21, 7, 22, 16, 9, 23, 29, 19, 9, 25, 14, 4, 17, 13, 8, 11]);

/// [`Encoder`] for messages with 17 ECC symbols
pub const ENCODER_17: Encoder = Encoder::new(polynom![1, 14, 19, 29, 12, 5, 10, 26, 1, 13, 4, 31, 18, 18, 26, 22, 13, 14]);

/// [`Encoder`] for messages with 18 ECC symbols
pub const ENCODER_18: Encoder = Encoder::new(polynom![1, 29, 26, 21, 13, 15, 31, 21, 22, 30, 29, 25, 16, 9, 1, 1, 16, 23, 9]);

/// [`Encoder`] for messages with 19 ECC symbols
pub const ENCODER_19: Encoder = Encoder::new(polynom![1, 30, 24, 30, 23, 24, 14, 17, 12, 1, 26, 27, 30, 28, 26, 2, 19, 2, 21, 27]);

/// [`Encoder`] for messages with 20 ECC symbols
pub const ENCODER_20: Encoder = Encoder::new(polynom![1, 24, 22, 4, 25, 5, 20, 16, 5, 12, 28, 13, 14, 18, 24, 20, 31, 7, 25, 10, 16]);

/// [`Encoder`] for messages with 21 ECC symbols
pub const ENCODER_21: Encoder = Encoder::new(polynom![1, 20, 7, 23, 12, 24, 13, 27, 27, 21, 6, 9, 24, 16, 30, 5, 20, 23, 24, 23, 7, 30]);

/// [`Encoder`] for messages with 22 ECC symbols
pub const ENCODER_22: Encoder = Encoder::new(polynom![1, 12, 17, 21, 23, 9, 10, 18, 17, 31, 8, 19, 30, 23, 7, 24, 3, 1, 3, 16, 28, 28, 29]);

/// [`Encoder`] for messages with 23 ECC symbols
pub const ENCODER_23: Encoder = Encoder::new(polynom![1, 25, 22, 23, 11, 26, 6, 4, 9, 29, 2, 10, 19, 8, 20, 28, 13, 27, 22, 10, 11, 12, 13, 5]);

/// [`Encoder`] for messages with 24 ECC symbols
pub const ENCODER_24: Encoder = Encoder::new(polynom![1, 22, 5, 27, 8, 28, 4, 3, 16, 5, 8, 20, 26, 18, 3, 14, 8, 26, 27, 6, 2, 10, 3, 4, 22]);

/// [`Encoder`] for messages with 25 ECC symbols
pub const ENCODER_25: Encoder = Encoder::new(polynom![1, 8, 29, 18, 18, 23, 14, 20, 23, 19, 1, 31, 27, 22, 12, 9, 13, 17, 31, 28, 12, 19, 17, 3, 1, 24]);

/// [`Encoder`] for messages with 26 ECC symbols
pub const ENCODER_26: Encoder = Encoder::new(polynom![1, 17, 11, 31, 12, 9, 2, 30, 21, 31, 6, 6, 1, 7, 25, 20, 2, 21, 15, 6, 24, 14, 22, 19, 15, 1, 31]);

/// [`Encoder`] for messages with 27 ECC symbols
pub const ENCODER_27: Encoder = Encoder::new(polynom![1, 6, 14, 10, 29, 22, 28, 21, 19, 12, 23, 27, 28, 16, 19, 24, 6, 30, 28, 5, 5, 21, 2, 28, 1, 2, 8, 17]);

/// [`Encoder`] for messages with 28 ECC symbols
pub const ENCODER_28: Encoder = Encoder::new(polynom![1, 13, 17, 7, 25, 7, 2, 15, 16, 16, 12, 14, 18, 10, 18, 4, 21, 1, 16, 31, 7, 23, 1, 10, 27, 9, 30, 3, 10]);

/// [`Encoder`] for messages with 29 ECC symbols
pub const ENCODER_29: Encoder = Encoder::new(polynom![1, 27, 20, 19, 20, 18, 15, 6, 28, 18, 14, 29, 8, 1, 26, 15, 7, 7, 6, 29, 9, 26, 14, 28, 19, 21, 9, 27, 21, 8]);

/// [`Encoder`] for messages with 30 ECC symbols
pub const ENCODER_30: Encoder = Encoder::new(polynom![1, 18, 9, 22, 11, 23, 25, 30, 15, 21, 24, 12, 6, 3, 19, 27, 31, 29, 28, 14, 7, 17, 26, 13, 20, 10, 5, 16, 8, 4, 2]);

/// Reed-Solomon BCH encoder
#[derive(Debug, Copy, Clone)]
pub struct Encoder {
    generator: Polynom,
}

impl Encoder {
    const fn new(generator: Polynom) -> Self {
        Encoder { generator }
    }

    /// Encodes passed `&[u8]` slice and returns `Buffer` with result.
    ///
    /// The number of ecc symbols used will depend on `Encoder` constant
    /// that was used.
    ///
    /// # Example
    /// ```rust
    /// use reed_solomon_32::encoder::ENCODER_8;
    ///
    /// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///
    /// let encoded = ENCODER_8.encode(&data).unwrap();
    ///
    /// println!("whole: {:?}", &encoded[..]);
    /// println!("data:  {:?}", encoded.data());
    /// println!("ecc:   {:?}", encoded.ecc());
    /// ```
    pub fn encode(&self, data: &[u8]) -> Result<Buffer, UsageError> {
        if data.len() > 31 {
            return Err(invalid_data_len());
        }
        if data.len() + self.generator.len() - 1 > 31 {
            return Err(invalid_combined_len());
        }
        if data.iter().any(|&x| x > 31) {
            return Err(invalid_symbol());
        }

        let mut data_out = Polynom::from(data);
        let data_len = data.len();

        data_out.set_length(data_len + self.generator.len() - 1);

        let gen = self.generator;
        let mut lgen = Polynom::with_length(self.generator.len());
        for (i, gen_i) in gen.iter().enumerate() {
            lgen[i] = gf::LOG[*gen_i as usize];
        }

        for i in 0..data_len {
            let coef = data_out[i];
            if coef != 0 {
                let lcoef = gf::LOG[coef as usize] as usize;
                for j in 1..gen.len() {
                    data_out[i + j] ^= gf::EXP[(lcoef + lgen[j] as usize)];
                }
            }
        }

        data_out[..data_len].copy_from_slice(data);
        Ok(Buffer::from_polynom(data_out, data_len))
    }
}

/// Encodes passed `&[u8]` slice and returns `Buffer` with result using
/// `ecc` error correcting symbols.
///
/// # Example
/// ```rust
/// use reed_solomon_32::encode;
///
/// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// let encoded = encode(&data, 8).unwrap();
///
/// println!("whole: {:?}", &encoded[..]);
/// println!("data:  {:?}", encoded.data());
/// println!("ecc:   {:?}", encoded.ecc());
/// ```
pub fn encode(data: &[u8], ecc: usize) -> Result<Buffer, UsageError> {
    match ecc {
        0 => ENCODER_0.encode(data),
        1 => ENCODER_1.encode(data),
        2 => ENCODER_2.encode(data),
        3 => ENCODER_3.encode(data),
        4 => ENCODER_4.encode(data),
        5 => ENCODER_5.encode(data),
        6 => ENCODER_6.encode(data),
        7 => ENCODER_7.encode(data),
        8 => ENCODER_8.encode(data),
        9 => ENCODER_9.encode(data),
        10 => ENCODER_10.encode(data),
        11 => ENCODER_11.encode(data),
        12 => ENCODER_12.encode(data),
        13 => ENCODER_13.encode(data),
        14 => ENCODER_14.encode(data),
        15 => ENCODER_15.encode(data),
        16 => ENCODER_16.encode(data),
        17 => ENCODER_17.encode(data),
        18 => ENCODER_18.encode(data),
        19 => ENCODER_19.encode(data),
        20 => ENCODER_20.encode(data),
        21 => ENCODER_21.encode(data),
        22 => ENCODER_22.encode(data),
        23 => ENCODER_23.encode(data),
        24 => ENCODER_24.encode(data),
        25 => ENCODER_25.encode(data),
        26 => ENCODER_26.encode(data),
        27 => ENCODER_27.encode(data),
        28 => ENCODER_28.encode(data),
        29 => ENCODER_29.encode(data),
        30 => ENCODER_30.encode(data),
        _ => Err(invalid_ecc()),
    }
}

#[cfg(test)]
mod tests {
    use crate::gf::poly_math::Mul as _;
    use crate::gf::poly::Polynom;
    use crate::gf;

    #[test]
    fn generator_poly() {
        fn generator_poly(ecclen: usize) -> Polynom {
            let mut gen = polynom![1];
            let mut mm = [1, 0];
            for i in 0..ecclen {
                mm[1] = gf::pow(2, i as i32);
                gen = gen.mul(&mm);
            }
            gen
        }

        assert_eq!(&super::ENCODER_0.generator[..], &generator_poly(0)[..]);
        assert_eq!(&super::ENCODER_1.generator[..], &generator_poly(1)[..]);
        assert_eq!(&super::ENCODER_2.generator[..], &generator_poly(2)[..]);
        assert_eq!(&super::ENCODER_3.generator[..], &generator_poly(3)[..]);
        assert_eq!(&super::ENCODER_4.generator[..], &generator_poly(4)[..]);
        assert_eq!(&super::ENCODER_5.generator[..], &generator_poly(5)[..]);
        assert_eq!(&super::ENCODER_6.generator[..], &generator_poly(6)[..]);
        assert_eq!(&super::ENCODER_7.generator[..], &generator_poly(7)[..]);
        assert_eq!(&super::ENCODER_8.generator[..], &generator_poly(8)[..]);
        assert_eq!(&super::ENCODER_9.generator[..], &generator_poly(9)[..]);
        assert_eq!(&super::ENCODER_10.generator[..], &generator_poly(10)[..]);
        assert_eq!(&super::ENCODER_11.generator[..], &generator_poly(11)[..]);
        assert_eq!(&super::ENCODER_12.generator[..], &generator_poly(12)[..]);
        assert_eq!(&super::ENCODER_13.generator[..], &generator_poly(13)[..]);
        assert_eq!(&super::ENCODER_14.generator[..], &generator_poly(14)[..]);
        assert_eq!(&super::ENCODER_15.generator[..], &generator_poly(15)[..]);
        assert_eq!(&super::ENCODER_16.generator[..], &generator_poly(16)[..]);
        assert_eq!(&super::ENCODER_17.generator[..], &generator_poly(17)[..]);
        assert_eq!(&super::ENCODER_18.generator[..], &generator_poly(18)[..]);
        assert_eq!(&super::ENCODER_19.generator[..], &generator_poly(19)[..]);
        assert_eq!(&super::ENCODER_20.generator[..], &generator_poly(20)[..]);
        assert_eq!(&super::ENCODER_21.generator[..], &generator_poly(21)[..]);
        assert_eq!(&super::ENCODER_22.generator[..], &generator_poly(22)[..]);
        assert_eq!(&super::ENCODER_23.generator[..], &generator_poly(23)[..]);
        assert_eq!(&super::ENCODER_24.generator[..], &generator_poly(24)[..]);
        assert_eq!(&super::ENCODER_25.generator[..], &generator_poly(25)[..]);
        assert_eq!(&super::ENCODER_26.generator[..], &generator_poly(26)[..]);
        assert_eq!(&super::ENCODER_27.generator[..], &generator_poly(27)[..]);
        assert_eq!(&super::ENCODER_28.generator[..], &generator_poly(28)[..]);
        assert_eq!(&super::ENCODER_29.generator[..], &generator_poly(29)[..]);
        assert_eq!(&super::ENCODER_30.generator[..], &generator_poly(30)[..]);
    }

    #[test]
    fn encode() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        let ecc = [5, 10, 26, 18, 9, 22, 13, 21];

        let encoded = super::encode(&data[..], ecc.len()).unwrap();

        assert_eq!(data, encoded.data());
        assert_eq!(ecc, encoded.ecc());
    }
}
