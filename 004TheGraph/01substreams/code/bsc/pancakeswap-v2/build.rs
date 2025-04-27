fn main() {
    prost_build::compile_protos(&["proto/pancake.proto"], &["proto/"]).unwrap();
}