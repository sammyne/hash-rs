use std::io::Write;

use crate::crc32::{self, Table, CASTAGNOLI_TABLE, IEEE_TABLE, SIZE};
use crate::{Hash, Hash32};

pub struct Digest {
    crc: u32,
    table: Table,
}

impl Digest {
    pub fn new(crc: u32, table: Table) -> Self {
        Self { crc, table }
    }
}

impl Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.crc = if self.table.eq(&CASTAGNOLI_TABLE) {
            crc32::update_castagnoli(self.crc, buf)
        } else if self.table.eq(&IEEE_TABLE) {
            crc32::update_ieee(self.crc, buf)
        } else {
            crc32::simple::update(self.crc, &self.table, buf)
        };

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Hash for Digest {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8> {
        let s = self.sum32().to_be_bytes();
        match b {
            Some(v) => {
                let mut v = v;
                v.extend_from_slice(&s);
                v
            }
            None => s.to_vec(),
        }
    }

    fn reset(&mut self) {
        self.crc = 0
    }

    fn size(&self) -> usize {
        SIZE
    }

    fn block_size(&self) -> isize {
        1
    }
}

impl Hash32 for Digest {
    fn sum32(&mut self) -> u32 {
        self.crc
    }
}
