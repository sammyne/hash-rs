use std::io::Write;

use crate::fnv::{OFFSET128_HIGHER, OFFSET128_LOWER};
use crate::Hash;

use super::{PRIME128_LOWER, PRIME128_SHIFT};

pub struct Sum128a(u64, u64);

impl Sum128a {
    pub fn new() -> Self {
        Self(OFFSET128_HIGHER, OFFSET128_LOWER)
    }
}

impl Hash for Sum128a {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8> {
        let (s0, s1) = (self.0.to_be_bytes(), self.1.to_be_bytes());
        match b {
            Some(mut v) => {
                v.extend_from_slice(&s0);
                v.extend_from_slice(&s1);
                v
            }
            None => [s0, s1].concat().to_vec(),
        }
    }

    fn reset(&mut self) {
        self.0 = OFFSET128_HIGHER;
        self.1 = OFFSET128_LOWER;
    }

    fn size(&self) -> usize {
        16
    }

    fn block_size(&self) -> isize {
        1
    }
}

impl Write for Sum128a {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for &v in buf {
            self.1 ^= v as u64;

            let (mut s0, s1) = {
                let v = PRIME128_LOWER.wrapping_mul(self.1 as u128);
                ((v >> 64) as u64, v as u64)
            };

            s0 += self
                .1
                .wrapping_shl(PRIME128_SHIFT)
                .wrapping_add(PRIME128_LOWER.wrapping_mul(self.0 as u128) as u64);

            self.1 = s1;
            self.0 = s0;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
