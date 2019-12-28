//! A library for generating an `iMXRT1060` Firmware Configuration Block (FCB)
//! from a `build.rs`
//!
//! # Rationale
//!
//! The `iMXRT1060` Firmware Configuration Block (FCB) is an array that
//! describes how the processor should initiate a boot. It's expected to be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with NAND- / NOR-based FLASH memory. The FCB has a lot of magic
//! numbers, and it would be nice to have an API to generate the FCB.
//!
//! The `imxrt1060-fcb-gen` crate provides an API for generating the FCB. As of this
//! writing, it supports only the generation of an FCB for reading NOR Flash via
//! FlexSPI. Other configurations, such as NAND Flash and / or the SEMC interface,
//! may be added later.
//!
//! # Usage
//!
//! Add `imxrt1060-fcb-gen` to your build dependencies:
//!
//! ```toml
//! [build-dependencies]
//! imxrt1060-fcb-gen = { path = "../imxrt1060-fcb-gen" }
//! ```
//!
//! Then, instantiate a `Builder` in your `build.rs` build script. Unless a
//! member of the `Builder` implements `Default`, it's probably required and
//! it requires your input. See the `iMXRT1060` reference manual for details
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

mod fcb;
pub use fcb::*;

use std::convert::TryFrom;
use std::marker::PhantomData;
use std::time::Duration;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ReadSampleClockSource {
    InternalLoopback = 0x00,
    LoopbackFromDQSPad = 0x01,
    FlashProvidedDQS = 0x03,
}

/// Serial flash CS hold time
///
/// Defaults to `0x03`, the 'recommended value'
pub struct CSHoldTime(pub u8);

impl Default for CSHoldTime {
    fn default() -> Self {
        CSHoldTime(0x03)
    }
}

/// Serial flash CS setup time
///
/// Defaults to `0x03`, the 'recommended value'
pub struct CSSetupTime(pub u8);

impl Default for CSSetupTime {
    fn default() -> Self {
        CSSetupTime(0x03)
    }
}

