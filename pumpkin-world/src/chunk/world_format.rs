use pumpkin_core::math::{vector2::Vector2, vector3::Vector3};
use thiserror::Error;

use crate::{coordinates::ChunkRelativeBlockCoordinates, WORLD_HEIGHT};

// Info from wiki:
// https://minecraft.wiki/w/Java_Edition_level_format#level.dat_format

/// Trait that other world formats MUST implement
pub(crate) trait WorldFormat: Sized {
    /// Initialize the world format handler and get ready to load the world
    async fn load_world(world_path: String) -> Result<Self, WorldHandlingError>;
    async fn get_world_info(&self) -> &WorldInfo;
    async fn read_chunk(&self, at: Vector2<i32>) -> Result<ChunkData, WorldHandlingError>;
}

/// Basic info that is needed fot the world to be loaded
/// World format specific things are not included
/// Things specified in configuration are not included since the config overrides anyway
#[derive(Debug, PartialEq)]
pub struct WorldInfo {
    /// Center of the world border
    pub border_center: Vector2<f64>,
    /// How much damage to do outside of the border
    pub border_damage_per_block: f64,
    /// The length and width of the border
    pub border_size: f64,
    pub border_safe_zone: f64,
    pub border_size_lerp_target: f64,
    pub border_warning_blocks: f64,
    pub border_warning_time: f64,
    /// How many ticks until clear weather ends
    pub clear_weather_time: i32,
    ///
    pub day_time: i64,
    pub seed: i64,
    pub raining: bool,
    pub rain_time: i32,
    pub spawn: Vector3<i32>,
    pub thundering: bool,
    pub thunder_time: i32,
    pub time: i64,
}

/// Errors that different world formats can encounter
#[derive(Error, Debug)]
pub enum WorldHandlingError {
    #[error("World already being used by another process?")]
    WorldInUse,
    #[error("IO error: {0}")]
    IoError(std::io::ErrorKind),
    #[error("File not found")]
    NotFound,
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Chunk not fully generated")]
    ChunkNotGenerated,
    #[error("Compression Error (Type: {0}): {1}")]
    CompressionError(String, std::io::ErrorKind), // There are so many types of compression, so the type will be a string
    #[error("World format too old, world upgrade required")]
    OutdatedWorldFormat,
    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for WorldHandlingError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            e @ _ => Self::IoError(e),
        }
    }
}

pub const CHUNK_AREA: usize = 16 * 16;
pub const SUBCHUNK_VOLUME: usize = CHUNK_AREA * 16;
pub const CHUNK_VOLUME: usize = CHUNK_AREA * WORLD_HEIGHT;

pub struct ChunkData {
    // this needs to be boxed, otherwise it will cause a stack-overflow
    pub blocks: Box<[u16; CHUNK_VOLUME]>,
    pub motion_blocking_map: Vec<i64>,
    pub world_surface_map: Vec<i64>,
    // TODO: Add other maps
    pub position: Vector2<i32>,
}

impl ChunkData {
    // convenience methods
    pub fn set_block_no_heightmap_update(
        &mut self,
        position: ChunkRelativeBlockCoordinates,
        block: u16,
    ) -> u16 {
        std::mem::replace(&mut self.blocks[Self::convert_index(position)], block)
    }

    fn convert_index(index: ChunkRelativeBlockCoordinates) -> usize {
        // % works for negative numbers as intended.
        index.y.get_absolute() as usize * CHUNK_AREA + *index.z as usize * 16 + *index.x as usize
    }
}
