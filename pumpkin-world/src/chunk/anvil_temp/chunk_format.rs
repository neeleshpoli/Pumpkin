use std::cmp::max;

use pumpkin_core::math::vector2::Vector2;
use serde::Deserialize;

use crate::{block::BlockState, chunk::world_format::{ChunkData, WorldHandlingError, CHUNK_AREA, CHUNK_VOLUME, SUBCHUNK_VOLUME}, coordinates::{ChunkRelativeBlockCoordinates, Height}};

/// The current data version of the anvil world format
const DATA_VERSION: i32 = 4189;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "Status")]
enum ChunkStatus {
    #[serde(rename = "minecraft:empty")]
    Empty,
    #[serde(rename = "minecraft:structure_starts")]
    StructureStarts,
    #[serde(rename = "minecraft:structure_references")]
    StructureReferences,
    #[serde(rename = "minecraft:biomes")]
    Biomes,
    #[serde(rename = "minecraft:noise")]
    Noise,
    #[serde(rename = "minecraft:surface")]
    Surface,
    #[serde(rename = "minecraft:carvers")]
    Carvers,
    #[serde(rename = "minecraft:features")]
    Features,
    #[serde(rename = "minecraft:initialize_light")]
    InitLight,
    #[serde(rename = "minecraft:light")]
    Light,
    #[serde(rename = "minecraft:spawn")]
    Spawn,
    #[serde(rename = "minecraft:full")]
    Full,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnvilChunkData {
    data_version: i32,
    #[serde(rename = "sections")]
    sections: Vec<ChunkSection>,
    heightmaps: ChunkHeightmaps,
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct ChunkSection {
    #[serde(rename = "Y")]
    y: i32,
    block_states: Option<ChunkSectionBlockStates>,
}

#[derive(Deserialize)]
struct ChunkSectionBlockStates {
    //  #[serde(with = "LongArray")]
    data: Option<Vec<i64>>,
    palette: Vec<PaletteEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PaletteEntry {
    name: String,
    // TODO: add properties field
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ChunkHeightmaps {
    motion_blocking: Vec<i64>,
    world_surface: Vec<i64>,
    // TODO add all other heightmaps
}

impl AnvilChunkData {
    pub(super) fn to_chunk_data(chunk_data: Vec<u8>, at: Vector2<i32>) -> Result<ChunkData, WorldHandlingError> {
        // Ensure the chunk has finished generating
        if fastnbt::from_bytes::<ChunkStatus>(&chunk_data)
            .map_err(|e| WorldHandlingError::DeserializationError(e.to_string()))?
            != ChunkStatus::Full
        {
            return Err(WorldHandlingError::ChunkNotGenerated);
        }

        let chunk_data = fastnbt::from_bytes::<AnvilChunkData>(chunk_data.as_slice())
            .map_err(|e| WorldHandlingError::DeserializationError(e.to_string()))?;

        // Ensure we are reading chunks for the correct format version
        if chunk_data.data_version != DATA_VERSION {
            return Err(WorldHandlingError::OutdatedWorldFormat)
        }
        
        let mut chunk = ChunkData {
            blocks: Box::new([0u16; CHUNK_VOLUME]),
            motion_blocking_map: chunk_data.heightmaps.motion_blocking,
            world_surface_map: chunk_data.heightmaps.world_surface,
            position: at,
        };
        let mut block_index = 0;

        for section in chunk_data.sections {
            let block_states = match section.block_states {
                Some(states) => states,
                None => continue, // TODO @lukas0008 this should instead fill all blocks with the only element of the palette
            };

            let pallete = block_states.palette
                .iter()
                .map(|entry| match BlockState::new(&entry.name) {
                    // Block not found, Often the case when World has an newer or older version then block registry
                    Some(state) => state,
                    None => BlockState::AIR,                    
                })
                .collect::<Vec<_>>();

            let block_data = match block_states.data {
                Some(d) => d,
                None => {
                    // We skipped placing an empty subchunk.
                    // We need to increase the y coordinate of the next subchunk being placed.
                    block_index += SUBCHUNK_VOLUME;
                    continue;
                },
            };

            let block_bit_size = {
                let size = 64 - (pallete.len() as i64 - 1).leading_zeros();
                max(4, size)
            };
            // How many blocks there are in one of the palletes u64s
            let blocks_in_pallete = 64 / block_bit_size;
            let mask = (1 << block_bit_size) - 1;

            'block_loop: for block in block_data {
                for i in 0..blocks_in_pallete {
                    let index = (block >> (i * block_bit_size)) & mask;
                    let block = &pallete[index as usize];

                    // TODO allow indexing blocks directly so we can just use block_index and save some time?
                    // this is fine because we initalized the heightmap of `blocks`
                    // from the cached value in the world file
                    chunk.set_block_no_heightmap_update(ChunkRelativeBlockCoordinates {
                        x: (block_index % 16).into(),
                        y: Height::from_absolute((block_index / CHUNK_AREA) as u16),
                        z: ((block_index % CHUNK_AREA) / 16).into(),
                    }, block.get_id());

                    block_index += 1;

                    // if `SUBCHUNK_VOLUME `is not divisible by `blocks_in_pallete` the block_data
                    // can sometimes spill into other subchunks. We avoid that by aborting early
                    if (block_index % SUBCHUNK_VOLUME) == 0 {
                        break 'block_loop;
                    }
                }
            }
        }

        Ok(chunk)
    }
}
