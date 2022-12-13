//! Crate hash provides interfaces for hash functions.
//!

use std::io::Write;

/// Hash is the common interface implemented by all hash functions.
///
/// [`std::io::Write`] trait requirement adds more data to the running hash.
/// It never returns an error.
pub trait Hash: Write {
    /// sum appends the current hash to b and returns the resulting slice.
    /// It does not change the underlying hash state.
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8>;

    /// reset resets the Hash to its initial state.
    fn reset(&mut self);

    /// size returns the number of bytes Sum will return.
    fn size(&self) -> isize;

    /// block_size returns the hash's underlying block size.
    /// The write method must be able to accept any amount
    /// of data, but it may operate more efficiently if all writes
    /// are a multiple of the block size.
    fn block_size(&self) -> isize;
}

/// Hash32 is the common interface implemented by all 32-bit hash functions.
pub trait Hash32: Hash {
    fn sum32(&mut self) -> u32;
}

/// Hash64 is the common interface implemented by all 64-bit hash functions.
pub trait Hash64: Hash {
    fn sum64(&mut self) -> u64;
}

pub mod crc32;
