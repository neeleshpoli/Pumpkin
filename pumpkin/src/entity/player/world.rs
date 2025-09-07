use std::{collections::VecDeque, num::NonZeroU8, ops::AddAssign, sync::Arc};

use pumpkin_protocol::{
    bedrock::client::set_time::CSetTime,
    codec::var_int::VarInt,
    java::client::play::{CChangeDifficulty, CUnloadChunk, CUpdateTime},
};
use pumpkin_util::{
    GameMode,
    math::{boundingbox::BoundingBox, position::BlockPos, vector2::Vector2, vector3::Vector3},
};
use pumpkin_world::{
    chunk::{ChunkData, ChunkEntityData},
    cylindrical_chunk_iterator::Cylindrical,
    level::{SyncChunk, SyncEntityChunk},
};
use tokio::sync::RwLock;

use crate::{entity::player::Player, net::ClientPlatform, world::World};

impl Player {
    pub fn world(&self) -> &Arc<World> {
        &self.living_entity.entity.world
    }

    /// Sets the player's difficulty level.
    pub async fn send_difficulty_update(&self) {
        let world = self.world();
        let level_info = world.level_info.read().await;
        self.client
            .enqueue_packet(&CChangeDifficulty::new(
                level_info.difficulty as u8,
                level_info.difficulty_locked,
            ))
            .await;
    }

    /// Sends the world time to only this player.
    pub async fn send_time(&self, world: &World) {
        let l_world = world.level_time.lock().await;
        match &self.client {
            ClientPlatform::Java(java_client) => {
                java_client
                    .enqueue_packet(&CUpdateTime::new(
                        l_world.world_age,
                        l_world.time_of_day,
                        true,
                    ))
                    .await;
            }
            ClientPlatform::Bedrock(bedrock_client) => {
                bedrock_client
                    .send_game_packet(&CSetTime {
                        time: VarInt(l_world.query_daytime() as _),
                    })
                    .await;
            }
        }
    }

    pub(super) async fn unload_watched_chunks(&self, world: &World) {
        let radial_chunks = self.watched_section.load().all_chunks_within();
        let level = &world.level;
        let chunks_to_clean = level.mark_chunks_as_not_watched(&radial_chunks).await;
        level.clean_chunks(&chunks_to_clean).await;
        for chunk in chunks_to_clean {
            self.client
                .enqueue_packet(&CUnloadChunk::new(chunk.x, chunk.y))
                .await;
        }

        self.watched_section.store(Cylindrical::new(
            Vector2::new(0, 0),
            NonZeroU8::new(1).unwrap(),
        ));
    }

    pub fn block_interaction_range(&self) -> f64 {
        if self.gamemode.load() == GameMode::Creative {
            5.0
        } else {
            4.5
        }
    }

    pub fn can_interact_with_block_at(&self, position: &BlockPos, additional_range: f64) -> bool {
        let d = self.block_interaction_range() + additional_range;
        let box_pos = BoundingBox::from_block(position);
        let entity_pos = self.living_entity.entity.pos.load();
        let standing_eye_height = self.living_entity.entity.standing_eye_height;
        box_pos.squared_magnitude(Vector3 {
            x: entity_pos.x,
            y: entity_pos.y + f64::from(standing_eye_height),
            z: entity_pos.z,
        }) < d * d
    }

    /// Removes the [`Player`] out of the current [`World`].
    pub async fn remove(self: &Arc<Self>) {
        let world = self.world();
        world.remove_player(self, true).await;

        let cylindrical = self.watched_section.load();

        // Radial chunks are all of the chunks the player is theoretically viewing.
        // Given enough time, all of these chunks will be in memory.
        let radial_chunks = cylindrical.all_chunks_within();

        log::debug!(
            "Removing player {}, unwatching {} chunks",
            self.gameprofile.name,
            radial_chunks.len()
        );

        let level = &world.level;

        // Decrement the value of watched chunks
        let chunks_to_clean = level.mark_chunks_as_not_watched(&radial_chunks).await;
        // Remove chunks with no watchers from the cache
        level.clean_chunks(&chunks_to_clean).await;
        level.clean_entity_chunks(&chunks_to_clean).await;
        // Remove left over entries from all possiblily loaded chunks
        level.clean_memory();

        log::debug!(
            "Removed player id {} from world {} ({} chunks remain cached)",
            self.gameprofile.name,
            "world", // TODO: Add world names
            level.loaded_chunk_count(),
        );

        level.clean_up_log().await;

        //self.world().level.list_cached();
    }
}

