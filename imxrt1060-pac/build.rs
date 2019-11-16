use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    for rerun_if_changed in &["device.x", "build.rs"] {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let device_x = include_bytes!("device.x");
    let mut script = File::create(out_dir.join("device.x")).unwrap();
    script.write_all(device_x).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
}
