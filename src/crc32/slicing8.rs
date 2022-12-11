use std::ops::{Deref, DerefMut};

use crate::crc32::{simple, Table};

pub const SLICING8_CUTOFF: usize = 16;

#[derive(Default)]
pub struct Slicing8Table(pub [Table; 8]);

impl Deref for Slicing8Table {
    type Target = [Table; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Slicing8Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn make_table(poly: u32) -> Slicing8Table {
    let mut out = Slicing8Table::default();
    simple::populate_table(poly, &mut out[0]);

    for i in 0usize..256 {
        let mut crc = out[0][i];
        for j in 1usize..8 {
            crc = out[0][(crc & 0xff) as usize] ^ (crc >> 8);
            out[j][i] = crc
        }
    }

    /*
    fn show(t: &Table, i: usize) {
        print!("[{i}] ");
        for v in t.iter() {
            print!("{v} ");
        }
        println!();
    }
    for (i, v) in out.iter().enumerate() {
        show(v, i);
    }
    */

    out
}

pub fn update(crc: u32, t: &Slicing8Table, p: &[u8]) -> u32 {
    let (mut crc, mut p) = (crc, p);
    if p.len() >= SLICING8_CUTOFF {
        crc = !crc;
        while p.len() > 8 {
            crc ^= (p[0] as u32)
                | ((p[1] as u32) << 8)
                | ((p[2] as u32) << 16)
                | ((p[3] as u32) << 24);
            crc = t[0][p[7] as usize]
                ^ t[1][p[6] as usize]
                ^ t[2][p[5] as usize]
                ^ t[3][p[4] as usize]
                ^ t[4][(crc >> 24) as usize]
                ^ t[5][((crc >> 16) & 0xff) as usize]
                ^ t[6][((crc >> 8) & 0xff) as usize]
                ^ t[7][(crc & 0xff) as usize];
            p = &p[8..];
        }
        crc = !crc;
    }
    if p.len() == 0 {
        return crc;
    }

    simple::update(crc, &t[0], p)
}
