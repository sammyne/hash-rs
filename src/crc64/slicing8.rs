use crate::crc64::Table;

pub fn make_table(t: Table) -> [Table; 8] {
    let mut out = [Table::default(); 8];

    out[0] = t;
    for i in 0..256 {
        let mut crc = t[i];
        for j in 1..8 {
            crc = t[(crc & 0xff) as usize] ^ (crc >> 8);
            out[j][i] = crc
        }
    }

    out
}
