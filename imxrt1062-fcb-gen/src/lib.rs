//! A library for generating an iMXRT106X Firmware Configuration Block (FCB)
//! from Rust. Intended to be used in `build.rs` build scripts to generate
//! FCBs for iMXRT106x-based systems.
//!
//! # Rationale
//!
//! The iMXRT106x Firmware Configuration Block (FCB) is an array that
//! describes how the processor should initiate a boot. It's expected to be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with NAND- / NOR-based FLASH memory. The FCB has a lot of magic
//! numbers, and it would be nice to have an API to generate the FCB.
//!
//! The `imxrt1062-fcb-gen` crate provides an API for generating the FCB. As of this
//! writing, it supports only the generation of an FCB for reading NOR Flash via
//! FlexSPI. Other configurations, such as NAND Flash and / or the SEMC interface,
//! may be added later.
//!
//! # Usage
//!
//! Add `imxrt1062-fcb-gen` to your build dependencies:
//!
//! ```toml
//! [build-dependencies]
//! imxrt1062-fcb-gen = { path = "../imxrt1062-fcb-gen" }
//! ```
//!
//! Then, instantiate a `Builder` in your `build.rs` build script. Unless a
//! member of the `Builder` implements `Default`, it's probably required and
//! it requires your input. See the `iMXRT1062` reference manual for details
//! that may be missing from this library.
//!
//! Once the builder is built, write the FCB instance from your build script, and
//! include it using the `include!` macro in your crate. See the `teensy4-fcb`
//! crate for an example.
//!
//! # ABI
//!
//! The output is a single `u8` array, called `FIRMWARE_CONFIGURATION_BLOCK`.
//! The name is not mangled. It may be referenced in a linker script by its section, `".fcb"`.
//!
//! # Examples
//!
//! The example below demonstrates how we might define the FCB for a Teensy 4.
//!
//! ```rust
//! use imxrt1062_fcb_gen::*;
//!
//! // We're using serial NOR flash.
//! let nor_cb = nor::ConfigurationBlock {
//!     page_size: nor::PageSize::new(256),
//!     sector_size: nor::SectorSize::new(4096),
//!     ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
//! };
//! // Load the lookup table with our magic numbers. Numbers
//! // that are not specifed are set to `0`.
//! let lookup_table = {
//!     let mut lookup = LookupTable::new();
//!     lookup.insert_u32(0, 0x0A18_04EB);
//!     lookup.insert_u32(1, 0x2604_3206);
//!     lookup.insert_u32(4, 0x2404_0405);
//!     lookup.insert_u32(12, 0x0000_0406);
//!     lookup.insert_u32(20, 0x0818_0420);
//!     lookup.insert_u32(32, 0x0818_04D8);
//!     lookup.insert_u32(36, 0x0818_0402);
//!     lookup.insert_u32(37, 0x0000_2004);
//!     lookup.insert_u32(44, 0x0000_0460);
//!     lookup
//! };
//! // Define the FCB
//! let builder = Builder {
//!     read_sample_clock_source: ReadSampleClockSource::LoopbackFromDQSPad,
//!     cs_hold_time: CSHoldTime::new(0x01),
//!     cs_setup_time: CSSetupTime::new(0x02),
//!     column_address_width: ColumnAddressWidth::other_devices(),
//!     device_mode_configuration: DeviceModeConfiguration::Disabled,
//!     wait_time_cfg_commands: WaitTimeConfigurationCommands::disable(),
//!     device_mode_seq: DeviceModeSequence::new(0, 0),
//!     flash_a1_size: SerialFlashSize::new(0x0020_0000),
//!     flash_a2_size: SerialFlashSize::default(),
//!     flash_b1_size: SerialFlashSize::default(),
//!     flash_b2_size: SerialFlashSize::default(),
//!     serial_clk_freq: SerialClockFrequency::MHz60,
//!     serial_flash_pad_type: FlashPadType::Quad,
//!     device_type: DeviceType::SerialNOR(nor_cb),
//!     lookup_table,
//! };
//! // Build the FCB
//! let fcb = builder.build().unwrap();
//! // Write it to a string. It could also be written to
//! // a file, or to an `std::io::Writer`.
//! let fcb_string = format!("{}", fcb);
//!
//! // The string will resemble something like the following...
//! //
//! // #[link_section = ".fcb"]
//! // #[no_mangle]
//! // pub static FIRMWARE_CONFIGURATION_BLOCK: [u8; 512] = [
//! //     0x46, // 0x000 Tag 'FCFB'
//! //     0x43, // 0x001
//! //     0x46, // 0x002
//! //     0x42, // 0x003
//! //     0x00, // 0x004 Version 'bugfix'
//! //     0x00, // 0x005 Version 'minor'
//! //     0x01, // 0x006 Version 'major
//! //     0x56, // 0x007 Version 'V'
//! //     0x00, // 0x008 RESERVED
//! //     0x00, // 0x009 RESERVED
//! //     0x00, // 0x00A RESERVED
//! //     0x00, // 0x00B RESERVED
//! //     0x01, // 0x00C readSampleClkSrc
//! //     0x01, // 0x00D csHoldTime
//! //     0x02, // 0x00E csSetupTime
//! //     0x00, // 0x00F columnAddressWidth
//! //     0x00, // 0x010 deviceModeCfgEnable
//! //     0x00, // 0x011 RESERVED
//! //     0x00, // 0x012
//! //     0x00, // 0x013 waitTimeCfgCommands
//! //     0x00, // 0x014 deviceModeSeq
//! //     // Rest elided for clarity...
//! // ];
//! ```

#[macro_use]
mod macros;

mod builder;
mod fcb;
mod fields;
mod lookup;

pub mod nor;

pub use builder::*;
pub use fcb::*;
pub use fields::*;
pub use lookup::*;
