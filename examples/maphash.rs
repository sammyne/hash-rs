use std::io::Write;

use hash::{maphash::Hash as MapHash, Hash, Hash64};

fn main() {
    let mut h = MapHash::new();

    h.write_string("hello, ").unwrap();
    println!("{:#016x}", h.sum64());

    h.write(&[b'w', b'o', b'r', b'l', b'd']).unwrap();
    println!("{:#016x}", h.sum64());

    h.reset();

    let mut h2 = MapHash::new();
    h2.set_seed(*h.seed());

    h.write_string("same").unwrap();
    h2.write_string("same").unwrap();

    println!("{:#016x} == {:#016x}", h.sum64(), h2.sum64());
}
