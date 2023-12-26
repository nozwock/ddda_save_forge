use anyhow::Result;
use endian_codec::{DecodeBE, DecodeLE, EncodeBE, EncodeLE, PackedSize};

/// A zlib compressed archive follows a 256-byte header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DDDASave {
    header: DDDASaveHeader,
    compressed_save_data: Vec<u8>,
}

/// References:
/// - https://www.fluffyquack.com/tools/source/DDsavetool.rar
#[derive(Debug, Clone, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
pub struct DDDASaveHeader {
    version: u32, // 0x15 for DDDA on console/PC, and 0x5 for the original DD on console
    uncompressed_size: u32,
    compressed_size: u32,
    _unknown0: u32, // 0x334D234D
    _pad0: u32,     // 0x00000000
    _unknown1: u32, // 0x334D4044
    checksum: u32,  // Checksum of compressed save data
    _unknown2: u32, // 0x40565235
}

impl TryFrom<&[u8]> for DDDASaveHeader {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut header = DDDASaveHeader::decode_from_le_bytes(value);
        if header.version != 0x15 {
            header = DDDASaveHeader::decode_from_be_bytes(value);
            if header.version != 0x15 {
                anyhow::bail!("Invalid save header");
            }
        }

        // println!("{:#X?}", header);

        Ok(header)
    }
}

impl TryFrom<&[u8]> for DDDASave {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(DDDASave {
            header: DDDASaveHeader::try_from(value)?,
            compressed_save_data: value
                .get(32..)
                .ok_or_else(|| anyhow::anyhow!("Invalid save data"))?
                .into(),
        })
    }
}

impl DDDASave {
    pub fn unpack(&self) -> Result<Vec<u8>> {
        use std::io::prelude::*;

        let mut data = Vec::<u8>::with_capacity(self.header.uncompressed_size as usize);
        flate2::read::ZlibDecoder::new(self.compressed_save_data.as_slice())
            .read_to_end(&mut data)?;

        Ok(data)
    }
}
