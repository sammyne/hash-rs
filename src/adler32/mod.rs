use std::io::Write;

use crate::{Hash, Hash32};

pub const SIZE: isize = 4;

const MODULO: u32 = 65521;
const NMAX: usize = 5552;

pub fn checksum(data: &[u8]) -> u32 {
    update(1, data)
}

pub fn new() -> impl Hash32 {
    Digest::new()
}

pub struct Digest(u32);

impl Digest {
    fn new() -> Self {
        let mut out = Self(0);
        out.reset();
        out
    }
}

impl Hash for Digest {
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
        self.0 = 1;
    }

    fn size(&self) -> isize {
        crate::adler32::SIZE
    }

    fn block_size(&self) -> isize {
        4
    }
}

impl Hash32 for Digest {
    fn sum32(&mut self) -> u32 {
        self.0
    }
}

impl Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0 = update(self.0, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn update(d: u32, p: &[u8]) -> u32 {
    let (mut s1, mut s2) = (d & 0xffff, d >> 16);

    let mut p = p;
    while p.len() > 0 {
        let mut q: &[u8] = &[];

        if p.len() > NMAX {
            q = &p[NMAX..];
            p = &p[..NMAX];
        }

        while p.len() >= 4 {
            s1 += p[0] as u32;
            s2 += s1;
            s1 += p[1] as u32;
            s2 += s1;
            s1 += p[2] as u32;
            s2 += s1;
            s1 += p[3] as u32;
            s2 += s1;
            p = &p[4..];
        }

        for &x in p {
            s1 += x as u32;
            s2 += s1;
        }

        s1 %= MODULO;
        s2 %= MODULO;
        p = q;
    }

    (s2 << 16) | s1
}

#[cfg(test)]
mod tests;
