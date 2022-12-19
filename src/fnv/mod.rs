//! Module fnv implements FNV-1 and FNV-1a, non-cryptographic hash functions
//! created by Glenn Fowler, Landon Curt Noll, and Phong Vo.
//! 
//! See
//! <https://en.wikipedia.org/wiki/Fowler-Noll-Vo_hash_function>.
//!

use crate::{Hash, Hash32, Hash64};

/// new128 returns a new 128-bit FNV-1 [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new128() -> impl Hash {
    Sum128::new()
}

/// new128a returns a new 128-bit FNV-1a [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new128a() -> impl Hash {
    Sum128a::new()
}

/// new32 returns a new 32-bit FNV-1 [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new32() -> impl Hash32 {
    Sum32::new()
}

/// new32a returns a new 32-bit FNV-1a [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new32a() -> impl Hash32 {
    Sum32a::new()
}

/// new64 returns a new 64-bit FNV-1 [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new64() -> impl Hash64 {
    Sum64::new()
}

/// new64a returns a new 64-bit FNV-1a [Hash][crate::Hash].
/// Its [sum][crate::Hash::sum] method will lay the value out in big-endian byte order.
pub fn new64a() -> impl Hash64 {
    Sum64a::new()
}

const OFFSET32: u32 = 2166136261;
const OFFSET64: u64 = 14695981039346656037;
const OFFSET128_LOWER: u64 = 0x62b821756295c58d;
const OFFSET128_HIGHER: u64 = 0x6c62272e07bb0142;
const PRIME32: u32 = 16777619;
const PRIME64: u64 = 1099511628211;
const PRIME128_LOWER: u128 = 0x013b;
const PRIME128_SHIFT: u32 = 24;

mod sum128;
mod sum128a;
mod sum32;
mod sum32a;
mod sum64;
mod sum64a;

use sum128::Sum128;
use sum128a::Sum128a;
use sum32::Sum32;
use sum32a::Sum32a;
use sum64::Sum64;
use sum64a::Sum64a;

#[cfg(test)]
mod tests;
