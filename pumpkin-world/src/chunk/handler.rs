use pumpkin_core::math::vector2::Vector2;

use super::{
    anvil_temp::AnvilWorldFormat,
    world_format::{ChunkData, WorldFormat, WorldHandlingError, WorldInfo},
};

/// Different world formats that can be used
/// Wrapper around the different world formats
pub struct WorldLoader {
    format: Format,
}

// Keep this hidden so that helper functions are used (abstraction)
enum Format {
    Anvil(AnvilWorldFormat),
}

// TODO: Maybe make this a macro?
impl WorldLoader {
    /// Load a world with the Anvil world format
    pub async fn anvil(world_path: String) -> Result<Self, WorldHandlingError> {
        let anvil_format = AnvilWorldFormat::load_world(world_path).await?;

        Ok(Self {
            format: Format::Anvil(anvil_format),
        })
    }

    // Use match statements here to match the correct function
    // Match statements are a zero cost abstraction so no performance hit here

    /// Get the stored world info
    pub async fn get_world_info(&self) -> &WorldInfo {
        match &self.format {
            Format::Anvil(anvil_world_format) => anvil_world_format.get_world_info().await,
        }
    }

    /// Read the chunk at the specified position
    pub async fn read_chunk(&self, at: Vector2<i32>) -> Result<ChunkData, WorldHandlingError> {
        match &self.format {
            Format::Anvil(anvil_world_format) => anvil_world_format.read_chunk(at).await,
        }
    }
}
