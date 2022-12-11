use super::*;

#[test]
fn golden() {
    golden_ieee(checksum_ieee);

    for delta in 1..=7 {
        let f = |b: &[u8]| -> u32 {
            let mut ieee = new_ieee();
            let d = b.len().min(delta as usize);
            let _ = ieee.write(&b[..d]);
            let _ = ieee.write(&b[d..]);
            ieee.sum32()
        };
        golden_ieee(f);
    }

    let castagnoli_table = make_table(CASTAGNOLI);

    let f = |b: &[u8]| -> u32 {
        let mut c = new(castagnoli_table);
        let _ = c.write(b);
        c.sum32()
    };
    golden_castagnoli(f);

    for delta in 1..=7 {
        let f = |b: &[u8]| -> u32 {
            let mut c = new(castagnoli_table);
            let d = b.len().min(delta as usize);
            let _ = c.write(&b[..d]);
            let _ = c.write(&b[d..]);
            c.sum32()
        };
        golden_castagnoli(f);
    }
}

#[test]
fn simple() {
    let ieee = simple::make_table(IEEE);
    golden_ieee(|b: &[u8]| simple::update(0, &ieee, b));

    let castagnoli = simple::make_table(CASTAGNOLI);
    golden_castagnoli(|b: &[u8]| simple::update(0, &castagnoli, b));
}

#[test]
fn slicing() {
    let ieee = slicing8::make_table(IEEE);
    golden_ieee(|b: &[u8]| slicing8::update(0, &ieee, b));

    let castagnoli = slicing8::make_table(CASTAGNOLI);
    golden_castagnoli(|b: &[u8]| slicing8::update(0, &castagnoli, b));

    for poly in [IEEE, CASTAGNOLI, KOOPMAN, 0xD5828281] {
        let t1 = simple::make_table(poly);
        let f1 = |crc: u32, b: &[u8]| -> u32 { simple::update(crc, &t1, b) };

        let t2 = slicing8::make_table(poly);
        let f2 = |crc: u32, b: &[u8]| -> u32 { slicing8::update(crc, &t2, b) };

        cross_check(f1, f2);
    }
}

#[test]
fn table_eq() {
    let a = simple::make_table(IEEE);
    assert!(a.eq(&IEEE_TABLE));
}

struct Test {
    ieee: u32,
    castagnoli: u32,
    input: &'static [u8],
    _half_state_ieee: &'static [u8],
    _half_state_castagnoli: &'static [u8],
}

impl Test {
    fn new(
        ieee: u32,
        castagnoli: u32,
        input: &'static [u8],
        _half_state_ieee: &'static [u8],
        _half_state_castagnoli: &'static [u8],
    ) -> Self {
        Self {
            ieee,
            castagnoli,
            input,
            _half_state_ieee,
            _half_state_castagnoli,
        }
    }
}

