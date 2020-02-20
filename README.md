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
E5-2630 v3 @ 2.40GHz (performance[sic]) | 125 GiB | SAS | Ubuntu 18.04 | 4.15.0-72-generic #81-Ubuntu SMP | local/loopback | 35.200s
E5-2630 v3 @ 2.40GHz (powersave[sic]) | 125 GiB | SAS | Ubuntu 18.04 | 4.15.0-72-generic #81-Ubuntu SMP | local/loopback | 23.961s
E3-1240 v5 @ 3.50GHz | 15 GiB | HDD | Ubuntu 16.04.5 | 4.4.0-112-generic #135-Ubuntu SMP | local/loopback | 18.386s
i7-7500U CPU @ 2.70GHz | 16 GiB | SSD (NVM) | Ubuntu 19.10 | 5.3.0-29-generic #31-Ubuntu SMP | local/loopback | 8.686s 
i5-4278U CPU @ 2.60GHz | 8 GiB | SSD | macOS 10.15.3 | XNU | local/loopback | 16.979s