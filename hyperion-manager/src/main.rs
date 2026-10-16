use std::fs;

use hyperion_manager::config::{Config, RuntimeSource};

fn main() -> Result<(), ()> {
    let t = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&t).unwrap();

    let mut sources: Vec<RuntimeSource> = config
        .sources
        .into_iter()
        .map(|s| s.build_source().unwrap())
        .collect();

    sources.iter_mut().for_each(|s| s.run());

    Ok(())
}
