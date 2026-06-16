use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod advancement;
mod attributes;
mod biome;
mod bitsets;
mod block;
mod carver;
pub mod chest_loot;
mod chunk_gen_settings;
mod chunk_status;
mod composter_increase_chance;
mod configured_feature;
mod damage_type;
mod data_component;
mod dimension;
mod effect;
mod enchantments;
mod entity_pose;
mod entity_status;
mod entity_type;
mod flower_pot_transformations;
mod fluid;
mod fuels;
mod game_event;
mod game_rules;
mod item;
mod jukebox_song;
pub mod loot;
mod message_type;
mod meta_data_type;
mod noise_parameter;
mod noise_router;
mod packet;
mod particle;
mod placed_feature;
mod potion;
mod potion_brewing;
mod recipe_remainder;
mod recipes;
mod registry;
mod remap;
mod scoreboard_slot;
mod screen;
mod sound;
mod sound_category;
mod spawn_egg;
mod statistic;
mod structures;
mod tag;
mod tracked_data;
mod translations;
mod version;
mod villager;
mod wit;
mod world_event;

/// Output directory where all generated Rust source files are written.
pub const GENERATED_DIR: &str = "../pumpkin-data/src/generated";
pub const NETWORK_SERIALIZATION_DIR: &str = "../pumpkin-protocol/src/java/client/play";
pub const PLUGIN_BINDINGS_DIR: &str = "../pumpkin/src/plugin/loader/wasm/wasm_host/wit/v0_1";

/// Entry point for the code generator. Runs all registered builder functions in parallel
pub fn main() {
    type BuilderFn = fn() -> TokenStream;

    fs::create_dir_all(GENERATED_DIR).expect("Failed to create output directory");

    wit::main();

    let mut build_functions: Vec<(BuilderFn, PathBuf)> = vec![
        (advancement::build, Path::new(GENERATED_DIR).join("advancement.rs")),
        (packet::build, Path::new(GENERATED_DIR).join("packet.rs")),
        (screen::build, Path::new(GENERATED_DIR).join("screen.rs")),
        (
            particle::build,
            Path::new(GENERATED_DIR).join("particle.rs"),
        ),
        (
            particle::build_serialization,
            Path::new(NETWORK_SERIALIZATION_DIR).join("particle_data.rs"),
        ),
        (
            particle::build_wit_bindings,
            Path::new(PLUGIN_BINDINGS_DIR).join("particle.rs"),
        ),
        (sound::build, Path::new(GENERATED_DIR).join("sound.rs")),
        (
            meta_data_type::build,
            Path::new(GENERATED_DIR).join("meta_data_type.rs"),
        ),
        (
            tracked_data::build,
            Path::new(GENERATED_DIR).join("tracked_data.rs"),
        ),
        (
            chunk_status::build,
            Path::new(GENERATED_DIR).join("chunk_status.rs"),
        ),
        (
            game_event::build,
            Path::new(GENERATED_DIR).join("game_event.rs"),
        ),
        (
            game_rules::build,
            Path::new(GENERATED_DIR).join("game_rules.rs"),
        ),
        (
            registry::build,
            Path::new(GENERATED_DIR).join("registry.rs"),
        ),
        (
            dimension::build,
            Path::new(GENERATED_DIR).join("dimension.rs"),
        ),
        (
            translations::build,
            Path::new(GENERATED_DIR).join("translation.rs"),
        ),
        (
            jukebox_song::build,
            Path::new(GENERATED_DIR).join("jukebox_song.rs"),
        ),
        (
            sound_category::build,
            Path::new(GENERATED_DIR).join("sound_category.rs"),
        ),
        (
            entity_pose::build,
            Path::new(GENERATED_DIR).join("entity_pose.rs"),
        ),
        (
            scoreboard_slot::build,
            Path::new(GENERATED_DIR).join("scoreboard_slot.rs"),
        ),
        (
            world_event::build,
            Path::new(GENERATED_DIR).join("world_event.rs"),
        ),
        (
            entity_type::build,
            Path::new(GENERATED_DIR).join("entity_type.rs"),
        ),
        (
            noise_parameter::build,
            Path::new(GENERATED_DIR).join("noise_parameter.rs"),
        ),
        (biome::build, Path::new(GENERATED_DIR).join("biome.rs")),
        (
            damage_type::build,
            Path::new(GENERATED_DIR).join("damage_type.rs"),
        ),
        (
            message_type::build,
            Path::new(GENERATED_DIR).join("message_type.rs"),
        ),
        (
            spawn_egg::build,
            Path::new(GENERATED_DIR).join("spawn_egg.rs"),
        ),
        (block::build, Path::new(GENERATED_DIR).join("block.rs")),
        (item::build, Path::new(GENERATED_DIR).join("item.rs")),
        (
            structures::build,
            Path::new(GENERATED_DIR).join("structures.rs"),
        ),
        (
            chunk_gen_settings::build,
            Path::new(GENERATED_DIR).join("chunk_gen_settings.rs"),
        ),
        (fluid::build, Path::new(GENERATED_DIR).join("fluid.rs")),
        (
            entity_status::build,
            Path::new(GENERATED_DIR).join("entity_status.rs"),
        ),
        (tag::build, Path::new(GENERATED_DIR).join("tag.rs")),
        (
            noise_router::build,
            Path::new(GENERATED_DIR).join("noise_router.rs"),
        ),
        (
            villager::build,
            Path::new(GENERATED_DIR).join("villager.rs"),
        ),
        (
            flower_pot_transformations::build,
            Path::new(GENERATED_DIR).join("flower_pot_transformations.rs"),
        ),
        (
            composter_increase_chance::build,
            Path::new(GENERATED_DIR).join("composter_increase_chance.rs"),
        ),
        (recipes::build, Path::new(GENERATED_DIR).join("recipes.rs")),
        (
            enchantments::build,
            Path::new(GENERATED_DIR).join("enchantment.rs"),
        ),
        (fuels::build, Path::new(GENERATED_DIR).join("fuels.rs")),
        (
            data_component::build,
            Path::new(GENERATED_DIR).join("data_component.rs"),
        ),
        (
            attributes::build,
            Path::new(GENERATED_DIR).join("attributes.rs"),
        ),
        (effect::build, Path::new(GENERATED_DIR).join("effect.rs")),
        (potion::build, Path::new(GENERATED_DIR).join("potion.rs")),
        (
            potion_brewing::build,
            Path::new(GENERATED_DIR).join("potion_brewing.rs"),
        ),
        (
            recipe_remainder::build,
            Path::new(GENERATED_DIR).join("recipe_remainder.rs"),
        ),
        (
            placed_feature::build_enum,
            Path::new(GENERATED_DIR).join("placed_feature.rs"),
        ),
        (
            placed_feature::build,
            Path::new(GENERATED_DIR).join("placed_features_generated.rs"),
        ),
        (
            configured_feature::build_enum,
            Path::new(GENERATED_DIR).join("configured_feature.rs"),
        ),
        (
            configured_feature::build,
            Path::new(GENERATED_DIR).join("configured_features_generated.rs"),
        ),
        (carver::build, Path::new(GENERATED_DIR).join("carver.rs")),
        (
            chest_loot::build,
            Path::new(GENERATED_DIR).join("chest_loot.rs"),
        ),
    ];
    build_functions.extend(remap::build());

    // If any arguments are given, treat them as file-stem filters.
    // e.g. `cargo run -- chest_loot` only regenerates chest_loot.rs.
    let filters: Vec<String> = std::env::args().skip(1).collect();
    let build_functions: Vec<_> = if filters.is_empty() {
        build_functions
    } else {
        build_functions
            .into_iter()
            .filter(|(_, file)| {
                let stem = file.file_stem().and_then(|s| s.to_str());

                let full_path_str = file.to_str();

                filters
                    .iter()
                    .any(|f| Some(f.as_str()) == stem || Some(f.as_str()) == full_path_str)
            })
            .collect()
    };

    build_functions.par_iter().for_each(|(build_fn, file)| {
        println!("Parsing {:#?}", file.file_name().unwrap());

        output_code(build_fn(), file);
    });
    println!("Done")
}

