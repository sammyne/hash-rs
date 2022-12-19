//! Module crc64 implements the 64-bit cyclic redundancy check, or CRC-64, checksum.
//!
//! See <https://en.wikipedia.org/wiki/Cyclic_redundancy_check> for information.
//!

use std::ops::{Deref, DerefMut};

use crate::Hash64;

/// The ECMA polynomial, defined in ECMA 182.
pub const ECMA: u64 = 0xC96C5795D7870F42;

/// The ISO polynomial, defined in ISO 3309 and used in HDLC.
pub const ISO: u64 = 0xD800000000000000;

/// The size of a CRC-64 checksum in bytes.
pub const SIZE: usize = 8;

lazy_static::lazy_static! {
    static ref SLICING8_TABLE_ECMA: [Table;8] =  slicing8::make_table(Table::from_poly(ECMA));

    static ref SLICING8_TABLE_ISO: [Table;8] = slicing8::make_table(Table::from_poly(ISO)) ;
}

/// Table is a 256-word table representing the polynomial for efficient processing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Table(pub [u64; 256]);

impl Table {
    fn from_poly(poly: u64) -> Self {
        let mut out = Self::default();

        for i in 0usize..256 {
            let mut crc = i as u64;
            for _j in 0..8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ poly
                } else {
                    crc >> 1
                }
            }
            out[i] = crc;
        }

        out
    }
}

impl Default for Table {
    fn default() -> Self {
        Self([0u64; 256])
    }
}

impl Deref for Table {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

/// checksum returns the CRC-64 checksum of data using the polynomial represented by the [Table].
///
/// # Example
/// ```
#[doc = include_str!("../../examples/crc64_checksum.rs")]
/// ```
pub fn checksum(data: &[u8], table: &Table) -> u64 {
    update(0, table, data)
}

/// make_table returns a [Table] constructed from the specified polynomial. The contents of this Table must not be
/// modified.
pub fn make_table(poly: u64) -> Table {
    match poly {
        ISO => SLICING8_TABLE_ISO[0],
        ECMA => SLICING8_TABLE_ECMA[0],
        _ => Table::from_poly(poly),
    }
}

/// new creates a new [hash::Hash64][crate::Hash] computing the CRC-64 checksum using the polynomial represented by the
/// [Table]. Its [sum](crate::Hash::sum) method will lay the value out in big-endian byte order.
pub fn new(table: Table) -> Box<dyn Hash64> {
    Box::new(digest::Digest::new(0, table))
}

/// update returns the result of adding the bytes in p to the crc.
pub fn update(crc: u64, table: &Table, p: &[u8]) -> u64 {
    let mut crc = !crc;

    let mut p = p;
    while p.len() >= 64 {
        let helper_table: [Table; 8] = if SLICING8_TABLE_ECMA[0].eq(table) {
            *SLICING8_TABLE_ECMA
        } else if SLICING8_TABLE_ISO[0].eq(table) {
            *SLICING8_TABLE_ISO
        } else if p.len() > 16384 {
            slicing8::make_table(*table)
        } else {
            break;
        };

        while p.len() > 8 {
            crc ^= (p[0] as u64)
                | ((p[1] as u64) << 8)
                | ((p[2] as u64) << 16)
                | ((p[3] as u64) << 24)
                | ((p[4] as u64) << 32)
                | ((p[5] as u64) << 40)
                | ((p[6] as u64) << 48)
                | ((p[7] as u64) << 56);

            crc = helper_table[7][(crc & 0xff) as usize]
                ^ helper_table[6][((crc >> 8) & 0xff) as usize]
                ^ helper_table[5][((crc >> 16) & 0xff) as usize]
                ^ helper_table[4][((crc >> 24) & 0xff) as usize]
                ^ helper_table[3][((crc >> 32) & 0xff) as usize]
                ^ helper_table[2][((crc >> 40) & 0xff) as usize]
                ^ helper_table[1][((crc >> 48) & 0xff) as usize]
                ^ helper_table[0][((crc >> 56) & 0xff) as usize];

            p = &p[8..];
        }
    }

    for &v in p {
        crc = table[((crc as u8) ^ v) as usize] ^ (crc >> 8);
    }

    !crc
}

mod digest;
mod slicing8;

#[cfg(test)]
mod tests;
