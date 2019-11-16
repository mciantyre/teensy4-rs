use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Related C sources
static C_SRCS: &[&str] = &["src/cache.c", "src/fault.c", "src/ivt.c"];

/// Files to watch for changes
static RERUN_IF_CHANGED: &[&str] = &["build.rs", "memory.x"];

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
];

fn main() {
    for &rerun_if_changed in RERUN_IF_CHANGED.iter().chain(C_SRCS.iter()) {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    // cc-rs configurations are passed in via environment
    // variables.
    env::set_var("CC", CC);
    env::set_var("AR", AR);
    env::set_var("CFLAGS", CFLAGS.join(" "));
    // Important! Otherwise, cc will specify incorrect flags
    // that may invalidate ours...
    env::set_var("CRATE_CC_NO_DEFAULTS", "1");

    let mut builder = cc::Build::new();
    for c_src in C_SRCS {
        builder.file(c_src);
    }
    builder.compile("boot");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_x = include_bytes!("memory.x");
    let mut script = File::create(out_dir.join("memory.x")).unwrap();
    script.write_all(memory_x).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
}
