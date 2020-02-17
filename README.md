# Benchmark GRPC

The benchmark service.

This repo provides:

- Server
- Client

**Status**: Ready for streaming benchmarks with grpc.

Just run 
```bash
cd server && cargo run --release -- --config config.toml
```
and
```bash
cd client && cargo run --release
```
you are going to yield on serverside:
* a low RAM use.

On the client side you will only see the following:
```bash
This took 23.196666541s for 5120 MiB
```

Example `config.toml`:
```toml
[server]
# ip address the server listens on.
# optional (default = "::")
# value range: valid ip4 or ip6 address
ip = "::"
# port the server listens on.
# optional (default = 50066)
# value range: 0-65536
port = 50066
```

Test Results
============

CPU | RAM | Disk | OS | Kernel | Data size | Timing
--- | --- | --- | --- | --- | --- | ---
i5-3320M CPU @ 2.60GHz | 16 GiB | SSD | Ubuntu 18.04 | 4.15.0-76-generic #86-Ubuntu SMP | 5,120 MiB | 23.196666541s
i5-3320M CPU @ 2.60GHz | 16 GiB | SSD | Ubuntu 18.04 |  4.15.0-76-generic #86-Ubuntu SMP | 10,240 MiB | 60.057229252s
