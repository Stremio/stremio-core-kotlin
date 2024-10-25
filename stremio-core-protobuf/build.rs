use std::{fs::DirBuilder, path::PathBuf};

use glob::glob;
use prost_build::Config;

pub const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
fn main() {
    let proto_dir = PathBuf::from(CARGO_MANIFEST_DIR).join("proto");

    let proto_paths = glob(format!("{}/**/*.proto", proto_dir.display()).as_str())
        .unwrap()
        .filter_map(|result| {
            if let Err(err) = &result {
                eprintln!("Glob error: {err}");
            }

            result.ok()
        })
        .collect::<Vec<_>>();

    for path in &proto_paths {
        let display_path = path.display();
        println!("cargo:rerun-if-changed={display_path}");
    }
    let file_descriptors =
        protox::compile(proto_paths, [proto_dir]).expect("Expected file descriptors");

    let protobuf_gen_dir = PathBuf::from(CARGO_MANIFEST_DIR)
        .join("src")
        .join("protobuf");
    // create protobuf folder if it doesn't exist
    DirBuilder::new()
        .recursive(true)
        .create(protobuf_gen_dir.clone())
        .expect("Should create src/protobuf dir if it does not exist.");

    Config::new()
        .compile_well_known_types()
        .out_dir(protobuf_gen_dir.display().to_string())
        .include_file("mod.rs")
        .compile_fds(file_descriptors)
        .expect("Expected successful protobuf codegen");
}
