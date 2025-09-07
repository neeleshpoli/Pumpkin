use pumpkin_data::{
    Block,
    tag::{self, Taggable},
};
use pumpkin_protocol::java::client::play::{CGameEvent, CPlayerSpawnPosition, GameEvent};
use pumpkin_registry::VanillaDimensionType;
use pumpkin_util::math::{position::BlockPos, vector3::Vector3};

use crate::entity::player::Player;

impl Player {
    pub async fn set_respawn_point(
        &self,
        dimension: VanillaDimensionType,
        block_pos: BlockPos,
        yaw: f32,
    ) -> bool {
        if let Some(respawn_point) = self.respawn_point.load()
            && dimension == respawn_point.dimension
            && block_pos == respawn_point.position
        {
            return false;
        }

        self.respawn_point.store(Some(RespawnPoint {
            dimension,
            position: block_pos,
            yaw,
            force: false,
        }));

        self.client
            .send_packet_now(&CPlayerSpawnPosition::new(block_pos, yaw))
            .await;
        true
    }

    pub async fn get_respawn_point(&self) -> Option<(Vector3<f64>, f32)> {
        let respawn_point = self.respawn_point.load()?;

        let block = self.world().get_block(&respawn_point.position).await;

        if respawn_point.dimension == VanillaDimensionType::Overworld
            && block.is_tagged_with_by_tag(&tag::Block::MINECRAFT_BEDS)
        {
            // TODO: calculate respawn position
            Some((respawn_point.position.to_f64(), respawn_point.yaw))
        } else if respawn_point.dimension == VanillaDimensionType::TheNether
            && block == &Block::RESPAWN_ANCHOR
        {
            // TODO: calculate respawn position
            // TODO: check if there is fuel for respawn
            Some((respawn_point.position.to_f64(), respawn_point.yaw))
        } else {
            self.client
                .send_packet_now(&CGameEvent::new(GameEvent::NoRespawnBlockAvailable, 0.0))
                .await;

            None
        }
    }
}

/// Represents the player's respawn point.
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct RespawnPoint {
    pub dimension: VanillaDimensionType,
    pub position: BlockPos,
    pub yaw: f32,
    pub force: bool,
}
