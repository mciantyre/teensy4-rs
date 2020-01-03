//! Tool to maintain the version of the PAC crate and subcrates
//!
//! The entier PAC is versioned with one version number. This tool lets us
//! specify that version number, which will be updated in every subcrate
//! of the PAC, as well as the top-level PAC.
//!
//! The tool assumes that we're running it from the top of the `tools`
//! workspace:
//!
//! ```text
//! cargo run -p version -- 0.1.2
//! ```

use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let version = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("Please supply a semver"))?;

    let version = semver::Version::parse(&version)?;

    let pac_cargo_contents = fs::read_to_string("../imxrt1062-pac/Cargo.toml")?;
    let mut pac_cargo_toml: cargo_toml::Krate = ::toml::from_str(&pac_cargo_contents)?;
    pac_cargo_toml.set_version(&version);

    let pac_subcrates: Vec<_> = pac_cargo_toml
        .dependencies()
        .filter(|dep| dep.contains("imxrt1062-"))
        .cloned()
        .collect();

    pac_subcrates.into_iter().try_for_each(|pac_subcrate| {
        pac_cargo_toml.update_dependency(&pac_subcrate, "../imxrt1062-pac", |krate| {
            krate.set_version(&version);
            krate.set_categories(&["embedded", "hardware-support", "no-std"]);
            krate.set_keywords(&["arm", "svd2rust", "imxrt1062", "cortex-m"]);
            krate.set_license("MIT OR Apache-2.0");
            krate.set_repository("https://github.com/mciantyre/teensy4-rs");
            krate.set_description(
                r#"An imxrt1062-pac subcrate. See the imxrt1062-pac for more details.

Part of the teensy4-rs project.        
"#,
            );
        })
    })?;

    let pac_cargo_contents = ::toml::to_string_pretty(&pac_cargo_toml)?;
    fs::write("../imxrt1062-pac/Cargo.toml", &pac_cargo_contents)?;

    Ok(())
}
