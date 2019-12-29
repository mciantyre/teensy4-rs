//! The `Builder` API
//!
//! The builder turns the fields into the FCB.

use crate::fcb;
use crate::fields::*;
use crate::lookup::LookupTable;

/// Builder for a firmware configuration block
/// definition
///
/// See the documentation on the types for more information.
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
    /// Turns the `Builder` into an `FCB`, or returns an error if there
    /// is something incorrect.
    ///
    /// # Panics
    ///
    /// `build` may panic if there is an error in the implementation that
    /// ends up writing a field to a reserved offset in the FCB.
    pub fn build(self) -> Result<fcb::FCB, Box<dyn std::error::Error>> {
        let mut fcb = fcb::FCB::new();
        fcb.field_comment(
            0x00C,
            &(self.read_sample_clock_source as u8).to_le_bytes(),
            "readSampleClkSrc",
        );
        fcb.field_comment(0x00D, &self.cs_hold_time, "csHoldTime");
        fcb.field_comment(0x00E, &self.cs_setup_time, "csSetupTime");
        fcb.field_comment(0x00F, &self.column_address_width, "columnAddressWidth");
        fcb.field_comment(
            0x010,
            match self.device_mode_configuration {
                DeviceModeConfiguration::Disabled => &[0],
                DeviceModeConfiguration::Enabled(_) => &[1],
            },
            "deviceModeCfgEnable",
        );
        fcb.field_comment(0x013, &self.wait_time_cfg_commands, "waitTimeCfgCommands");
        fcb.field_comment(0x014, &self.device_mode_seq, "deviceModeSeq");

        if let DeviceModeConfiguration::Enabled(arg) = self.device_mode_configuration {
            fcb.field_comment(0x018, &arg, "deviceModeArg");
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
        let lookup_table_offset = 0x080;
        for (idx, byte) in self.lookup_table.iter().enumerate() {
            fcb.field_comment(
                lookup_table_offset + idx,
                &[*byte],
                format!("lookupTable[{}]", idx,),
            );
        }
        match self.device_type {
            DeviceType::SerialNOR(norcb) => {
                fcb.field_comment(0x1C0, &norcb.page_size, "pageSize");
                fcb.field_comment(0x1C4, &norcb.sector_size, "sectorSize");
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::nor;

    #[test]
    fn teensy4_fcb() {
        let nor_cb = nor::ConfigurationBlock {
            page_size: nor::PageSize::new(256),
            sector_size: nor::SectorSize::new(4096),
            ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
        };
        let lookup_table = {
            let mut lookup = LookupTable::new();
            lookup.insert_u32(0, 0x0A18_04EB);
            lookup.insert_u32(1, 0x2604_3206);
            lookup.insert_u32(4, 0x2404_0405);
            lookup.insert_u32(12, 0x0000_0406);
            lookup.insert_u32(20, 0x0818_0420);
            lookup.insert_u32(32, 0x0818_04D8);
            lookup.insert_u32(36, 0x0818_0402);
            lookup.insert_u32(37, 0x0000_2004);
            lookup.insert_u32(44, 0x0000_0460);
            lookup
        };
        let builder = Builder {
            read_sample_clock_source: ReadSampleClockSource::LoopbackFromDQSPad,
            cs_hold_time: CSHoldTime::new(0x01),
            cs_setup_time: CSSetupTime::new(0x02),
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
        // A known, working FCB for the Teensy 4. The Builder above defines this table.
        // Note that this is a u32 array, where as we serialize a u8 array. We will convert
        // across the two arrays in our comparison.
        let expected: [u32; 128] = [
            // 448 byte common FlexSPI configuration block, 8.6.3.1 page 223 (RT1062 rev 0)
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
