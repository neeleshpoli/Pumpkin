use std::fs;

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;

/// Generates the `TokenStream` for the Particles
pub fn build() -> TokenStream {
    let mut particles: serde_json::Map<String, Value> =
        serde_json::from_str(&fs::read_to_string("../assets/particles.json").unwrap())
            .expect("Failed to parse particles.json");

    // TODO: Handle vibration and item particles
    // These particles has some complicated data
    // For now we will skip these particle
    // They will still be a particle, just without the fields
    particles.insert("vibration".to_string(), Value::Array(vec![]));
    particles.insert("item".to_string(), Value::Array(vec![]));

    let structs = particles
        .iter()
        .map(|(name, fields)| {
            let fields = fields.as_array().unwrap();
            let upper_name_str = name.to_upper_camel_case();
            let upper_name = format_ident!("{}", upper_name_str);

            let struct_fields = fields.iter().map(|f| {
                let f = f.as_str().unwrap();

                let field_name = format_ident!("{}", f.to_snake_case());
                let field_type = match_field_type_helper(f);
                quote! { pub #field_name: #field_type }
            });

            let new_params = fields.iter().map(|f| {
                let f = f.as_str().unwrap();

                let field_name = format_ident!("{}", f.to_snake_case());
                let field_type = match_field_type_helper(f);
                quote! { #field_name: #field_type }
            });

            let new_inits = fields.iter().map(|f| {
                let f = f.as_str().unwrap();

                let field = format_ident!("{}", f.to_snake_case());
                quote! { #field }
            });

            if fields.is_empty() {
                quote! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct #upper_name;
                }
            } else {
                quote! {
                    #[derive(Clone, Copy, Debug)]
                    pub struct #upper_name {
                        #( #struct_fields ),*
                    }

                    impl #upper_name {
                        pub fn new(#( #new_params ),*) -> Self {
                            Self {
                                #( #new_inits ),*
                            }
                        }
                    }
                }
            }
        })
        .collect::<TokenStream>();

    quote! {
        use pumpkin_util::math::vector3::Vector3;

        #structs
    }
}

fn match_field_type_helper(field_name: &str) -> TokenStream {
    match field_name {
        "state" | "color" | "fromColor" | "toColor" | "duration" | "delay" => quote! { i32 },
        "power" | "scale" | "roll" => quote! { f32 },
        "target" => quote! { Vector3<f64> },
        _ => unreachable!("Unknown field name: {field_name}"),
    }
}

// Particle is special, because with the particle we need to generate the proper networking code
/// Generates the code for network serialiation for particles
pub fn build_serialization() -> TokenStream {
    let mut particles: serde_json::Map<String, Value> =
        serde_json::from_str(&fs::read_to_string("../assets/particles.json").unwrap())
            .expect("Failed to parse particles.json");

    // TODO: Handle vibration and item particles
    // These particles has some complicated data
    // For now we will skip these particle
    // They will still be a particle, just without the fields
    particles.insert("vibration".to_string(), Value::Array(vec![]));
    particles.insert("item".to_string(), Value::Array(vec![]));

    let particle_serialization = particles.iter().enumerate().map(|(id, (name, fields))| {
        let fields = fields.as_array().unwrap();
        let upper_name_str = name.to_upper_camel_case();
        let upper_name = format_ident!("{}", upper_name_str);

        let id = id as i32;
        let is_simple_particle = fields.is_empty();

        if is_simple_particle {
            quote! {
                impl SerializeParticleData for #upper_name {
                    fn id(&self) -> i32 {
                        #id
                    }
                }
            }
        } else {
            let serialization = fields
                .iter()
                .map(|f| match f.as_str().unwrap() {
                    "state" => quote! { write.write_var_int(&VarInt(self.state))?; },
                    "power" => quote! { write.write_f32_be(self.power)?; },
                    "color" => quote! { write.write_i32_be(self.color)?; },
                    "scale" => quote! { write.write_f32_be(self.scale)?; },
                    "fromColor" => quote! { write.write_i32_be(self.from_color)?; },
                    "toColor" => quote! { write.write_i32_be(self.to_color)?; },
                    "roll" => quote! { write.write_f32_be(self.roll)?; },
                    "target" => quote! {
                        write.write_f64_be(self.target.x)?;
                        write.write_f64_be(self.target.y)?;
                        write.write_f64_be(self.target.z)?;
                    },
                    "duration" => quote! { write.write_var_int(&VarInt(self.duration))?; },
                    "delay" => quote! { write.write_var_int(&VarInt(self.delay))?; },
                    _ => unreachable!("Unknown field name: {f}"),
                })
                .collect::<TokenStream>();

            quote! {
                impl SerializeParticleData for #upper_name {
                    fn id(&self) -> i32 {
                        #id
                    }

                    fn to_bytes(&self, mut write: &mut [u8]) -> Result<usize, WritingError> {
                        let initial_len = write.len();
                        #serialization
                        Ok(initial_len - write.len())
                    }
                }
            }
        }
    });

    quote! {
        use std::fmt::Debug;

        #[allow(clippy::wildcard_imports)]
        use pumpkin_data::particle::*;

        use crate::{codec::var_int::VarInt, ser::{NetworkWriteExt, WritingError}};

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

        #( #particle_serialization )*
    }
}

