use crate::{Hash, Hash32, Hash64};

/*
pub fn new128() -> impl Hash {
    todo!()
}

pub fn new128a() -> impl Hash {
    todo!()
}
*/

pub fn new32() -> impl Hash32 {
    Sum32::new()
}

/*
pub fn new32a() -> impl Hash32 {
    todo!()
}

pub fn new64() -> impl Hash64 {
    todo!()
}

pub fn new64a() -> impl Hash64 {
    todo!()
}
*/

const OFFSET32: u32 = 2166136261;
const OFFSET64: u64 = 14695981039346656037;
const OFFSET128_LOWER: usize = 0x62b821756295c58d;
const OFFSET128_HIGHER: usize = 0x6c62272e07bb0142;
const PRIME32: u32 = 16777619;
const PRIME64: u64 = 1099511628211;
const PRIME128_LOWER: usize = 0x13b;
const PRIME128_SHIFT: usize = 24;

mod sum32;

use sum32::Sum32;

#[cfg(test)]
mod tests;
