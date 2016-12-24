use std::fs;
use std::path::Path;
use std::process::Command;
use std::env::{var, set_current_dir};
use std::io::ErrorKind;


fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.status()
            .unwrap()
            .success());
}

fn main() {
    // Check dependency
    if let Err(e) = Command::new("cmake").status() {
        if let ErrorKind::NotFound = e.kind() {
            panic!("cmake not found");
        } else {
            panic!("unexpected error {:?}", e);
        }
    }

    let cargo_manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = var("OUT_DIR").unwrap();
    let root_dir = Path::new(&cargo_manifest_dir);
    let out_dir = Path::new(&out_dir);
    let build_dir = out_dir.join("build");
    fs::create_dir_all(&build_dir).unwrap();
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).unwrap();

    let profile = "RelWithDebugInfo";

    set_current_dir(&build_dir).unwrap();
    run(Command::new("cmake")
        .arg(root_dir.join("forestdb"))
        .arg(format!("-DCMAKE_BUILD_TYPE={}", profile))
        .arg(format!("-DCMAKE_LIBRARY_OUTPUT_DIRECTORY={}", lib_dir.display()))
        .arg("-DSNAPPY_OPTION=Disable")
        );

    run(Command::new("cmake")
        .arg("--build").arg(".")
        .arg("--target").arg("forestdb"));

    println!("cargo:rustc-flags=-l forestdb -L {}", lib_dir.display());
}
