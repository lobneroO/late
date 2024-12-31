// build script
// right now only needs to copy the resources folder

use std::path::Path;
use std::env;
use std::process::Command;
use std::path::PathBuf;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    path
}

fn main() {
    // there's probably a nicer way than rebuilding when the icon changes
    // but realistically this won't happen for some time and even then be rare
    println!("cargo::rerun-if-changed=resources/late.ico");
    println!("cargo::rerun-if-changed=build.rs");

    // actually copy the resources folder
    let out_dir = get_output_path();
    Command::new("cp")
        .arg("-r")
        .arg("resources")
        .arg(&out_dir)
        .spawn()
        .expect("failed to spawn copy process");
}

