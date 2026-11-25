# Hyperion building stage
FROM rust:1.99 AS builder

WORKDIR /hyperion

# Main hyperion crates
COPY Cargo.toml .
COPY hyperion-core ./hyperion-core
COPY hyperion-value ./hyperion-value
COPY hyperion-lang ./hyperion-lang
COPY hyperion-manager ./hyperion-manager

# Event crates
COPY hyperion-kafka ./hyperion-kafka

# Store crates
COPY hyperion-sql ./hyperion-sql
COPY hyperion-mql ./hyperion-mql
COPY hyperion-cypher ./hyperion-cypher

# Hyperion features. Features are seperated by commas (FEATURES=...,...,...>)
ARG FEATURES=all

RUN cargo build --release -p hyperion-manager --features ${FEATURES}

# Hyperion running stage
FROM builder AS runner
COPY --from=builder /hyperion/target/release/hyperion-manager /hyperion-manager

ENV CONFIG_FILE=""
CMD ["/bin/bash", "-c", "exec /hyperion-manager --config-file ${CONFIG_FILE}"]
