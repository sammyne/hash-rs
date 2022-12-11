use hash::crc32;

fn main() {
    let crc32q = crc32::make_table(0xD5828281);

    let expect: u32 = 0x2964d064;

    let got = crc32::checksum(b"Hello world", &crc32q);

    assert_eq!(expect, got);
}
