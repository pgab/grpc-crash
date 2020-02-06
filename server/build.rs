use tonic_build;

fn main() {
    if std::env::var("SKIP_GEN").unwrap_or("0".to_string()) == "1" {
        return;
    }
    tonic_build::compile_protos("../proto/crash.proto").unwrap();
    println!("cargo:rerun-if-changed=../proto/crash.proto");
}
