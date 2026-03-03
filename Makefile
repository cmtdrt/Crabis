build:
	cargo build

run:
	cargo run

run-aof:
	cargo run -- AOF

start: build run

start-aof: build run-aof