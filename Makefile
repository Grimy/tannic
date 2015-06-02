target/release/scar: src/** Cargo.toml
	cargo build --release

stat: target/release/scar
	cargo build --release
	perf stat $^ >/dev/null

flame: target/release/scar
	cargo build --release
	perf record -F 99 -g -- $^ >/dev/null
	perf script | stackcollapse-perf.pl >perf-data.folded
	flamegraph.pl perf-data.folded >perf-data.svg

perf: target/release/scar
	cargo build --release
	perf record $^ >/dev/null
	perf report
