use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("./bin/libt4usb.a", out_dir.join("libt4usb.a")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
}
