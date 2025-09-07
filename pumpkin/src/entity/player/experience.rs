use std::sync::atomic::Ordering;

use pumpkin_protocol::java::client::play::CSetExperience;
use pumpkin_util::math::experience;

use crate::entity::player::Player;

impl Player {
    /// Add experience levels to the player.
    pub async fn add_experience_levels(&self, added_levels: i32) {
        let current_level = self.experience_level.load(Ordering::Relaxed);
        let new_level = current_level + added_levels;
        self.set_experience_level(new_level, true).await;
    }

    /// Set the player's experience points directly. Returns `true` if successful.
    pub async fn set_experience_points(&self, new_points: i32) -> bool {
        let current_points = self.experience_points.load(Ordering::Relaxed);

        if new_points == current_points {
            return true;
        }

        let current_level = self.experience_level.load(Ordering::Relaxed);
        let max_points = experience::points_in_level(current_level);

        if new_points < 0 || new_points > max_points {
            return false;
        }

        let progress = new_points as f32 / max_points as f32;
        self.set_experience(current_level, progress, new_points)
            .await;
        true
    }

    /// Add experience points to the player.
    pub async fn add_experience_points(&self, added_points: i32) {
        let current_level = self.experience_level.load(Ordering::Relaxed);
        let current_points = self.experience_points.load(Ordering::Relaxed);
        let total_exp = experience::points_to_level(current_level) + current_points;
        let new_total_exp = total_exp + added_points;
        let (new_level, new_points) = experience::total_to_level_and_points(new_total_exp);
        let progress = experience::progress_in_level(new_points, new_level);
        self.set_experience(new_level, progress, new_points).await;
    }

    pub async fn tick_experience(&self) {
        let level = self.experience_level.load(Ordering::Relaxed);
        if self.last_sent_xp.load(Ordering::Relaxed) != level {
            let progress = self.experience_progress.load();
            let points = self.experience_points.load(Ordering::Relaxed);

            self.last_sent_xp.store(level, Ordering::Relaxed);

            self.client
                .send_packet_now(&CSetExperience::new(
                    progress.clamp(0.0, 1.0),
                    points.into(),
                    level.into(),
                ))
                .await;
        }
    }

    /// Sets the player's experience level and notifies the client.
    pub async fn set_experience(&self, level: i32, progress: f32, points: i32) {
        // TODO: These should be atomic together, not isolated; make a struct containing these. can cause ABA issues
        self.experience_level.store(level, Ordering::Relaxed);
        self.experience_progress.store(progress.clamp(0.0, 1.0));
        self.experience_points.store(points, Ordering::Relaxed);
        self.last_sent_xp.store(-1, Ordering::Relaxed);
        self.tick_experience().await;

        self.client
            .enqueue_packet(&CSetExperience::new(
                progress.clamp(0.0, 1.0),
                points.into(),
                level.into(),
            ))
            .await;
    }

    /// Sets the player's experience level directly.
    pub async fn set_experience_level(&self, new_level: i32, keep_progress: bool) {
        let progress = self.experience_progress.load();
        let mut points = self.experience_points.load(Ordering::Relaxed);

        // If `keep_progress` is `true` then calculate the number of points needed to keep the same progress scaled.
        if keep_progress {
            // Get our current level
            let current_level = self.experience_level.load(Ordering::Relaxed);
            let current_max_points = experience::points_in_level(current_level);
            // Calculate the max value for the new level
            let new_max_points = experience::points_in_level(new_level);
            // Calculate the scaling factor
            let scale = new_max_points as f32 / current_max_points as f32;
            // Scale the points (Vanilla doesn't seem to recalculate progress so we won't)
            points = (points as f32 * scale) as i32;
        }

        self.set_experience(new_level, progress, points).await;
    }
}
