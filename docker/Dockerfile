FROM rust:1.80.0 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin app

FROM debian:bookworm-slim AS runtime
RUN adduser --group --system app \
    && mkdir /data \
    && chown -R app:app /data
COPY --from=builder /app/target/release/app /usr/local/bin/
USER app
EXPOSE 3000
CMD ["/usr/local/bin/app"]
