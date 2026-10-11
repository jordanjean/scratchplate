FROM rust:1.90-bookworm AS chef
WORKDIR /app
RUN cargo install --locked cargo-chef --version 0.1.72

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
RUN cargo build --locked --release

FROM builder AS test
RUN cargo test --locked

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/scratchplate-app-rust /app/scratchplate-app-rust
EXPOSE 8080 8081
USER 10001:10001
ENTRYPOINT ["/app/scratchplate-app-rust"]
