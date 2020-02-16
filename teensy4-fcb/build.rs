use imxrt_fcb_gen::serial_flash::opcodes::sdr::*;
use imxrt_fcb_gen::serial_flash::*;

//
// Sequences for lookup table
//

const SEQ_READ: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0xEB),
    Instr::new(RADDR, Pads::Four, 0x18),
    Instr::new(DUMMY, Pads::Four, 0x06),
    Instr::new(READ, Pads::Four, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_READ_STATUS: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0x05),
    Instr::new(READ, Pads::One, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_WRITE_ENABLE: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0x06),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_ERASE_SECTOR: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0x20),
    Instr::new(RADDR, Pads::One, 0x18),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_PAGE_PROGRAM: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0x02),
    Instr::new(RADDR, Pads::One, 0x18),
    Instr::new(WRITE, Pads::One, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_CHIP_ERASE: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0x60),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let nor_cb = nor::ConfigurationBlock {
        page_size: nor::PageSize::new(256),
        sector_size: nor::SectorSize::new(4096),
        ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
    };
    let lookup_table = {
        use imxrt_fcb_gen::serial_flash::SequenceCommand::*;
        let mut lut = LookupTable::new();
        lut[Read] = SEQ_READ;
        lut[ReadStatus] = SEQ_READ_STATUS;
        lut[WriteEnable] = SEQ_WRITE_ENABLE;
        lut[EraseSector] = SEQ_ERASE_SECTOR;
        lut[PageProgram] = SEQ_PAGE_PROGRAM;
        lut[ChipErase] = SEQ_CHIP_ERASE;
        lut
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

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fcb.rs");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "{}", fcb).unwrap();
}
