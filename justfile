watch:
    cargo watch -x "run b" -w src -w templates -w Cargo.toml -w ./content -w assets/css/styles.css

test:
    cargo watch -x "test" -w src

check:
    cargo watch -x "check" -w src -w templates -w Cargo.toml

watch-release:
    cargo watch -x "run --release b" -w src -w templates -w Cargo.toml -w ./content -w assets/css/styles.css

fmt:
    cargo clippy -- -D warnings
    cargo fmt --all -- --check
