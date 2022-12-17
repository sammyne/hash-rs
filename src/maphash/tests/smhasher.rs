use std::{collections::HashSet, io::Write};

use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::{
    maphash::{self, Hash, Seed},
    Hash64,
};

#[test]
fn appended_zeros() {
    let s = {
        let ss = b"hello";
        let mut v = Vec::with_capacity(ss.len() + 255);
        v.extend_from_slice(ss);
        while v.len() < v.capacity() {
            v.push(0);
        }
        v
    };

    let mut h = MyHashSet::default();
    for i in 0..s.len() {
        h.add_bytes(&s[..i]);
    }
    h.check();
}

#[test]
#[ignore]
fn cyclic() {
    const REPEAT: usize = 8;
    const N: usize = 1000000;

    let mut r = StdRng::seed_from_u64(1234);
    for n in 4..=12 {
        let mut h = MyHashSet::default();
        let mut b = vec![0u8; REPEAT * n];
        for i in 0..N {
            b[0] = (i * 79 % 97) as u8;
            b[1] = (i * 43 % 137) as u8;
            b[2] = (i * 151 % 197) as u8;
            b[3] = (i * 199 % 251) as u8;

            r.fill_bytes(&mut b[4..]);
            for j in n..(n * REPEAT) {
                b[j] = b[j - n];
            }
            h.add_bytes(&b);
        }
        h.check();
    }
}

#[test]
fn sanity() {
    let mut r = StdRng::seed_from_u64(1234);

    const REP: usize = 10;
    const KEYMAX: usize = 128;
    const PAD: usize = 16;
    const OFFMAX: usize = 16;

    for _k in 0..REP {
        for n in 0..KEYMAX {
            for i in 0..OFFMAX {
                let mut b = vec![0u8; KEYMAX + OFFMAX + 2 * PAD];
                let mut c = vec![0u8; KEYMAX + OFFMAX + 2 * PAD];

                r.fill_bytes(&mut b);
                r.fill_bytes(&mut c);

                let cc = &mut c[(PAD + i)..(PAD + i + n)];
                let bb = &b[PAD..(PAD + n)];
                maphash::copy(cc, bb);

                assert_eq!(
                    bytes_hash(bb),
                    bytes_hash(cc),
                    "hash depends on bytes outside key"
                );
            }
        }
    }
}

#[test]
fn small_keys() {
    let mut h = MyHashSet::default();
    let mut b = [0u8; 3];
    for i in 0u8..255 {
        b[0] = i;
        h.add_bytes(&b[..1]);
        for j in 0u8..=255 {
            b[1] = j;
            h.add_bytes(&b[..2]);
            // todo: figure out why this takes too long
            //for k in 0u8..=255 {
            //    b[2] = k;
            //    h.add_bytes(&b[..3]);
            //}
        }
    }
    h.check();
}

#[test]
#[ignore]
fn two_nonzero() {
    let mut h = MyHashSet::default();
    for n in 2..=16 {
        test_tow_non_zero(&mut h, n);
    }
    h.check();
}

#[test]
fn zeros() {
    // todo: figure out why this takes too long
    //const N: usize = 256 * 1024;
    const N: usize = 1024;

    let mut h = MyHashSet::default();
    let b = [0u8; N];
    for i in 0..N {
        h.add_bytes(&b[..i]);
    }
    h.check();
}

lazy_static::lazy_static! {
  static ref FIXED_SEED: Seed = maphash::make_seed();
}

const HASH_SIZE: usize = 64;

#[derive(Default)]
struct MyHashSet {
    m: HashSet<u64>,
    n: usize,
}

impl MyHashSet {
    fn add(&mut self, h: u64) {
        self.m.insert(h);
        self.n += 1;
    }

    fn add_bytes(&mut self, x: &[u8]) {
        self.add(bytes_hash(x));
    }

    /*
    fn add_str<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        self.add(bytes_hash(s.as_ref().as_bytes()));
    }

    fn add_str_seed<S>(&mut self, s: S, seed: Seed)
    where
        S: AsRef<str>,
    {
        let mut h = Hash::new();
        h.set_seed(seed);
        h.write_string(s).unwrap();

        self.add(h.sum64());
    }
    */

    fn check(&self) {
        const SLOP: f64 = 10.0;

        let collisions = (self.n - self.m.len()) as f64;
        let pairs = (self.n as i64) * ((self.n - 1) as i64) / 2;
        let expected = (pairs as f64) / 2.0f64.powi(HASH_SIZE as i32);
        let stddev = expected.sqrt();

        assert!(
            collisions <= (expected + SLOP * (3.0 * stddev + 1.0)),
            "unexpected number of collisions: got={} mean={} stddev={}",
            collisions,
            expected,
            stddev
        );
    }
}

fn bytes_hash(b: &[u8]) -> u64 {
    let mut h = Hash::new();
    h.set_seed(*FIXED_SEED);
    h.write(b).unwrap();
    h.sum64()
}

fn test_tow_non_zero(h: &mut MyHashSet, n: usize) {
    let mut b = vec![0u8; n];

    // all 0
    h.add_bytes(&b);

    // 1 non-zero byte
    for i in 0..n {
        for x in 1u8..=255 {
            b[i] = x;
            h.add_bytes(&b);
            b[i] = 0;
        }
    }

    // 2 non-zero byte
    for i in 0..n {
        for x in 1u8..=255 {
            b[i] = x;
            for j in (i + 1)..n {
                for y in 1..=255 {
                    b[j] = y;
                    h.add_bytes(&b);
                    b[j] = 0;
                }
            }
            b[i] = 0;
        }
    }
}
