#[cfg(feature = "rt")]
fn main() {
    use imxrt_rt::{Family, FlexRamBanks, Memory, RuntimeBuilder};

    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 1984 * 1024)
        .flexram_banks(FlexRamBanks {
            ocram: 0,
            itcm: 6,
            dtcm: 10,
        })
        .heap(Memory::Ocram)
        .heap_size(16 * 1024)
        .heap_size_env_override("TEENSY4_HEAP_SIZE")
        .stack(Memory::Dtcm)
        .stack_size(16 * 1024)
        .stack_size_env_override("TEENSY4_STACK_SIZE")
        .vectors(Memory::Dtcm)
        .text(Memory::Itcm)
        .data(Memory::Dtcm)
        .bss(Memory::Dtcm)
        .uninit(Memory::Ocram)
        .linker_script_name("t4link.x")
        .build()
        .unwrap();
}

#[cfg(not(feature = "rt"))]
fn main() {}
