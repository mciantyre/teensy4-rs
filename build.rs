use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    if env::var("CARGO_FEATURE_USB_LOGGING").is_ok() {
        fs::copy("./bin/libt4usb.a", out_dir.join("libt4usb.a")).unwrap();
    }

    let link_x = include_bytes!("t4link.x");
    let mut script = File::create(out_dir.join("t4link.x")).unwrap();
    script.write_all(link_x).unwrap();
}
