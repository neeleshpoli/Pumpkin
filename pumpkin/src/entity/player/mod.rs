mod abilities;
mod actions;
mod chat;
mod effect;
mod entity_base;
mod experience;
mod inventory;
mod last_seen;
mod respawn;
mod screen_handler;
mod sound;
mod teleport;
mod title;
mod world;

use std::num::NonZeroU8;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU8, AtomicU32, Ordering};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use crossbeam::atomic::AtomicCell;
use pumpkin_protocol::bedrock::client::level_chunk::CLevelChunk;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

use pumpkin_config::advanced_config;
use pumpkin_data::entity::{EntityPose, EntityStatus, EntityType};
use pumpkin_data::particle::Particle;
use pumpkin_inventory::player::{
    player_inventory::PlayerInventory, player_screen_handler::PlayerScreenHandler,
};
use pumpkin_inventory::screen_handler::{ScreenHandler, ScreenHandlerListener};
use pumpkin_inventory::sync_handler::SyncHandler;
use pumpkin_macros::send_cancellable;
use pumpkin_nbt::compound::NbtCompound;
use pumpkin_protocol::codec::var_int::VarInt;
use pumpkin_protocol::java::client::play::{
    CAcknowledgeBlockChange, CChunkBatchEnd, CChunkBatchStart, CChunkData, CCombatDeath,
    CGameEvent, CKeepAlive, CParticle, CPlayerInfoUpdate, CSetHealth, GameEvent, MetaDataType,
    Metadata, PlayerAction, PlayerInfoFlags,
};
use pumpkin_util::GameMode;
use pumpkin_util::math::{self, position::BlockPos, vector2::Vector2, vector3::Vector3};
use pumpkin_util::permission::PermissionLvl;
use pumpkin_util::text::TextComponent;
use pumpkin_world::cylindrical_chunk_iterator::Cylindrical;
use pumpkin_world::entity::entity_data_flags::{
    DATA_PLAYER_MAIN_HAND, DATA_PLAYER_MODE_CUSTOMISATION,
};
use pumpkin_world::item::ItemStack;

use crate::PERMISSION_MANAGER;
use crate::command::client_suggestions;
use crate::command::dispatcher::CommandDispatcher;
use crate::data::op_data::OPERATOR_CONFIG;
use crate::entity::hunger::HungerManager;
use crate::entity::living::LivingEntity;
use crate::entity::player::screen_handler::ScreenListener;
use crate::entity::player::{Abilities, ChunkManager, RespawnPoint};
use crate::entity::{Entity, EntityBase, NBTStorage, NBTStorageInit};
use crate::net::{ClientPlatform, GameProfile};
use crate::net::{DisconnectReason, PlayerConfig};
use crate::plugin::player::player_gamemode_change::PlayerGamemodeChangeEvent;
use crate::server::Server;
use crate::world::World;

pub use abilities::*;
pub use chat::*;
pub use last_seen::*;
pub use respawn::*;
pub use title::*;
pub use world::*;

pub const DATA_VERSION: i32 = 4438; // 1.21.8

