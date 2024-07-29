dev:
  cargo watch -x run

clean:
    rm -rf target/

lint:
  cargo fmt && cargo clippy
