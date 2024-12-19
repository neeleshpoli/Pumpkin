use pumpkin_core::math::{vector2::Vector2, vector3::Vector3};
use serde::Deserialize;
use tokio::{fs::OpenOptions, io::AsyncReadExt};

use crate::chunk::world_format::{WorldHandlingError, WorldInfo};

use super::compression::Compression;

#[derive(Deserialize)]
pub(super) struct LevelDat {
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Data {
    border_center_x: f64,
    border_center_z: f64,
    border_damage_per_block: f64,
    border_size: f64,
    border_safe_zone: f64,
    border_size_lerp_target: f64,
    border_warning_blocks: f64,
    border_warning_time: f64,
    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,
    day_time: i64,
    world_gen_settings: WorldGenSettings,
    #[serde(rename = "raining")]
    raining: bool,
    #[serde(rename = "rainTime")]
    rain_time: i32,
    spawn_x: i32,
    spawn_y: i32,
    spawn_z: i32,
    #[serde(rename = "thundering")]
    thundering: bool,
    #[serde(rename = "thunderTime")]
    thunder_time: i32,
    time: i64,
}

#[derive(Deserialize)]
struct WorldGenSettings {
    seed: i64,
    // TODO add other fields
}

impl LevelDat {
    pub async fn read_level_dat(world_path: &String) -> Result<WorldInfo, WorldHandlingError> {
        // Load level.dat
        let mut level_info_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("{world_path}/level.dat"))
            .await?;

        // Read the entirity of level.dat
        let mut buffer = Vec::new();
        level_info_file.read_to_end(&mut buffer).await?;

        let decompressed_data = Compression::GZip.decompress_data(buffer)?;

        // Parse level.dat
        let info = fastnbt::from_bytes::<LevelDat>(&decompressed_data)
            .map_err(|e| WorldHandlingError::DeserializationError(e.to_string()))?;

        // Final world info
        Ok(info.into())
    }
}

impl From<LevelDat> for WorldInfo {
    fn from(value: LevelDat) -> Self {
        let value = value.data;
        Self {
            border_center: Vector2::new(value.border_center_x, value.border_center_z),
            border_damage_per_block: value.border_damage_per_block,
            border_size: value.border_size,
            border_safe_zone: value.border_safe_zone,
            border_size_lerp_target: value.border_size_lerp_target,
            border_warning_blocks: value.border_warning_blocks,
            border_warning_time: value.border_warning_time,
            clear_weather_time: value.clear_weather_time,
            day_time: value.day_time,
            seed: value.world_gen_settings.seed,
            raining: value.raining,
            rain_time: value.rain_time,
            spawn: Vector3::new(value.spawn_x, value.spawn_y, value.spawn_z),
            thundering: value.thundering,
            thunder_time: value.thunder_time,
            time: value.time,
        }
    }
}

#[tokio::test]
async fn test_level_dat_reading_sample_1() {
    const PATH: &str = "../test/anvil-world-saving/sample-1";
    let info = LevelDat::read_level_dat(&PATH.to_string()).await.unwrap();

    let correct_info = WorldInfo {
        border_center: Vector2::new(0f64, 0f64),
        border_damage_per_block: 0.2,
        border_size: 59999968f64,
        border_safe_zone: 5f64,
        border_size_lerp_target: 59999968f64,
        border_warning_blocks: 5f64,
        border_warning_time: 15f64,
        clear_weather_time: 0,
        day_time: 6075,
        seed: -7121061153453964786,
        raining: false,
        rain_time: 32005,
        spawn: Vector3::new(-48, 68, 176),
        thundering: false,
        thunder_time: 19458,
        time: 6075,
    };

    assert_eq!(info, correct_info)
}

#[tokio::test]
async fn test_level_dat_reading_sample_2() {
    const PATH: &str = "../test/anvil-world-saving/sample-2";
    let info = LevelDat::read_level_dat(&PATH.to_string()).await.unwrap();

    let correct_info = WorldInfo {
        border_center: Vector2::new(0f64, 0f64),
        border_damage_per_block: 0.2,
        border_size: 59999968f64,
        border_safe_zone: 5f64,
        border_size_lerp_target: 59999968f64,
        border_warning_blocks: 5f64,
        border_warning_time: 15f64,
        clear_weather_time: 0,
        day_time: 13824,
        seed: -79717552349559436,
        raining: false,
        rain_time: 77361,
        spawn: Vector3::new(0, 83, 0),
        thundering: false,
        thunder_time: 75815,
        time: 13824,
    };

    assert_eq!(info, correct_info)
}
