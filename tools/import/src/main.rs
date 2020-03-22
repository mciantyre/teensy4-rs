//! Allows us to auto-import a iMXRT1062 module
//! from the svd2rust output into the imxrt1062-pac
//! megacrate. It code-ifies some manual work.
//!
//! We expect that this tool is run from the `tools`
//! directory.
//!
//! This could probably use some better error handling...

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

/// The name of the PAC crate into which we will
/// put subcrates
static OUTPUT_PAC_NAME: &str = "imxrt1062-pac";

/// Add dependencies into the Cargo.toml of the new PAC subcrate
/// identified by crate_path
///
/// TODO this should use the cargo-toml abstractions.
fn add_deps(crate_path: &Path) {
    /// Dependencies that are added to each PAC subcrate
    static CARGO_TOML_DEPENDENCIES: &str = r#"vcell = "0.1.2"
"#;

    /// Additional directives to add into each PAC subcrate's
    /// Cargo.toml
    static CARGO_NO_TESTS_BENCH: &str = r#"
[lib]
bench = false
test = false
"#;

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

fn copy_generic_rs(crate_path: &Path) {
    static GENERIC_RS: &[u8] = include_bytes!("generic.rs");
    let mut generic_rs = fs::File::create(crate_path.join("src").join("generic.rs"))
        .expect("Unable to create generic.rs");
    generic_rs
        .write_all(GENERIC_RS)
        .expect("Unable to write generic.rs");
}

/// Migrate the `lib.rs` of the PAC subscrate, adding
/// our necessary header to the top of the file.
fn write_lib<R: Read>(crate_path: &Path, mut src: R) {
    static LIB_PRELUDE: &str = r#"#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

"#;
    let mut crate_lib =
        fs::File::create(crate_path.join("src").join("lib.rs")).expect("Unable to create lib.rs");
    crate_lib
        .write_all(LIB_PRELUDE.as_bytes())
        .expect("Unable to write lib.rs prelude");
    io::copy(&mut src, &mut crate_lib).unwrap();
}

/// Recursively copies the files from the provided iterator into the directory `crate_path`
fn copy_contents<I: Iterator<Item = io::Result<fs::DirEntry>>>(crate_path: &Path, dir: I) {
    for entry in dir {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let dst_dir = crate_path.join(entry.path().file_name().unwrap());
            if !dst_dir.exists() {
                fs::create_dir(&dst_dir).unwrap();
            }
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

/// We don't modify the PAC's `lib.rs` with the re-exports. To help
/// our a user, suggest the new re-exports. They may copy and paste
/// this into the file.
fn suggest_reexports(crate_names: &[String]) {
    println!("imxrt1062-pac reexport additions...");
    for crate_name in crate_names {
        let crate_name = crate_name.replace("-", "_");
        // pub use imxrt1062_foo_bar as foo_bar
        let module = crate_name
            .split('_')
            .skip(1)
            .map(String::from)
            .collect::<Vec<String>>()
            .join("_");
        println!("pub use {} as {};", crate_name, module);
    }
}

/// Update the workspace Cargo.toml.
/// We assume that we're running this binary from
/// this directory, and that the workspace Cargo.toml
/// is one level up
fn update_workspace_toml(crate_names: &[String]) {
    static WORKSPACE_CARGO_TOML: &str = "../Cargo.toml";
    let mut workspace: cargo_toml::Workspace = {
        let file = fs::read(WORKSPACE_CARGO_TOML).expect("Cannot read workspace Cargo.toml");
        ::toml::de::from_slice(&file).unwrap()
    };
    for crate_name in crate_names {
        workspace.add_member(PathBuf::from(OUTPUT_PAC_NAME).join(crate_name));
    }
    let new_toml = ::toml::ser::to_string_pretty(&workspace).unwrap();
    fs::write(WORKSPACE_CARGO_TOML, new_toml).unwrap();
}

/// Add the new dependencies into the top-level PAC's Cargo.toml
fn update_pac_dependencies(output_pac: &Path, crate_names: &[String]) {
    let output_pac_toml = output_pac.join("Cargo.toml");
    let mut krate: cargo_toml::Krate = {
        let file = fs::read(&output_pac_toml).unwrap();
        ::toml::de::from_slice(&file).unwrap()
    };
    for crate_name in crate_names {
        krate.add_versioned_dependency(crate_name, crate_name, "0.1.0");
    }
    let new_toml = ::toml::ser::to_string_pretty(&krate).unwrap();
    fs::write(&output_pac_toml, new_toml).unwrap();
}

fn add_cargo_contents(output_pac: &Path) {
    let output_pac_toml = output_pac.join("Cargo.toml");
    let mut krate: cargo_toml::Krate = {
        let file = fs::read(&output_pac_toml).unwrap();
        ::toml::de::from_slice(&file).unwrap()
    };
    krate.set_categories(&["embedded", "hardware-support", "no-std"]);
    krate.set_keywords(&["arm", "svd2rust", "imxrt1062", "cortex-m"]);
    krate.set_license("MIT OR Apache-2.0");
    krate.set_repository("https://github.com/mciantyre/teensy4-rs");
    krate.set_description(
        r#"An imxrt1062-pac subcrate. See the imxrt1062-pac for more details.

This crate is not maintained. Consider using the register access layer provided by the
[imxrt-rs](https://github.com/imxrt-rs/imxrt-rs) project.

Formerly part of the teensy4-rs project.
"#,
    );
    let new_toml = ::toml::ser::to_string_pretty(&krate).unwrap();
    fs::write(&output_pac_toml, new_toml).unwrap();
}

fn main() {
    let output_pac: PathBuf = PathBuf::from("../").join(OUTPUT_PAC_NAME);
    let mut args = env::args().skip(1);
    let svd_crate_path = match args.next() {
        Some(path) => PathBuf::from(path),
        None => {
            println!("usage: path/to/svd2rust/output module_name ...");
            process::exit(1);
        }
    };

    let mut new_pac_crates: Vec<String> = Vec::new();
    for module_name in args {
        let module_name = &module_name;
        let peripheral_module_src = fs::File::open(
            svd_crate_path
                .join("src")
                .join(format!("{}.rs", module_name)),
        )
        .unwrap_or_else(|_| panic!("Unable to find main module for {}", module_name));
        let peripheral_dir_src = fs::read_dir(svd_crate_path.join("src").join(module_name))
            .unwrap_or_else(|_| panic!("Unable to find module directory for {}", module_name));

        let crate_name = format!("imxrt1062-{}", module_name.replace("_", "-"));
        let peripheral_crate_path = output_pac.join(crate_name.clone());
        if !peripheral_crate_path.exists() {
            process::Command::new("cargo")
                .args(&[
                    "new",
                    "--lib",
                    &format!("{}", peripheral_crate_path.display()),
                    "--vcs",
                    "none",
                ])
                .output()
                .unwrap_or_else(|_| panic!("Cannot create peripheral crate for '{}'", module_name));
            add_deps(&peripheral_crate_path);
        }
        add_cargo_contents(&peripheral_crate_path);
        write_lib(&peripheral_crate_path, peripheral_module_src);
        copy_contents(&peripheral_crate_path.join("src"), peripheral_dir_src);
        copy_generic_rs(&peripheral_crate_path);
        new_pac_crates.push(crate_name);
    }

    if !new_pac_crates.is_empty() {
        suggest_reexports(&new_pac_crates);
        update_workspace_toml(&new_pac_crates);
        update_pac_dependencies(&output_pac, &new_pac_crates);
    }
}
