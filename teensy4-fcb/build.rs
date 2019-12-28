use imxrt1060_fcb_gen::*;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // We're using serial NOR flash.
    let nor_cb = nor::ConfigurationBlock {
        page_size: nor::PageSize::new(256),
        sector_size: nor::SectorSize::new(4096),
        ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
    };
    // Load the lookup table with our magic numbers. Numbers
    // that are not specifed are set to `0`.
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
    // Define the FCB
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
    // Build the FCB
    let fcb = builder.build().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fcb.rs");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "{}", fcb).unwrap();
}