enum BatchState {
    Initial,
    Waiting,
    Count(u8),
}

pub struct ChunkManager {
    chunks_per_tick: usize,
    chunk_queue: VecDeque<(Vector2<i32>, SyncChunk)>,
    entity_chunk_queue: VecDeque<(Vector2<i32>, SyncEntityChunk)>,
    batches_sent_since_ack: BatchState,
}

impl ChunkManager {
    pub const NOTCHIAN_BATCHES_WITHOUT_ACK_UNTIL_PAUSE: u8 = 10;

    #[must_use]
    pub fn new(chunks_per_tick: usize) -> Self {
        Self {
            chunks_per_tick,
            chunk_queue: VecDeque::new(),
            entity_chunk_queue: VecDeque::new(),
            batches_sent_since_ack: BatchState::Initial,
        }
    }

    pub fn handle_acknowledge(&mut self, chunks_per_tick: f32) {
        self.batches_sent_since_ack = BatchState::Count(0);
        self.chunks_per_tick = chunks_per_tick.ceil() as usize;
    }

    pub fn push_chunk(&mut self, position: Vector2<i32>, chunk: SyncChunk) {
        self.chunk_queue.push_back((position, chunk));
    }

    pub fn push_entity(&mut self, position: Vector2<i32>, chunk: SyncEntityChunk) {
        self.entity_chunk_queue.push_back((position, chunk));
    }

    #[must_use]
    pub fn can_send_chunk(&self) -> bool {
        let state_available = match self.batches_sent_since_ack {
            BatchState::Count(count) => count < Self::NOTCHIAN_BATCHES_WITHOUT_ACK_UNTIL_PAUSE,
            BatchState::Initial => true,
            BatchState::Waiting => false,
        };

        state_available && !self.chunk_queue.is_empty()
    }

    pub fn next_chunk(&mut self) -> Box<[SyncChunk]> {
        let chunk_size = self.chunk_queue.len().min(self.chunks_per_tick);
        let chunks: Vec<Arc<RwLock<ChunkData>>> = self
            .chunk_queue
            .drain(0..chunk_size)
            .map(|(_, chunk)| chunk)
            .collect();

        match &mut self.batches_sent_since_ack {
            BatchState::Count(count) => {
                count.add_assign(1);
            }
            state @ BatchState::Initial => *state = BatchState::Waiting,
            BatchState::Waiting => (),
        }

        chunks.into_boxed_slice()
    }

    pub fn next_entity(&mut self) -> Box<[SyncEntityChunk]> {
        let chunk_size = self.entity_chunk_queue.len().min(self.chunks_per_tick);
        let chunks: Vec<Arc<RwLock<ChunkEntityData>>> = self
            .entity_chunk_queue
            .drain(0..chunk_size)
            .map(|(_, chunk)| chunk)
            .collect();

        match &mut self.batches_sent_since_ack {
            BatchState::Count(count) => {
                count.add_assign(1);
            }
            state @ BatchState::Initial => *state = BatchState::Waiting,
            BatchState::Waiting => unreachable!(),
        }

        chunks.into_boxed_slice()
    }

    #[must_use]
    pub fn is_chunk_pending(&self, pos: &Vector2<i32>) -> bool {
        // This is probably comparable to hashmap speed due to the relatively small count of chunks
        // (guestimated to be ~ 1024)
        self.chunk_queue.iter().any(|(elem_pos, _)| elem_pos == pos)
    }
}
