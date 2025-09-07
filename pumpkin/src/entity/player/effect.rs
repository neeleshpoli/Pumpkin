use pumpkin_data::{effect::StatusEffect, potion::Effect};
use pumpkin_protocol::{codec::var_int::VarInt, java::client::play::CUpdateMobEffect};

use crate::entity::player::Player;

impl Player {
    pub async fn add_effect(&self, effect: Effect) {
        self.send_effect(effect.clone()).await;
        self.living_entity.add_effect(effect).await;
    }

    pub async fn send_active_effects(&self) {
        let effects = self.living_entity.active_effects.lock().await;
        for effect in effects.values() {
            self.send_effect(effect.clone()).await;
        }
    }

    pub async fn send_effect(&self, effect: Effect) {
        let mut flag: i8 = 0;

        if effect.ambient {
            flag |= 1;
        }
        if effect.show_particles {
            flag |= 2;
        }
        if effect.show_icon {
            flag |= 4;
        }
        if effect.blend {
            flag |= 8;
        }

        let effect_id = VarInt(i32::from(effect.effect_type.id));
        self.client
            .enqueue_packet(&CUpdateMobEffect::new(
                self.entity_id().into(),
                effect_id,
                effect.amplifier.into(),
                effect.duration.into(),
                flag,
            ))
            .await;
    }

    pub async fn remove_effect(&self, effect_type: &'static StatusEffect) {
        let effect_id = VarInt(i32::from(effect_type.id));
        self.client
            .enqueue_packet(
                &pumpkin_protocol::java::client::play::CRemoveMobEffect::new(
                    self.entity_id().into(),
                    effect_id,
                ),
            )
            .await;
        self.living_entity.remove_effect(effect_type).await;

        // TODO broadcast metadata
    }

    pub async fn remove_all_effect(&self) -> u8 {
        let mut count = 0;
        let mut effect_list = vec![];
        for effect in self.living_entity.active_effects.lock().await.keys() {
            effect_list.push(*effect);
            let effect_id = VarInt(i32::from(effect.id));
            self.client
                .enqueue_packet(
                    &pumpkin_protocol::java::client::play::CRemoveMobEffect::new(
                        self.entity_id().into(),
                        effect_id,
                    ),
                )
                .await;
            count += 1;
        }
        //Need to remove effect after because the player effect are lock in the for before
        for effect in effect_list {
            self.living_entity.remove_effect(effect).await;
        }

        count
    }

    pub(super) async fn get_haste_amplifier(&self) -> u32 {
        let mut i = 0;
        let mut j = 0;
        if let Some(effect) = self.living_entity.get_effect(&StatusEffect::HASTE).await {
            i = effect.amplifier;
        }
        if let Some(effect) = self
            .living_entity
            .get_effect(&StatusEffect::CONDUIT_POWER)
            .await
        {
            j = effect.amplifier;
        }
        u32::from(i.max(j))
    }
}
