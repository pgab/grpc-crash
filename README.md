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
* tremendous RAM use
* an error message and backtrace
```bash
[2020-02-05T21:41:32Z ERROR log_panics] thread 'grpc-server-loop' panicked at 'refresh_buffer must not be called on CodedOutputStream create from slice': /home/paul/.cargo/registry/src/github.com-1ecc6299db9ec823/protobuf-2.6.2/src/stream.rs:1039
stack backtrace:
   0: log_panics::init::{{closure}}
   1: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
   2: std::panicking::begin_panic
   3: protobuf::stream::CodedOutputStream::refresh_buffer
   4: protobuf::stream::CodedOutputStream::write_raw_bytes
   5: protobuf::stream::CodedOutputStream::write_float
   6: protobuf::core::Message::write_to_bytes
   7: <grpc::protobuf::MarshallerProtobuf as grpc::marshall::Marshaller<M>>::write
   8: <futures::stream::and_then::AndThen<S,F,U> as futures::stream::Stream>::poll
   9: <alloc::boxed::Box<S> as futures::stream::Stream>::poll
  10: <futures::stream::map::Map<S,F> as futures::stream::Stream>::poll
  11: <alloc::boxed::Box<S> as futures::stream::Stream>::poll
  12: <futures::stream::then::Then<S,F,U> as futures::stream::Stream>::poll
  13: <futures::stream::map_err::MapErr<S,F> as futures::stream::Stream>::poll
  14: <futures::stream::chain::Chain<S1,S2> as futures::stream::Stream>::poll
  15: <futures::stream::map::Map<S,F> as futures::stream::Stream>::poll
  16: <futures::stream::chain::Chain<S1,S2> as futures::stream::Stream>::poll
  17: <futures::future::flatten_stream::FlattenStream<F> as futures::stream::Stream>::poll
  18: <alloc::boxed::Box<S> as futures::stream::Stream>::poll
  19: std::panicking::try::do_call
  20: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  21: <futures::stream::then::Then<S,F,U> as futures::stream::Stream>::poll
  22: <httpbis::data_or_headers_with_flag::DataOrHeadersWithFlagStream as futures::stream::Stream>::poll
  23: <httpbis::common::pump_stream_to_write_loop::PumpStreamToWrite<T> as futures::future::Future>::poll
  24: <futures::future::lazy::Lazy<F,R> as futures::future::Future>::poll
  25: <futures::future::map_err::MapErr<A,F> as futures::future::Future>::poll
  26: futures::task_impl::std::set
  27: tokio_current_thread::CurrentRunner::set_spawn
  28: tokio_current_thread::scheduler::Scheduler<U>::tick
  29: tokio_current_thread::Entered<P>::turn
  30: scoped_tls::ScopedKey<T>::set
  31: std::thread::local::LocalKey<T>::with
  32: std::thread::local::LocalKey<T>::with
  33: std::thread::local::LocalKey<T>::with
  34: tokio_core::reactor::Core::poll
  35: tokio_core::reactor::Core::run
  36: std::sys_common::backtrace::__rust_begin_short_backtrace
  37: std::panicking::try::do_call
  38: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:81
  39: core::ops::function::FnOnce::call_once{{vtable.shim}}
  40: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5/src/liballoc/boxed.rs:942
  41: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/1bd30ce2aac40c7698aa4a1b9520aa649ff2d1c5/src/liballoc/boxed.rs:942
      std::sys_common::thread::start_thread
             at src/libstd/sys_common/thread.rs:13
      std::sys::unix::thread::Thread::new::thread_start
             at src/libstd/sys/unix/thread.rs:79
  42: start_thread
  43: __clone
```

On the client side you will only see the following:
```bash
Err(Error { kind: Internal, message: "http error: Encountered HTTP named error" })
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
