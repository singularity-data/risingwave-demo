use std::{env, path::PathBuf};
use std::fs;

fn main() {
    let proto_file = "./actor.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    fs::copy("./src/recommender.rs", "../simulator/src/recommender.rs").unwrap();
    println!("cargo:rerun-if-changed={}", proto_file);
}
