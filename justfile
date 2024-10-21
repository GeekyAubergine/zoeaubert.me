watch:
    cargo watch -x "run" -w src -w templates -w Cargo.toml -w assets -w ./content

test:
    cargo watch -x "test" -w src

check:
    cargo watch -x "check" -w src -w templates -w Cargo.toml