/// Represents a Minecraft player entity.
///
/// A `Player` is a special type of entity that represents a human player connected to the server.
pub struct Player {
    /// The underlying living entity object that represents the player.
    pub living_entity: LivingEntity,
    /// The player's game profile information, including their username and UUID.
    pub gameprofile: GameProfile,
    /// The client connection associated with the player.
    pub client: ClientPlatform,
    /// The player's inventory.
    pub inventory: Arc<PlayerInventory>,
    /// The player's configuration settings. Changes when the player changes their settings.
    pub config: RwLock<PlayerConfig>,
    /// The player's current gamemode (e.g., Survival, Creative, Adventure).
    pub gamemode: AtomicCell<GameMode>,
    /// The player's previous gamemode
    pub previous_gamemode: AtomicCell<Option<GameMode>>,
    /// The player's spawnpoint
    pub respawn_point: AtomicCell<Option<RespawnPoint>>,
    /// The player's sleep status
    pub sleeping_since: AtomicCell<Option<u8>>,
    /// Manages the player's hunger level.
    pub hunger_manager: HungerManager,
    /// The ID of the currently open container (if any).
    pub open_container: AtomicCell<Option<u64>>,
    /// The item currently being held by the player.
    pub carried_item: Mutex<Option<ItemStack>>,
    /// The player's abilities and special powers.
    ///
    /// This field represents the various abilities that the player possesses, such as flight, invulnerability, and other special effects.
    ///
    /// **Note:** When the `abilities` field is updated, the server should send a `send_abilities_update` packet to the client to notify them of the changes.
    pub abilities: Mutex<Abilities>,
    /// The current stage of block destruction of the block the player is breaking.
    pub current_block_destroy_stage: AtomicI32,
    /// Indicates if the player is currently mining a block.
    pub mining: AtomicBool,
    pub start_mining_time: AtomicI32,
    pub tick_counter: AtomicI32,
    pub packet_sequence: AtomicI32,
    pub mining_pos: Mutex<BlockPos>,
    /// A counter for teleport IDs used to track pending teleports.
    pub teleport_id_count: AtomicI32,
    /// The pending teleport information, including the teleport ID and target location.
    pub awaiting_teleport: Mutex<Option<(VarInt, Vector3<f64>)>>,
    /// The coordinates of the chunk section the player is currently watching.
    pub watched_section: AtomicCell<Cylindrical>,
    /// Whether we are waiting for a response after sending a keep alive packet.
    pub wait_for_keep_alive: AtomicBool,
    /// The keep alive packet payload we send. The client should respond with the same id.
    pub keep_alive_id: AtomicI64,
    /// The last time we sent a keep alive packet.
    pub last_keep_alive_time: AtomicCell<Instant>,
    /// The amount of ticks since the player's last attack.
    pub last_attacked_ticks: AtomicU32,
    /// The player's last known experience level.
    pub last_sent_xp: AtomicI32,
    pub last_sent_health: AtomicI32,
    pub last_sent_food: AtomicU8,
    pub last_food_saturation: AtomicBool,
    /// The player's permission level.
    pub permission_lvl: AtomicCell<PermissionLvl>,
    /// Whether the client has reported that it has loaded.
    pub client_loaded: AtomicBool,
    /// The amount of time (in ticks) the client has to report having finished loading before being timed out.
    pub client_loaded_timeout: AtomicU32,
    /// The player's experience level.
    pub experience_level: AtomicI32,
    /// The player's experience progress (`0.0` to `1.0`)
    pub experience_progress: AtomicCell<f32>,
    /// The player's total experience points.
    pub experience_points: AtomicI32,
    pub experience_pick_up_delay: Mutex<u32>,
    pub chunk_manager: Mutex<ChunkManager>,
    pub has_played_before: AtomicBool,
    pub chat_session: Arc<Mutex<ChatSession>>,
    pub signature_cache: Mutex<MessageCache>,
    pub player_screen_handler: Arc<Mutex<PlayerScreenHandler>>,
    pub current_screen_handler: Mutex<Arc<Mutex<dyn ScreenHandler>>>,
    pub screen_handler_sync_id: AtomicU8,
    pub screen_handler_listener: Arc<dyn ScreenHandlerListener>,
    pub screen_handler_sync_handler: Arc<SyncHandler>,
}

