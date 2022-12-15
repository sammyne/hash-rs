//! Module crc32 implements the 32-bit cyclic redundancy check, or CRC-32, checksum.
//!
//! See <https://en.wikipedia.org/wiki/Cyclic_redundancy_check> for information.
//!
//! Polynomials are represented in LSB-first form also known as reversed representation.
//!
//! See <https://en.wikipedia.org/wiki/Mathematics_of_cyclic_redundancy_checks#Reversed_representations_and_reciprocal_polynomials> for information.
//!
use std::ops::{Deref, DerefMut};

/// Castagnoli's polynomial, used in iSCSI.
/// Has better error detection characteristics than IEEE.
/// <https://dx.doi.org/10.1109/26.231911>
pub const CASTAGNOLI: u32 = 0x82f63b78;

/// IEEE is by far and away the most common CRC-32 polynomial.
/// Used by ethernet (IEEE 802.3), v.42, fddi, gzip, zip, png, ...
pub const IEEE: u32 = 0xedb88320;

/// Koopman's polynomial.
/// Also has better error detection characteristics than IEEE.
/// <https://dx.doi.org/10.1109/DSN.2002.1028931>
pub const KOOPMAN: u32 = 0xeb31d82e;

/// The size of a CRC-32 checksum in bytes.
pub const SIZE: isize = 4;

lazy_static::lazy_static! {

  /// IEEE_TABLE is the table for the IEEE polynomial.
  pub static ref IEEE_TABLE: Table = simple::make_table(IEEE);

  static ref  CASTAGNOLI_TABLE: Table = simple::make_table(CASTAGNOLI);

  static ref CASTAGNOLI_TABLE8: Slicing8Table = slicing8::make_table(CASTAGNOLI);


  static ref IEEE_TABLE8: Slicing8Table = slicing8::make_table(IEEE);
}

/// Table is a 256-word table representing the polynomial for efficient processing.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Table(pub [u32; 256]);

impl Default for Table {
    fn default() -> Self {
        Self([0u32; 256])
    }
}

impl Deref for Table {
    type Target = [u32; 256];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// checksum returns the CRC-32 checksum of data using the polynomial represented by the Table.
pub fn checksum(data: &[u8], table: &Table) -> u32 {
    update(0, table, data)
}

/// checksum_ieee returns the CRC-32 checksum of data using the IEEE polynomial.
pub fn checksum_ieee(data: &[u8]) -> u32 {
    update_ieee(0, data)
}

/// make_table returns a Table constructed from the specified polynomial. The contents of this Table must not be
/// modified.
///
/// # Example
/// ```
#[doc = include_str!("../../examples/crc32_make_table.rs")]
/// ```
///
pub fn make_table(poly: u32) -> Table {
    match poly {
        IEEE => *IEEE_TABLE,
        CASTAGNOLI => *CASTAGNOLI_TABLE,
        _ => simple::make_table(poly),
    }
}

/// new creates a new [`Hash32`](crate::Hash32) computing the CRC-32 checksum using the polynomial represented by the Table. Its
/// [`sum`](crate::Hash::sum) method will lay the value out in big-endian byte order.
pub fn new(t: Table) -> Box<dyn crate::Hash32> {
    Box::new(Digest::new(0, t))
}

/// new_ieee creates a new [`Hash32`](crate::Hash32) computing the CRC-32 checksum using the IEEE polynomial. Its
/// [`sum`](crate::Hash::sum) method will lay the value out in big-endian byte order.
pub fn new_ieee() -> Box<dyn crate::Hash32> {
    new(*IEEE_TABLE)
}

/// update returns the result of adding the bytes in p to the crc.
pub fn update(crc: u32, t: &Table, p: &[u8]) -> u32 {
    if t.eq(&CASTAGNOLI_TABLE) {
        update_castagnoli(crc, p)
    } else if t.eq(&IEEE_TABLE) {
        update_ieee(crc, p)
    } else {
        simple::update(crc, t, p)
    }
}

fn update_castagnoli(crc: u32, p: &[u8]) -> u32 {
    slicing8::update(crc, &CASTAGNOLI_TABLE8, p)
}

fn update_ieee(crc: u32, p: &[u8]) -> u32 {
    slicing8::update(crc, &IEEE_TABLE8, p)
}

mod digest;
mod simple;
mod slicing8;

use digest::Digest;
use slicing8::Slicing8Table;

#[cfg(test)]
mod tests;
