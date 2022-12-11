use std::io::Write;

pub trait Hash: Write {
    fn sum(&mut self, b: Option<Vec<u8>>) -> Vec<u8>;

    fn reset(&mut self);

    fn size(&self) -> isize;

    fn block_size(&self) -> isize;
}

pub trait Hash32: Hash {
    fn sum32(&mut self) -> u32;
}

pub trait Hash64: Hash {
    fn sum64(&mut self) -> u64;
}

pub mod crc32;
