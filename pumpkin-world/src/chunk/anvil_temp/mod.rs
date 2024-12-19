use chunk_format::AnvilChunkData;
use compression::Compression;
use level_dat::LevelDat;
use pumpkin_core::math::vector2::Vector2;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt},
};

use super::world_format::{ChunkData, WorldFormat, WorldHandlingError, WorldInfo};

mod chunk_format;
mod compression;
mod level_dat;

pub(super) struct AnvilWorldFormat {
    _lock: File,
    info: WorldInfo,
    world_path: String,
}

impl WorldFormat for AnvilWorldFormat {
    async fn load_world(world_path: String) -> Result<Self, WorldHandlingError> {
        // Obtain file lock on session.lock to indicate the world folder is being used by the process
        // If it fails to obtain the lock then the world is being used by another process
        // https://minecraft.wiki/w/Java_Edition_level_format#session.lock_format
        let lock = OpenOptions::new()
            .write(true)
            .open(format!("{world_path}/session.lock"))
            .await
            .map_err(|_| WorldHandlingError::WorldInUse)?;

        Ok(Self {
            // Keep the lock so other processes know world is being used
            _lock: lock,
            // The info of the world
            info: LevelDat::read_level_dat(&world_path).await?,
            world_path,
        })
    }

    async fn get_world_info(&self) -> &WorldInfo {
        &self.info
    }

    async fn read_chunk(&self, at: Vector2<i32>) -> Result<ChunkData, WorldHandlingError> {
        // Each region file stores 32x32 chunks
        // Using a >> 5 is equalivant to dividing by 32
        // This helps find which region file the chunk is in
        let region = (at.x >> 5, at.z >> 5);

        // Open the region file
        let mut region_file = OpenOptions::new()
            .read(true)
            .open(format!(
                "./{}/region/r.{}.{}.mca",
                &self.world_path, region.0, region.1
            ))
            .await?;

        // Get the location and timestamp tables
        let mut location_table: [u8; 4096] = [0; 4096];
        let mut timestamp_table: [u8; 4096] = [0; 4096];

        region_file.read_exact(&mut location_table).await?;
        region_file.read_exact(&mut timestamp_table).await?;

        // Find the offset
        let modulus = |a: i32, b: i32| ((a % b) + b) % b;
        let chunk_x = modulus(at.x, 32) as u32;
        let chunk_z = modulus(at.z, 32) as u32;
        let table_entry = (chunk_x + chunk_z * 32) * 4;

        let mut offset = vec![0u8];
        offset.extend_from_slice(&location_table[table_entry as usize..table_entry as usize + 3]);
        let offset = u32::from_be_bytes(offset.try_into().unwrap()) as u64 * 4096;
        let size = location_table[table_entry as usize + 3] as usize * 4096;

        if offset == 0 && size == 0 {
            return Err(WorldHandlingError::NotFound);
        }

        // Read the file using the offset and size
        let mut file_buf = {
            region_file.seek(std::io::SeekFrom::Start(offset)).await?;
            let mut out = vec![0; size];
            region_file.read_exact(&mut out).await?;
            out
        };

        // TODO: check checksum to make sure chunk is not corrupted
        let header: Vec<u8> = file_buf.drain(0..5).collect();
        let compression = Compression::try_from(header[4])?;

        let size = u32::from_be_bytes(header[..4].try_into().unwrap());

        // size includes the compression scheme byte, so we need to subtract 1
        let chunk_data = file_buf.drain(0..size as usize - 1).collect();
        let decompressed_chunk = compression.decompress_data(chunk_data)?;

        AnvilChunkData::to_chunk_data(decompressed_chunk, at)
    }
}
