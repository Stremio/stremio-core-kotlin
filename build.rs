use glob::glob;

fn main() {
    // let proto_paths = glob("src/main/proto/**/*.proto").unwrap()
    //     .filter_map(Result::ok)
    //     .collect::<Vec<_>>();
    // prost_build::compile_protos(&proto_paths, &["src/main/proto/"])
    //     .expect("Protobuf files generated");
    prost_build::compile_protos(&["src/main/proto/stremio/core/types/video.proto"], &["src/main/proto/"]).expect("Expect proto file");
}