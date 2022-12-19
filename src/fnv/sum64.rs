use std::io::Write;

use crate::fnv::{OFFSET64, PRIME64};
use crate::{Hash, Hash64};

pub struct Sum64(u64);

impl Sum64 {
    pub fn new() -> Self {
        Self(OFFSET64)
    }
}

impl Hash for Sum64 {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8> {
        let s = self.0.to_be_bytes();
        match b {
            Some(mut v) => {
                v.extend_from_slice(&s);
                v
            }
            None => s.to_vec(),
        }
    }

    fn reset(&mut self) {
        self.0 = OFFSET64;
    }

    fn size(&self) -> usize {
        8
    }

    fn block_size(&self) -> isize {
        1
    }
}

impl Hash64 for Sum64 {
    fn sum64(&mut self) -> u64 {
        self.0
    }
}

impl Write for Sum64 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut h = self.0;
        for &v in buf {
            h = h.wrapping_mul(PRIME64);
            h ^= v as u64;
        }
        self.0 = h;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
