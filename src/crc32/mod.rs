use std::ops::{Deref, DerefMut};

pub const IEEE: u32 = 0xedb88320;

pub const CASTAGNOLI: u32 = 0x82f63b78;

pub const SIZE: usize = 4;

lazy_static::lazy_static! {

  static ref  CASTAGNOLI_TABLE: Table = simple::make_table(CASTAGNOLI);

  static ref CASTAGNOLI_TABLE8: Slicing8Table = slicing8::make_table(CASTAGNOLI);

  static ref IEEE_TABLE: Table = simple::make_table(IEEE);

  static ref IEEE_TABLE8: Slicing8Table = slicing8::make_table(IEEE);
}

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

pub fn checksum(data: &[u8], table: &Table) -> u32 {
    todo!()
}

pub fn checksum_ieee(data: &[u8]) -> u32 {
    update_ieee(0, data)
}

pub fn make_table(poly: u32) -> Table {
    todo!()
}

pub fn new(t: &Table) -> Box<dyn crate::Hash> {
    todo!()
}

pub fn new_ieee(t: &Table) -> Box<dyn crate::Hash> {
    todo!()
}

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

mod simple;
mod slicing8;

use slicing8::Slicing8Table;

#[cfg(test)]
mod tests;
