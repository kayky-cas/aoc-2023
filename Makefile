cd: build_create_day 
	./target/release/create_day $(DAY)

build_create_day: ./src/bin/create_day.rs 
	cargo build --release --bin create_day

test:
	cargo watch -q -x test

run:
	cargo run --bin day$(DAY)
