use std::sync::Arc;

use pumpkin_data::particle::{
    AngryVillager, Ash, Bubble, BubbleColumnUp, BubblePop, CampfireCosySmoke, CampfireSignalSmoke,
    CherryLeaves, Cloud, Composter, CopperFireFlame, CrimsonSpore, Crit, CurrentDown,
    DamageIndicator, Dolphin, DrippingDripstoneLava, DrippingDripstoneWater, DrippingHoney,
    DrippingLava, DrippingObsidianTear, DrippingWater, DustPlume, EggCrack, ElderGuardian,
    ElectricSpark, Enchant, EnchantedHit, EndRod, Explosion, ExplosionEmitter,
    FallingDripstoneLava, FallingDripstoneWater, FallingHoney, FallingLava, FallingNectar,
    FallingObsidianTear, FallingSporeBlossom, FallingWater, Firefly, Firework, Fishing, Flame,
    Glow, GlowSquidInk, Gust, GustEmitterLarge, GustEmitterSmall, HappyVillager, Heart, Infested,
    ItemCobweb, ItemSlime, ItemSnowball, LandingHoney, LandingLava, LandingObsidianTear,
    LargeSmoke, Lava, Mycelium, Nautilus, Note, OminousSpawning, PaleOakLeaves, PauseMobGrowth,
    Poof, Portal, RaidOmen, Rain, ResetMobGrowth, ReversePortal, Scrape, SculkChargePop, SculkSoul,
    SmallFlame, SmallGust, Smoke, Sneeze, Snowflake, SonicBoom, Soul, SoulFireFlame, Spit, Splash,
    SporeBlossomAir, SquidInk, SweepAttack, TotemOfUndying, TrialOmen, TrialSpawnerDetection,
    TrialSpawnerDetectionOminous, Underwater, VaultConnection, Vibration, WarpedSpore, WaxOff,
    WaxOn, WhiteAsh, WhiteSmoke, Witch,
};
use pumpkin_protocol::java::client::play::{
    ArgumentType, SerializeParticleData, SuggestionProviders,
};

use crate::command::{
    CommandSender,
    args::{
        Arg, ArgumentConsumer, ConsumeResult, ConsumedArgs, DefaultNameArgConsumer, FindArg,
        GetClientSideArgParser,
    },
    dispatcher::CommandError,
    tree::RawArgs,
};
use crate::server::Server;

pub struct ParticleArgumentConsumer;

impl GetClientSideArgParser for ParticleArgumentConsumer {
    fn get_client_side_parser(&self) -> ArgumentType<'_> {
        ArgumentType::Resource {
            identifier: "particle_type",
        }
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<SuggestionProviders> {
        None
    }
}

impl ArgumentConsumer for ParticleArgumentConsumer {
    fn consume<'a, 'b>(
        &'a self,
        _sender: &'a CommandSender,
        _server: &'a Server,
        args: &'b mut RawArgs<'a>,
    ) -> ConsumeResult<'a> {
        let name_opt: Option<&'a str> = args.pop().map(|arg| arg.value);

        let result: Option<Arg<'a>> = name_opt.map_or_else(
            || None,
            |name| from_name(name.strip_prefix("minecraft:").unwrap_or(name)).map(Arg::Particle),
        );

        Box::pin(async move { result })
    }
}

impl DefaultNameArgConsumer for ParticleArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "particle_type"
    }
}

impl<'a> FindArg<'a> for ParticleArgumentConsumer {
    type Data = Arc<dyn SerializeParticleData>;

