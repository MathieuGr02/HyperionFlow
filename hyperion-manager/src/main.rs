use std::fs;

use hyperion_manager::{
    config::components::ComponentsConfig,
    runtime::engine::{Engine, EngineError},
    tracing::init_tracing,
};
use toml::de::Error;
use tracing::{debug, error};

fn main() -> Result<(), EngineError> {
    init_tracing();

    debug!("Initializing engine");
    let mut engine = Engine::init();

    let t = fs::read_to_string("config.toml").unwrap();
    let res: Result<ComponentsConfig, Error> = toml::from_str(&t);
    if let Ok(config) = res {
        config
            .pipelines
            .into_iter()
            .try_for_each(|p| engine.setup_pipeline(p))?;

        loop {}
    } else if let Err(e) = res {
        error!(error = %e, "Error parsing TOML configuration");
    }

    Ok(())
}
