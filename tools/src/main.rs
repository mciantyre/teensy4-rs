//! Allows us to auto-import a iMXRT1060 module
//! from the svd2rust output into the imxrt1060-pac
//! megacrate. It code-ifies some manual work.
//!
//! This could probably use some better error handling...

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

static CARGO_TOML_DEPENDENCIES: &str = r#"bare-metal = "0.2.5"
cortex-m = "0.6.1"
vcell = "0.1.2"
"#;

static CARGO_NO_TESTS_BENCH: &str = r#"
[lib]
bench = false
test = false
"#;

fn add_deps(crate_path: &Path) {
    let mut cargo_toml = fs::OpenOptions::new()
        .append(true)
        .open(crate_path.join("Cargo.toml"))
        .expect("Cannot open Cargo.toml");

    cargo_toml
        .write_all(CARGO_TOML_DEPENDENCIES.as_bytes())
        .expect("Failed to update Cargo.toml");

    cargo_toml
        .write_all(CARGO_NO_TESTS_BENCH.as_bytes())
        .expect("Failed to update Cargo.toml");
}

static LIB_PRELUDE: &str = r#"#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

"#;

fn write_lib<R: Read>(crate_path: &Path, mut src: R) {
    let mut crate_lib =
        fs::File::create(crate_path.join("src").join("lib.rs")).expect("Unable to crate lib.rs");
    crate_lib
        .write_all(LIB_PRELUDE.as_bytes())
        .expect("Unable to write lib.rs prelude");
    io::copy(&mut src, &mut crate_lib).unwrap();
}

fn copy_contents<I: Iterator<Item = io::Result<fs::DirEntry>>>(crate_path: &Path, dir: I) {
    for entry in dir {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let dst_dir = crate_path.join(entry.path().file_name().unwrap());
            fs::create_dir(&dst_dir).unwrap();
            copy_contents(&dst_dir, fs::read_dir(entry.path()).unwrap());
        } else {
            fs::copy(
                entry.path(),
                crate_path.join(entry.path().file_name().unwrap()),
            )
            .unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 3 {
        println!("usage: path/to/svd2rust/output path/to/output/pac module_name ...");
        process::exit(1);
    }

    let svd_crate_path = PathBuf::from(&args[0]);
    if !svd_crate_path.exists() || !svd_crate_path.is_dir() {
        println!("Cannot find crate directory {}", svd_crate_path.display());
        process::exit(1);
    }

    let output_pac = PathBuf::from(&args[1]);
    if !output_pac.exists() || !output_pac.is_dir() {
        println!("Cannot find output PAC directory {}", output_pac.display());
        process::exit(1);
    }

    for module_name in args.iter().skip(2) {
        let peripheral_module_src = fs::File::open(
            svd_crate_path
                .join("src")
                .join(format!("{}.rs", module_name)),
        )
        .expect(&format!("Unable to find main module for {}", module_name));
        let peripheral_dir_src = fs::read_dir(svd_crate_path.join("src").join(module_name)).expect(
            &format!("Unable to find module directory for {}", module_name),
        );

        let peripheral_crate_path =
            output_pac.join(format!("imxrt1060-{}", module_name.replace("_", "-")));
        if peripheral_crate_path.exists() {
            println!(
                "{} peripheral crate seems to already exist! Skipping...",
                peripheral_crate_path.display()
            );
            continue;
        }
        process::Command::new("cargo")
            .args(&[
                "new",
                "--lib",
                &format!("{}", peripheral_crate_path.display()),
                "--vcs",
                "none",
            ])
            .output()
            .expect(&format!(
                "Cannot create peripheral crate for '{}'",
                module_name
            ));

        add_deps(&peripheral_crate_path);
        write_lib(&peripheral_crate_path, peripheral_module_src);
        copy_contents(&peripheral_crate_path.join("src"), peripheral_dir_src);

        println!("{} crate was created! Add the crate to the workspace, re-export it from the main PAC crate, and enable the relevant structs in the PAC crate", peripheral_crate_path.display());
    }
}