lazy_static::lazy_static! {
  // ref: https://rust-lang.github.io/rfcs/0326-restrict-xXX-to-ascii.html
  // ref: https://getkt.com/blog/characters-and-strings-in-go-language/
  // ref: https://www.sobyte.net/post/2022-07/rust-string/
  // todo: make a blog
  static ref GOLDEN_TEST_VECTOR: Vec<Test> = vec![
    // '\xca\x87' is encoded as bytes '\xca\x87'
    Test::new(0x0,0x0,b"",b"crc\x01\xca\x87\x91M\x00\x00\x00\x00",b"crc\x01wB\x84\x81\x00\x00\x00\x00"),
    Test::new(0xe8b7be43, 0xc1d04330, b"a", b"crc\x01\xca\x87\x91M\x00\x00\x00\x00", b"crc\x01wB\x84\x81\x00\x00\x00\x00"),
    Test::new(0x9e83486d, 0xe2a22936, b"ab", b"crc\x01\xca\x87\x91M\xc2\x91C", b"crc\x01wB\x84\x81\xc1\xd0C0"),
    Test::new(0x352441c2, 0x364b3fb7, b"abc", b"crc\x01\xca\x87\x91M\xc2\x91C", b"crc\x01wB\x84\x81\xc1\xd0C0"),
    Test::new(0xed82cd11, 0x92c80a31, b"abcd", b"crc\x01\xca\x87\x91M\x9e\x83Hm", b"crc\x01wB\x84\x81\xe2\xa2)6"),
    Test::new(0x8587d865, 0xc450d697, b"abcde", b"crc\x01\xca\x87\x91M\x9e\x83Hm", b"crc\x01wB\x84\x81\xe2\xa2)6"),
    Test::new(0x4b8e39ef, 0x53bceff1, b"abcdef", b"crc\x01\xca\x87\x91M5$A\xc2", b"crc\x01wB\x84\x816K?\xb7"),
    Test::new(0x312a6aa6, 0xe627f441, b"abcdefg", b"crc\x01\xca\x87\x91M5$A\xc2", b"crc\x01wB\x84\x816K?\xb7"),
    Test::new(0xaeef2a50, 0xa9421b7, b"abcdefgh", b"crc\x01\xca\x87\x91M\xed\x82\xcd\x11", b"crc\x01wB\x84\x81\x92\xc8\n1"),
    Test::new(0x8da988af, 0x2ddc99fc, b"abcdefghi", b"crc\x01\xca\x87\x91M\xed\x82\xcd\x11", b"crc\x01wB\x84\x81\x92\xc8\n1"),
    Test::new(0x3981703a, 0xe6599437, b"abcdefghij", b"crc\x01\xca\x87\x91M\x85\x87\xd8e", b"crc\x01wB\x84\x81\xc4P\xd6\x97"),
    Test::new(0x6b9cdfe7, 0xb2cc01fe, b"Discard medicine more than two years old.", b"crc\x01\xca\x87\x91M\xfd\xe5\xc2J", b"crc\x01wB\x84\x81S\"(\xe0"),
    Test::new(0xc90ef73f, 0xe28207f, b"He who has a shady past knows that nice guys finish last.", b"crc\x01\xca\x87\x91M\x01\xc7\x8b+", b"crc\x01wB\x84\x81'\xdaR\x15"),
    Test::new(0xb902341f, 0xbe93f964, b"I wouldn't marry him with a ten foot pole.", b"crc\x01\xca\x87\x91M\x9d\x13\xce\x10", b"crc\x01wB\x84\x81\xc3\xed\xabG"),
    Test::new(0x42080e8, 0x9e3be0c3, b"Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave", b"crc\x01\xca\x87\x91M-\xed\xf7\x94", b"crc\x01wB\x84\x81\xce\xceb\x81"),
    Test::new(0x154c6d11, 0xf505ef04, b"The days of the digital watch are numbered.  -Tom Stoppard", b"crc\x01\xca\x87\x91MOa\xa5\r", b"crc\x01wB\x84\x81\xd3s\x9dP"),
    Test::new(0x4c418325, 0x85d3dc82, b"Nepal premier won't resign.", b"crc\x01\xca\x87\x91M\xa8S9\x85", b"crc\x01wB\x84\x81{\x90\x8a\x14"),
    Test::new(0x33955150, 0xc5142380, b"For every action there is an equal and opposite government program.", b"crc\x01\xca\x87\x91Ma\xe9>\x86", b"crc\x01wB\x84\x81\xaa@\xc4\x1c"),
    Test::new(0x26216a4b, 0x75eb77dd, b"His money is twice tainted: 'taint yours and 'taint mine.", b"crc\x01\xca\x87\x91M\\\x1an\x88", b"crc\x01wB\x84\x81W\x078Z"),
    Test::new(0x1abbe45e, 0x91ebe9f7, b"There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977", b"crc\x01\xca\x87\x91M\xb7\xf5\xf2\xca", b"crc\x01wB\x84\x81\xc4o\x9d\x85"),
    Test::new(0xc89a94f7, 0xf0b1168e, b"It's a tiny change to the code and not completely disgusting. - Bob Manchek", b"crc\x01\xca\x87\x91M\x84g1\xe8", b"crc\x01wB\x84\x81#\x98\x0c\xab"),
    Test::new(0xab3abe14, 0x572b74e2, b"size:  a.out:  bad magic", b"crc\x01\xca\x87\x91M\x8a\x0f\xad\x08", b"crc\x01wB\x84\x81\x80\xc9n\xd8"),
    Test::new(0xbab102b6, 0x8a58a6d5, b"The major problem is with sendmail.  -Mark Horton", b"crc\x01\xca\x87\x91M\x07\xf0\xb3\x15", b"crc\x01wB\x84\x81liS\xcc"),
    Test::new(0x999149d7, 0x9c426c50, b"Give me a rock, paper and scissors and I will move the world.  CCFestoon", b"crc\x01\xca\x87\x91M\x0fa\xbc.", b"crc\x01wB\x84\x81\xdb\xcd\x8fC"),
    Test::new(0x6d52a33c, 0x735400a4, b"If the enemy is within range, then so are you.", b"crc\x01\xca\x87\x91My\x1b\x99\xf8", b"crc\x01wB\x84\x81\xaaB\x037"),
    Test::new(0x90631e8d, 0xbec49c95, b"It's well we cannot hear the screams/That we create in others' dreams.", b"crc\x01\xca\x87\x91M\x08qfY", b"crc\x01wB\x84\x81\x16y\xa1\xd2"),
    Test::new(0x78309130, 0xa95a2079, b"You remind me of a TV show, but that's all right: I watch it anyway.", b"crc\x01\xca\x87\x91M\xbdO,\xc2", b"crc\x01wB\x84\x81f&\xc5\xe4"),
    Test::new(0x7d0a377f, 0xde2e65c5, b"C is as portable as Stonehedge!!", b"crc\x01\xca\x87\x91M\xf7\xd6\x00\xd5", b"crc\x01wB\x84\x81de\\\xf8"),
    Test::new(0x8c79fd79, 0x297a88ed, b"Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley", b"crc\x01\xca\x87\x91Ml+\xb8\xa7", b"crc\x01wB\x84\x81\xbf\xd6S\xdd"),
    Test::new(0xa20b7167, 0x66ed1d8b, b"The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule", b"crc\x01\xca\x87\x91M<lR[", b"crc\x01wB\x84\x81{\xaco\xb1"),
    Test::new(0x8e0bb443, 0xdcded527, b"How can you write a big system without C++?  -Paul Glick", b"crc\x01\xca\x87\x91M\x0e\x88\x89\xed", b"crc\x01wB\x84\x813\xd7C\x7f"),
  ];
}

