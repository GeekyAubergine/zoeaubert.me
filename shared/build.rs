use std::path::{Path, PathBuf};

fn main() {
    compile_protobuf();
}

fn compile_protobuf() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let path = Path::new("./proto/zoeaubert.proto");

    // match prost_build::compile_protos(&[path], &["./proto/"]) {
    //     Ok(_) => println!("Compiled protobufs successfully"),
    //     Err(e) => {
    //         println!("Failed to compile protobufs: {}", e);
    //         std::process::exit(1);
    //     }
    // }

    match tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("zoeaubert_descriptor.bin"))
        .compile(&[path], &[path.parent().unwrap()])
    {
        Ok(_) => println!("Compiled protobufs successfully"),
        Err(e) => {
            println!("Failed to compile protobufs: {}", e);
            std::process::exit(1);
        }
    }
}
