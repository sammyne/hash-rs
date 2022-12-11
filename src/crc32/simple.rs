use crate::crc32::Table;

pub fn make_table(poly: u32) -> Table {
    let mut out = Table::default();
    populate_table(poly, &mut out);
    out
}

pub fn populate_table(poly: u32, t: &mut Table) {
    for i in 0usize..256 {
        let mut crc = i as u32;
        for _j in 0..8 {
            crc = if (crc & 1) == 1 {
                (crc >> 1) ^ poly
            } else {
                crc >> 1
            };
        }
        t[i] = crc
    }
}

pub fn update(crc: u32, t: &Table, p: &[u8]) -> u32 {
    let mut crc = !crc;
    for v in p {
        crc = t[((crc as u8) ^ v) as usize] ^ (crc >> 8);
    }

    !crc
}