/// Writes the generated code with a warning and proper format
///
/// # Arguments
/// - `token_stream` - The code to write.
/// - `file` - The file to write to.
pub fn output_code(token_stream: TokenStream, file: &PathBuf) {
    let raw_code = token_stream.to_string();

    let header = "/* This file is generated. Do not edit manually. */\n";

    let final_code = format_code(&raw_code).map_or_else(
        |_| format!("{header}{raw_code}"),
        |formatted| format!("{header}{formatted}"),
    );

    write_generated_file(&final_code, file);
}

/// Converts a slice of strings into a `TokenStream` of PascalCase enum variants.
///
/// # Arguments
/// - `array` – Slice of raw name strings to convert into variant identifiers.
#[must_use]
pub fn array_to_tokenstream(array: &[String]) -> TokenStream {
    let variants = array.iter().map(|item| {
        let name = format_ident!("{}", item.to_pascal_case());
        quote! { #name, }
    });

    quote! {
        #(#variants)*
    }
}

/// Writes generated source code to a file in [`OUT_DIR`], skipping the write if the
/// content is unchanged.
///
/// # Arguments
/// - `new_code` – The formatted source code string to write.
/// - `path` – The file to write to.
pub fn write_generated_file(new_code: &str, path: &PathBuf) {
    if path.exists()
        && let Ok(existing_code) = fs::read_to_string(&path)
        && existing_code == new_code
    {
        return;
    }

    fs::write(&path, new_code)
        .unwrap_or_else(|_| panic!("Failed to write to file: {}", path.display()));
}

/// Error returned when `rustfmt` is unavailable or fails to format code.
pub struct RustFmtError;

/// Formats a Rust source string by piping it through `rustfmt`.
///
/// # Arguments
/// - `unformatted_code` – Raw Rust source code to format.
///
/// # Returns
/// The formatted source string, or `Err(RustFmtError)` if `rustfmt` is not available
/// or formatting fails.
pub fn format_code(unformatted_code: &str) -> Result<String, RustFmtError> {
    let child_result = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();

    let Ok(mut child) = child_result else {
        return Err(RustFmtError);
    };

    // Write the code to rustfmt's stdin
    if let Some(mut stdin) = child.stdin.take()
        && stdin.write_all(unformatted_code.as_bytes()).is_err()
    {
        return Err(RustFmtError);
    }

    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).map_err(|_| RustFmtError)
        }
        _ => Err(RustFmtError),
    }
}
