use crate::{Hash, Hash32, Hash64};

#[test]
fn golden32() {
    test_golden(super::new32(), GOLDEN32_TEST_VECTOR.as_slice());
}

#[test]
fn golden32a() {
    test_golden(super::new32a(), GOLDEN32A_TEST_VECTOR.as_slice());
}

#[test]
fn golden64() {
    test_golden(super::new64(), GOLDEN64_TEST_VECTOR.as_slice());
}

#[test]
fn golden64a() {
    test_golden(super::new64a(), GOLDEN64A_TEST_VECTOR.as_slice());
}

#[test]
fn integrity32() {
    test_integrity32(super::new32());
}

#[test]
fn integrity32a() {
    test_integrity32(super::new32a());
}

#[test]
fn integrity64() {
    test_integrity64(super::new64());
}

#[test]
fn integrity64a() {
    test_integrity64(super::new64a());
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

fn test_integrity64<H>(h: H)
where
    H: Hash64,
{
    let mut h = h;

    let data = &[b'1', b'2', 3, 4, 5];

    let _ = h.write(data).unwrap();
    let sum = h.sum(None);

    let sum64 = h.sum64().to_be_bytes();
    assert_eq!(sum, sum64, "sum != sum64");

    h.reset();
    test_integrity(h);
}

struct Test {
    out: &'static [u8],
    input: &'static str,
}

impl Test {
    fn new(out: &'static [u8], input: &'static str) -> Self {
        Self { out, input }
    }
}

lazy_static::lazy_static! {
  static ref GOLDEN32_TEST_VECTOR: Vec<Test> = vec![
    Test::new(&[0x81, 0x1c, 0x9d, 0xc5], ""),
    Test::new(&[0x05, 0x0c, 0x5d, 0x7e], "a"),
    Test::new(&[0x70, 0x77, 0x2d, 0x38], "ab"),
    Test::new(&[0x43, 0x9c, 0x2f, 0x4b], "abc"),
  ];

  static ref GOLDEN32A_TEST_VECTOR: Vec<Test> = vec![
    Test::new(&[0x81, 0x1c, 0x9d, 0xc5], ""),
    Test::new(&[0xe4, 0x0c, 0x29, 0x2c], "a"),
    Test::new(&[0x4d, 0x25, 0x05, 0xca], "ab"),
    Test::new(&[0x1a, 0x47, 0xe9, 0x0b], "abc"),
  ];

  static ref GOLDEN64_TEST_VECTOR: Vec<Test> = vec![
    Test::new(&[0xcb, 0xf2, 0x9c, 0xe4, 0x84, 0x22, 0x23, 0x25], ""),
    Test::new(&[0xaf, 0x63, 0xbd, 0x4c, 0x86, 0x01, 0xb7, 0xbe], "a"),
    Test::new(&[0x08, 0x32, 0x67, 0x07, 0xb4, 0xeb, 0x37, 0xb8], "ab"),
    Test::new(&[0xd8, 0xdc, 0xca, 0x18, 0x6b, 0xaf, 0xad, 0xcb], "abc"),
  ];

  static ref GOLDEN64A_TEST_VECTOR: Vec<Test> = vec![
    Test::new(&[0xcb, 0xf2, 0x9c, 0xe4, 0x84, 0x22, 0x23, 0x25], ""),
    Test::new(&[0xaf, 0x63, 0xdc, 0x4c, 0x86, 0x01, 0xec, 0x8c], "a"),
    Test::new(&[0x08, 0x9c, 0x44, 0x07, 0xb5, 0x45, 0x98, 0x6a], "ab"),
    Test::new(&[0xe7, 0x1f, 0xa2, 0x19, 0x05, 0x41, 0x57, 0x4b], "abc"),
  ];

  static ref GOLDEN128_TEST_VECTOR: Vec<Test> = vec![
    Test::new(&[0x6c, 0x62, 0x27, 0x2e, 0x07, 0xbb, 0x01, 0x42, 0x62, 0xb8, 0x21, 0x75, 0x62, 0x95, 0xc5, 0x8d], ""),
    Test::new(&[0xd2, 0x28, 0xcb, 0x69, 0x10, 0x1a, 0x8c, 0xaf, 0x78, 0x91, 0x2b, 0x70, 0x4e, 0x4a, 0x14, 0x1e], "a"),
    Test::new(&[0x08, 0x80, 0x94, 0x5a, 0xee, 0xab, 0x1b, 0xe9, 0x5a, 0xa0, 0x73, 0x30, 0x55, 0x26, 0xc0, 0x88], "ab"),
    Test::new(&[0xa6, 0x8b, 0xb2, 0xa4, 0x34, 0x8b, 0x58, 0x22, 0x83, 0x6d, 0xbc, 0x78, 0xc6, 0xae, 0xe7, 0x3b], "abc"),
  ];

  static ref GOLDEN128A_TEST_VECTOR: Vec<Test>=vec![
    Test::new(&[0x6c, 0x62, 0x27, 0x2e, 0x07, 0xbb, 0x01, 0x42, 0x62, 0xb8, 0x21, 0x75, 0x62, 0x95, 0xc5, 0x8d], ""),
    Test::new(&[0xd2, 0x28, 0xcb, 0x69, 0x6f, 0x1a, 0x8c, 0xaf, 0x78, 0x91, 0x2b, 0x70, 0x4e, 0x4a, 0x89, 0x64], "a"),
    Test::new(&[0x08, 0x80, 0x95, 0x44, 0xbb, 0xab, 0x1b, 0xe9, 0x5a, 0xa0, 0x73, 0x30, 0x55, 0xb6, 0x9a, 0x62], "ab"),
    Test::new(&[0xa6, 0x8d, 0x62, 0x2c, 0xec, 0x8b, 0x58, 0x22, 0x83, 0x6d, 0xbc, 0x79, 0x77, 0xaf, 0x7f, 0x3b], "abc"),
  ];
}
