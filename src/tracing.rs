use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    // Default to 'info', but allow overrides via RUST_LOG (e.g., RUST_LOG=debug cargo run)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Check if we want JSON logs (good for production) or pretty logs (good for local dev)
    let use_json = std::env::var("LOG_FORMAT").unwrap_or_default() == "json";

    if use_json {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}
