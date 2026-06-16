/* This file is generated. Do not edit manually. */
use crate::plugin::loader::wasm::wasm_host::state::*;
use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::common::Position;
use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::particles::*;
use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::world::World;
use wasmtime::component::Resource;
impl Host for PluginHostState {
    async fn spawn_angry_villager_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::AngryVillager,
        );
        Ok(())
    }
    async fn spawn_block_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Block>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<BlockParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_block_marker_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<BlockMarker>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<BlockMarkerParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_bubble_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Bubble,
        );
        Ok(())
    }
    async fn spawn_cloud_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Cloud,
        );
        Ok(())
    }
    async fn spawn_copper_fire_flame_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CopperFireFlame,
        );
        Ok(())
    }
    async fn spawn_crit_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Crit,
        );
        Ok(())
    }
    async fn spawn_damage_indicator_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DamageIndicator,
        );
        Ok(())
    }
    async fn spawn_dragon_breath_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<DragonBreath>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<DragonBreathParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_dripping_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingLava,
        );
        Ok(())
    }
    async fn spawn_falling_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingLava,
        );
        Ok(())
    }
    async fn spawn_landing_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::LandingLava,
        );
        Ok(())
    }
    async fn spawn_dripping_water_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingWater,
        );
        Ok(())
    }
    async fn spawn_falling_water_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingWater,
        );
        Ok(())
    }
    async fn spawn_dust_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Dust>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<DustParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_dust_color_transition_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<DustColorTransition>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<DustColorTransitionParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_effect_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Effect>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<EffectParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_elder_guardian_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ElderGuardian,
        );
        Ok(())
    }
    async fn spawn_enchanted_hit_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::EnchantedHit,
        );
        Ok(())
    }
    async fn spawn_enchant_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Enchant,
        );
        Ok(())
    }
    async fn spawn_end_rod_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::EndRod,
        );
        Ok(())
    }
    async fn spawn_entity_effect_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<EntityEffect>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<EntityEffectParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_explosion_emitter_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ExplosionEmitter,
        );
        Ok(())
    }
    async fn spawn_explosion_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Explosion,
        );
        Ok(())
    }
    async fn spawn_gust_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Gust,
        );
        Ok(())
    }
    async fn spawn_small_gust_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SmallGust,
        );
        Ok(())
    }
    async fn spawn_gust_emitter_large_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::GustEmitterLarge,
        );
        Ok(())
    }
    async fn spawn_gust_emitter_small_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::GustEmitterSmall,
        );
        Ok(())
    }
    async fn spawn_sonic_boom_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SonicBoom,
        );
        Ok(())
    }
    async fn spawn_falling_dust_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<FallingDust>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<FallingDustParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_firework_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Firework,
        );
        Ok(())
    }
    async fn spawn_fishing_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Fishing,
        );
        Ok(())
    }
    async fn spawn_flame_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Flame,
        );
        Ok(())
    }
    async fn spawn_infested_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Infested,
        );
        Ok(())
    }
    async fn spawn_cherry_leaves_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CherryLeaves,
        );
        Ok(())
    }
    async fn spawn_pale_oak_leaves_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::PaleOakLeaves,
        );
        Ok(())
    }
    async fn spawn_tinted_leaves_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<TintedLeaves>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<TintedLeavesParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_sculk_soul_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SculkSoul,
        );
        Ok(())
    }
    async fn spawn_sculk_charge_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<SculkCharge>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<SculkChargeParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_sculk_charge_pop_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SculkChargePop,
        );
        Ok(())
    }
    async fn spawn_soul_fire_flame_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SoulFireFlame,
        );
        Ok(())
    }
    async fn spawn_soul_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Soul,
        );
        Ok(())
    }
    async fn spawn_flash_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Flash>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<FlashParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_happy_villager_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::HappyVillager,
        );
        Ok(())
    }
    async fn spawn_composter_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Composter,
        );
        Ok(())
    }
    async fn spawn_heart_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Heart,
        );
        Ok(())
    }
    async fn spawn_instant_effect_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<InstantEffect>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<InstantEffectParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_item_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Item,
        );
        Ok(())
    }
    async fn spawn_vibration_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Vibration,
        );
        Ok(())
    }
    async fn spawn_trail_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Trail>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<TrailParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_pause_mob_growth_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::PauseMobGrowth,
        );
        Ok(())
    }
    async fn spawn_reset_mob_growth_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ResetMobGrowth,
        );
        Ok(())
    }
    async fn spawn_item_slime_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ItemSlime,
        );
        Ok(())
    }
    async fn spawn_item_cobweb_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ItemCobweb,
        );
        Ok(())
    }
    async fn spawn_item_snowball_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ItemSnowball,
        );
        Ok(())
    }
    async fn spawn_large_smoke_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::LargeSmoke,
        );
        Ok(())
    }
    async fn spawn_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Lava,
        );
        Ok(())
    }
    async fn spawn_mycelium_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Mycelium,
        );
        Ok(())
    }
    async fn spawn_note_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Note,
        );
        Ok(())
    }
    async fn spawn_poof_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Poof,
        );
        Ok(())
    }
    async fn spawn_portal_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Portal,
        );
        Ok(())
    }
    async fn spawn_rain_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Rain,
        );
        Ok(())
    }
    async fn spawn_smoke_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Smoke,
        );
        Ok(())
    }
    async fn spawn_white_smoke_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::WhiteSmoke,
        );
        Ok(())
    }
    async fn spawn_sneeze_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Sneeze,
        );
        Ok(())
    }
    async fn spawn_spit_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Spit,
        );
        Ok(())
    }
    async fn spawn_squid_ink_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SquidInk,
        );
        Ok(())
    }
    async fn spawn_sweep_attack_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SweepAttack,
        );
        Ok(())
    }
    async fn spawn_totem_of_undying_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::TotemOfUndying,
        );
        Ok(())
    }
    async fn spawn_underwater_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Underwater,
        );
        Ok(())
    }
    async fn spawn_splash_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Splash,
        );
        Ok(())
    }
    async fn spawn_witch_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Witch,
        );
        Ok(())
    }
    async fn spawn_bubble_pop_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::BubblePop,
        );
        Ok(())
    }
    async fn spawn_current_down_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CurrentDown,
        );
        Ok(())
    }
    async fn spawn_bubble_column_up_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::BubbleColumnUp,
        );
        Ok(())
    }
    async fn spawn_nautilus_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Nautilus,
        );
        Ok(())
    }
    async fn spawn_dolphin_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Dolphin,
        );
        Ok(())
    }
    async fn spawn_campfire_cosy_smoke_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CampfireCosySmoke,
        );
        Ok(())
    }
    async fn spawn_campfire_signal_smoke_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CampfireSignalSmoke,
        );
        Ok(())
    }
    async fn spawn_dripping_honey_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingHoney,
        );
        Ok(())
    }
    async fn spawn_falling_honey_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingHoney,
        );
        Ok(())
    }
    async fn spawn_landing_honey_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::LandingHoney,
        );
        Ok(())
    }
    async fn spawn_falling_nectar_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingNectar,
        );
        Ok(())
    }
    async fn spawn_falling_spore_blossom_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingSporeBlossom,
        );
        Ok(())
    }
    async fn spawn_ash_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Ash,
        );
        Ok(())
    }
    async fn spawn_crimson_spore_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::CrimsonSpore,
        );
        Ok(())
    }
    async fn spawn_warped_spore_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::WarpedSpore,
        );
        Ok(())
    }
    async fn spawn_spore_blossom_air_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SporeBlossomAir,
        );
        Ok(())
    }
    async fn spawn_dripping_obsidian_tear_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingObsidianTear,
        );
        Ok(())
    }
    async fn spawn_falling_obsidian_tear_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingObsidianTear,
        );
        Ok(())
    }
    async fn spawn_landing_obsidian_tear_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::LandingObsidianTear,
        );
        Ok(())
    }
    async fn spawn_reverse_portal_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ReversePortal,
        );
        Ok(())
    }
    async fn spawn_white_ash_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::WhiteAsh,
        );
        Ok(())
    }
    async fn spawn_small_flame_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::SmallFlame,
        );
        Ok(())
    }
    async fn spawn_snowflake_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Snowflake,
        );
        Ok(())
    }
    async fn spawn_dripping_dripstone_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingDripstoneLava,
        );
        Ok(())
    }
    async fn spawn_falling_dripstone_lava_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingDripstoneLava,
        );
        Ok(())
    }
    async fn spawn_dripping_dripstone_water_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DrippingDripstoneWater,
        );
        Ok(())
    }
    async fn spawn_falling_dripstone_water_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::FallingDripstoneWater,
        );
        Ok(())
    }
    async fn spawn_glow_squid_ink_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::GlowSquidInk,
        );
        Ok(())
    }
    async fn spawn_glow_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Glow,
        );
        Ok(())
    }
    async fn spawn_wax_on_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::WaxOn,
        );
        Ok(())
    }
    async fn spawn_wax_off_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::WaxOff,
        );
        Ok(())
    }
    async fn spawn_electric_spark_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::ElectricSpark,
        );
        Ok(())
    }
    async fn spawn_scrape_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Scrape,
        );
        Ok(())
    }
    async fn spawn_shriek_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<Shriek>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<ShriekParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_egg_crack_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::EggCrack,
        );
        Ok(())
    }
    async fn spawn_dust_plume_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::DustPlume,
        );
        Ok(())
    }
    async fn spawn_trial_spawner_detection_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::TrialSpawnerDetection,
        );
        Ok(())
    }
    async fn spawn_trial_spawner_detection_ominous_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::TrialSpawnerDetectionOminous,
        );
        Ok(())
    }
    async fn spawn_vault_connection_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::VaultConnection,
        );
        Ok(())
    }
    async fn spawn_dust_pillar_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<DustPillar>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<DustPillarParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_ominous_spawning_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::OminousSpawning,
        );
        Ok(())
    }
    async fn spawn_raid_omen_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::RaidOmen,
        );
        Ok(())
    }
    async fn spawn_trial_omen_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::TrialOmen,
        );
        Ok(())
    }
    async fn spawn_block_crumble_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
        particle: Resource<BlockCrumble>,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            self.resource_table
                .get::<BlockCrumbleParticleResource>(&Resource::new_own(particle.rep()))?
                .provider,
        );
        Ok(())
    }
    async fn spawn_firefly_particle(
        &mut self,
        world: Resource<World>,
        position: Position,
        offset: Offset,
        max_speed: f32,
        particle_count: i32,
    ) -> wasmtime::Result<()> {
        let world = self.get_world_res(&world)?;
        world.provider.spawn_particle(
            position.into(),
            offset.into(),
            max_speed,
            particle_count,
            pumpkin_data::particle::Firefly,
        );
        Ok(())
    }
}
pub type BlockParticleResource = WasmResource<pumpkin_data::particle::Block>;
impl HostBlock for PluginHostState {
    async fn new(&mut self, state: i32) -> wasmtime::Result<Resource<Block>> {
        let provider = pumpkin_data::particle::Block::new(state);
        let resource = self
            .resource_table
            .push(BlockParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Block>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<BlockParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type BlockMarkerParticleResource = WasmResource<pumpkin_data::particle::BlockMarker>;
impl HostBlockMarker for PluginHostState {
    async fn new(&mut self, state: i32) -> wasmtime::Result<Resource<BlockMarker>> {
        let provider = pumpkin_data::particle::BlockMarker::new(state);
        let resource = self
            .resource_table
            .push(BlockMarkerParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<BlockMarker>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<BlockMarkerParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type DragonBreathParticleResource = WasmResource<pumpkin_data::particle::DragonBreath>;
impl HostDragonBreath for PluginHostState {
    async fn new(&mut self, power: f32) -> wasmtime::Result<Resource<DragonBreath>> {
        let provider = pumpkin_data::particle::DragonBreath::new(power);
        let resource = self
            .resource_table
            .push(DragonBreathParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<DragonBreath>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<DragonBreathParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type DustParticleResource = WasmResource<pumpkin_data::particle::Dust>;
impl HostDust for PluginHostState {
    async fn new(&mut self, color: i32, scale: f32) -> wasmtime::Result<Resource<Dust>> {
        let provider = pumpkin_data::particle::Dust::new(color, scale);
        let resource = self
            .resource_table
            .push(DustParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Dust>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<DustParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type DustColorTransitionParticleResource =
    WasmResource<pumpkin_data::particle::DustColorTransition>;
impl HostDustColorTransition for PluginHostState {
    async fn new(
        &mut self,
        from_color: i32,
        to_color: i32,
        scale: f32,
    ) -> wasmtime::Result<Resource<DustColorTransition>> {
        let provider =
            pumpkin_data::particle::DustColorTransition::new(from_color, to_color, scale);
        let resource = self
            .resource_table
            .push(DustColorTransitionParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<DustColorTransition>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<DustColorTransitionParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type EffectParticleResource = WasmResource<pumpkin_data::particle::Effect>;
impl HostEffect for PluginHostState {
    async fn new(&mut self, color: i32, power: f32) -> wasmtime::Result<Resource<Effect>> {
        let provider = pumpkin_data::particle::Effect::new(color, power);
        let resource = self
            .resource_table
            .push(EffectParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Effect>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<EffectParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type EntityEffectParticleResource = WasmResource<pumpkin_data::particle::EntityEffect>;
impl HostEntityEffect for PluginHostState {
    async fn new(&mut self, color: i32) -> wasmtime::Result<Resource<EntityEffect>> {
        let provider = pumpkin_data::particle::EntityEffect::new(color);
        let resource = self
            .resource_table
            .push(EntityEffectParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<EntityEffect>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<EntityEffectParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type FallingDustParticleResource = WasmResource<pumpkin_data::particle::FallingDust>;
impl HostFallingDust for PluginHostState {
    async fn new(&mut self, state: i32) -> wasmtime::Result<Resource<FallingDust>> {
        let provider = pumpkin_data::particle::FallingDust::new(state);
        let resource = self
            .resource_table
            .push(FallingDustParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<FallingDust>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<FallingDustParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type TintedLeavesParticleResource = WasmResource<pumpkin_data::particle::TintedLeaves>;
impl HostTintedLeaves for PluginHostState {
    async fn new(&mut self, color: i32) -> wasmtime::Result<Resource<TintedLeaves>> {
        let provider = pumpkin_data::particle::TintedLeaves::new(color);
        let resource = self
            .resource_table
            .push(TintedLeavesParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<TintedLeaves>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<TintedLeavesParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type SculkChargeParticleResource = WasmResource<pumpkin_data::particle::SculkCharge>;
impl HostSculkCharge for PluginHostState {
    async fn new(&mut self, roll: f32) -> wasmtime::Result<Resource<SculkCharge>> {
        let provider = pumpkin_data::particle::SculkCharge::new(roll);
        let resource = self
            .resource_table
            .push(SculkChargeParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<SculkCharge>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<SculkChargeParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type FlashParticleResource = WasmResource<pumpkin_data::particle::Flash>;
impl HostFlash for PluginHostState {
    async fn new(&mut self, color: i32) -> wasmtime::Result<Resource<Flash>> {
        let provider = pumpkin_data::particle::Flash::new(color);
        let resource = self
            .resource_table
            .push(FlashParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Flash>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<FlashParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type InstantEffectParticleResource = WasmResource<pumpkin_data::particle::InstantEffect>;
impl HostInstantEffect for PluginHostState {
    async fn new(&mut self, color: i32, power: f32) -> wasmtime::Result<Resource<InstantEffect>> {
        let provider = pumpkin_data::particle::InstantEffect::new(color, power);
        let resource = self
            .resource_table
            .push(InstantEffectParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<InstantEffect>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<InstantEffectParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type TrailParticleResource = WasmResource<pumpkin_data::particle::Trail>;
impl HostTrail for PluginHostState {
    async fn new(
        &mut self,
        target: Position,
        color: i32,
        duration: i32,
    ) -> wasmtime::Result<Resource<Trail>> {
        let provider = pumpkin_data::particle::Trail::new(target.into(), color, duration);
        let resource = self
            .resource_table
            .push(TrailParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Trail>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<TrailParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type ShriekParticleResource = WasmResource<pumpkin_data::particle::Shriek>;
impl HostShriek for PluginHostState {
    async fn new(&mut self, delay: i32) -> wasmtime::Result<Resource<Shriek>> {
        let provider = pumpkin_data::particle::Shriek::new(delay);
        let resource = self
            .resource_table
            .push(ShriekParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<Shriek>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<ShriekParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type DustPillarParticleResource = WasmResource<pumpkin_data::particle::DustPillar>;
impl HostDustPillar for PluginHostState {
    async fn new(&mut self, state: i32) -> wasmtime::Result<Resource<DustPillar>> {
        let provider = pumpkin_data::particle::DustPillar::new(state);
        let resource = self
            .resource_table
            .push(DustPillarParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<DustPillar>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<DustPillarParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
pub type BlockCrumbleParticleResource = WasmResource<pumpkin_data::particle::BlockCrumble>;
impl HostBlockCrumble for PluginHostState {
    async fn new(&mut self, state: i32) -> wasmtime::Result<Resource<BlockCrumble>> {
        let provider = pumpkin_data::particle::BlockCrumble::new(state);
        let resource = self
            .resource_table
            .push(BlockCrumbleParticleResource { provider })?;
        Ok(Resource::new_own(resource.rep()))
    }
    async fn drop(&mut self, rep: Resource<BlockCrumble>) -> wasmtime::Result<()> {
        self.resource_table
            .delete::<BlockCrumbleParticleResource>(Resource::new_own(rep.rep()))
            .map_err(wasmtime::Error::from)?;
        Ok(())
    }
}
