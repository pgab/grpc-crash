# Crash GRPC

The crash service.

This repo provides:

- Server
- Client

**Status**: Shows high RAM use of client (7.3 GiB for a 3 GiB message) and server (6.2 GiB for a 3 GiB message).

Just run 
```bash
cd server && cargo run --release -- --config config.toml
```
and
```bash
cd client && cargo run --release
```
you are going to yield on serverside:
* tremendous RAM use
```bash
/usr/bin/time --verbose ../target/release/server --config config.toml 
^CCommand terminated by signal 2
	Command being timed: "../target/release/server --config config.toml"
	User time (seconds): 8.37
	System time (seconds): 3.55
	Percent of CPU this job got: 65%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:18.31
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 6293996
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 0
	Minor (reclaiming a frame) page faults: 2359595
	Voluntary context switches: 115070
	Involuntary context switches: 978
	Swaps: 0
	File system inputs: 0
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
```

On the client side you will only see the following:
```bash
/usr/bin/time --verbose ../target/release/client-test
3072 MiB
	Command being timed: "../target/release/client-test"
	User time (seconds): 2.81
	System time (seconds): 4.02
	Percent of CPU this job got: 43%
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:15.81
	Average shared text size (kbytes): 0
	Average unshared data size (kbytes): 0
	Average stack size (kbytes): 0
	Average total size (kbytes): 0
	Maximum resident set size (kbytes): 7344300
	Average resident set size (kbytes): 0
	Major (requiring I/O) page faults: 1
	Minor (reclaiming a frame) page faults: 1835317
	Voluntary context switches: 173080
	Involuntary context switches: 259
	Swaps: 0
	File system inputs: 48
	File system outputs: 0
	Socket messages sent: 0
	Socket messages received: 0
	Signals delivered: 0
	Page size (bytes): 4096
	Exit status: 0
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
