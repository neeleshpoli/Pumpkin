use std::sync::{Arc, atomic::Ordering};

use log::warn;
use pumpkin_config::{BASIC_CONFIG, advanced_config};
use pumpkin_data::{
    Block, BlockState,
    damage::DamageType,
    data_component_impl::{AttributeModifiersImpl, Operation},
    effect::StatusEffect,
    entity::EntityPose,
    sound::{Sound, SoundCategory},
};
use pumpkin_protocol::{
    codec::var_int::VarInt,
    java::{
        client::play::{Animation, CEntityAnimation, MetaDataType, Metadata},
        server::play::SClickSlot,
    },
};
use pumpkin_util::{
    GameMode, Hand,
    math::{position::BlockPos, vector3::Vector3},
};
use pumpkin_world::entity::entity_data_flags::SLEEPING_POS_ID;

use crate::{
    block::{self, blocks::bed::BedBlock},
    entity::{
        EntityBase,
        combat::{self, AttackType, player_attack_sound},
        player::Player,
    },
    world::World,
};

// Player actions
impl Player {
    pub async fn attack(&self, victim: Arc<dyn EntityBase>) {
        let world = self.world();
        let victim_entity = victim.get_entity();
        let attacker_entity = &self.living_entity.entity;
        let config = &advanced_config().pvp;

        let inventory = self.inventory();
        let item_stack = inventory.held_item();

        let base_damage = 1.0;
        let base_attack_speed = 4.0;

        let mut damage_multiplier = 1.0;
        let mut add_damage = 0.0;
        let mut add_speed = 0.0;

        // Get the attack damage
        // TODO: this should be cached in memory, we shouldn't just use default here either
        if let Some(modifiers) = item_stack
            .lock()
            .await
            .get_data_component::<AttributeModifiersImpl>()
        {
            for item_mod in modifiers.attribute_modifiers.iter() {
                if item_mod.operation == Operation::AddValue {
                    if item_mod.id == "minecraft:base_attack_damage" {
                        add_damage = item_mod.amount;
                    } else if item_mod.id == "minecraft:base_attack_speed" {
                        add_speed = item_mod.amount;
                    }
                }
            }
        }

        let attack_speed = base_attack_speed + add_speed;

        let attack_cooldown_progress = self.get_attack_cooldown_progress(0.5, attack_speed);
        self.last_attacked_ticks.store(0, Ordering::Relaxed);

        // Only reduce attack damage if in cooldown
        // TODO: Enchantments are reduced in the same way, just without the square.
        if attack_cooldown_progress < 1.0 {
            damage_multiplier = 0.2 + attack_cooldown_progress.powi(2) * 0.8;
        }
        // Modify the added damage based on the multiplier.
        let mut damage = base_damage + add_damage * damage_multiplier;

        let pos = victim_entity.pos.load();

        let attack_type = AttackType::new(self, attack_cooldown_progress as f32).await;

        if matches!(attack_type, AttackType::Critical) {
            damage *= 1.5;
        }

        if !victim
            .damage_with_context(
                victim.clone(),
                damage as f32,
                DamageType::PLAYER_ATTACK,
                None,
                Some(&self.living_entity.entity),
                Some(&self.living_entity.entity),
            )
            .await
        {
            world
                .play_sound(
                    Sound::EntityPlayerAttackNodamage,
                    SoundCategory::Players,
                    &self.living_entity.entity.pos.load(),
                )
                .await;
            return;
        }

        if victim.get_living_entity().is_some() {
            let mut knockback_strength = 1.0;
            player_attack_sound(&pos, world, attack_type).await;
            match attack_type {
                AttackType::Knockback => knockback_strength += 1.0,
                AttackType::Sweeping => {
                    combat::spawn_sweep_particle(attacker_entity, world, &pos).await;
                }
                _ => {}
            }
            if config.knockback {
                combat::handle_knockback(attacker_entity, world, victim_entity, knockback_strength)
                    .await;
            }
        }

        if config.swing {}
    }

    pub async fn sleep(&self, bed_head_pos: BlockPos) {
        // TODO: Stop riding

        self.get_entity().set_pose(EntityPose::Sleeping).await;
        self.living_entity
            .entity
            .set_pos(bed_head_pos.to_f64().add_raw(0.5, 0.6875, 0.5));
        self.get_entity()
            .send_meta_data(&[Metadata::new(
                SLEEPING_POS_ID,
                MetaDataType::OptionalBlockPos,
                Some(bed_head_pos),
            )])
            .await;
        self.get_entity().set_velocity(Vector3::default()).await;

        self.sleeping_since.store(Some(0));
    }

    pub async fn wake_up(&self) {
        let world = self.world();
        let respawn_point = self
            .respawn_point
            .load()
            .expect("Player waking up should have it's respawn point set on the bed.");

        let (bed, bed_state) = world.get_block_and_state_id(&respawn_point.position).await;
        BedBlock::set_occupied(false, world, bed, &respawn_point.position, bed_state).await;

        self.living_entity
            .entity
            .set_pose(EntityPose::Standing)
            .await;
        self.living_entity.entity.set_pos(self.position());
        self.living_entity
            .entity
            .send_meta_data(&[Metadata::new(
                SLEEPING_POS_ID,
                MetaDataType::OptionalBlockPos,
                None::<BlockPos>,
            )])
            .await;

        world
            .broadcast_packet_all(&CEntityAnimation::new(
                self.entity_id().into(),
                Animation::LeaveBed,
            ))
            .await;

        self.sleeping_since.store(None);
    }

