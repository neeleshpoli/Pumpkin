use std::sync::{Arc, atomic::Ordering};

use async_trait::async_trait;
use pumpkin_data::damage::DamageType;
use pumpkin_macros::send_cancellable;
use pumpkin_protocol::java::client::play::CEntityPositionSync;
use pumpkin_util::{
    GameMode,
    math::vector3::Vector3,
    text::{TextComponent, click::ClickEvent, hover::HoverEvent},
};

use crate::{
    entity::{Entity, EntityBase, NBTStorage, living::LivingEntity, player::Player},
    plugin::player::player_teleport::PlayerTeleportEvent,
    world::World,
};

#[async_trait]
impl EntityBase for Player {
    async fn damage_with_context(
        &self,
        caller: Arc<dyn EntityBase>,
        amount: f32,
        damage_type: DamageType,
        position: Option<Vector3<f64>>,
        source: Option<&dyn EntityBase>,
        cause: Option<&dyn EntityBase>,
    ) -> bool {
        if self.abilities.lock().await.invulnerable && damage_type != DamageType::GENERIC_KILL {
            return false;
        }
        let dyn_self = self
            .living_entity
            .entity
            .world
            .get_entity_by_id(self.living_entity.entity.entity_id)
            .await
            .expect("Entity not found in world");
        let result = self
            .living_entity
            .damage_with_context(caller, amount, damage_type, position, source, cause)
            .await;
        if result {
            let health = self.living_entity.health.load();
            if health <= 0.0 {
                let death_message =
                    LivingEntity::get_death_message(&*dyn_self, damage_type, source, cause).await;
                self.handle_killed(death_message).await;
            }
        }
        result
    }

    async fn teleport(
        self: Arc<Self>,
        position: Vector3<f64>,
        yaw: Option<f32>,
        pitch: Option<f32>,
        world: Arc<World>,
    ) {
        if Arc::ptr_eq(&world, self.world()) {
            // Same world
            let yaw = yaw.unwrap_or(self.living_entity.entity.yaw.load());
            let pitch = pitch.unwrap_or(self.living_entity.entity.pitch.load());
            send_cancellable! {{
                PlayerTeleportEvent {
                    player: self.clone(),
                    from: self.living_entity.entity.pos.load(),
                    to: position,
                    cancelled: false,
                };
                'after: {
                    let position = event.to;
                    let entity = self.get_entity();
                    self.request_teleport(position, yaw, pitch).await;
                    entity
                        .world
                        .broadcast_packet_except(&[self.gameprofile.id], &CEntityPositionSync::new(
                            self.living_entity.entity.entity_id.into(),
                            position,
                            Vector3::new(0.0, 0.0, 0.0),
                            yaw,
                            pitch,
                            entity.on_ground.load(Ordering::SeqCst),
                        ))
                        .await;
                }
            }}
        } else {
            self.teleport_world(world, position, yaw, pitch).await;
        }
    }

    fn get_entity(&self) -> &Entity {
        &self.living_entity.entity
    }

    fn get_living_entity(&self) -> Option<&LivingEntity> {
        Some(&self.living_entity)
    }

    fn get_player(&self) -> Option<&Player> {
        Some(self)
    }

    fn is_spectator(&self) -> bool {
        self.gamemode.load() == GameMode::Spectator
    }

    fn get_name(&self) -> TextComponent {
        //TODO: team color
        TextComponent::text(self.gameprofile.name.clone())
    }

    async fn get_display_name(&self) -> TextComponent {
        let name = self.get_name();
        let name_clone = name.clone();
        let mut name = name.click_event(ClickEvent::SuggestCommand {
            command: format!("/tell {} ", self.gameprofile.name.clone()).into(),
        });
        name = name.hover_event(HoverEvent::show_entity(
            self.living_entity.entity.entity_uuid.to_string(),
            self.living_entity.entity.entity_type.resource_name.into(),
            Some(name_clone),
        ));
        name.insertion(self.gameprofile.name.clone())
    }

    fn as_nbt_storage(&self) -> &dyn NBTStorage {
        self
    }
}
