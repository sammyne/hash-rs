use std::io::Write;

use crate::crc64::{self, Table, SIZE};
use crate::{Hash, Hash64};

pub struct Digest {
    crc: u64,
    table: Table,
}

impl Digest {
    pub fn new(crc: u64, table: Table) -> Self {
        Self { crc, table }
    }
}

impl Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.crc = crc64::update(self.crc, &self.table, buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Hash for Digest {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8> {
        let s = self.sum64().to_be_bytes();
        match b {
            Some(mut v) => {
                v.extend_from_slice(&s);
                v
            }
            None => s.to_vec(),
        }
    }

    fn reset(&mut self) {
        self.crc = 0;
    }

    fn size(&self) -> usize {
        SIZE
    }

    fn block_size(&self) -> isize {
        1
    }
}

impl Hash64 for Digest {
    fn sum64(&mut self) -> u64 {
        self.crc
    }
}
