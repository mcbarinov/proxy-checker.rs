set dotenv-load
project_name := `grep APP_NAME .env | cut -d '=' -f 2-`
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
docker_build_platform := env_var_or_default("DOCKER_BUILD_PLATFORM", "linux/amd64")
docker_image := "proxy-checker-rs"

dev:
  cargo watch -x run

clean:
  rm -rf target/

lint:
  cargo fmt && cargo clippy

lint-fix:
  cargo fmt && cargo clippy --allow-dirty --allow-staged --fix --lib -p proxy-checker

migrate:
  sqlx migrate run

reset-db:
  sqlx database reset

sqlx-offline:
    cargo sqlx prepare -- --all-targets --all-features

docker-lint:
    hadolint --ignore DL3008 docker/Dockerfile

docker-build: docker-lint
  docker buildx build --platform {{docker_build_platform}} -t {{docker_image}}:{{version}} -f docker/Dockerfile .
  docker tag {{project_name}}:{{version}} {{project_name}}:latest

docker-compose:
  docker compose --file docker/docker-compose.yml up --build
