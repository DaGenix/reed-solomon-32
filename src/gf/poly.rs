#[derive(Copy)]
pub struct Polynom {
    array: [u8; crate::POLYNOMIAL_MAX_LENGTH * 3 / 2],
    length: usize,
    dirty: bool,
}

impl Polynom {
    #[inline]
    pub fn new() -> Polynom {
        Polynom {
            array: [0; crate::POLYNOMIAL_MAX_LENGTH * 3 / 2],
            length: 0,
            dirty: false,
        }
    }

    #[inline]
    pub const fn from(in_array: &[u8]) -> Polynom {
        let mut poly = Polynom {
            array: [0u8; crate::POLYNOMIAL_MAX_LENGTH * 3 / 2],
            length: in_array.len(),
            dirty: false,
        };
        if in_array.len() > poly.array.len() {
            // TODO: Make this a regular assert!() once panics in const
            //       functions are allowed: https://rust-lang.github.io/rfcs/2345-const-panic.html
            #[allow(unconditional_panic)]
            ["in_array must not be bigger than crate::POLYNOMIAL_MAX_LENGTH * 3 / 2"][1000];
        }
        // NOTE: rustc seems to be able to convert this into a memcpy for us - and the
        // assert above seems to be key in helping it do that.
        // We can't use memcpy directly, however, since its not usable in
        // const functions.
        let mut i = 0;
        while i < in_array.len() {
            poly.array[i] = in_array[i];
            i += 1;
        }
        poly
    }

    #[inline]
    pub fn with_length(len: usize) -> Polynom {
        let mut p = Polynom::new();
        p.length = len;
        p
    }

    #[inline]
    pub fn set_length(&mut self, new_len: usize) {
        let old_len = self.len();
        self.length = new_len;
        
        if self.dirty && new_len > old_len {
            for x in self.iter_mut().skip(old_len)
                                    .take(new_len - old_len) 
            {
                *x = 0;
            }
        } else if new_len < old_len {
            self.dirty = true;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn reverse(mut self) -> Self {
        (*self).reverse();
        self
    }

    #[inline]
    pub fn push(&mut self, x: u8) {
        self.array[self.length] = x;
        self.length += 1;
    }
}

impl Clone for Polynom {
    #[inline]
    fn clone(&self) -> Polynom {
        *self
    }
}

impl Default for Polynom {
    fn default() -> Self {
        Self::new()
    }
}

use core::ops::Deref;
impl Deref for Polynom {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        let len = self.len();
        &self.array[0..len]
    }
}

use core::ops::DerefMut;
impl DerefMut for Polynom {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.len();
        &mut self.array[0..len]
    }
}

use core::fmt;
impl fmt::Debug for Polynom {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", &self[..])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn push() {
        let mut poly = polynom![];
        for i in 0..10 {
            poly.push(i);
            for j in 0..(i as usize) {
                assert!(poly[j] == j as u8);
            }
        }
    }

    #[test]
    fn reverse() {
        let poly = polynom![5, 4, 3, 2, 1, 0];
        for (i, x) in poly.reverse().iter().enumerate() {
            assert_eq!(i, *x as usize);
        }
    }

    #[test]
    fn set_length() {
        let mut poly = polynom![1; 8];
        poly.set_length(2);
        poly.set_length(6);

        for i in 0..2 {
            assert_eq!(poly.array[i], 1);
        }

        for i in 2..6 {
            assert_eq!(poly.array[i], 0);
        }
    }
}
