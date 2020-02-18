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

Note: Put your CPU into "performance" mode:
```bash
# find the available modes
$ cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
performance powersave
# find the current mode
$ cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
powersave
# set to performance mode
$ echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
performance
# check that nothing resets to powersave
$ watch -n 1 cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
```

Test Results
============

CPU | RAM | Disk | OS | Kernel | Connection Type | Timing
--- | --- | --- | --- | --- | --- | ---
i5-3320M CPU @ 2.60GHz | 16 GiB | SSD | Ubuntu 18.04 | 4.15.0-76-generic #86-Ubuntu SMP | local/loopback | 17.903s
AMD Ryzen 7 2700 | 16 GiB | SSD | Ubuntu 18.04 |  4.15.0-76-generic #86-Ubuntu SMP | local/loopback | 12.608s
i7-8700B CPU @ 3.20GHz | 16 GiB | SSD | macOS 10.14.6 | XNU | local/loopback | 6.869s