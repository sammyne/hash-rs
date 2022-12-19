use std::{collections::HashSet, io::Write};

use crate::{Hash, Hash64};

use super::Hash as MapHash;

#[test]
fn hash_bytes_vs_string() {
    let s = "foo";
    let b = s.as_bytes();

    let mut h1 = MapHash::new();
    let mut h2 = MapHash::new();
    h2.set_seed(*h1.seed());

    let n1 = h1.write_string(s).expect("h1.write_string");
    assert_eq!(s.len(), n1, "write_string consumes wrong length");

    let n2 = h2.write(b).expect("h2.write");
    assert_eq!(b.len(), n2, "write consumes wrong length");

    assert_eq!(
        h1.sum64(),
        h2.sum64(),
        "hash of string and bytes not identical"
    );
}

#[test]
fn hash_grouping() {
    let b = {
        let s = "foo";
        const N: usize = 100;
        let mut out = String::with_capacity(s.len() * N);
        for _i in 0..N {
            out.push_str(s);
        }
        out
    };

    let mut hh = Vec::with_capacity(7);
    for _i in 0..hh.capacity() {
        hh.push(MapHash::new());
    }
    let seed0 = *hh[0].seed();
    for h in hh.iter_mut().skip(1) {
        h.set_seed(seed0);
    }
    hh[0].write(b.as_bytes()).unwrap();
    hh[1].write_string(&b).unwrap();

    let must_write_byte = |h: &mut MapHash, b: u8| {
        h.write_byte(b).expect("write_byte");
    };
    let must_write_single_byte = |h: &mut MapHash, b: u8| {
        h.write(&[b]).expect("write_byte");
    };
    let must_write_string_single_byte = |h: &mut MapHash, b: u8| {
        let mut s = String::with_capacity(1);
        s.push(b as char);
        h.write_string(s).expect("write_byte");
    };

    for (i, &x) in b.as_bytes().iter().enumerate() {
        must_write_byte(&mut hh[2], x);
        must_write_single_byte(&mut hh[3], x);

        if i == 0 {
            must_write_byte(&mut hh[4], x);
        } else {
            must_write_single_byte(&mut hh[4], x);
        }

        must_write_string_single_byte(&mut hh[5], x);
        if i == 0 {
            must_write_byte(&mut hh[6], x);
        } else {
            must_write_single_byte(&mut hh[6], x);
        }
    }

    let sum = hh[0].sum64();
    for (i, h) in hh.iter_mut().enumerate().skip(1) {
        assert_eq!(sum, h.sum64(), "hash {i} identical to a single Write");
    }
}

#[test]
fn hash_high_bytes() {
    // See issue https://github.com/golang/go/issues/34925
    const N: usize = 10;
    let mut m = HashSet::<u64>::new();
    for _i in 0..N {
        let mut h = MapHash::new();
        h.write_string("foo").expect("write_string");
        m.insert(h.sum64() >> 32);
    }

    assert!(
        m.len() >= N / 2,
        "from {N} seeds, not enough different hashes"
    );
}

#[test]
fn repeat() {
    let mut h1 = MapHash::new();
    h1.write_string("testing").unwrap();
    let sum1 = h1.sum64();

    h1.reset();
    h1.write_string("testing").unwrap();
    let sum2 = h1.sum64();

    assert_eq!(sum1, sum2, "different sum after reseting");

    let mut h2 = MapHash::new();
    h2.set_seed(*h1.seed());
    h2.write_string("testing").unwrap();
    let sum3 = h2.sum64();

    assert_eq!(sum1, sum3, "different sum on the same seed");
}

#[test]
fn seed_from_flush() {
    let b = [0u8; 65];

    let mut h1 = MapHash::new();
    h1.write(&b).unwrap();
    let x = h1.sum64();

    let mut h2 = MapHash::new();
    h2.set_seed(*h1.seed());
    h2.write(&b).unwrap();
    let y = h2.sum64();

    assert_eq!(x, y, "hashes don't match");
}

#[test]
fn seed_from_reset() {
    let mut h1 = MapHash::new();
    h1.write_string("foo").unwrap();
    h1.reset();
    h1.write_string("foo").unwrap();
    let x = h1.sum64();

    let mut h2 = MapHash::new();
    h2.set_seed(*h1.seed());
    h2.write_string("foo").unwrap();
    let y = h2.sum64();

    assert_eq!(x, y, "hashes don't match");
}

#[test]
fn seed_from_sum64() {
    let mut h1 = MapHash::new();
    h1.write_string("foo").unwrap();
    let x = h1.sum64();

    let mut h2 = MapHash::new();
    h2.set_seed(*h1.seed());
    h2.write_string("foo").unwrap();
    let y = h2.sum64();

    assert_eq!(x, y, "hashes don't match");

    // @note: SeedFromSeed has been demo by this test too.
}

#[test]
fn seeded_hash() {
    let s = super::make_seed();
    let mut m = HashSet::<u64>::new();

    let expect = 1000usize;
    for _i in 0..expect {
        let mut h = MapHash::new();
        h.set_seed(s);
        m.insert(h.sum64());
    }

    assert_eq!(m.len(), 1, "seeded hash is random",);
}

#[test]
fn unseeded_hash() {
    let mut m = HashSet::<u64>::new();

    let expect = 1000usize;
    for _i in 0..expect {
        let mut h = MapHash::new();
        m.insert(h.sum64());
    }

    assert!(
        m.len() >= 900,
        "empty hash not sufficiently random: expect {}, got {}",
        expect,
        m.len()
    );
}

mod smhasher;
