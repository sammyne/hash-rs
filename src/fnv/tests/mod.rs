use crate::{Hash, Hash32};

#[test]
fn golden32() {
    test_golden(super::new32(), GOLDEN32_TEST_VECTOR.as_slice());
}

#[test]
fn golden32a() {
    test_golden(super::new32a(), GOLDEN32A_TEST_VECTOR.as_slice());
}

#[test]
fn integrity32() {
    test_integrity32(super::new32());
}

#[test]
fn integrity32a() {
    test_integrity32(super::new32a());
}

fn test_golden<H>(h: H, test_vector: &[Test])
where
    H: Hash,
{
    let mut h = h;
    for g in test_vector {
        h.reset();

        let done = h.write(g.input.as_bytes()).expect("write");
        assert_eq!(g.input.len(), done, "wrong #(byte) written");

        let got = h.sum(None);
        assert_eq!(g.out, got.as_slice(), "hash({})", g.input);
    }
}

fn test_integrity<H>(h: H)
where
    H: Hash,
{
    let mut h = h;

    let data = &[b'1', b'2', 3, 4, 5];

    let _ = h.write(data).unwrap();
    let sum = h.sum(None);

    assert_eq!(h.size() as usize, sum.len(), "bad output size");

    let a = h.sum(None);
    assert_eq!(sum, a, "double sum produces different outputs");

    h.reset();
    let _ = h.write(data).unwrap();

    let a = h.sum(None);
    assert_eq!(sum, a, "sum after reset produces different outputs");

    h.reset();
    let _ = h.write(&data[..2]).unwrap();
    let _ = h.write(&data[2..]).unwrap();
    let a = h.sum(None);
    assert_eq!(sum, a, "sum with partial write produces different outputs");
}

fn test_integrity32<H>(h: H)
where
    H: Hash32,
{
    let mut h = h;

    let data = &[b'1', b'2', 3, 4, 5];

    let _ = h.write(data).unwrap();
    let sum = h.sum(None);

    let sum32 = h.sum32().to_be_bytes();
    assert_eq!(sum, sum32, "sum != sum32");

    h.reset();
    test_integrity(h);
}

struct Test {
    out: &'static [u8],
    input: &'static str,
}

impl Test {
    fn new(out: &'static [u8], input: &'static str) -> Self {
        let s = &[0x01, 2, 3, 3];
        Self { out, input }
    }
}

lazy_static::lazy_static! {
  static ref GOLDEN32_TEST_VECTOR: Vec<Test>=vec![
    Test::new(&[0x81, 0x1c, 0x9d, 0xc5], ""),
    Test::new(&[0x05, 0x0c, 0x5d, 0x7e], "a"),
    Test::new(&[0x70, 0x77, 0x2d, 0x38], "ab"),
    Test::new(&[0x43, 0x9c, 0x2f, 0x4b], "abc"),
  ];

  static ref GOLDEN32A_TEST_VECTOR: Vec<Test>=vec![
    Test::new(&[0x81, 0x1c, 0x9d, 0xc5], ""),
    Test::new(&[0xe4, 0x0c, 0x29, 0x2c], "a"),
    Test::new(&[0x4d, 0x25, 0x05, 0xca], "ab"),
    Test::new(&[0x1a, 0x47, 0xe9, 0x0b], "abc"),
  ];
}
