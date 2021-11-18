//! Operations over GF(2^5), using pre-calculated tables for 0x25 primitive polynomial
pub mod poly;
pub mod poly_math;

const EXP_SIZE: usize = 62;
pub const EXP: [u8; EXP_SIZE] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x5, 0xa, 0x14, 0xd, 0x1a, 0x11, 0x7, 0xe, 0x1c, 0x1d, 0x1f, 0x1b,
    0x13, 0x3, 0x6, 0xc, 0x18, 0x15, 0xf, 0x1e, 0x19, 0x17, 0xb, 0x16, 0x9, 0x12, 0x1, 0x2, 0x4,
    0x8, 0x10, 0x5, 0xa, 0x14, 0xd, 0x1a, 0x11, 0x7, 0xe, 0x1c, 0x1d, 0x1f, 0x1b, 0x13, 0x3, 0x6,
    0xc, 0x18, 0x15, 0xf, 0x1e, 0x19, 0x17, 0xb, 0x16, 0x9, 0x12,
];

const LOG_SIZE: usize = 32;
pub const LOG: [u8; LOG_SIZE] = [
    0x0, 0x0, 0x1, 0x12, 0x2, 0x5, 0x13, 0xb, 0x3, 0x1d, 0x6, 0x1b, 0x14, 0x8, 0xc, 0x17, 0x4, 0xa,
    0x1e, 0x11, 0x7, 0x16, 0x1c, 0x1a, 0x15, 0x19, 0x9, 0x10, 0xd, 0xe, 0x18, 0xf,
];

/// Primitive operations over Galua Fields

#[allow(dead_code)]
#[inline]
pub fn add(x: u8, y: u8) -> u8 {
    x ^ y
}

#[inline]
pub fn sub(x: u8, y: u8) -> u8 {
    x ^ y
}

#[inline]
pub fn mul(x: u8, y: u8) -> u8 {
    if x == 0 || y == 0 {
        0
    } else {
        let log_x = LOG[x as usize];
        let log_y = LOG[y as usize];
        let exp_index = log_x as usize + 
                        log_y as usize;

        EXP[exp_index]
    }
}

#[inline]
pub fn div(x: u8, y: u8) -> u8 {
    debug_assert!(y != 0);
    if x == 0 {
        0
    } else {
        let log_x = LOG[x as usize] as usize;
        let log_y = LOG[y as usize] as usize;
        let exp_index = (log_x + 31 - log_y) % 31;

        EXP[exp_index]
    }
}

#[inline]
pub fn pow(x: u8, power: i32) -> u8 {
    let mut i = LOG[x as usize] as i32
            * power
            % 31;

    if i < 0 {
        i += 31;
    }

    EXP[i as usize]
}

#[inline]
pub fn inverse(x: u8) -> u8 {
    let exp_index = 31 - LOG[x as usize];
    EXP[exp_index as usize]
}

#[cfg(test)]
mod tests {
    use super::EXP;
    use super::LOG;
    use super::LOG_SIZE;

    #[test]
    fn add() {
        let answers: [u8; LOG_SIZE] = [
            1, 2, 5, 26, 18, 0, 25, 31, 14, 7, 23, 28, 26, 20, 17, 8,
            31, 25, 29, 23, 11, 14, 9, 21, 11, 0, 30, 27, 27, 7, 10, 14,
        ];

        for i in 0..LOG_SIZE {
            assert_eq!(super::add(LOG[i], EXP[i]), answers[i]);
        }
    }

    #[test]
    fn sub() {
        add();
    }

    #[test]
    fn mul() {
        let answers: [u8; LOG_SIZE] = [
            0, 0, 4, 4, 5, 17, 15, 8, 23, 15, 9, 11, 6, 27, 8, 17,
            3, 15, 7, 9, 1, 3, 16, 2, 31, 6, 30, 1, 5, 17, 12, 15,
        ];

        for i in 0..LOG_SIZE {
            assert_eq!(super::mul(LOG[i], EXP[i]), answers[i]);
        }
    }

    #[test]
    fn div() {
        let answers: [u8; LOG_SIZE] = [
            0, 0, 9, 11, 22, 1, 7, 12, 17, 5, 26, 5, 23, 24, 10, 7,
            19, 12, 10, 21, 21, 20, 21, 19, 9, 1, 8, 13, 7, 29, 21, 15,
        ];

        for i in 0..LOG_SIZE {
            assert_eq!(super::div(LOG[i], EXP[i]), answers[i]);
        }
    }

    #[test]
    fn pow() {
        let answers: [u8; LOG_SIZE] = [
            1, 1, 1, 15, 27, 25, 31, 28, 19, 15, 28, 6, 5, 21, 21, 1,
            15, 24, 17, 9, 13, 24, 25, 7, 26, 5, 27, 28, 24, 31, 10, 15,
        ];

        for i in 0..LOG_SIZE {
            assert_eq!(super::pow(LOG[i], EXP[i] as i32), answers[i]);
        }
    }
}