    fn find_arg(args: &'a ConsumedArgs, name: &str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::Particle(data)) => Ok(data.clone()),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}

#[must_use]
pub fn from_name(name: &str) -> Option<Arc<dyn SerializeParticleData>> {
    match name {
        "angry_villager" => Some(Arc::new(AngryVillager)),
        "bubble" => Some(Arc::new(Bubble)),
        "cloud" => Some(Arc::new(Cloud)),
        "copper_fire_flame" => Some(Arc::new(CopperFireFlame)),
        "crit" => Some(Arc::new(Crit)),
        "damage_indicator" => Some(Arc::new(DamageIndicator)),
        "dripping_lava" => Some(Arc::new(DrippingLava)),
        "falling_lava" => Some(Arc::new(FallingLava)),
        "landing_lava" => Some(Arc::new(LandingLava)),
        "dripping_water" => Some(Arc::new(DrippingWater)),
        "falling_water" => Some(Arc::new(FallingWater)),
        "elder_guardian" => Some(Arc::new(ElderGuardian)),
        "enchanted_hit" => Some(Arc::new(EnchantedHit)),
        "enchant" => Some(Arc::new(Enchant)),
        "end_rod" => Some(Arc::new(EndRod)),
        "explosion_emitter" => Some(Arc::new(ExplosionEmitter)),
        "explosion" => Some(Arc::new(Explosion)),
        "gust" => Some(Arc::new(Gust)),
        "small_gust" => Some(Arc::new(SmallGust)),
        "gust_emitter_large" => Some(Arc::new(GustEmitterLarge)),
        "gust_emitter_small" => Some(Arc::new(GustEmitterSmall)),
        "sonic_boom" => Some(Arc::new(SonicBoom)),
        "firework" => Some(Arc::new(Firework)),
        "fishing" => Some(Arc::new(Fishing)),
        "flame" => Some(Arc::new(Flame)),
        "infested" => Some(Arc::new(Infested)),
        "cherry_leaves" => Some(Arc::new(CherryLeaves)),
        "pale_oak_leaves" => Some(Arc::new(PaleOakLeaves)),
        "sculk_soul" => Some(Arc::new(SculkSoul)),
        "sculk_charge_pop" => Some(Arc::new(SculkChargePop)),
        "soul_fire_flame" => Some(Arc::new(SoulFireFlame)),
        "soul" => Some(Arc::new(Soul)),
        "happy_villager" => Some(Arc::new(HappyVillager)),
        "composter" => Some(Arc::new(Composter)),
        "heart" => Some(Arc::new(Heart)),
        "vibration" => Some(Arc::new(Vibration)),
        "pause_mob_growth" => Some(Arc::new(PauseMobGrowth)),
        "reset_mob_growth" => Some(Arc::new(ResetMobGrowth)),
        "item_slime" => Some(Arc::new(ItemSlime)),
        "item_cobweb" => Some(Arc::new(ItemCobweb)),
        "item_snowball" => Some(Arc::new(ItemSnowball)),
        "large_smoke" => Some(Arc::new(LargeSmoke)),
        "lava" => Some(Arc::new(Lava)),
        "mycelium" => Some(Arc::new(Mycelium)),
        "note" => Some(Arc::new(Note)),
        "poof" => Some(Arc::new(Poof)),
        "portal" => Some(Arc::new(Portal)),
        "rain" => Some(Arc::new(Rain)),
        "smoke" => Some(Arc::new(Smoke)),
        "white_smoke" => Some(Arc::new(WhiteSmoke)),
        "sneeze" => Some(Arc::new(Sneeze)),
        "spit" => Some(Arc::new(Spit)),
        "squid_ink" => Some(Arc::new(SquidInk)),
        "sweep_attack" => Some(Arc::new(SweepAttack)),
        "totem_of_undying" => Some(Arc::new(TotemOfUndying)),
        "underwater" => Some(Arc::new(Underwater)),
        "splash" => Some(Arc::new(Splash)),
        "witch" => Some(Arc::new(Witch)),
        "bubble_pop" => Some(Arc::new(BubblePop)),
        "current_down" => Some(Arc::new(CurrentDown)),
        "bubble_column_up" => Some(Arc::new(BubbleColumnUp)),
        "nautilus" => Some(Arc::new(Nautilus)),
        "dolphin" => Some(Arc::new(Dolphin)),
        "campfire_cosy_smoke" => Some(Arc::new(CampfireCosySmoke)),
        "campfire_signal_smoke" => Some(Arc::new(CampfireSignalSmoke)),
        "dripping_honey" => Some(Arc::new(DrippingHoney)),
        "falling_honey" => Some(Arc::new(FallingHoney)),
        "landing_honey" => Some(Arc::new(LandingHoney)),
        "falling_nectar" => Some(Arc::new(FallingNectar)),
        "falling_spore_blossom" => Some(Arc::new(FallingSporeBlossom)),
        "ash" => Some(Arc::new(Ash)),
        "crimson_spore" => Some(Arc::new(CrimsonSpore)),
        "warped_spore" => Some(Arc::new(WarpedSpore)),
        "spore_blossom_air" => Some(Arc::new(SporeBlossomAir)),
        "dripping_obsidian_tear" => Some(Arc::new(DrippingObsidianTear)),
        "falling_obsidian_tear" => Some(Arc::new(FallingObsidianTear)),
        "landing_obsidian_tear" => Some(Arc::new(LandingObsidianTear)),
        "reverse_portal" => Some(Arc::new(ReversePortal)),
        "white_ash" => Some(Arc::new(WhiteAsh)),
        "small_flame" => Some(Arc::new(SmallFlame)),
        "snowflake" => Some(Arc::new(Snowflake)),
        "dripping_dripstone_lava" => Some(Arc::new(DrippingDripstoneLava)),
        "falling_dripstone_lava" => Some(Arc::new(FallingDripstoneLava)),
        "dripping_dripstone_water" => Some(Arc::new(DrippingDripstoneWater)),
        "falling_dripstone_water" => Some(Arc::new(FallingDripstoneWater)),
        "glow_squid_ink" => Some(Arc::new(GlowSquidInk)),
        "glow" => Some(Arc::new(Glow)),
        "wax_on" => Some(Arc::new(WaxOn)),
        "wax_off" => Some(Arc::new(WaxOff)),
        "electric_spark" => Some(Arc::new(ElectricSpark)),
        "scrape" => Some(Arc::new(Scrape)),
        "egg_crack" => Some(Arc::new(EggCrack)),
        "dust_plume" => Some(Arc::new(DustPlume)),
        "trial_spawner_detection" => Some(Arc::new(TrialSpawnerDetection)),
        "trial_spawner_detection_ominous" => Some(Arc::new(TrialSpawnerDetectionOminous)),
        "vault_connection" => Some(Arc::new(VaultConnection)),
        "ominous_spawning" => Some(Arc::new(OminousSpawning)),
        "raid_omen" => Some(Arc::new(RaidOmen)),
        "trial_omen" => Some(Arc::new(TrialOmen)),
        "firefly" => Some(Arc::new(Firefly)),
        _ => None,
    }
}
