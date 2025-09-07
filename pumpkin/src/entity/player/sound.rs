use pumpkin_data::sound::SoundCategory;
use pumpkin_protocol::{
    IdOr,
    java::client::play::{CSoundEffect, CStopSound},
};
use pumpkin_util::{math::vector3::Vector3, resource_location::ResourceLocation};

use crate::entity::player::Player;

impl Player {
    pub async fn play_sound(
        &self,
        sound_id: u16,
        category: SoundCategory,
        position: &Vector3<f64>,
        volume: f32,
        pitch: f32,
        seed: f64,
    ) {
        self.client
            .enqueue_packet(&CSoundEffect::new(
                IdOr::Id(sound_id),
                category,
                position,
                volume,
                pitch,
                seed,
            ))
            .await;
    }

    /// Stops a sound playing on the client.
    ///
    /// # Arguments
    ///
    /// * `sound_id`: An optional [`ResourceLocation`] specifying the sound to stop. If [`None`], all sounds in the specified category (if any) will be stopped.
    /// * `category`: An optional [`SoundCategory`] specifying the sound category to stop. If [`None`], all sounds with the specified resource location (if any) will be stopped.
    pub async fn stop_sound(
        &self,
        sound_id: Option<ResourceLocation>,
        category: Option<SoundCategory>,
    ) {
        self.client
            .enqueue_packet(&CStopSound::new(sound_id, category))
            .await;
    }
}
