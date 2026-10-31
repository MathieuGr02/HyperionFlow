FROM rust:1.99

WORKDIR /hyperion

COPY hyperion-core ./hyperion-core
COPY hyperion-kafka ./hyperion-kafka
COPY hyperion-manager ./hyperion-manager
COPY hyperion-sql ./hyperion-sql
COPY Cargo.toml .
COPY Cargo.toml .

RUN cargo build -p hyperion-manager --features all

ENV CONFIG_FILE=""

CMD ["/bin/bash", "-c", "exec ./target/debug/hyperion-manager --config-file ${CONFIG_FILE}"]
