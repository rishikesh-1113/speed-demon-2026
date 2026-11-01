# Measuring performance

Run the following from WSL Ubuntu in this directory:

```bash
cargo build --release
mkdir -p /tmp/out
OUT=/tmp/out/summary.txt /usr/bin/time -v ./target/release/ingest /path/to/requests.log 2> /tmp/time.txt
grep -E "Elapsed|Maximum resident" /tmp/time.txt
sha256sum /tmp/out/summary.txt
```

For repeated runs, install `hyperfine` and run:

```bash
hyperfine --warmup 1 'OUT=/tmp/out/summary.txt ./target/release/ingest /path/to/requests.log'
```

To build and run with Docker, from this directory:

```bash
docker build --network=none -t ingest .
docker run --rm --cpuset-cpus=0-11 --memory=8g --network=none --read-only \
  --tmpfs /tmp:rw,size=1g -v /path/to/data:/data:ro -v /tmp/out:/out ingest
```

The Docker entrypoint expects the input at `/data/requests.log` and creates
`/out/summary.txt`.

## Local baseline (WSL Ubuntu)

Measured against the public full seed-1 dataset on this development machine:

| Implementation | Wall time | Peak RSS |
| --- | ---: | ---: |
| Rust release build | 0.98 s | 275,264 kB |
| Python reference | 31.32 s | 217,956 kB |

The Rust output matched the reference byte-for-byte, including when run in the
pinned Docker image. Output SHA-256:
`495ca09682b9673ef836a506496177680bb4a3af54255174befddb1f83481cd3`.
Dataset SHA-256:
`7ec80ba45bceef51ac7f79d3a14521fc6187b2dcb87b5e5542d08063fe5d962a`.

These are single-run local measurements, not the scoring-host five-run
aggregate. The local Rust peak RSS is higher than the Python reference.
