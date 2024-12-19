use std::io::Read;

use flate2::bufread::{GzDecoder, ZlibDecoder};

use crate::chunk::world_format::WorldHandlingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    /// GZip Compression
    GZip,
    /// ZLib Compression
    ZLib,
    /// Uncompressed (since a version before 1.15.1)
    None,
    /// LZ4 Compression (since 24w04a)
    LZ4,
    /// Custom compression algorithm (since 24w05a)
    Custom,
}

impl Compression {
    pub fn decompress_data(&self, compressed_data: Vec<u8>) -> Result<Vec<u8>, WorldHandlingError> {
        match self {
            Compression::GZip => {
                let mut decoder = GzDecoder::new(&compressed_data[..]);
                let mut chunk_data = Vec::new();
                decoder.read_to_end(&mut chunk_data).map_err(|e| {
                    WorldHandlingError::CompressionError("Gzip".to_string(), e.kind())
                })?;
                Ok(chunk_data)
            }
            Compression::ZLib => {
                let mut decoder = ZlibDecoder::new(&compressed_data[..]);
                let mut chunk_data = Vec::new();
                decoder.read_to_end(&mut chunk_data).map_err(|e| {
                    WorldHandlingError::CompressionError("Zlib".to_string(), e.kind())
                })?;
                Ok(chunk_data)
            }
            Compression::None => Ok(compressed_data),
            Compression::LZ4 => {
                let mut decoder = lz4::Decoder::new(compressed_data.as_slice()).map_err(|e| {
                    WorldHandlingError::CompressionError("LZ4".to_string(), e.kind())
                })?;
                let mut decompressed_data = Vec::new();
                decoder.read_to_end(&mut decompressed_data).map_err(|e| {
                    WorldHandlingError::CompressionError("LZ4".to_string(), e.kind())
                })?;
                Ok(decompressed_data)
            }
            Compression::Custom => todo!(),
        }
    }
}

impl TryFrom<u8> for Compression {
    type Error = WorldHandlingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::GZip),
            2 => Ok(Self::ZLib),
            3 => Ok(Self::None),
            4 => Ok(Self::LZ4),
            // Creative i guess?
            127 => Ok(Self::Custom),
            // Return error for unknown compression values
            _ => Err(WorldHandlingError::CompressionError(
                "Unknown".to_string(),
                std::io::ErrorKind::NotFound,
            )),
        }
    }
}
