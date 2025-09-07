use async_trait::async_trait;
use pumpkin_nbt::compound::NbtCompound;
use pumpkin_protocol::{
    bedrock::client::update_abilities::{Ability, AbilityLayer, CUpdateAbilities},
    java::client::play::CPlayerAbilities,
};
use pumpkin_util::GameMode;

use crate::{
    entity::{NBTStorage, NBTStorageInit, player::Player},
    net::ClientPlatform,
};

/// Represents a player's abilities and special powers.
///
/// This struct contains information about the player's current abilities, such as flight, invulnerability, and creative mode.
pub struct Abilities {
    /// Indicates whether the player is invulnerable to damage.
    pub invulnerable: bool,
    /// Indicates whether the player is currently flying.
    pub flying: bool,
    /// Indicates whether the player is allowed to fly (if enabled).
    pub allow_flying: bool,
    /// Indicates whether the player is in creative mode.
    pub creative: bool,
    /// Indicates whether the player is allowed to modify the world.
    pub allow_modify_world: bool,
    /// The player's flying speed.
    pub fly_speed: f32,
    /// The field of view adjustment when the player is walking or sprinting.
    pub walk_speed: f32,
}

#[async_trait]
impl NBTStorage for Abilities {
    async fn write_nbt(&self, nbt: &mut NbtCompound) {
        let mut component = NbtCompound::new();
        component.put_bool("invulnerable", self.invulnerable);
        component.put_bool("flying", self.flying);
        component.put_bool("mayfly", self.allow_flying);
        component.put_bool("instabuild", self.creative);
        component.put_bool("mayBuild", self.allow_modify_world);
        component.put_float("flySpeed", self.fly_speed);
        component.put_float("walkSpeed", self.walk_speed);
        nbt.put_component("abilities", component);
    }

    async fn read_nbt(&mut self, nbt: &mut NbtCompound) {
        if let Some(component) = nbt.get_compound("abilities") {
            self.invulnerable = component.get_bool("invulnerable").unwrap_or(false);
            self.flying = component.get_bool("flying").unwrap_or(false);
            self.allow_flying = component.get_bool("mayfly").unwrap_or(false);
            self.creative = component.get_bool("instabuild").unwrap_or(false);
            self.allow_modify_world = component.get_bool("mayBuild").unwrap_or(false);
            self.fly_speed = component.get_float("flySpeed").unwrap_or(0.05);
            self.walk_speed = component.get_float("walkSpeed").unwrap_or(0.1);
        }
    }
}

impl NBTStorageInit for Abilities {}

impl Default for Abilities {
    fn default() -> Self {
        Self {
            invulnerable: false,
            flying: false,
            allow_flying: false,
            creative: false,
            allow_modify_world: true,
            fly_speed: 0.05,
            walk_speed: 0.1,
        }
    }
}

impl Abilities {
    pub fn set_for_gamemode(&mut self, gamemode: GameMode) {
        match gamemode {
            GameMode::Creative => {
                // self.flying = false; // Start not flying
                self.allow_flying = true;
                self.creative = true;
                self.invulnerable = true;
            }
            GameMode::Spectator => {
                self.flying = true;
                self.allow_flying = true;
                self.creative = false;
                self.invulnerable = true;
            }
            _ => {
                self.flying = false;
                self.allow_flying = false;
                self.creative = false;
                self.invulnerable = false;
            }
        }
    }
}

impl Player {
    /// Updates the current abilities the player has.
    pub async fn send_abilities_update(&self) {
        match &self.client {
            ClientPlatform::Java(java) => {
                let mut b = 0;
                let abilities = &self.abilities.lock().await;

                if abilities.invulnerable {
                    b |= 1;
                }
                if abilities.flying {
                    b |= 2;
                }
                if abilities.allow_flying {
                    b |= 4;
                }
                if abilities.creative {
                    b |= 8;
                }
                java.enqueue_packet(&CPlayerAbilities::new(
                    b,
                    abilities.fly_speed,
                    abilities.walk_speed,
                ))
                .await;
            }
            ClientPlatform::Bedrock(bedrock) => {
                let mut ability_value = 0;
                let abilities = &self.abilities.lock().await;

                if abilities.invulnerable {
                    ability_value |= 1 << Ability::Invulnerable as u32;
                }

                if abilities.flying {
                    ability_value |= 1 << Ability::Flying as u32;
                }

                if abilities.allow_flying {
                    ability_value |= 1 << Ability::MayFly as u32;
                }

                if abilities.creative {
                    ability_value |= 1 << Ability::OperatorCommands as u32;
                    ability_value |= 1 << Ability::Teleport as u32;
                    ability_value |= 1 << Ability::Invulnerable as u32;
                }

                // Todo: Integrate this into the system
                ability_value |= 1 << Ability::AttackMobs as u32;
                ability_value |= 1 << Ability::AttackPlayers as u32;
                ability_value |= 1 << Ability::Build as u32;
                ability_value |= 1 << Ability::DoorsAndSwitches as u32;
                ability_value |= 1 << Ability::Instabuild as u32;
                ability_value |= 1 << Ability::Mine as u32;

                let packet = CUpdateAbilities {
                    target_player_raw_id: self.entity_id().into(),
                    player_permission: 2,
                    command_permission: 4,
                    layers: vec![AbilityLayer {
                        serialized_layer: 1,
                        abilities_set: (1 << Ability::AbilityCount as u32) - 1,
                        ability_value,
                        fly_speed: 0.05,
                        vertical_fly_speed: 1.0,
                        walk_speed: 0.1,
                    }],
                };

                bedrock.send_game_packet(&packet).await;
            }
        }
    }

    pub async fn is_flying(&self) -> bool {
        let abilities = self.abilities.lock().await;
        abilities.flying
    }
}
