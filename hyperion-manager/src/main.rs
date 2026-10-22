use std::fs;

use hyperion_manager::tracing::init_tracing;
use hyperion_manager::{components::components::ComponentsConfig, engine::Engine};
use tracing::debug;

fn main() -> Result<(), ()> {
    init_tracing();

    debug!("Initializing engine");
    let mut engine = Engine::init();

    let t = fs::read_to_string("config.toml").unwrap();
    let config: ComponentsConfig = toml::from_str(&t).unwrap();

    config
        .pipelines
        .into_iter()
        .try_for_each(|p| engine.setup_pipeline(p))?;

    loop {}

    Ok(())
}
