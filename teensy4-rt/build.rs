static C_SRCS: &[&str] = &["./src/fault.c", "./src/ivt.c", "./src/reset.c"];

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// The C compiler
static CC: &str = "arm-none-eabi-gcc";
/// The archiver
static AR: &str = "arm-none-eabi-gcc-ar";
/// Compiler flags
static CFLAGS: &[&str] = &[
    "-c",
    "-Wall",
    "-MMD",
    "-g",
    "-O2",
    "-ffunction-sections",
    "-fdata-sections",
    "-mcpu=cortex-m7",
    "-mthumb",
    "-mfloat-abi=hard",
    "-mfpu=fpv5-d16",
    "-std=gnu11",
];

/// Files to watch for changes
static RERUN_IF_CHANGED: &[&str] = &["build.rs", "link.x"];

fn main() {
    for &rerun_if_changed in RERUN_IF_CHANGED.iter().chain(C_SRCS.iter()) {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let link_x = include_bytes!("link.x");
    let mut script = File::create(out_dir.join("link.x")).unwrap();
    script.write_all(link_x).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    let mut builder = cc::Build::new();
    builder.compiler(CC);
    builder.archiver(AR);
    builder.no_default_flags(true);
    builder.files(C_SRCS);
    for &flag in CFLAGS.iter() {
        builder.flag(flag);
    }
    builder.compile("boot");
}
