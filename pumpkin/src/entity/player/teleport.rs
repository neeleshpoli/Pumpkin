use std::sync::{Arc, atomic::Ordering};

use pumpkin_inventory::screen_handler::InventoryPlayer;
use pumpkin_macros::send_cancellable;
use pumpkin_protocol::{
    codec::var_int::VarInt,
    java::client::play::{CPlayerPosition, CRespawn, CSetSelectedSlot},
};
use pumpkin_util::math::{position::BlockPos, vector3::Vector3};
use pumpkin_world::biome;

use crate::{
    entity::{EntityBase, player::Player},
    plugin::player::{
        player_change_world::PlayerChangeWorldEvent, player_teleport::PlayerTeleportEvent,
    },
    world::World,
};

impl Player {
    /// Teleports the player to a different world or dimension with an optional position, yaw, and pitch.
    pub async fn teleport_world(
        self: &Arc<Self>,
        new_world: Arc<World>,
        position: Vector3<f64>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    ) {
        let current_world = self.living_entity.entity.world.clone();
        let yaw = yaw.unwrap_or(new_world.level_info.read().await.spawn_angle);
        let pitch = pitch.unwrap_or(10.0);

        send_cancellable! {{
            PlayerChangeWorldEvent {
                player: self.clone(),
                previous_world: current_world.clone(),
                new_world: new_world.clone(),
                position,
                yaw,
                pitch,
                cancelled: false,
            };

            'after: {
                // TODO: this is duplicate code from world
                let position = event.position;
                let yaw = event.yaw;
                let pitch = event.pitch;
                let new_world = event.new_world;

                self.set_client_loaded(false);
                let uuid = self.gameprofile.id;
                current_world.remove_player(self, false).await;
                new_world.players.write().await.insert(uuid, Arc::new(Self::new(
                            self.client.clone(),
                            self.gameprofile.clone(),
                            self.config.read().await.clone(),
                            new_world.clone(),
                            self.gamemode.load(),
                        )
                        .await));
                self.unload_watched_chunks(&current_world).await;

                let last_pos = self.living_entity.entity.last_pos.load();
                let death_dimension = self.world().dimension_type.resource_location();
                let death_location = BlockPos(Vector3::new(
                    last_pos.x.round() as i32,
                    last_pos.y.round() as i32,
                    last_pos.z.round() as i32,
                ));
                self.client
                    .send_packet_now(&CRespawn::new(
                        (new_world.dimension_type as u8).into(),
                        new_world.dimension_type.resource_location(),
                        biome::hash_seed(new_world.level.seed.0), // seed
                        self.gamemode.load() as u8,
                        self.gamemode.load() as i8,
                        false,
                        false,
                        Some((death_dimension, death_location)),
                        VarInt(self.get_entity().portal_cooldown.load(Ordering::Relaxed) as i32),
                        new_world.sea_level.into(),
                        1,
                    )).await
                    ;
                self.send_permission_lvl_update().await;
                self.clone().request_teleport(position, yaw, pitch).await;
                self.living_entity.entity.last_pos.store(position);
                self.send_abilities_update().await;
                self.enqueue_set_held_item_packet(&CSetSelectedSlot::new(
                    self.get_inventory().get_selected_slot() as i8,
                )).await;
                self.on_screen_handler_opened(self.player_screen_handler.clone()).await;
                self.send_health().await;

                new_world.send_world_info(self, position, yaw, pitch).await;
            }
        }}
    }

    /// `yaw` and `pitch` are in degrees.
    /// Rarly used, for example when waking up the player from a bed or their first time spawn. Otherwise, the `teleport` method should be used.
    /// The player should respond with the `SConfirmTeleport` packet.
    pub async fn request_teleport(self: &Arc<Self>, position: Vector3<f64>, yaw: f32, pitch: f32) {
        // This is the ultra special magic code used to create the teleport id
        // This returns the old value
        // This operation wraps around on overflow.

        send_cancellable! {{
            PlayerTeleportEvent {
                player: self.clone(),
                from: self.living_entity.entity.pos.load(),
                to: position,
                cancelled: false,
            };

            'after: {
                let position = event.to;
                let i = self
                    .teleport_id_count
                    .fetch_add(1, Ordering::Relaxed);
                let teleport_id = i + 1;
                self.living_entity.entity.set_pos(position);
                let entity = &self.living_entity.entity;
                entity.set_rotation(yaw, pitch);
                *self.awaiting_teleport.lock().await = Some((teleport_id.into(), position));
                self.client
                    .send_packet_now(&CPlayerPosition::new(
                        teleport_id.into(),
                        position,
                        Vector3::new(0.0, 0.0, 0.0),
                        yaw,
                        pitch,
                        // TODO
                        Vec::new(),
                    )).await;
            }
        }}
    }
}
