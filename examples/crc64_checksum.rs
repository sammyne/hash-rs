use hash::crc64;

fn main() {
    let msg = "hello world";
    let expect = 13388989860809387070u64;

    let table = crc64::make_table(crc64::ISO);

    let got = crc64::checksum(msg.as_bytes(), &table);

    assert_eq!(expect, got);
}
