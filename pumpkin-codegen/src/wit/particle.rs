use heck::{ToKebabCase, ToSnakeCase};
use semver::Version;
use serde_json::Value;
use std::fs;
use wit_encoder::{
    Interface, InterfaceItem, Package, PackageName, ResourceFunc, StandaloneFunc, Type, TypeDef,
    Use,
};

pub fn build() -> String {
    let mut particles: serde_json::Map<String, Value> =
        serde_json::from_str(&fs::read_to_string("../assets/particles.json").unwrap())
            .expect("Failed to parse particles.json");

    // TODO: Handle vibration and item particles
    // These particles has some complicated data
    // For now we will skip these particle
    // They will still be a particle, just without the fields
    particles.insert("vibration".to_string(), Value::Array(vec![]));
    particles.insert("item".to_string(), Value::Array(vec![]));

    let mut package = Package::new(PackageName::new(
        "pumpkin",
        "plugin",
        Some(Version::new(0, 1, 0)),
    ));
    let mut interface = Interface::new("particles");

    let mut use_item_stack = Use::new("item-stack");
    use_item_stack.item("item-stack", None);
    interface.use_(use_item_stack);

    let mut use_position = Use::new("common");
    use_position.item("position", None);
    interface.use_(use_position);

    let mut use_position = Use::new("world");
    use_position.item("world", None);
    interface.use_(use_position);

    let offset_type = TypeDef::type_("offset", Type::tuple(vec![Type::F32, Type::F32, Type::F32]));
    interface.type_def(offset_type);

    for (particle_name, fields) in particles {
        let fields = fields.as_array().unwrap();
        let mut spawn_function =
            StandaloneFunc::new(format!("spawn-{}-particle", particle_name.to_kebab_case()));

        spawn_function
            .params_mut()
            .push("world", Type::borrow("%world"));
        spawn_function
            .params_mut()
            .push("pos", Type::named("position"));
        spawn_function
            .params_mut()
            .push("offset", Type::named("offset"));
        spawn_function.params_mut().push("max-speed", Type::F32);
        spawn_function
            .params_mut()
            .push("particle-count", Type::S32);
        if !fields.is_empty() {
            spawn_function
                .params_mut()
                .push("particle", Type::named(particle_name.to_kebab_case()));
        }
        spawn_function.set_docs(Some(format!(
            "Spawns `{}` particles at the specified position.",
            particle_name.to_snake_case()
        )));

        interface.function(spawn_function);

        // There is no need to declare a particle, if it has no fields
        // We will directly generate the spawn particles function for these particle types
        if !fields.is_empty() {
            let mut constructor = ResourceFunc::constructor();

            for field in fields {
                let field = field.as_str().unwrap();
                constructor
                    .params_mut()
                    .push(field.to_kebab_case(), match_field_type_helper(&field));
            }

            interface.item(InterfaceItem::TypeDef(TypeDef::resource(
                particle_name.to_kebab_case(),
                vec![constructor],
            )));
        }
    }

    package.interface(interface);
    package.to_string()
}

fn match_field_type_helper(field_name: &str) -> Type {
    match field_name {
        "state" | "color" | "fromColor" | "toColor" | "duration" | "delay" => Type::S32,
        "power" | "scale" | "roll" => Type::F32,
        "target" => Type::named("position"),
        _ => unreachable!("Unknown field name: {field_name}"),
    }
}