impl Player {
    pub async fn new(
        client: ClientPlatform,
        gameprofile: GameProfile,
        config: PlayerConfig,
        world: Arc<World>,
        gamemode: GameMode,
    ) -> Self {
        let player_uuid = gameprofile.id;

        let living_entity = LivingEntity::new(Entity::new(
            player_uuid,
            world,
            Vector3::new(0.0, 100.0, 0.0),
            &EntityType::PLAYER,
            matches!(gamemode, GameMode::Creative | GameMode::Spectator),
        ));

        let inventory = Arc::new(PlayerInventory::new(
            living_entity.entity_equipment.clone(),
            living_entity.equipment_slots.clone(),
        ));

        let player_screen_handler = Arc::new(Mutex::new(
            PlayerScreenHandler::new(&inventory, None, 0).await,
        ));

        Self {
            living_entity,
            config: RwLock::new(config),
            gameprofile,
            client,
            awaiting_teleport: Mutex::new(None),
            // TODO: Load this from previous instance
            hunger_manager: HungerManager::default(),
            current_block_destroy_stage: AtomicI32::new(-1),
            open_container: AtomicCell::new(None),
            tick_counter: AtomicI32::new(0),
            packet_sequence: AtomicI32::new(-1),
            start_mining_time: AtomicI32::new(0),
            carried_item: Mutex::new(None),
            experience_pick_up_delay: Mutex::new(0),
            teleport_id_count: AtomicI32::new(0),
            mining: AtomicBool::new(false),
            mining_pos: Mutex::new(BlockPos::ZERO),
            abilities: Mutex::new(Abilities::default()),
            gamemode: AtomicCell::new(gamemode),
            previous_gamemode: AtomicCell::new(None),
            // TODO: Send the CPlayerSpawnPosition packet when the client connects with proper values
            respawn_point: AtomicCell::new(None),
            sleeping_since: AtomicCell::new(None),
            // We want this to be an impossible watched section so that `chunker::update_position`
            // will mark chunks as watched for a new join rather than a respawn.
            // (We left shift by one so we can search around that chunk)
            watched_section: AtomicCell::new(Cylindrical::new(
                Vector2::new(0, 0),
                // Since 1 is not possible in vanilla it is used as uninit
                NonZeroU8::new(1).unwrap(),
            )),
            wait_for_keep_alive: AtomicBool::new(false),
            keep_alive_id: AtomicI64::new(0),
            last_keep_alive_time: AtomicCell::new(std::time::Instant::now()),
            last_attacked_ticks: AtomicU32::new(0),
            client_loaded: AtomicBool::new(false),
            client_loaded_timeout: AtomicU32::new(60),
            // Minecraft has no way to change the default permission level of new players.
            // Minecraft's default permission level is 0.
            permission_lvl: OPERATOR_CONFIG.read().await.get_entry(&player_uuid).map_or(
                AtomicCell::new(advanced_config().commands.default_op_level),
                |op| AtomicCell::new(op.level),
            ),
            inventory,
            // TODO: enderChestInventory
            experience_level: AtomicI32::new(0),
            experience_progress: AtomicCell::new(0.0),
            experience_points: AtomicI32::new(0),
            // Default to sending 16 chunks per tick.
            chunk_manager: Mutex::new(ChunkManager::new(16)),
            last_sent_xp: AtomicI32::new(-1),
            last_sent_health: AtomicI32::new(-1),
            last_sent_food: AtomicU8::new(0),
            last_food_saturation: AtomicBool::new(true),
            has_played_before: AtomicBool::new(false),
            chat_session: Arc::new(Mutex::new(ChatSession::default())), // Placeholder value until the player actually sets their session id
            signature_cache: Mutex::new(MessageCache::default()),
            player_screen_handler: player_screen_handler.clone(),
            current_screen_handler: Mutex::new(player_screen_handler),
            screen_handler_sync_id: AtomicU8::new(0),
            screen_handler_listener: Arc::new(ScreenListener {}),
            screen_handler_sync_handler: Arc::new(SyncHandler::new()),
        }
    }

