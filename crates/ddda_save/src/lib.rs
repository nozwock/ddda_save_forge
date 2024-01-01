pub mod crc;

use anyhow::Result;
use crc::Crc32ITUv42;
use endian_codec::{DecodeBE, DecodeLE, EncodeBE, EncodeLE, PackedSize};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::prelude::*;

const MAX_SAVE_SIZE: usize = 524288;

/// A zlib compressed archive follows a 256-byte header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DDDASave {
    header: DDDASaveHeader,
    compressed_save_data: Vec<u8>,
}

/// References:
/// - https://www.fluffyquack.com/tools/source/DDsavetool.rar
#[derive(Debug, Clone, PartialEq, Eq, PackedSize, EncodeLE, DecodeLE, EncodeBE, DecodeBE)]
struct DDDASaveHeader {
    version: u32, // 0x15 for DDDA on console/PC, and 0x5 for the original DD on console
    uncompressed_size: u32,
    compressed_size: u32,
    _unknown0: u32, // 0x334D234D
    _pad0: u32,     // 0x00000000
    _unknown1: u32, // 0x334D4044
    checksum: u32,  // CRC32 ITU V.42 checksum of the compressed data
    _unknown2: u32, // 0x40565235
}

impl Default for DDDASaveHeader {
    fn default() -> Self {
        Self {
            version: 0x15,
            uncompressed_size: Default::default(),
            compressed_size: Default::default(),
            _unknown0: 0x334D234D,
            _pad0: 0x0,
            _unknown1: 0x334D4044,
            checksum: Default::default(),
            _unknown2: 0x40565235,
        }
    }
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
        let mut data = Vec::<u8>::with_capacity(self.header.uncompressed_size as usize);
        ZlibDecoder::new(self.compressed_save_data.as_slice()).read_to_end(&mut data)?;

        Ok(data)
    }
    pub fn repack(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(data)?;
        let compressed_data = encoder.finish()?;

        let header = DDDASaveHeader {
            uncompressed_size: data.len() as u32,
            compressed_size: compressed_data.len() as u32,
            checksum: Crc32ITUv42::new().checksum(&compressed_data),
            ..Default::default()
        };

        let mut header_bytes = [0; DDDASaveHeader::PACKED_LEN];
        // Panics for Vec, and needs exact PACKED_LEN sized array.
        // Also, the PC game doesn't seem to care about the header endianness, either le or be are fine.
        header.encode_as_le_bytes(&mut header_bytes);
        let mut compressed_save = header_bytes.to_vec();
        compressed_save.extend(compressed_data.into_iter());

        let pad_size = MAX_SAVE_SIZE.checked_sub(compressed_save.len()).expect(
            "Compressed data size shouldn't be bigger than the MAX_SAVE_SIZE of 524288-byte",
        );
        if pad_size > 0 {
            compressed_save.extend(vec![0; pad_size].into_iter())
        }

        Ok(compressed_save)
    }
}
