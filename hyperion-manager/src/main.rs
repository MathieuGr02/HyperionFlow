use std::fs;

use hyperion_manager::{config::Config, engine::Engine};

fn main() -> Result<(), ()> {
    let mut engine = Engine::init();

    let t = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&t).unwrap();

    config
        .sources
        .into_iter()
        .for_each(|s| engine.setup_source(s).unwrap());

    Ok(())
}
