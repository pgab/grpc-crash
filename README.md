# Crash GRPC

The crash service.

This repo provides:

- Server
- Client

**Status**: Ready for crashing grpc.

Just run 
```bash
cd server && cargo run --release -- --config config.toml
```
and
```bash
cd client && cargo run --release
```
you are going to yield on serverside:
* tremendous RAM use, even more than with GRPC
* an error message and backtrace
```bash
[2020-02-05T23:58:04Z ERROR log_panics] thread 'tokio-runtime-worker' panicked at 'assertion failed: len <= std::u32::MAX as usize': /home/paul/.cargo/registry/src/github.com-1ecc6299db9ec823/tonic-0.1.1/src/codec/encode.rs:47
stack backtrace:
   0: log_panics::init::{{closure}}
   1: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
   2: std::panicking::begin_panic
   3: <std::future::GenFuture<T> as core::future::future::Future>::poll
   4: <S as futures_core::stream::TryStream>::try_poll_next
   5: <T as tonic::body::Body>::poll_data
   6: <T as hyper::body::payload::Payload>::poll_data
   7: <hyper::proto::h2::PipeToSendStream<S> as core::future::future::Future>::poll
   8: <hyper::proto::h2::server::H2Stream<F,B> as core::future::future::Future>::poll
   9: tokio::task::core::Core<T>::poll
  10: std::panicking::try::do_call
  11: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  12: tokio::task::harness::Harness<T,S>::poll
  13: tokio::runtime::thread_pool::worker::GenerationGuard::run_task
  14: tokio::runtime::thread_pool::worker::GenerationGuard::run
  15: std::thread::local::LocalKey<T>::with
  16: std::thread::local::LocalKey<T>::with
  17: tokio::runtime::thread_pool::worker::Worker::run
  18: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  19: std::panicking::try::do_call
  20: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  21: tokio::task::harness::Harness<T,S>::poll
  22: tokio::runtime::blocking::pool::Inner::run2
  23: std::thread::local::LocalKey<T>::with
  24: tokio::runtime::time::variant::with_default
  25: tokio::runtime::blocking::pool::Inner::run
  26: std::sys_common::backtrace::__rust_begin_short_backtrace
  27: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  28: core::ops::function::FnOnce::call_once{{vtable.shim}}
  29: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5/src/liballoc/boxed.rs:942
  30: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5/src/liballoc/boxed.rs:942
      std::sys_common::thread::start_thread
             at src/libstd/sys_common/thread.rs:13
      std::sys::unix::thread::Thread::new::thread_start
             at src/libstd/sys/unix/thread.rs:79
  31: start_thread
  32: __clone
```

On the client side you will only see the following:
```bash
Error: Error { kind: Internal, message: "Error { kind: Internal, message: \"grpc-status: Unknown, grpc-message: \\\"error reading a body from connection: protocol error: stream no longer needed\\\"\" }" }
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
