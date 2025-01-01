watch:
    cargo watch -x "run b" -w src -w templates -w Cargo.toml -w ./content

test:
    cargo watch -x "test" -w src

check:
    cargo watch -x "check" -w src -w templates -w Cargo.toml

watch-release:
    cargo watch -x "run --release b" -w src -w templates -w Cargo.toml -w ./content
