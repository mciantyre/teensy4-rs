use imxrt1060_fcb_gen::*;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
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
    let mut fcb = builder.build().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fcb.rs");
    let f = File::create(&dest_path).unwrap();
    fcb.write(f).unwrap();
}
