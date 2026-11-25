use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    // Default to 'info', but allow overrides via RUST_LOG (e.g., RUST_LOG=debug cargo run)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(
            "hyperion_core=trace,hyperion_manager=trace,hyperion_kafka=trace,hyperion_sql=trace",
        )
    });

    // Check if we want JSON logs (good for production) or pretty logs (good for local dev)
    let use_json = std::env::var("LOG_FORMAT").unwrap_or_default() == "json";

    let fmt_layer = if use_json {
        tracing_subscriber::fmt::layer()
            .json()
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .boxed()
    } else {
        tracing_subscriber::fmt::layer()
            .compact()
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .boxed()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}
