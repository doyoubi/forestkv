use std::fs;
use std::path::Path;
use std::process::Command;
use std::env::{var, set_current_dir};

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.status()
            .unwrap()
            .success());
}

fn main() {
    let cargo_manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = var("OUT_DIR").unwrap();
    let root_dir = Path::new(&cargo_manifest_dir);
    let out_dir = Path::new(&out_dir);
    let build_dir = out_dir.join("build");
    fs::create_dir_all(&build_dir).unwrap();
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).unwrap();

    set_current_dir(&build_dir).unwrap();
    run(Command::new("cmake").arg(root_dir.join("forestdb")));
    run(Command::new("make").arg("all"));

    println!("Successfully build forestdb");
}
