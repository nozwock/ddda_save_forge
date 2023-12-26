/// References:
/// - https://github.com/Michaelangel007/crc32
/// - https://wiki.osdev.org/CRC32
pub struct Crc32ITUv42 {
    table: [u32; 256],
}

impl Crc32ITUv42 {
    pub fn new() -> Self {
        let mut table = [0u32; 256];
        for i in 0..table.len() {
            table[i] = i as u32;
            for _ in 0..8 {
                table[i] = if table[i] & 1 != 0 {
                    (table[i] >> 1) ^ 0xEDB88320
                } else {
                    table[i] >> 1
                };
            }
        }

        Self { table }
    }
    pub fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = 0xffffffffu32;
        for data in bytes.iter() {
            crc = self.table[((crc ^ *data as u32) as usize) & 0xff] ^ (crc >> 8);
        }

        return crc;
    }
}
