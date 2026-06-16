/* This file is generated. Do not edit manually. */
use crate::{
    codec::var_int::VarInt,
    ser::{NetworkWriteExt, WritingError},
};
#[allow(clippy::wildcard_imports)]
use pumpkin_data::particle::*;
use std::fmt::Debug;
pub trait SerializeParticleData: Send + Sync + Debug {
    fn id(&self) -> i32;
    fn to_bytes(&self, _write: &mut [u8]) -> Result<usize, WritingError> {
        Ok(0)
    }
}
impl<T: SerializeParticleData + ?Sized> SerializeParticleData for &T {
    fn id(&self) -> i32 {
        (**self).id()
    }
    fn to_bytes(&self, write: &mut [u8]) -> Result<usize, WritingError> {
        (**self).to_bytes(write)
    }
}
impl SerializeParticleData for AngryVillager {
    fn id(&self) -> i32 {
        0i32
    }
}
impl SerializeParticleData for Block {
    fn id(&self) -> i32 {
        1i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.state))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for BlockMarker {
    fn id(&self) -> i32 {
        2i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.state))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for Bubble {
    fn id(&self) -> i32 {
        3i32
    }
}
impl SerializeParticleData for Cloud {
    fn id(&self) -> i32 {
        4i32
    }
}
impl SerializeParticleData for CopperFireFlame {
    fn id(&self) -> i32 {
        5i32
    }
}
impl SerializeParticleData for Crit {
    fn id(&self) -> i32 {
        6i32
    }
}
impl SerializeParticleData for DamageIndicator {
    fn id(&self) -> i32 {
        7i32
    }
}
impl SerializeParticleData for DragonBreath {
    fn id(&self) -> i32 {
        8i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_f32_be(self.power)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for DrippingLava {
    fn id(&self) -> i32 {
        9i32
    }
}
impl SerializeParticleData for FallingLava {
    fn id(&self) -> i32 {
        10i32
    }
}
impl SerializeParticleData for LandingLava {
    fn id(&self) -> i32 {
        11i32
    }
}
impl SerializeParticleData for DrippingWater {
    fn id(&self) -> i32 {
        12i32
    }
}
impl SerializeParticleData for FallingWater {
    fn id(&self) -> i32 {
        13i32
    }
}
impl SerializeParticleData for Dust {
    fn id(&self) -> i32 {
        14i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        write.write_f32_be(self.scale)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for DustColorTransition {
    fn id(&self) -> i32 {
        15i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.from_color)?;
        write.write_i32_be(self.to_color)?;
        write.write_f32_be(self.scale)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for Effect {
    fn id(&self) -> i32 {
        16i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        write.write_f32_be(self.power)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for ElderGuardian {
    fn id(&self) -> i32 {
        17i32
    }
}
impl SerializeParticleData for EnchantedHit {
    fn id(&self) -> i32 {
        18i32
    }
}
impl SerializeParticleData for Enchant {
    fn id(&self) -> i32 {
        19i32
    }
}
impl SerializeParticleData for EndRod {
    fn id(&self) -> i32 {
        20i32
    }
}
impl SerializeParticleData for EntityEffect {
    fn id(&self) -> i32 {
        21i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for ExplosionEmitter {
    fn id(&self) -> i32 {
        22i32
    }
}
impl SerializeParticleData for Explosion {
    fn id(&self) -> i32 {
        23i32
    }
}
impl SerializeParticleData for Gust {
    fn id(&self) -> i32 {
        24i32
    }
}
impl SerializeParticleData for SmallGust {
    fn id(&self) -> i32 {
        25i32
    }
}
impl SerializeParticleData for GustEmitterLarge {
    fn id(&self) -> i32 {
        26i32
    }
}
impl SerializeParticleData for GustEmitterSmall {
    fn id(&self) -> i32 {
        27i32
    }
}
impl SerializeParticleData for SonicBoom {
    fn id(&self) -> i32 {
        28i32
    }
}
impl SerializeParticleData for FallingDust {
    fn id(&self) -> i32 {
        29i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.state))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for Firework {
    fn id(&self) -> i32 {
        30i32
    }
}
impl SerializeParticleData for Fishing {
    fn id(&self) -> i32 {
        31i32
    }
}
impl SerializeParticleData for Flame {
    fn id(&self) -> i32 {
        32i32
    }
}
impl SerializeParticleData for Infested {
    fn id(&self) -> i32 {
        33i32
    }
}
impl SerializeParticleData for CherryLeaves {
    fn id(&self) -> i32 {
        34i32
    }
}
impl SerializeParticleData for PaleOakLeaves {
    fn id(&self) -> i32 {
        35i32
    }
}
impl SerializeParticleData for TintedLeaves {
    fn id(&self) -> i32 {
        36i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for SculkSoul {
    fn id(&self) -> i32 {
        37i32
    }
}
impl SerializeParticleData for SculkCharge {
    fn id(&self) -> i32 {
        38i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_f32_be(self.roll)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for SculkChargePop {
    fn id(&self) -> i32 {
        39i32
    }
}
impl SerializeParticleData for SoulFireFlame {
    fn id(&self) -> i32 {
        40i32
    }
}
impl SerializeParticleData for Soul {
    fn id(&self) -> i32 {
        41i32
    }
}
impl SerializeParticleData for Flash {
    fn id(&self) -> i32 {
        42i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for HappyVillager {
    fn id(&self) -> i32 {
        43i32
    }
}
impl SerializeParticleData for Composter {
    fn id(&self) -> i32 {
        44i32
    }
}
impl SerializeParticleData for Heart {
    fn id(&self) -> i32 {
        45i32
    }
}
impl SerializeParticleData for InstantEffect {
    fn id(&self) -> i32 {
        46i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_i32_be(self.color)?;
        write.write_f32_be(self.power)?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for Item {
    fn id(&self) -> i32 {
        47i32
    }
}
impl SerializeParticleData for Vibration {
    fn id(&self) -> i32 {
        48i32
    }
}
impl SerializeParticleData for Trail {
    fn id(&self) -> i32 {
        49i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_f64_be(self.target.x)?;
        write.write_f64_be(self.target.y)?;
        write.write_f64_be(self.target.z)?;
        write.write_i32_be(self.color)?;
        write.write_var_int(&VarInt(self.duration))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for PauseMobGrowth {
    fn id(&self) -> i32 {
        50i32
    }
}
impl SerializeParticleData for ResetMobGrowth {
    fn id(&self) -> i32 {
        51i32
    }
}
impl SerializeParticleData for ItemSlime {
    fn id(&self) -> i32 {
        52i32
    }
}
impl SerializeParticleData for ItemCobweb {
    fn id(&self) -> i32 {
        53i32
    }
}
impl SerializeParticleData for ItemSnowball {
    fn id(&self) -> i32 {
        54i32
    }
}
impl SerializeParticleData for LargeSmoke {
    fn id(&self) -> i32 {
        55i32
    }
}
impl SerializeParticleData for Lava {
    fn id(&self) -> i32 {
        56i32
    }
}
impl SerializeParticleData for Mycelium {
    fn id(&self) -> i32 {
        57i32
    }
}
impl SerializeParticleData for Note {
    fn id(&self) -> i32 {
        58i32
    }
}
impl SerializeParticleData for Poof {
    fn id(&self) -> i32 {
        59i32
    }
}
impl SerializeParticleData for Portal {
    fn id(&self) -> i32 {
        60i32
    }
}
impl SerializeParticleData for Rain {
    fn id(&self) -> i32 {
        61i32
    }
}
impl SerializeParticleData for Smoke {
    fn id(&self) -> i32 {
        62i32
    }
}
impl SerializeParticleData for WhiteSmoke {
    fn id(&self) -> i32 {
        63i32
    }
}
impl SerializeParticleData for Sneeze {
    fn id(&self) -> i32 {
        64i32
    }
}
impl SerializeParticleData for Spit {
    fn id(&self) -> i32 {
        65i32
    }
}
impl SerializeParticleData for SquidInk {
    fn id(&self) -> i32 {
        66i32
    }
}
impl SerializeParticleData for SweepAttack {
    fn id(&self) -> i32 {
        67i32
    }
}
impl SerializeParticleData for TotemOfUndying {
    fn id(&self) -> i32 {
        68i32
    }
}
impl SerializeParticleData for Underwater {
    fn id(&self) -> i32 {
        69i32
    }
}
impl SerializeParticleData for Splash {
    fn id(&self) -> i32 {
        70i32
    }
}
impl SerializeParticleData for Witch {
    fn id(&self) -> i32 {
        71i32
    }
}
impl SerializeParticleData for BubblePop {
    fn id(&self) -> i32 {
        72i32
    }
}
impl SerializeParticleData for CurrentDown {
    fn id(&self) -> i32 {
        73i32
    }
}
impl SerializeParticleData for BubbleColumnUp {
    fn id(&self) -> i32 {
        74i32
    }
}
impl SerializeParticleData for Nautilus {
    fn id(&self) -> i32 {
        75i32
    }
}
impl SerializeParticleData for Dolphin {
    fn id(&self) -> i32 {
        76i32
    }
}
impl SerializeParticleData for CampfireCosySmoke {
    fn id(&self) -> i32 {
        77i32
    }
}
impl SerializeParticleData for CampfireSignalSmoke {
    fn id(&self) -> i32 {
        78i32
    }
}
impl SerializeParticleData for DrippingHoney {
    fn id(&self) -> i32 {
        79i32
    }
}
impl SerializeParticleData for FallingHoney {
    fn id(&self) -> i32 {
        80i32
    }
}
impl SerializeParticleData for LandingHoney {
    fn id(&self) -> i32 {
        81i32
    }
}
impl SerializeParticleData for FallingNectar {
    fn id(&self) -> i32 {
        82i32
    }
}
impl SerializeParticleData for FallingSporeBlossom {
    fn id(&self) -> i32 {
        83i32
    }
}
impl SerializeParticleData for Ash {
    fn id(&self) -> i32 {
        84i32
    }
}
impl SerializeParticleData for CrimsonSpore {
    fn id(&self) -> i32 {
        85i32
    }
}
impl SerializeParticleData for WarpedSpore {
    fn id(&self) -> i32 {
        86i32
    }
}
impl SerializeParticleData for SporeBlossomAir {
    fn id(&self) -> i32 {
        87i32
    }
}
impl SerializeParticleData for DrippingObsidianTear {
    fn id(&self) -> i32 {
        88i32
    }
}
impl SerializeParticleData for FallingObsidianTear {
    fn id(&self) -> i32 {
        89i32
    }
}
impl SerializeParticleData for LandingObsidianTear {
    fn id(&self) -> i32 {
        90i32
    }
}
impl SerializeParticleData for ReversePortal {
    fn id(&self) -> i32 {
        91i32
    }
}
impl SerializeParticleData for WhiteAsh {
    fn id(&self) -> i32 {
        92i32
    }
}
impl SerializeParticleData for SmallFlame {
    fn id(&self) -> i32 {
        93i32
    }
}
impl SerializeParticleData for Snowflake {
    fn id(&self) -> i32 {
        94i32
    }
}
impl SerializeParticleData for DrippingDripstoneLava {
    fn id(&self) -> i32 {
        95i32
    }
}
impl SerializeParticleData for FallingDripstoneLava {
    fn id(&self) -> i32 {
        96i32
    }
}
impl SerializeParticleData for DrippingDripstoneWater {
    fn id(&self) -> i32 {
        97i32
    }
}
impl SerializeParticleData for FallingDripstoneWater {
    fn id(&self) -> i32 {
        98i32
    }
}
impl SerializeParticleData for GlowSquidInk {
    fn id(&self) -> i32 {
        99i32
    }
}
impl SerializeParticleData for Glow {
    fn id(&self) -> i32 {
        100i32
    }
}
impl SerializeParticleData for WaxOn {
    fn id(&self) -> i32 {
        101i32
    }
}
impl SerializeParticleData for WaxOff {
    fn id(&self) -> i32 {
        102i32
    }
}
impl SerializeParticleData for ElectricSpark {
    fn id(&self) -> i32 {
        103i32
    }
}
impl SerializeParticleData for Scrape {
    fn id(&self) -> i32 {
        104i32
    }
}
impl SerializeParticleData for Shriek {
    fn id(&self) -> i32 {
        105i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.delay))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for EggCrack {
    fn id(&self) -> i32 {
        106i32
    }
}
impl SerializeParticleData for DustPlume {
    fn id(&self) -> i32 {
        107i32
    }
}
impl SerializeParticleData for TrialSpawnerDetection {
    fn id(&self) -> i32 {
        108i32
    }
}
impl SerializeParticleData for TrialSpawnerDetectionOminous {
    fn id(&self) -> i32 {
        109i32
    }
}
impl SerializeParticleData for VaultConnection {
    fn id(&self) -> i32 {
        110i32
    }
}
impl SerializeParticleData for DustPillar {
    fn id(&self) -> i32 {
        111i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.state))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for OminousSpawning {
    fn id(&self) -> i32 {
        112i32
    }
}
impl SerializeParticleData for RaidOmen {
    fn id(&self) -> i32 {
        113i32
    }
}
impl SerializeParticleData for TrialOmen {
    fn id(&self) -> i32 {
        114i32
    }
}
impl SerializeParticleData for BlockCrumble {
    fn id(&self) -> i32 {
        115i32
    }
    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
        let initial_len = write.len();
        write.write_var_int(&VarInt(self.state))?;
        Ok(initial_len - write.len())
    }
}
impl SerializeParticleData for Firefly {
    fn id(&self) -> i32 {
        116i32
    }
}
