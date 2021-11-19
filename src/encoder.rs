use crate::gf::poly_math::*;
use crate::gf::poly::Polynom;
use crate::buffer::Buffer;
use crate::gf;

/// Encoder error
#[derive(Copy, Clone)]
pub enum EncoderError {
    /// An invalid input symbol was detected. All inputs must be numbers between 0 and 31, inclusive.
    InvalidSymbol,
    /// A message was too long. Messages must be no longer than 31 symbols.
    MessageTooLong,
}

impl core::fmt::Debug for EncoderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EncoderError::InvalidSymbol =>
                write!(f, "Invalid symbol. All symbols must be be in the range [0, 31]."),
            EncoderError::MessageTooLong =>
                write!(f, "Message is too long: Length is greater than maximum of 31."),
        }
    }
}

impl core::fmt::Display for EncoderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncoderError { }

type Result<T> = core::result::Result<T, EncoderError>;

fn check_data(msg: &[u8]) -> Result<()> {
    if msg.len() > 31 {
        return Err(EncoderError::MessageTooLong);
    }
    if msg.iter().any(|&x| x > 31) {
        return Err(EncoderError::InvalidSymbol);
    }
    Ok(())
}

/// Reed-Solomon BCH encoder
#[derive(Debug, Copy, Clone)]
pub struct Encoder {
    generator: Polynom,
}

impl Encoder {
    /// Constructs a new `Encoder` and calculates generator polynomial of given `ecc_len`.
    ///
    /// # Example
    /// ```rust
    /// use reed_solomon_32::Encoder;
    ///
    /// let encoder = Encoder::new(8);
    /// ```
    pub fn new(ecc_len: usize) -> Self {
        Encoder { generator: generator_poly(ecc_len) }
    }

    /// Encodes passed `&[u8]` slice and returns `Buffer` with result and `ecc` offset.
    ///
    /// # Example
    /// ```rust
    /// use reed_solomon_32::Encoder;
    ///
    /// let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let encoder = Encoder::new(8);
    ///
    /// let encoded = encoder.encode(&data).unwrap();
    ///
    /// println!("whole: {:?}", &encoded[..]);
    /// println!("data:  {:?}", encoded.data());
    /// println!("ecc:   {:?}", encoded.ecc());
    /// ```
    pub fn encode(&self, data: &[u8]) -> Result<Buffer> {
        check_data(data)?;

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

fn generator_poly(ecclen: usize) -> Polynom {
    let mut gen = polynom![1];
    let mut mm = [1, 0];
    for i in 0..ecclen {
        mm[1] = gf::pow(2, i as i32);
        gen = gen.mul(&mm);
    }
    gen
}


#[cfg(test)]
mod tests {
    #[test]
    fn generator_poly() {
        let answers =
            [polynom![1, 1],
             polynom![1, 3, 2],
             polynom![1, 7, 14, 8],
             polynom![1, 15, 19, 23, 10],
             polynom![1, 31, 24, 15, 24, 17],
             polynom![1, 26, 20, 24, 14, 6, 31]];

        let mut ecclen = 1;
        for i in 0..6 {
            assert_eq!(*answers[i], *super::generator_poly(ecclen));
            ecclen += 1;
        }
    }

    #[test]
    fn encode() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        let ecc = [5, 10, 26, 18, 9, 22, 13, 21];

        let encoder = super::Encoder::new(ecc.len());
        let encoded = encoder.encode(&data[..]).unwrap();

        assert_eq!(data, encoded.data());
        assert_eq!(ecc, encoded.ecc());
    }

}
