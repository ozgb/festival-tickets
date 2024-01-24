fn main() {
    tonic_build::compile_protos("proto/purchase.proto").unwrap();
}