/// Builds the rust side of the WIT bindings code
pub fn build_wit_bindings() -> TokenStream {
    let mut particles: serde_json::Map<String, Value> =
        serde_json::from_str(&fs::read_to_string("../assets/particles.json").unwrap())
            .expect("Failed to parse particles.json");

    // TODO: Handle vibration and item particles
    // These particles has some complicated data
    // For now we will skip these particle
    // They will still be a particle, just without the fields
    particles.insert("vibration".to_string(), Value::Array(vec![]));
    particles.insert("item".to_string(), Value::Array(vec![]));

    let mut new_impls = vec![];

    let spawn_functions = particles.iter().map(|(name, fields)| {
        let fields = fields.as_array().unwrap();
        let camel_case = format_ident!("{}", name.to_upper_camel_case());
        let snake_case = format_ident!("{}", name.to_snake_case());
        let particle_type: syn::Path = syn::parse_str(&format!("pumpkin_data::particle::{}", camel_case)).unwrap();
        let spawn_function_name = format_ident!("spawn_{}_particle", snake_case);

        let mut function_params = vec![];
        let function_params_types: Vec<_> = fields.iter().map(|f| {
            let f = f.as_str().unwrap();
            let field_type = if f != "position" && f != "target" {
                match_field_type_helper(f)
            } else {
                quote!{Position}
            };
            let f = format_ident!("{}", f.to_snake_case());

            if f == "target" {
                function_params.push(quote! {#f.into()});
            } else {
                function_params.push(quote! {#f});
            }

            quote! {
                #f: #field_type
            }
        }).collect();

        if !fields.is_empty() {
            let resource_type = format_ident!("{}ParticleResource", camel_case);
            let host_impl_name = format_ident!("Host{}", camel_case);

            new_impls.push(quote! {
                pub type #resource_type = WasmResource<#particle_type>;
                impl #host_impl_name for PluginHostState {
                    async fn new(&mut self, #( #function_params_types ),*) -> wasmtime::Result<Resource<#camel_case>> {
                        let provider = #particle_type::new(#( #function_params ),*);
                        let resource = self.resource_table.push(#resource_type { provider })?;

                        Ok(Resource::new_own(resource.rep()))
                    }

                    async fn drop(&mut self, rep: Resource<#camel_case>) -> wasmtime::Result<()> {
                        self.resource_table
                            .delete::<#resource_type>(Resource::new_own(rep.rep()))
                            .map_err(wasmtime::Error::from)?;
                        Ok(())
                    }
                }
            });

            quote! {
                async fn #spawn_function_name(&mut self, world: Resource<World>, position: Position, offset: Offset, max_speed: f32, particle_count: i32, particle: Resource<#camel_case>) -> wasmtime::Result<()> {
                    let world = self.get_world_res(&world)?;

                    world.provider.spawn_particle(position.into(), offset.into(), max_speed, particle_count, self.resource_table.get::<#resource_type>(&Resource::new_own(particle.rep()))?.provider);

                    Ok(())
                }
            }
        } else {
            quote! {
                async fn #spawn_function_name(&mut self, world: Resource<World>, position: Position, offset: Offset, max_speed: f32, particle_count: i32) -> wasmtime::Result<()> {
                    let world = self.get_world_res(&world)?;

                    world.provider.spawn_particle(position.into(), offset.into(), max_speed, particle_count, #particle_type);

                    Ok(())
                }
            }
        }
    });

    quote! {
        use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::common::Position;
        #[allow(clippy::wildcard_imports)]
        use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::particles::*;
        use crate::plugin::loader::wasm::wasm_host::wit::v0_1::pumpkin::plugin::world::World;
        #[allow(clippy::wildcard_imports)]
        use crate::plugin::loader::wasm::wasm_host::state::*;
        use wasmtime::component::Resource;

        impl Host for PluginHostState {
            #( #spawn_functions )*
        }

        #( #new_impls )*
    }
}
