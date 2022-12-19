//! Module adler32 implements the Adler-32 checksum.
//!
//! It is defined in [RFC 1950]:
//! ```text
//! Adler-32 is composed of two sums accumulated per byte: s1 is
//! the sum of all bytes, s2 is the sum of all s1 values. Both sums
//! are done modulo 65521. s1 is initialized to 1, s2 to zero. The
//! Adler-32 checksum is stored as s2*65536 + s1 in most-
//! significant-byte first (network) order.
//! ```
//!
//! [RFC 1950]: https://rfc-editor.org/rfc/rfc1950.html
//!

use std::io::Write;

use crate::{Hash, Hash32};

/// The size of an Adler-32 checksum in bytes.
pub const SIZE: usize = 4;

/// MODULO is the largest prime that is less than 65536.
const MODULO: u32 = 65521;

/// NMAX is the largest n such that
/// 255 * n * (n+1) / 2 + (n+1) * (mod-1) <= 2^32-1.
/// It is mentioned in RFC 1950 (search for "5552").
const NMAX: usize = 5552;

/// checksum returns the Adler-32 checksum of data.
pub fn checksum(data: &[u8]) -> u32 {
    update(1, data)
}

/// new returns a new hash.Hash32 computing the Adler-32 checksum. Its [sum][crate::Hash::sum] method will lay the
/// value out in big-endian byte order.
pub fn new() -> impl Hash32 {
    Digest::new()
}

/// digest represents the partial evaluation of a checksum.
/// The low 16 bits are s1, the high 16 bits are s2.
struct Digest(u32);

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

    fn size(&self) -> usize {
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
