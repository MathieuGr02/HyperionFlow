use std::fs;

use hyperion_manager::{
    config::components::ComponentsConfig, runtime::engine::Engine, tracing::init_tracing,
};
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
