watch:
    cargo watch -x "run" -w src -w Cargo.toml -w templates -w assets -w config.json -w content

test:
    cargo watch -x "test" -w src