use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// Files to watch for changes
static RERUN_IF_CHANGED: &[&str] = &["build.rs", "link.x"];

fn main() {
    for &rerun_if_changed in RERUN_IF_CHANGED.iter() {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let link_x = include_bytes!("link.x");
    let mut script = File::create(out_dir.join("link.x")).unwrap();
    script.write_all(link_x).unwrap();

    fs::copy("./bin/libt4boot.a", out_dir.join("libt4boot.a")).unwrap();
    println!("cargo:rustc-link-lib=static=t4boot");
    println!("cargo:rustc-link-search={}", out_dir.display());
}
