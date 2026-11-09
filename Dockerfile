# Binary build stage
FROM rust:1.99 AS builder

ARG FEATURES=all

WORKDIR /hyperion

COPY hyperion-core ./hyperion-core
COPY hyperion-kafka ./hyperion-kafka
COPY hyperion-manager ./hyperion-manager
COPY hyperion-sql ./hyperion-sql
COPY hyperion-mql ./hyperion-mql
COPY Cargo.toml .
COPY Cargo.toml .

RUN cargo build -p hyperion-manager --features ${FEATURES}

# Run stage
FROM builder AS runner
COPY --from=builder /hyperion/target/debug/hyperion-manager /hyperion-manager

ENV CONFIG_FILE=""

CMD ["/bin/bash", "-c", "exec /hyperion-manager --config-file ${CONFIG_FILE}"]
