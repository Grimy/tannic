perf:
	perf record cargo run --release >/dev/null
	perf report
