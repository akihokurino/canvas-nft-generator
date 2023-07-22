use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("target dir = {:?}", out_dir);
    tonic_build::configure()
        .build_server(false)
        .format(false)
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .compile(&["proto/api.proto"], &["proto"])
        .unwrap();
    Ok(())
}