    /// Spawns a task associated with this player-client. All tasks spawned with this method are awaited
    /// when the client. This means tasks should complete in a reasonable amount of time or select
    /// on `Self::await_close_interrupt` to cancel the task when the client is closed
    ///
    /// Returns an `Option<JoinHandle<F::Output>>`. If the client is closed, this returns `None`.
    pub fn spawn_task<F>(&self, task: F) -> Option<JoinHandle<F::Output>>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.client.spawn_task(task)
    }

    pub async fn get_off_ground_speed(&self) -> f64 {
        let sprinting = self.get_entity().sprinting.load(Ordering::Relaxed);

        if !self.get_entity().has_vehicle().await {
            let fly_speed = {
                let abilities = self.abilities.lock().await;

                abilities.flying.then_some(f64::from(abilities.fly_speed))
            };

            if let Some(flying) = fly_speed {
                return if sprinting { flying * 2.0 } else { flying };
            }
        }

        if sprinting { 0.025_999_999 } else { 0.02 }
    }

    pub async fn spawn_particle(
        &self,
        position: Vector3<f64>,
        offset: Vector3<f32>,
        max_speed: f32,
        particle_count: i32,
        particle: Particle,
    ) {
        self.client
            .enqueue_packet(&CParticle::new(
                false,
                false,
                position,
                offset,
                max_speed,
                particle_count,
                VarInt(particle as i32),
                &[],
            ))
            .await;
    }

    // TODO Abstract the chunk sending
    #[allow(clippy::too_many_lines)]
    pub async fn tick(self: &Arc<Self>, server: &Server) {
        self.current_screen_handler
            .lock()
            .await
            .lock()
            .await
            .send_content_updates()
            .await;

        // if self.client.closed.load(Ordering::Relaxed) {
        //     return;
        // }

        if self.packet_sequence.load(Ordering::Relaxed) > -1 {
            self.client
                .enqueue_packet(&CAcknowledgeBlockChange::new(
                    self.packet_sequence.swap(-1, Ordering::Relaxed).into(),
                ))
                .await;
        }
        {
            let mut xp = self.experience_pick_up_delay.lock().await;
            if *xp > 0 {
                *xp -= 1;
            }
        }

        let chunk_of_chunks = {
            let mut chunk_manager = self.chunk_manager.lock().await;
            if let ClientPlatform::Java(_) = self.client {
                // Java clients can only send a limited amount of chunks per tick.
                // If we have sent too many chunks without receiving an ack, we stop sending chunks.
                chunk_manager
                    .can_send_chunk()
                    .then(|| chunk_manager.next_chunk())
            } else {
                Some(chunk_manager.next_chunk())
            }
        };

        if let Some(chunk_of_chunks) = chunk_of_chunks {
            let chunk_count = chunk_of_chunks.len();
            match &self.client {
                ClientPlatform::Java(java_client) => {
                    java_client.send_packet_now(&CChunkBatchStart).await;
                    for chunk in chunk_of_chunks {
                        let chunk = chunk.read().await;
                        // TODO: Can we check if we still need to send the chunk? Like if it's a fast moving
                        // player or something.
                        java_client.send_packet_now(&CChunkData(&chunk)).await;
                    }
                    java_client
                        .send_packet_now(&CChunkBatchEnd::new(chunk_count as u16))
                        .await;
                }
                ClientPlatform::Bedrock(bedrock_client) => {
                    for chunk in chunk_of_chunks {
                        let chunk = chunk.read().await;

                        bedrock_client
                            .send_game_packet(&CLevelChunk {
                                dimension: 0,
                                cache_enabled: false,
                                chunk: &chunk,
                            })
                            .await;
                    }
                }
            }
        }

        self.tick_counter.fetch_add(1, Ordering::Relaxed);
        if let Some(sleeping_since) = self.sleeping_since.load()
            && sleeping_since < 101
        {
            self.sleeping_since.store(Some(sleeping_since + 1));
        }

        if self.mining.load(Ordering::Relaxed) {
            let pos = self.mining_pos.lock().await;
            let world = self.world();
            let state = world.get_block_state(&pos).await;
            // Is the block broken?
            if state.is_air() {
                world
                    .set_block_breaking(&self.living_entity.entity, *pos, -1)
                    .await;
                self.current_block_destroy_stage
                    .store(-1, Ordering::Relaxed);
                self.mining.store(false, Ordering::Relaxed);
            } else {
                self.continue_mining(
                    *pos,
                    world,
                    state,
                    self.start_mining_time.load(Ordering::Relaxed),
                )
                .await;
            }
        }

        self.last_attacked_ticks.fetch_add(1, Ordering::Relaxed);

        self.living_entity.tick(self.clone(), server).await;
        self.hunger_manager.tick(self).await;

        // experience handling
        self.tick_experience().await;
        self.tick_health().await;

        // Timeout/keep alive handling
        self.tick_client_load_timeout();

        // TODO This should only be handled by the ClientPlatform
        let now = Instant::now();
        if now.duration_since(self.last_keep_alive_time.load()) >= Duration::from_secs(15) {
            if matches!(self.client, ClientPlatform::Bedrock(_)) {
                return;
            }
            // We never got a response from the last keep alive we sent.
            if self.wait_for_keep_alive.load(Ordering::Relaxed) {
                self.kick(
                    DisconnectReason::Timeout,
                    TextComponent::translate("disconnect.timeout", []),
                )
                .await;
                return;
            }
            self.wait_for_keep_alive.store(true, Ordering::Relaxed);
            self.last_keep_alive_time.store(now);
            let id = now.elapsed().as_millis() as i64;
            self.keep_alive_id.store(id, Ordering::Relaxed);
            self.client.enqueue_packet(&CKeepAlive::new(id)).await;
        }
    }

    pub fn has_client_loaded(&self) -> bool {
        self.client_loaded.load(Ordering::Relaxed)
            || self.client_loaded_timeout.load(Ordering::Relaxed) == 0
    }

    pub fn set_client_loaded(&self, loaded: bool) {
        if !loaded {
            self.client_loaded_timeout.store(60, Ordering::Relaxed);
        }
        self.client_loaded.store(loaded, Ordering::Relaxed);
    }

    pub const fn entity_id(&self) -> i32 {
        self.living_entity.entity.entity_id
    }

    pub fn position(&self) -> Vector3<f64> {
        self.living_entity.entity.pos.load()
    }

    pub fn eye_position(&self) -> Vector3<f64> {
        let eye_height = if self.living_entity.entity.pose.load() == EntityPose::Crouching {
            1.27
        } else {
            f64::from(self.living_entity.entity.standing_eye_height)
        };
        Vector3::new(
            self.living_entity.entity.pos.load().x,
            self.living_entity.entity.pos.load().y + eye_height,
            self.living_entity.entity.pos.load().z,
        )
    }

    pub fn rotation(&self) -> (f32, f32) {
        (
            self.living_entity.entity.yaw.load(),
            self.living_entity.entity.pitch.load(),
        )
    }

    /// Updates the client of the player's current permission level.
    pub async fn send_permission_lvl_update(&self) {
        let status = match self.permission_lvl.load() {
            PermissionLvl::Zero => EntityStatus::SetOpLevel0,
            PermissionLvl::One => EntityStatus::SetOpLevel1,
            PermissionLvl::Two => EntityStatus::SetOpLevel2,
            PermissionLvl::Three => EntityStatus::SetOpLevel3,
            PermissionLvl::Four => EntityStatus::SetOpLevel4,
        };
        self.world()
            .send_entity_status(&self.living_entity.entity, status)
            .await;
    }

    /// Sets the player's permission level and notifies the client.
    pub async fn set_permission_lvl(
        self: &Arc<Self>,
        lvl: PermissionLvl,
        command_dispatcher: &CommandDispatcher,
    ) {
        self.permission_lvl.store(lvl);
        self.send_permission_lvl_update().await;
        client_suggestions::send_c_commands_packet(self, command_dispatcher).await;
    }

    pub async fn kick(&self, reason: DisconnectReason, message: TextComponent) {
        self.client.kick(reason, message).await;
    }

    pub fn can_food_heal(&self) -> bool {
        let health = self.living_entity.health.load();
        let max_health = 20.0; // TODO
        health > 0.0 && health < max_health
    }

    pub async fn add_exhaustion(&self, exhaustion: f32) {
        if self.abilities.lock().await.invulnerable {
            return;
        }
        self.hunger_manager.add_exhaustion(exhaustion);
    }

    pub async fn send_health(&self) {
        self.client
            .enqueue_packet(&CSetHealth::new(
                self.living_entity.health.load(),
                self.hunger_manager.level.load().into(),
                self.hunger_manager.saturation.load(),
            ))
            .await;
    }

    pub async fn tick_health(&self) {
        let health = self.living_entity.health.load() as i32;
        let food = self.hunger_manager.level.load();
        let saturation = self.hunger_manager.saturation.load();

        let last_health = self.last_sent_health.load(Ordering::Relaxed);
        let last_food = self.last_sent_food.load(Ordering::Relaxed);
        let last_saturation = self.last_food_saturation.load(Ordering::Relaxed);

        if health != last_health || food != last_food || (saturation == 0.0) != last_saturation {
            self.last_sent_health.store(health, Ordering::Relaxed);
            self.last_sent_food.store(food, Ordering::Relaxed);
            self.last_food_saturation
                .store(saturation == 0.0, Ordering::Relaxed);
            self.send_health().await;
        }
    }

    pub async fn set_health(&self, health: f32) {
        self.living_entity.set_health(health).await;
        self.send_health().await;
    }

    pub fn tick_client_load_timeout(&self) {
        if !self.client_loaded.load(Ordering::Relaxed) {
            let timeout = self.client_loaded_timeout.load(Ordering::Relaxed);
            self.client_loaded_timeout
                .store(timeout.saturating_sub(1), Ordering::Relaxed);
        }
    }

    pub(super) async fn handle_killed(&self, death_msg: TextComponent) {
        self.set_client_loaded(false);
        self.client
            .send_packet_now(&CCombatDeath::new(self.entity_id().into(), &death_msg))
            .await;
    }

    pub async fn set_gamemode(self: &Arc<Self>, gamemode: GameMode) {
        // We could send the same gamemode without any problems. But why waste bandwidth?
        assert_ne!(
            self.gamemode.load(),
            gamemode,
            "Attempt to set the gamemode to the already current gamemode"
        );
        send_cancellable! {{
            PlayerGamemodeChangeEvent {
                player: self.clone(),
                new_gamemode: gamemode,
                previous_gamemode: self.gamemode.load(),
                cancelled: false,
            };

            'after: {
                let gamemode = event.new_gamemode;
                self.gamemode.store(gamemode);
                // TODO: Fix this when mojang fixes it
                // This is intentional to keep the pure vanilla mojang experience
                // self.previous_gamemode.store(self.previous_gamemode.load());
                {
                    // Use another scope so that we instantly unlock `abilities`.
                    let mut abilities = self.abilities.lock().await;
                    abilities.set_for_gamemode(gamemode);
                };
                self.send_abilities_update().await;

                self.living_entity.entity.invulnerable.store(
                    matches!(gamemode, GameMode::Creative | GameMode::Spectator),
                    Ordering::Relaxed,
                );
                self.living_entity
                    .entity
                    .world
                    .broadcast_packet_all(&CPlayerInfoUpdate::new(
                        PlayerInfoFlags::UPDATE_GAME_MODE.bits(),
                        &[pumpkin_protocol::java::client::play::Player {
                            uuid: self.gameprofile.id,
                            actions: &[PlayerAction::UpdateGameMode((gamemode as i32).into())],
                        }],
                    ))
                    .await;

                self.client
                    .enqueue_packet(&CGameEvent::new(
                        GameEvent::ChangeGameMode,
                        gamemode as i32 as f32,
                    )).await;
            }
        }}
    }

    /// Send the player's skin layers and used hand to all players.
    pub async fn send_client_information(&self) {
        let config = self.config.read().await;
        self.living_entity
            .entity
            .send_meta_data(&[
                Metadata::new(
                    DATA_PLAYER_MODE_CUSTOMISATION,
                    MetaDataType::Byte,
                    config.skin_parts,
                ),
                Metadata::new(
                    DATA_PLAYER_MAIN_HAND,
                    MetaDataType::Byte,
                    config.main_hand as u8,
                ),
            ])
            .await;
    }

    /// Check if the player has a specific permission
    pub async fn has_permission(&self, node: &str) -> bool {
        let perm_manager = PERMISSION_MANAGER.read().await;
        perm_manager
            .has_permission(&self.gameprofile.id, node, self.permission_lvl.load())
            .await
    }

    pub fn is_creative(&self) -> bool {
        self.gamemode.load() == GameMode::Creative
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.gameprofile.id == other.gameprofile.id
    }
}

