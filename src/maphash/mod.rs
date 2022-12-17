use std::io::{self, Write};
use std::mem;

use crate::Hash64;

const BUF_SIZE: usize = 128;

pub struct Hash {
    seed: Seed,
    state: Seed,
    buf: [u8; BUF_SIZE],
    n: usize,
}

#[derive(Clone, Copy)]
pub struct Seed(u64);

impl Hash {
    pub fn new() -> Self {
        let seed = Seed::new();

        Self {
            seed,
            state: seed,
            buf: [0u8; BUF_SIZE],
            n: 0,
        }
    }

    pub fn seed(&self) -> &Seed {
        &self.seed
    }

    pub fn set_seed(&mut self, s: Seed) {
        self.seed = s;
        self.state = s;
        self.n = 0;
    }

    pub fn write_byte(&mut self, b: u8) -> io::Result<()> {
        if self.n == self.buf.len() {
            let _ = self.flush();
        }

        self.buf[self.n] = b;
        self.n += 1;

        Ok(())
    }

    pub fn write_string<S>(&mut self, s: S) -> io::Result<usize>
    where
        S: AsRef<str>,
    {
        self.write(s.as_ref().as_bytes())
    }
}

impl crate::Hash for Hash {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8> {
        let s = self.sum64().to_le_bytes();
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
        self.state = self.seed;
        self.n = 0;
    }

    fn size(&self) -> isize {
        8
    }

    fn block_size(&self) -> isize {
        self.buf.len() as isize
    }
}

impl Hash64 for Hash {
    fn sum64(&mut self) -> u64 {
        rthash(self.buf.as_ref(), self.n, self.state.0)
    }
}

impl Write for Hash {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut b = buf;
        let size = b.len();

        // Deal with bytes left over in h.buf.
        // h.n <= bufSize is always true.
        // Checking it is ~free and it lets the compiler eliminate a bounds check.
        if self.n > 0 && self.n <= BUF_SIZE {
            let k = copy(&mut self.buf[self.n..], b);
            self.n += k;
            if self.n < BUF_SIZE {
                // Copied the entirety of b to h.buf.
                return Ok(size);
            }
            b = &b[k..];
            let _ = self.flush();
            // No need to set h.n = 0 here; it happens just before exit.
        }

        while b.len() > BUF_SIZE {
            self.state.0 = rthash(b, BUF_SIZE, self.state.0);
            b = &b[BUF_SIZE..];
        }

        // copy the tail
        let _ = copy(&mut self.buf, b);
        self.n = b.len();

        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // todo: determine if enforce self.n==self.buf.len()
        self.state.0 = rthash(self.buf.as_ref(), self.n, self.state.0);
        self.n = 0;
        Ok(())
    }
}

impl Seed {
    pub fn new() -> Seed {
        let mut s = 0u64;
        while s == 0 {
            s = rand_u64();
        }
        Self(s)
    }
}

pub fn make_seed() -> Seed {
    Seed::new()
}

pub(self) fn copy(dst: &mut [u8], src: &[u8]) -> usize {
    let n = dst.len().min(src.len());

    dst[..n].copy_from_slice(&src[..n]);

    n
}

fn rand_u64() -> u64 {
    let mut b = [0u8; 8];
    getrandom::getrandom(&mut b).expect("getrandom");

    u64::from_be_bytes(b)
}

pub fn rthash(ptr: &[u8], len: usize, seed: u64) -> u64 {
    if len == 0 {
        return seed;
    }

    if mem::size_of::<usize>() == 8 {
        return unsafe { memhash::sum(ptr, seed as usize, len) as u64 };
    }

    let (lo, hi) = unsafe {
        let lo = memhash::sum(ptr, seed as usize, len);
        let hi = memhash::sum(ptr, (seed >> 32) as usize, len);

        (lo, hi)
    };

    ((hi as u64) << 32) | (lo as u64)
}

mod memhash;

#[cfg(test)]
mod tests;
