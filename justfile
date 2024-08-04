webserver:
    cargo watch -x "run -p webserver" -w webserver -w Cargo.toml

test:
    cargo watch -x "test" -w src

check:
    cargo watch -x "check" -w cli -w shared -w webserver -w Cargo.toml
