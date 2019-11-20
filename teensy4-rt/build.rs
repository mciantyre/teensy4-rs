use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// Files to watch for changes
static RERUN_IF_CHANGED: &[&str] = &["build.rs", "memory.x"];

static MISSING_LIBBOOT: &str = "Unable to find boot/boot.a! Run 'make' inside the boot directory";

fn main() {
    for &rerun_if_changed in RERUN_IF_CHANGED.iter() {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::copy("../boot/boot.a", out_dir.join("libboot.a")).expect(MISSING_LIBBOOT);
    println!("cargo:rustc-link-lib=static=boot");

    let memory_x = include_bytes!("memory.x");
    let mut script = File::create(out_dir.join("memory.x")).unwrap();
    script.write_all(memory_x).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
}
