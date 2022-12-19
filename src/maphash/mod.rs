//! Module maphash provides hash functions on byte sequences.
//!
//! These hash functions are intended to be used to implement hash tables or
//! other data structures that need to map arbitrary strings or byte
//! sequences to a uniform distribution on unsigned 64-bit integers.
//! Each different instance of a hash table or data structure should use its own [Seed].
//!
//! The hash functions are not cryptographically secure.
//! (See crypto/sha256 and crypto/sha512 for cryptographic use.)
//!

use std::io::{self, Write};
use std::mem;

use crate::Hash64;

const BUF_SIZE: usize = 128;

/// A Hash computes a seeded hash of a byte sequence.
///
/// The zero Hash is a valid Hash ready to use.
/// A zero Hash chooses a random seed for itself.
/// For control over the seed, use [set_seed][Self::set_seed].
///
/// The computed hash values depend only on the initial seed and
/// the sequence of bytes provided to the Hash object, not on the way
/// in which the bytes are provided. For example, the three sequences
///
/// ```no_test
/// h.write(&[b'f', b'o', b'o']);
/// h.write_byte(b'f'); h.write_byte(b'o'); h.write_byte(b'o');
/// h.write_string("foo");
/// ```
///
/// all have the same effect.
///
/// Hashes are intended to be collision-resistant, even for situations
/// where an adversary controls the byte sequences being hashed.
///
/// A Hash is not safe for concurrent use by multiple goroutines, but a [Seed] is.
/// If multiple goroutines must compute the same seeded hash,
/// each can declare its own Hash and call [set_seed][Self::set_seed] with a common [Seed].
///
/// # Example
/// ```
#[doc = include_str!("../../examples/maphash.rs")]
/// ```
pub struct Hash {
    seed: Seed,
    state: Seed,
    buf: [u8; BUF_SIZE],
    n: usize,
}

/// A Seed is a random value that selects the specific hash function
/// computed by a [Hash][struct@Hash].
///
/// If two Hashes use the same Seeds, they
/// will compute the same hash values for any given input.
/// If two Hashes use different Seeds, they are very likely to compute
/// distinct hash values for any given input.
///
/// A Seed must be initialized by calling [make_seed].
/// The zero seed is uninitialized and not valid for use with [Hash][struct@Hash]'s
/// [set_seed][Hash::set_seed] method.
///
/// Each Seed value is local to a single process and cannot be serialized
/// or otherwise recreated in a different process.
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

    /// seed returns h's seed value.
    pub fn seed(&self) -> &Seed {
        &self.seed
    }

    /// set_seed sets h to use seed, which must have been returned by [make_seed]
    /// or by another Hash's [seed][Self::seed] method.
    ///
    /// Two Hash objects with the same seed behave identically.
    /// Two Hash objects with different seeds will very likely behave differently.
    /// Any bytes added to h before this call will be discarded.
    pub fn set_seed(&mut self, s: Seed) {
        self.seed = s;
        self.state = s;
        self.n = 0;
    }

    /// write_byte adds b to the sequence of bytes hashed by h.
    /// It never fails.
    pub fn write_byte(&mut self, b: u8) -> io::Result<()> {
        if self.n == self.buf.len() {
            let _ = self.flush();
        }

        self.buf[self.n] = b;
        self.n += 1;

        Ok(())
    }

    /// write_string adds the bytes of s to the sequence of bytes hashed by h.
    /// It always writes all of s and never fails.
    pub fn write_string<S>(&mut self, s: S) -> io::Result<usize>
    where
        S: AsRef<str>,
    {
        self.write(s.as_ref().as_bytes())
    }
}

impl crate::Hash for Hash {
    /// sum appends the hash's current 64-bit value to b.
    /// It exists for implementing [Hash][crate::Hash].
    /// For direct calls, it is more efficient to use [sum64](#method.sum64).
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

    /// reset discards all bytes added to h. (The seed remains the same.)
    fn reset(&mut self) {
        self.state = self.seed;
        self.n = 0;
    }

    /// size returns h's hash value size, 8 bytes.
    fn size(&self) -> isize {
        8
    }

    /// block_size returns h's block size.
    fn block_size(&self) -> isize {
        self.buf.len() as isize
    }
}

impl Hash64 for Hash {
    /// sum64 returns h's current 64-bit value, which depends on
    /// h's seed and the sequence of bytes added to h since the
    /// last call to [reset][crate::Hash::reset] or [set_seed](#method.set_seed).
    ///
    /// All bits of the sum64 result are close to uniformly and
    /// independently distributed, so it can be safely reduced
    /// by using bit masking, shifting, or modular arithmetic.
    fn sum64(&mut self) -> u64 {
        rthash(self.buf.as_ref(), self.n, self.state.0)
    }
}

impl Write for Hash {
    /// write adds b to the sequence of bytes hashed by h.
    /// It always writes all of b and never fails; the count and error result are for implementing [std::io::Write].
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
    /// new makes a random seed.
    pub fn new() -> Seed {
        let mut s = 0u64;
        while s == 0 {
            s = rand_u64();
        }
        Self(s)
    }
}

/// make_seed returns a new random seed.
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

fn rthash(ptr: &[u8], len: usize, seed: u64) -> u64 {
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
