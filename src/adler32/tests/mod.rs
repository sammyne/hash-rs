use super::*;

#[test]
fn golden() {
    for (i, g) in GOLDEN_TEST_VECTOR.iter().enumerate() {
        let got = simpl_checksum(&g.input);
        assert_eq!(
            g.out, got,
            "simple implementation: #{i} expect 0x{:08x}, got 0x{:08x}",
            g.out, got
        );

        let got = checksum(&g.input);
        assert_eq!(
            g.out, got,
            "optimized implementation: #{i} expect 0x{:08x}, got 0x{:08x}",
            g.out, got
        );
    }
}

struct Test {
    out: u32,
    input: Vec<u8>,
}

impl Test {
    fn new<S>(out: u32, input: S) -> Self
    where
        S: AsRef<[u8]>,
    {
        Self {
            out,
            input: input.as_ref().to_vec(),
        }
    }
}

lazy_static::lazy_static! {
  static ref GOLDEN_TEST_VECTOR:Vec<Test> = vec![
  Test::new(  0x00000001, ""),
    Test::new(0x00620062, "a"),
    Test::new(0x012600c4, "ab"),
    Test::new(0x024d0127, "abc"),
    Test::new(0x03d8018b, "abcd"),
    Test::new(0x05c801f0, "abcde"),
    Test::new(0x081e0256, "abcdef"),
    Test::new(0x0adb02bd, "abcdefg"),
    Test::new(0x0e000325, "abcdefgh"),
    Test::new(0x118e038e, "abcdefghi"),
    Test::new(0x158603f8, "abcdefghij"),
    Test::new(0x3f090f02, "Discard medicine more than two years old."),
    Test::new(0x46d81477, "He who has a shady past knows that nice guys finish last."),
    Test::new(0x40ee0ee1, "I wouldn't marry him with a ten foot pole."),
    Test::new(0x16661315, "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"),
    Test::new(0x5b2e1480, "The days of the digital watch are numbered.  -Tom Stoppard"),
    Test::new(0x8c3c09ea, "Nepal premier won't resign."),
    Test::new(0x45ac18fd, "For every action there is an equal and opposite government program."),
    Test::new(0x53c61462, "His money is twice tainted: 'taint yours and 'taint mine."),
    Test::new(0x7e511e63, "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977"),
    Test::new(0xe4801a6a, "It's a tiny change to the code and not completely disgusting. - Bob Manchek"),
    Test::new(0x61b507df, "size:  a.out:  bad magic"),
    Test::new(0xb8631171, "The major problem is with sendmail.  -Mark Horton"),
    Test::new(0x8b5e1904, "Give me a rock, paper and scissors and I will move the world.  CCFestoon"),
    Test::new(0x7cc6102b, "If the enemy is within range, then so are you."),
    Test::new(0x700318e7, "It's well we cannot hear the screams/That we create in others' dreams."),
    Test::new(0x1e601747, "You remind me of a TV show, but that's all right: I watch it anyway."),
    Test::new(0xb55b0b09, "C is as portable as Stonehedge!!"),
    Test::new(0x39111dd0, "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley"),
    Test::new(0x91dd304f, "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule"),
    Test::new(0x2e5d1316, "How can you write a big system without C++?  -Paul Glick"),
    Test::new(0xd0201df6, "'Invariant assertions' is the most elegant programming technique!  -Tom Szymanski"),
    Test::new(0x211297c8, build_input(b"\xff",5548, Some("8"))),
    Test::new(0xbaa198c8, build_input(b"\xff", 5549, Some("9"))),
    Test::new(0x553499be, build_input(b"\xff", 5550, Some("0"))),
    Test::new(0xf0c19abe, build_input(b"\xff", 5551, Some("1"))),
    Test::new(0x8d5c9bbe, build_input(b"\xff", 5552, Some("2"))),
    Test::new(0x2af69cbe, build_input(b"\xff", 5553, Some("3"))),
    Test::new(0xc9809dbe, build_input(b"\xff", 5554, Some("4"))),
    Test::new(0x69189ebe, build_input(b"\xff", 5555, Some("5"))),
    Test::new(0x86af0001, build_input(b"\x00", 100000,None)),
    Test::new(0x79660b4d, build_input(b"a", 100000,None)),
    Test::new(0x110588ee, build_input(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ", 10000,None)),
  ];
}

fn build_input(s: &[u8], n: usize, suffix: Option<&str>) -> Vec<u8> {
    let olen = match &suffix {
        Some(ref v) => s.len() * n + v.len(),
        None => s.len(),
    };

    let mut out = Vec::with_capacity(olen);
    for _i in 0..n {
        out.extend_from_slice(s);
    }

    if let Some(v) = suffix {
        out.extend_from_slice(v.as_bytes());
    }

    out
}

fn simpl_checksum(p: &[u8]) -> u32 {
    let (mut s1, mut s2) = (1, 0);

    for &x in p {
        s1 = (s1 + (x as u32)) % MODULO;
        s2 = (s2 + s1) % MODULO;
    }

    (s2 << 16) | s1
}