/// Column address width
pub struct ColumnAddressWidth(u8);
impl ColumnAddressWidth {
    /// Returns the value that represnts 'other devices'
    pub fn other_devices() -> Self {
        ColumnAddressWidth(0)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DeviceModeArgument(pub u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeviceModeConfiguration {
    Disabled,
    Enabled(DeviceModeArgument),
}

impl Default for DeviceModeConfiguration {
    fn default() -> Self {
        DeviceModeConfiguration::Disabled
    }
}

/// Wait time for all configuration commands
pub struct WaitTimeConfigurationCommands(u16);
impl WaitTimeConfigurationCommands {
    pub fn disable() -> Self {
        WaitTimeConfigurationCommands(0)
    }

    /// Computes the wait time from the specified `wait_time`. The
    /// provided duration should be divisible by `100us`, since the
    /// value is a factor scaled by `100us`. Returns `None` if representing
    /// this as a factor of `100us` returns `0`, or if the factor cannot be
    /// expressed in a `u16`.
    pub fn from_duration(wait_time: Duration) -> Option<Self> {
        let us = wait_time.as_micros();
        if us < 100 {
            None
        } else {
            let factor = u16::try_from(us / 100).ok()?;
            Some(WaitTimeConfigurationCommands(factor))
        }
    }
}

/// Sequence parameter for device mode configuration
#[derive(Default)]
pub struct DeviceModeSequence(u32);
impl DeviceModeSequence {
    /// Create a new sequence parameter for device configuration
    ///
    /// `starting_lut_index`: starting LUT index of Device mode configuration command
    /// `number_of_luts`: number of LUT sequences for Device mode configuration command
    pub fn new(number_of_luts: u8, starting_lut_index: u8) -> Self {
        DeviceModeSequence((u32::from(starting_lut_index) << 8) | u32::from(number_of_luts))
    }
}

pub enum DeviceType {
    SerialNOR(nor::ConfigurationBlock),
}

#[repr(u8)]
pub enum FlashPadType {
    Single = 1,
    Dual = 2,
    Quad = 4,
    Octal = 8,
}

#[repr(u8)]
pub enum SerialClockFrequency {
    MHz30 = 1,
    MHz50 = 2,
    MHz60 = 3,
    MHz75 = 4,
    MHz80 = 5,
    MHz100 = 6,
    MHz120 = 7,
    HMz133 = 8,
    MHz166 = 9,
}

pub struct A1;
pub struct A2;
pub struct B1;
pub struct B2;

/// If we're using SPI NAND, the size wrapped
/// in this value will be multiplied by `2`.
pub struct SerialFlashSize<Region> {
    _region: PhantomData<Region>,
    size: u32,
}

impl<Region> SerialFlashSize<Region> {
    /// The `actual_size` will be multiplied by `2` in
    /// the final output if we're also using SPI NAND.
    pub fn new(actual_size: u32) -> Self {
        SerialFlashSize {
            _region: PhantomData,
            size: actual_size,
        }
    }
}

impl<Region> Default for SerialFlashSize<Region> {
    fn default() -> Self {
        Self::new(0)
    }
}

pub struct LookupTable(pub [u32; 64]);
impl Default for LookupTable {
    fn default() -> LookupTable {
        LookupTable([0; 64])
    }
}

/// Builder for a firmware configuration block
/// definition
pub struct Builder {
    pub read_sample_clock_source: ReadSampleClockSource,
    pub cs_hold_time: CSHoldTime,
    pub cs_setup_time: CSSetupTime,
    pub column_address_width: ColumnAddressWidth,
    pub device_mode_configuration: DeviceModeConfiguration,
    pub wait_time_cfg_commands: WaitTimeConfigurationCommands,
    pub device_mode_seq: DeviceModeSequence,
    pub device_type: DeviceType,
    pub serial_flash_pad_type: FlashPadType,
    pub serial_clk_freq: SerialClockFrequency,
    pub flash_a1_size: SerialFlashSize<A1>,
    pub flash_a2_size: SerialFlashSize<A2>,
    pub flash_b1_size: SerialFlashSize<B1>,
    pub flash_b2_size: SerialFlashSize<B2>,
    pub lookup_table: LookupTable,
}

impl Builder {
    pub fn build(self) -> Result<fcb::FCB, Box<dyn std::error::Error>> {
        let mut fcb = fcb::FCB::new();
        fcb.field_comment(
            0x00C,
            &(self.read_sample_clock_source as u8).to_le_bytes(),
            "readSampleClkSrc",
        );
        fcb.field_comment(0x00D, &self.cs_hold_time.0.to_le_bytes(), "csHoldTime");
        fcb.field_comment(0x00E, &self.cs_setup_time.0.to_le_bytes(), "csSetupTime");
        fcb.field_comment(
            0x00F,
            &self.column_address_width.0.to_le_bytes(),
            "columnAddressWidth",
        );
        fcb.field_comment(
            0x010,
            match self.device_mode_configuration {
                DeviceModeConfiguration::Disabled => &[0],
                DeviceModeConfiguration::Enabled(_) => &[1],
            },
            "deviceModeCfgEnable",
        );
        fcb.field_comment(
            0x013,
            &self.wait_time_cfg_commands.0.to_le_bytes(),
            "waitTimeCfgCommands",
        );
        fcb.field_comment(
            0x014,
            &self.device_mode_seq.0.to_le_bytes(),
            "deviceModeSeq",
        );

        if let DeviceModeConfiguration::Enabled(arg) = self.device_mode_configuration {
            fcb.field_comment(0x018, &arg.0.to_le_bytes(), "deviceModeArg");
        }

        fcb.field_comment(
            0x044,
            match self.device_type {
                DeviceType::SerialNOR(_) => &[1],
            },
            "deviceType",
        );
        fcb.field_comment(
            0x045,
            &(self.serial_flash_pad_type as u8).to_le_bytes(),
            "sflashPadType",
        );
        fcb.field_comment(
            0x046,
            &(self.serial_clk_freq as u8).to_le_bytes(),
            "serialClkFreq",
        );
        // TODO after adding SerialNAND, we have to multiply all
        // the flash sizes by 2
        fcb.field_comment(
            0x050,
            &self.flash_a1_size.size.to_le_bytes(),
            "sflashA1Size",
        );
        fcb.field_comment(
            0x054,
            &self.flash_a2_size.size.to_le_bytes(),
            "sflashA2Size",
        );
        fcb.field_comment(
            0x058,
            &self.flash_b1_size.size.to_le_bytes(),
            "sflashB1Size",
        );
        fcb.field_comment(
            0x05C,
            &self.flash_b2_size.size.to_le_bytes(),
            "sflashB2Size",
        );
        let mut lookup_table_offset = 0x080;
        for (idx, entry) in self.lookup_table.0.iter().enumerate() {
            let bytes = entry.to_le_bytes();
            fcb.field_comment(
                lookup_table_offset,
                &bytes,
                format!(
                    "lookupTable[{}..{}]",
                    idx * bytes.len(),
                    (idx + 1) * bytes.len()
                ),
            );
            lookup_table_offset += bytes.len();
        }
        match self.device_type {
            DeviceType::SerialNOR(norcb) => {
                fcb.field_comment(0x1C0, &norcb.page_size.0.to_le_bytes(), "pageSize");
                fcb.field_comment(0x1C4, &norcb.sector_size.0.to_le_bytes(), "sectorSize");
                fcb.field_comment(
                    0x1C8,
                    &(norcb.ip_cmd_serial_clk_freq as u8).to_le_bytes(),
                    "ipCmdSerialClkFreq",
                );
            }
        }
        Ok(fcb)
    }
}

pub mod nor {
    pub struct PageSize(pub u32);
    pub struct SectorSize(pub u32);
    #[repr(u8)]
    pub enum SerialClockFrequency {
        NoChange = 0,
        MHz30 = 1,
        MHz50 = 2,
        MHz60 = 3,
        MHz75 = 4,
        MHz80 = 5,
        MHz100 = 6,
        MHz120 = 7,
        HMz133 = 8,
        MHz166 = 9,
    }
    pub struct ConfigurationBlock {
        pub page_size: PageSize,
        pub sector_size: SectorSize,
        pub ip_cmd_serial_clk_freq: SerialClockFrequency,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn teensy4_fcb() {
        let nor_cb = nor::ConfigurationBlock {
            page_size: nor::PageSize(256),
            sector_size: nor::SectorSize(4096),
            ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
        };
        let lookup_table = LookupTable({
            let mut raw: [u32; 64] = [0; 64];
            raw[0] = 0x0A18_04EB;
            raw[1] = 0x2604_3206;
            raw[4] = 0x2404_0405;
            raw[12] = 0x0000_0406;
            raw[20] = 0x0818_0420;
            raw[32] = 0x0818_04D8;
            raw[36] = 0x0818_0402;
            raw[37] = 0x0000_2004;
            raw[44] = 0x0000_0460;
            raw
        });
        let builder = Builder {
            read_sample_clock_source: ReadSampleClockSource::LoopbackFromDQSPad,
            cs_hold_time: CSHoldTime(0x01),
            cs_setup_time: CSSetupTime(0x02),
            column_address_width: ColumnAddressWidth::other_devices(),
            device_mode_configuration: DeviceModeConfiguration::Disabled,
            wait_time_cfg_commands: WaitTimeConfigurationCommands::disable(),
            device_mode_seq: DeviceModeSequence::new(0, 0),
            flash_a1_size: SerialFlashSize::new(0x0020_0000),
            flash_a2_size: SerialFlashSize::default(),
            flash_b1_size: SerialFlashSize::default(),
            flash_b2_size: SerialFlashSize::default(),
            serial_clk_freq: SerialClockFrequency::MHz60,
            serial_flash_pad_type: FlashPadType::Quad,
            device_type: DeviceType::SerialNOR(nor_cb),
            lookup_table,
        };
        let fcb = builder.build().unwrap();
        // A known, working FCB for the Teensy 4. The Builder above defines this table
        let expected: [u32; 128] = [
            // 448 byte common FlexSPI configuration block, 8.6.3.1 page 223 (RT1060 rev 0)
            // MCU_Flashloader_Reference_Manual.pdf, 8.2.1, Table 8-2, page 72-75
            0x4246_4346, // Tag				0x00
            0x5601_0000, // Version
            0,           // reserved
            0x0002_0101, // columnAdressWidth,dataSetupTime,dataHoldTime,readSampleClkSrc
            0x0000_0000, // waitTimeCfgCommands,-,deviceModeCfgEnable
            0,           // deviceModeSeq
            0,           // deviceModeArg
            0x0000_0000, // -,-,-,configCmdEnable
            0,           // configCmdSeqs		0x20
            0,
            0,
            0,
            0, // cfgCmdArgs			0x30
            0,
            0,
            0,
            0x0000_0000, // controllerMiscOption		0x40
            0x0003_0401, // lutCustomSeqEnable,serialClkFreq,sflashPadType,deviceType
            0,           // reserved
            0,           // reserved
            0x0020_0000, // sflashA1Size			0x50
            0,           // sflashA2Size
            0,           // sflashB1Size
            0,           // sflashB2Size
            0,           // csPadSettingOverride		0x60
            0,           // sclkPadSettingOverride
            0,           // dataPadSettingOverride
            0,           // dqsPadSettingOverride
            0,           // timeoutInMs			0x70
            0,           // commandInterval
            0,           // dataValidTime
            0x0000_0000, // busyBitPolarity,busyOffset
            0x0A18_04EB, // lookupTable[0]		0x80
            0x2604_3206, // lookupTable[1]
            0,           // lookupTable[2]
            0,           // lookupTable[3]
            0x2404_0405, // lookupTable[4]		0x90
            0,           // lookupTable[5]
            0,           // lookupTable[6]
            0,           // lookupTable[7]
            0,           // lookupTable[8]		0xA0
            0,           // lookupTable[9]
            0,           // lookupTable[10]
            0,           // lookupTable[11]
            0x0000_0406, // lookupTable[12]		0xB0
            0,           // lookupTable[13]
            0,           // lookupTable[14]
            0,           // lookupTable[15]
            0,           // lookupTable[16]		0xC0
            0,           // lookupTable[17]
            0,           // lookupTable[18]
            0,           // lookupTable[19]
            0x0818_0420, // lookupTable[20]		0xD0
            0,           // lookupTable[21]
            0,           // lookupTable[22]
            0,           // lookupTable[23]
            0,           // lookupTable[24]		0xE0
            0,           // lookupTable[25]
            0,           // lookupTable[26]
            0,           // lookupTable[27]
            0,           // lookupTable[28]		0xF0
            0,           // lookupTable[29]
            0,           // lookupTable[30]
            0,           // lookupTable[31]
            0x0818_04D8, // lookupTable[32]		0x100
            0,           // lookupTable[33]
            0,           // lookupTable[34]
            0,           // lookupTable[35]
            0x0818_0402, // lookupTable[36]		0x110
            0x0000_2004, // lookupTable[37]
            0,           // lookupTable[38]
            0,           // lookupTable[39]
            0,           // lookupTable[40]		0x120
            0,           // lookupTable[41]
            0,           // lookupTable[42]
            0,           // lookupTable[43]
            0x0000_0460, // lookupTable[44]		0x130
            0,           // lookupTable[45]
            0,           // lookupTable[46]
            0,           // lookupTable[47]
            0,           // lookupTable[48]		0x140
            0,           // lookupTable[49]
            0,           // lookupTable[50]
            0,           // lookupTable[51]
            0,           // lookupTable[52]		0x150
            0,           // lookupTable[53]
            0,           // lookupTable[54]
            0,           // lookupTable[55]
            0,           // lookupTable[56]		0x160
            0,           // lookupTable[57]
            0,           // lookupTable[58]
            0,           // lookupTable[59]
            0,           // lookupTable[60]		0x170
            0,           // lookupTable[61]
            0,           // lookupTable[62]
            0,           // lookupTable[63]
            0,           // LUT 0: Read			0x180
            0,           // LUT 1: ReadStatus
            0,           // LUT 3: WriteEnable
            0,           // LUT 5: EraseSector
            0,           // LUT 9: PageProgram		0x190
            0,           // LUT 11: ChipErase
            0,           // LUT 15: Dummy
            0,           // LUT unused?
            0,           // LUT unused?			0x1A0
            0,           // LUT unused?
            0,           // LUT unused?
            0,           // LUT unused?
            0,           // reserved			0x1B0
            0,           // reserved
            0,           // reserved
            0,           // reserved
            // 64 byte Serial NOR configuration block, 8.6.3.2, page 346
            256,  // pageSize			0x1C0
            4096, // sectorSize
            1,    // ipCmdSerialClkFreq
            0,    // reserved
            0,    // reserved			0x1D0
            0,    // reserved
            0,    // reserved
            0,    // reserved
            0,    // reserved			0x1E0
            0,    // reserved
            0,    // reserved
            0,    // reserved
            0,    // reserved			0x1F0
            0,    // reserved
            0,    // reserved
            0,    // reserved
        ];
        let mut actual: [u32; 128] = [0; 128];
        for (bytes, slot) in fcb.raw.chunks_exact(4).zip(actual.iter_mut()) {
            use std::convert::TryInto;
            *slot = u32::from_le_bytes(bytes.try_into().unwrap());
        }
        const CHUNK_TEST_SIZE: usize = 16;
        for (idx, (actual_chunk, expected_chunk)) in actual
            .chunks(CHUNK_TEST_SIZE)
            .zip(expected.chunks(CHUNK_TEST_SIZE))
            .enumerate()
        {
            assert_eq!(
                actual_chunk,
                expected_chunk,
                "Start index {}",
                idx * CHUNK_TEST_SIZE
            );
        }
    }
}
