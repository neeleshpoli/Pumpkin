/* This file is generated. Do not edit manually. */
use pumpkin_util::math::vector3::Vector3;
#[derive(Clone, Copy, Debug)]
pub struct AngryVillager;
#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub state: i32,
}
impl Block {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct BlockMarker {
    pub state: i32,
}
impl BlockMarker {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Bubble;
#[derive(Clone, Copy, Debug)]
pub struct Cloud;
#[derive(Clone, Copy, Debug)]
pub struct CopperFireFlame;
#[derive(Clone, Copy, Debug)]
pub struct Crit;
#[derive(Clone, Copy, Debug)]
pub struct DamageIndicator;
#[derive(Clone, Copy, Debug)]
pub struct DragonBreath {
    pub power: f32,
}
impl DragonBreath {
    pub fn new(power: f32) -> Self {
        Self { power }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct DrippingLava;
#[derive(Clone, Copy, Debug)]
pub struct FallingLava;
#[derive(Clone, Copy, Debug)]
pub struct LandingLava;
#[derive(Clone, Copy, Debug)]
pub struct DrippingWater;
#[derive(Clone, Copy, Debug)]
pub struct FallingWater;
#[derive(Clone, Copy, Debug)]
pub struct Dust {
    pub color: i32,
    pub scale: f32,
}
impl Dust {
    pub fn new(color: i32, scale: f32) -> Self {
        Self { color, scale }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct DustColorTransition {
    pub from_color: i32,
    pub to_color: i32,
    pub scale: f32,
}
impl DustColorTransition {
    pub fn new(from_color: i32, to_color: i32, scale: f32) -> Self {
        Self {
            from_color,
            to_color,
            scale,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Effect {
    pub color: i32,
    pub power: f32,
}
impl Effect {
    pub fn new(color: i32, power: f32) -> Self {
        Self { color, power }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ElderGuardian;
#[derive(Clone, Copy, Debug)]
pub struct EnchantedHit;
#[derive(Clone, Copy, Debug)]
pub struct Enchant;
#[derive(Clone, Copy, Debug)]
pub struct EndRod;
#[derive(Clone, Copy, Debug)]
pub struct EntityEffect {
    pub color: i32,
}
impl EntityEffect {
    pub fn new(color: i32) -> Self {
        Self { color }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ExplosionEmitter;
#[derive(Clone, Copy, Debug)]
pub struct Explosion;
#[derive(Clone, Copy, Debug)]
pub struct Gust;
#[derive(Clone, Copy, Debug)]
pub struct SmallGust;
#[derive(Clone, Copy, Debug)]
pub struct GustEmitterLarge;
#[derive(Clone, Copy, Debug)]
pub struct GustEmitterSmall;
#[derive(Clone, Copy, Debug)]
pub struct SonicBoom;
#[derive(Clone, Copy, Debug)]
pub struct FallingDust {
    pub state: i32,
}
impl FallingDust {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Firework;
#[derive(Clone, Copy, Debug)]
pub struct Fishing;
#[derive(Clone, Copy, Debug)]
pub struct Flame;
#[derive(Clone, Copy, Debug)]
pub struct Infested;
#[derive(Clone, Copy, Debug)]
pub struct CherryLeaves;
#[derive(Clone, Copy, Debug)]
pub struct PaleOakLeaves;
#[derive(Clone, Copy, Debug)]
pub struct TintedLeaves {
    pub color: i32,
}
impl TintedLeaves {
    pub fn new(color: i32) -> Self {
        Self { color }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct SculkSoul;
#[derive(Clone, Copy, Debug)]
pub struct SculkCharge {
    pub roll: f32,
}
impl SculkCharge {
    pub fn new(roll: f32) -> Self {
        Self { roll }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct SculkChargePop;
#[derive(Clone, Copy, Debug)]
pub struct SoulFireFlame;
#[derive(Clone, Copy, Debug)]
pub struct Soul;
#[derive(Clone, Copy, Debug)]
pub struct Flash {
    pub color: i32,
}
impl Flash {
    pub fn new(color: i32) -> Self {
        Self { color }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct HappyVillager;
#[derive(Clone, Copy, Debug)]
pub struct Composter;
#[derive(Clone, Copy, Debug)]
pub struct Heart;
#[derive(Clone, Copy, Debug)]
pub struct InstantEffect {
    pub color: i32,
    pub power: f32,
}
impl InstantEffect {
    pub fn new(color: i32, power: f32) -> Self {
        Self { color, power }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Item;
#[derive(Clone, Copy, Debug)]
pub struct Vibration;
#[derive(Clone, Copy, Debug)]
pub struct Trail {
    pub target: Vector3<f64>,
    pub color: i32,
    pub duration: i32,
}
impl Trail {
    pub fn new(target: Vector3<f64>, color: i32, duration: i32) -> Self {
        Self {
            target,
            color,
            duration,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PauseMobGrowth;
#[derive(Clone, Copy, Debug)]
pub struct ResetMobGrowth;
#[derive(Clone, Copy, Debug)]
pub struct ItemSlime;
#[derive(Clone, Copy, Debug)]
pub struct ItemCobweb;
#[derive(Clone, Copy, Debug)]
pub struct ItemSnowball;
#[derive(Clone, Copy, Debug)]
pub struct LargeSmoke;
#[derive(Clone, Copy, Debug)]
pub struct Lava;
#[derive(Clone, Copy, Debug)]
pub struct Mycelium;
#[derive(Clone, Copy, Debug)]
pub struct Note;
#[derive(Clone, Copy, Debug)]
pub struct Poof;
#[derive(Clone, Copy, Debug)]
pub struct Portal;
#[derive(Clone, Copy, Debug)]
pub struct Rain;
#[derive(Clone, Copy, Debug)]
pub struct Smoke;
#[derive(Clone, Copy, Debug)]
pub struct WhiteSmoke;
#[derive(Clone, Copy, Debug)]
pub struct Sneeze;
#[derive(Clone, Copy, Debug)]
pub struct Spit;
#[derive(Clone, Copy, Debug)]
pub struct SquidInk;
#[derive(Clone, Copy, Debug)]
pub struct SweepAttack;
#[derive(Clone, Copy, Debug)]
pub struct TotemOfUndying;
#[derive(Clone, Copy, Debug)]
pub struct Underwater;
#[derive(Clone, Copy, Debug)]
pub struct Splash;
#[derive(Clone, Copy, Debug)]
pub struct Witch;
#[derive(Clone, Copy, Debug)]
pub struct BubblePop;
#[derive(Clone, Copy, Debug)]
pub struct CurrentDown;
#[derive(Clone, Copy, Debug)]
pub struct BubbleColumnUp;
#[derive(Clone, Copy, Debug)]
pub struct Nautilus;
#[derive(Clone, Copy, Debug)]
pub struct Dolphin;
#[derive(Clone, Copy, Debug)]
pub struct CampfireCosySmoke;
#[derive(Clone, Copy, Debug)]
pub struct CampfireSignalSmoke;
#[derive(Clone, Copy, Debug)]
pub struct DrippingHoney;
#[derive(Clone, Copy, Debug)]
pub struct FallingHoney;
#[derive(Clone, Copy, Debug)]
pub struct LandingHoney;
#[derive(Clone, Copy, Debug)]
pub struct FallingNectar;
#[derive(Clone, Copy, Debug)]
pub struct FallingSporeBlossom;
#[derive(Clone, Copy, Debug)]
pub struct Ash;
#[derive(Clone, Copy, Debug)]
pub struct CrimsonSpore;
#[derive(Clone, Copy, Debug)]
pub struct WarpedSpore;
#[derive(Clone, Copy, Debug)]
pub struct SporeBlossomAir;
#[derive(Clone, Copy, Debug)]
pub struct DrippingObsidianTear;
#[derive(Clone, Copy, Debug)]
pub struct FallingObsidianTear;
#[derive(Clone, Copy, Debug)]
pub struct LandingObsidianTear;
#[derive(Clone, Copy, Debug)]
pub struct ReversePortal;
#[derive(Clone, Copy, Debug)]
pub struct WhiteAsh;
#[derive(Clone, Copy, Debug)]
pub struct SmallFlame;
#[derive(Clone, Copy, Debug)]
pub struct Snowflake;
#[derive(Clone, Copy, Debug)]
pub struct DrippingDripstoneLava;
#[derive(Clone, Copy, Debug)]
pub struct FallingDripstoneLava;
#[derive(Clone, Copy, Debug)]
pub struct DrippingDripstoneWater;
#[derive(Clone, Copy, Debug)]
pub struct FallingDripstoneWater;
#[derive(Clone, Copy, Debug)]
pub struct GlowSquidInk;
#[derive(Clone, Copy, Debug)]
pub struct Glow;
#[derive(Clone, Copy, Debug)]
pub struct WaxOn;
#[derive(Clone, Copy, Debug)]
pub struct WaxOff;
#[derive(Clone, Copy, Debug)]
pub struct ElectricSpark;
#[derive(Clone, Copy, Debug)]
pub struct Scrape;
#[derive(Clone, Copy, Debug)]
pub struct Shriek {
    pub delay: i32,
}
impl Shriek {
    pub fn new(delay: i32) -> Self {
        Self { delay }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct EggCrack;
#[derive(Clone, Copy, Debug)]
pub struct DustPlume;
#[derive(Clone, Copy, Debug)]
pub struct TrialSpawnerDetection;
#[derive(Clone, Copy, Debug)]
pub struct TrialSpawnerDetectionOminous;
#[derive(Clone, Copy, Debug)]
pub struct VaultConnection;
#[derive(Clone, Copy, Debug)]
pub struct DustPillar {
    pub state: i32,
}
impl DustPillar {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct OminousSpawning;
#[derive(Clone, Copy, Debug)]
pub struct RaidOmen;
#[derive(Clone, Copy, Debug)]
pub struct TrialOmen;
#[derive(Clone, Copy, Debug)]
pub struct BlockCrumble {
    pub state: i32,
}
impl BlockCrumble {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Firefly;
