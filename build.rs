use glob::glob;

fn main() {
    std::env::set_var("OUT_DIR", "./src/rust/generated");
    for path in glob("src/main/proto/**/*.proto").unwrap().filter_map(Result::ok) {
        println!("Generating {}", path.display());
        prost_build::compile_protos(&[path], &["src/main/proto"]).expect("Expect proto file");
    }
}