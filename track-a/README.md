# Track A: Ingest

This Rust implementation reads `/data/requests.log` and writes
`/out/summary.txt`. It reports one row per UTC-hour and endpoint group, sorted
by hour and then by endpoint bytes.

The task contract is the [official Track A specification](https://github.com/hackathon-raptors/speed-demon-2026/blob/main/specs/a-ingest.md).

Each row contains the request count, nearest-rank p50/p95/p99 latency in
microseconds, the number of 5xx responses, and the integer error rate in parts
per million.

## Run

Build with `cargo build --release`. By default, the program reads
`/data/requests.log` and writes `/out/summary.txt`. For local runs, pass a
different input file as the first argument and set `OUT` to the output file
path.

The pinned build and runtime image digests match the official image allowlist.

## Full-dataset verification

The public seed-1 dataset output was compared byte-for-byte with the official
Python reference. Its expected output SHA-256 is
`495ca09682b9673ef836a506496177680bb4a3af54255174befddb1f83481cd3`.
