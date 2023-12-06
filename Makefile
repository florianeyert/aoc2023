all:
	cargo build --release && ./target/release/aoc2023 all

%:
	cargo build --release && ./target/release/aoc2023 $@ 1 && ./target/release/aoc2023 $@ 2