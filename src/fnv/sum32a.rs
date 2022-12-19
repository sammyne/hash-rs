use std::io::Write;

use crate::fnv::{OFFSET32, PRIME32};
use crate::{Hash, Hash32};

pub struct Sum32a(u32);

impl Sum32a {
    pub fn new() -> Self {
        Self(OFFSET32)
    }
}

impl Hash for Sum32a {
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
        self.0 = OFFSET32;
    }

    fn size(&self) -> usize {
        4
    }

    fn block_size(&self) -> isize {
        1
    }
}

impl Hash32 for Sum32a {
    fn sum32(&mut self) -> u32 {
        self.0
    }
}

impl Write for Sum32a {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut h = self.0;
        for &v in buf {
            h ^= v as u32;
            h = h.wrapping_mul(PRIME32);
        }
        self.0 = h;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
