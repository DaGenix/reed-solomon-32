use ::gf::poly::Polynom;
use core::ops::{Deref, DerefMut};

/// Buffer for block encoded data
#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    poly: Polynom,
    data_len: usize,
}

impl Buffer {
    /// Create buffer from internal polynom
    pub(crate) fn from_polynom(poly: Polynom, data_len: usize) -> Self {
        Buffer {
            poly: poly,
            data_len: data_len,
        }
    }

    /// Create buffer from [u8] slice
    pub(crate) fn from_slice(slice: &[u8], data_len: usize) -> Self {
        Buffer {
            poly: Polynom::from(slice),
            data_len: data_len,
        }
    }

    /// Slice with data of encoded block
    pub fn data(&self) -> &[u8] {
        &self[..self.data_len]
    }

    /// Slice with error correction core of encoced block
    pub fn ecc(&self) -> &[u8] {
        &self[self.data_len..]
    }
}

impl Deref for Buffer {
    type Target = Polynom;
    fn deref(&self) -> &Self::Target {
        &self.poly
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.poly
    }
}

impl From<Polynom> for Buffer {
    fn from(p: Polynom) -> Buffer {
        Buffer {
            data_len: p.len(),
            poly: p,
        }
    }
}
