extern crate napi_build;

use std::env;
use std::path::PathBuf;

fn main() {
    napi_build::setup();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dir = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("failed to resolve workspace root");
    let dll_src = workspace_dir
        .join("crates")
        .join("tapsdk-pc-sys")
        .join("sdk")
        .join("taptap_api.dll");
    let dll_dest = manifest_dir.join("taptap_api.dll");

    println!("cargo:rerun-if-changed={}", dll_src.display());

    if dll_src.exists() {
        std::fs::copy(&dll_src, &dll_dest).expect("failed to copy taptap_api.dll");
    }
}