    pub(super) async fn continue_mining(
        &self,
        location: BlockPos,
        world: &World,
        state: &BlockState,
        starting_time: i32,
    ) {
        let time = self.tick_counter.load(Ordering::Relaxed) - starting_time;
        let speed = block::calc_block_breaking(self, state, Block::from_state_id(state.id)).await
            * (time + 1) as f32;
        let progress = (speed * 10.0) as i32;
        if progress != self.current_block_destroy_stage.load(Ordering::Relaxed) {
            world
                .set_block_breaking(&self.living_entity.entity, location, progress)
                .await;
            self.current_block_destroy_stage
                .store(progress, Ordering::Relaxed);
        }
    }

    pub async fn jump(&self) {
        if self.living_entity.entity.sprinting.load(Ordering::Relaxed) {
            self.add_exhaustion(0.2).await;
        } else {
            self.add_exhaustion(0.05).await;
        }
    }

    /// Swing the hand of the player
    pub async fn swing_hand(&self, hand: Hand, all: bool) {
        let world = self.world();
        let entity_id = VarInt(self.entity_id());

        let animation = match hand {
            Hand::Left => Animation::SwingMainArm,
            Hand::Right => Animation::SwingOffhand,
        };

        let packet = CEntityAnimation::new(entity_id, animation);
        if all {
            world.broadcast_packet_all(&packet).await;
        } else {
            world
                .broadcast_packet_except(&[self.gameprofile.id], &packet)
                .await;
        }
    }

    pub async fn on_slot_click(&self, packet: SClickSlot) {
        let screen_handler = self.current_screen_handler.lock().await;
        let mut screen_handler = screen_handler.lock().await;
        let behaviour = screen_handler.get_behaviour();

        // behaviour is dropped here
        if i32::from(behaviour.sync_id) != packet.sync_id.0 {
            return;
        }

        if self.gamemode.load() == GameMode::Spectator {
            screen_handler.sync_state().await;
            return;
        }

        if !screen_handler.can_use(self) {
            warn!(
                "Player {} interacted with invalid menu {:?}",
                self.gameprofile.name,
                screen_handler.window_type()
            );
            return;
        }

        let slot = packet.slot;

        if !screen_handler.is_slot_valid(i32::from(slot)).await {
            warn!(
                "Player {} clicked invalid slot index: {}, available slots: {}",
                self.gameprofile.name,
                slot,
                screen_handler.get_behaviour().slots.len()
            );
            return;
        }

        let not_in_sync = packet.revision.0 != (behaviour.revision.load(Ordering::Relaxed) as i32);

        screen_handler.disable_sync().await;
        screen_handler
            .on_slot_click(
                i32::from(slot),
                i32::from(packet.button),
                packet.mode.clone(),
                self,
            )
            .await;

        for (key, value) in packet.array_of_changed_slots {
            screen_handler.set_received_hash(key as usize, value);
        }

        screen_handler.set_received_cursor_hash(packet.carried_item);
        screen_handler.enable_sync().await;

        if not_in_sync {
            screen_handler.update_to_client().await;
        } else {
            screen_handler.send_content_updates().await;
            drop(screen_handler);
        }
    }

    pub fn get_attack_cooldown_progress(&self, base_time: f64, attack_speed: f64) -> f64 {
        let x = f64::from(self.last_attacked_ticks.load(Ordering::Acquire)) + base_time;

        let progress_per_tick = f64::from(BASIC_CONFIG.tps) / attack_speed;
        let progress = x / progress_per_tick;
        progress.clamp(0.0, 1.0)
    }

    pub async fn can_harvest(&self, state: &BlockState, block: &'static Block) -> bool {
        !state.tool_required()
            || self
                .inventory
                .held_item()
                .lock()
                .await
                .is_correct_for_drops(block)
    }

    pub async fn heal(&self, additional_health: f32) {
        self.living_entity.heal(additional_health).await;
        self.send_health().await;
    }

    pub async fn get_mining_speed(&self, block: &'static Block) -> f32 {
        let mut speed = self.inventory.held_item().lock().await.get_speed(block);
        // Haste
        if self.living_entity.has_effect(&StatusEffect::HASTE).await
            || self
                .living_entity
                .has_effect(&StatusEffect::CONDUIT_POWER)
                .await
        {
            speed *= 1.0 + (self.get_haste_amplifier().await + 1) as f32 * 0.2;
        }
        // Fatigue
        if let Some(fatigue) = self
            .living_entity
            .get_effect(&StatusEffect::MINING_FATIGUE)
            .await
        {
            let fatigue_speed = match fatigue.amplifier {
                0 => 0.3,
                1 => 0.09,
                2 => 0.0027,
                _ => 8.1E-4,
            };
            speed *= fatigue_speed;
        }
        // TODO: Handle when in water
        if !self.living_entity.entity.on_ground.load(Ordering::Relaxed) {
            speed /= 5.0;
        }
        speed
    }

    #[expect(clippy::cast_precision_loss)]
    pub async fn progress_motion(&self, delta_pos: Vector3<f64>) {
        // TODO: Swimming, gliding...
        if self.living_entity.entity.on_ground.load(Ordering::Relaxed) {
            let delta = (delta_pos.horizontal_length() * 100.0).round() as i32;
            if delta > 0 {
                if self.living_entity.entity.sprinting.load(Ordering::Relaxed) {
                    self.add_exhaustion(0.1 * delta as f32 * 0.01).await;
                } else {
                    self.add_exhaustion(0.0 * delta as f32 * 0.01).await;
                }
            }
        }
    }
}
