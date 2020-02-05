use protoc_rust_grpc;

fn main() {
    if std::env::var("SKIP_GEN").unwrap_or("0".to_string()) == "1" {
        return;
    }
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["../proto"],
        input: &["../proto/crash.proto"],
        // also generate protobuf messages, not just services
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc-rust-grpc");
    println!("cargo:rerun-if-changed=../proto/crash.proto");
}
