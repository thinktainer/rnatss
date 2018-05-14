extern crate protoc_rust;
use std::io::prelude::*;
use std::fs::File;

fn gen_protos() {
    let root = env!("CARGO_MANIFEST_DIR");
    let proto_path = &format!("{}/lib", root);
    let includes = &[
        proto_path.as_str(),
    ];
    let protos: &[&str] = &[
        &format!("{}/go-nats-streaming/pb/protocol.proto", proto_path),
    ];
    let out_path = "src/protos";
    std::fs::create_dir_all(out_path).expect("create_dir_all");
    {
        let mut mod_wr = File::create(&format!("{}/mod.rs", out_path)).expect("create");
        {
            write!(mod_wr, "pub mod pb;\n").expect("write");
        }
    }
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &protos,
        includes: includes,
        customize: protoc_rust::Customize {
            ..Default::default()
        },
        //plugin: Some(&proto_plugin_path),
    }).expect("protoc");
}

fn main() {
    gen_protos()
}
