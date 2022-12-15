use super::*;

#[test]
fn golden() {
    let table_iso = super::make_table(ISO);
    let table_ecma = super::make_table(ECMA);

    for (i, g) in GOLDEN_TEST_VECTOR.iter().enumerate() {
        let mut c = super::new(table_iso);
        let _ = c.write_all(g.input.as_bytes());
        let s = c.sum64();
        assert_eq!(
            g.out_iso, s,
            "#{} ISO crc64({}) = {:016x}, expect {:016x}",
            i, g.input, s, g.out_iso
        );

        let mut c = super::new(table_ecma);
        let _ = c.write_all(g.input.as_bytes());
        let s = c.sum64();
        assert_eq!(
            g.out_ecma, s,
            "#{} ECMA crc64({}) = {:016x}, expect {:016x}",
            i, g.input, s, g.out_iso
        );
    }
}

struct Test {
    out_iso: u64,
    out_ecma: u64,
    input: &'static str,
    //half_state_iso: &'static [u8],
    //half_state_ecma: &'static [u8],
}

impl Test {
    fn new(out_iso: u64, out_ecma: u64, input: &'static str) -> Self {
        Self {
            out_iso,
            out_ecma,
            input,
        }
    }
}

lazy_static::lazy_static! {
  // ref: https://rust-lang.github.io/rfcs/0326-restrict-xXX-to-ascii.html
  // ref: https://getkt.com/blog/characters-and-strings-in-go-language/
  // ref: https://www.sobyte.net/post/2022-07/rust-string/
  // todo: make a blog
  static ref GOLDEN_TEST_VECTOR: Vec<Test> = vec![
Test::new(0x0, 0x0, ""),
    Test::new(0x3420000000000000, 0x330284772e652b05, "a"),
    Test::new(0x36c4200000000000, 0xbc6573200e84b046, "ab"),
    Test::new(0x3776c42000000000, 0x2cd8094a1a277627, "abc"),
    Test::new(0x336776c420000000, 0x3c9d28596e5960ba, "abcd"),
    Test::new(0x32d36776c4200000, 0x40bdf58fb0895f2, "abcde"),
    Test::new(0x3002d36776c42000, 0xd08e9f8545a700f4, "abcdef"),
    Test::new(0x31b002d36776c420, 0xec20a3a8cc710e66, "abcdefg"),
    Test::new(0xe21b002d36776c4, 0x67b4f30a647a0c59, "abcdefgh"),
    Test::new(0x8b6e21b002d36776, 0x9966f6c89d56ef8e, "abcdefghi"),
    Test::new(0x7f5b6e21b002d367, 0x32093a2ecd5773f4, "abcdefghij"),
    Test::new(0x8ec0e7c835bf9cdf, 0x8a0825223ea6d221, "Discard medicine more than two years old."),
    Test::new(0xc7db1759e2be5ab4, 0x8562c0ac2ab9a00d, "He who has a shady past knows that nice guys finish last."),
    Test::new(0xfbf9d9603a6fa020, 0x3ee2a39c083f38b4, "I wouldn't marry him with a ten foot pole."),
    Test::new(0xeafc4211a6daa0ef, 0x1f603830353e518a, "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"),
    Test::new(0x3e05b21c7a4dc4da, 0x2fd681d7b2421fd, "The days of the digital watch are numbered.  -Tom Stoppard"),
    Test::new(0x5255866ad6ef28a6, 0x790ef2b16a745a41, "Nepal premier won't resign."),
    Test::new(0x8a79895be1e9c361, 0x3ef8f06daccdcddf, "For every action there is an equal and opposite government program."),
    Test::new(0x8878963a649d4916, 0x49e41b2660b106d, "His money is twice tainted: 'taint yours and 'taint mine."),
    Test::new(0xa7b9d53ea87eb82f, 0x561cc0cfa235ac68, "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977"),
    Test::new(0xdb6805c0966a2f9c, 0xd4fe9ef082e69f59, "It's a tiny change to the code and not completely disgusting. - Bob Manchek"),
    Test::new(0xf3553c65dacdadd2, 0xe3b5e46cd8d63a4d, "size:  a.out:  bad magic"),
    Test::new(0x9d5e034087a676b9, 0x865aaf6b94f2a051, "The major problem is with sendmail.  -Mark Horton"),
    Test::new(0xa6db2d7f8da96417, 0x7eca10d2f8136eb4, "Give me a rock, paper and scissors and I will move the world.  CCFestoon"),
    Test::new(0x325e00cd2fe819f9, 0xd7dd118c98e98727, "If the enemy is within range, then so are you."),
    Test::new(0x88c6600ce58ae4c6, 0x70fb33c119c29318, "It's well we cannot hear the screams/That we create in others' dreams."),
    Test::new(0x28c4a3f3b769e078, 0x57c891e39a97d9b7, "You remind me of a TV show, but that's all right: I watch it anyway."),
    Test::new(0xa698a34c9d9f1dca, 0xa1f46ba20ad06eb7, "C is as portable as Stonehedge!!"),
    Test::new(0xf6c1e2a8c26c5cfc, 0x7ad25fafa1710407, "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley"),
    Test::new(0xd402559dfe9b70c, 0x73cef1666185c13f, "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule"),
    Test::new(0xdb6efff26aa94946, 0xb41858f73c389602, "How can you write a big system without C++?  -Paul Glick"),
    Test::new(0xe7fcf1006b503b61, 0x27db187fc15bbc72, "This is a test of the emergency broadcast system."),
  ];
}