fn cross_check<F, H>(crc_fn1: F, crc_fn2: H)
where
    F: Fn(u32, &[u8]) -> u32,
    H: Fn(u32, &[u8]) -> u32,
{
    let lengths = [
        0usize, 1, 2, 3, 4, 5, 10, 16, 50, 63, 64, 65, 100, 127, 128, 129, 255, 256, 257, 300, 312,
        384, 416, 448, 480, 500, 501, 502, 503, 504, 505, 512, 513, 1000, 1024, 2000, 4030, 4031,
        4032, 4033, 4036, 4040, 4048, 4096, 5000, 10000,
    ];

    for v in lengths {
        let mut p = vec![0u8; v];
        let _ = getrandom::getrandom(&mut p);
        let crc_init = rand_u32();

        let crc1 = crc_fn1(crc_init, &p);
        let crc2 = crc_fn2(crc_init, &p);
        assert_eq!(crc1, crc2, "mismatch for buffer length {}", v);
    }
}

fn golden_castagnoli<F>(crc_fn: F)
where
    F: Fn(&[u8]) -> u32,
{
    for v in GOLDEN_TEST_VECTOR.iter() {
        let got = crc_fn(v.input);
        assert_eq!(
            v.castagnoli,
            got,
            "Castagnoli({}) = {:#08x}, want {:#08x}",
            String::from_utf8_lossy(v.input),
            got,
            v.ieee
        );
    }
}

fn golden_ieee<F>(crc_fn: F)
where
    F: Fn(&[u8]) -> u32,
{
    for v in GOLDEN_TEST_VECTOR.iter() {
        let got = crc_fn(v.input);
        assert_eq!(
            v.ieee,
            got,
            "IEEE({}) = {:#08x}, want {:#08x}",
            String::from_utf8_lossy(v.input),
            got,
            v.ieee
        );
    }
}

fn rand_u32() -> u32 {
    let mut b = [0u8; 4];
    let _ = getrandom::getrandom(&mut b);
    u32::from_be_bytes(b)
}
