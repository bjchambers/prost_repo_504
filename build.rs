use std::env;
use std::path::PathBuf;

fn main() {
    let proto_files = &["proto/repro.proto"];
    println!("cargo:rerun-if-changed=build.rs");
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file)
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let derive_serde = "#[derive(::serde::Serialize, ::serde::Deserialize)]";
    let serde_flatten = "#[serde(flatten)]";

    prost_build::Config::new()
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .type_attribute("repro.Aaa", derive_serde)
        .type_attribute("repro.Aaa.bbb", derive_serde)
        .field_attribute("repro.Aaa.bbb", serde_flatten)
        .compile_protos(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
