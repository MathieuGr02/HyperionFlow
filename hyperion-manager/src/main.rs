use std::fs;

use hyperion_manager::{components::components::ComponentsConfig, engine::Engine};

fn main() -> Result<(), ()> {
    let mut engine = Engine::init();

    let t = fs::read_to_string("config.toml").unwrap();
    let config: ComponentsConfig = toml::from_str(&t).unwrap();

    config
        .sources
        .into_iter()
        .for_each(|s| engine.setup_source(s).unwrap());

    Ok(())
}
