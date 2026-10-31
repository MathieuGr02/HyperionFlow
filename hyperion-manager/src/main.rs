use std::fs;

use clap::Parser;
use hyperion_manager::{
    config::{args::Args, components::ComponentsConfig},
    runtime::engine::{Engine, EngineError},
    tracing::init_tracing,
};
use toml::de::Error;
use tracing::{debug, error, info};

fn main() -> Result<(), EngineError> {
    let args = Args::parse();

    init_tracing();

    debug!("Initializing engine");
    let mut engine = Engine::init();

    info!("Loading configuration file {}", args.config_file);

    let t = fs::read_to_string(args.config_file).expect("Unable to find config file");
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