#[async_trait]
impl NBTStorage for Player {
    async fn write_nbt(&self, nbt: &mut NbtCompound) {
        nbt.put_int("DataVersion", DATA_VERSION);
        self.living_entity.write_nbt(nbt).await;
        self.inventory.write_nbt(nbt).await;

        self.abilities.lock().await.write_nbt(nbt).await;

        // Store total XP instead of individual components
        let total_exp =
            math::experience::points_to_level(self.experience_level.load(Ordering::Relaxed))
                + self.experience_points.load(Ordering::Relaxed);
        nbt.put_int("XpTotal", total_exp);
        nbt.put_byte("playerGameType", self.gamemode.load() as i8);
        if let Some(previous_gamemode) = self.previous_gamemode.load() {
            nbt.put_byte("previousPlayerGameType", previous_gamemode as i8);
        }

        nbt.put_bool(
            "HasPlayedBefore",
            self.has_played_before.load(Ordering::Relaxed),
        );

        // Store food level, saturation, exhaustion, and tick timer
        self.hunger_manager.write_nbt(nbt).await;

        nbt.put_string(
            "Dimension",
            self.world().dimension_type.resource_location().to_string(),
        );
    }

    async fn read_nbt(&mut self, nbt: &mut NbtCompound) {
        self.living_entity.read_nbt(nbt).await;
        self.inventory.read_nbt_non_mut(nbt).await;
        self.abilities.lock().await.read_nbt(nbt).await;

        self.gamemode.store(
            GameMode::try_from(nbt.get_byte("playerGameType").unwrap_or(0))
                .unwrap_or(GameMode::Survival),
        );

        self.previous_gamemode.store(
            nbt.get_byte("previousPlayerGameType")
                .and_then(|byte| GameMode::try_from(byte).ok()),
        );

        self.has_played_before.store(
            nbt.get_bool("HasPlayedBefore").unwrap_or(false),
            Ordering::Relaxed,
        );

        // Load food level, saturation, exhaustion, and tick timer
        self.hunger_manager.read_nbt(nbt).await;

        // Load from total XP
        let total_exp = nbt.get_int("XpTotal").unwrap_or(0);
        let (level, points) = math::experience::total_to_level_and_points(total_exp);
        let progress = math::experience::progress_in_level(level, points);
        self.experience_level.store(level, Ordering::Relaxed);
        self.experience_progress.store(progress);
        self.experience_points.store(points, Ordering::Relaxed);
    }
}

impl NBTStorageInit for Player {}
