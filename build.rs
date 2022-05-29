use glob::glob;
use prost_build::Config;

fn main() {
    let proto_paths = glob("src/main/proto/**/*.proto")
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    Config::new()
        .compile_well_known_types()
        .out_dir("src/main/rust/protobuf")
        .include_file("mod.rs")
        .compile_protos(&proto_paths, &["src/main/proto/"])
        .expect("Expected successful protobuf codegen");
}
