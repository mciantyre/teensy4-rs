#[cfg(feature = "rt")]
use std::env;

const DEFAULT_HEAP_SIZE: usize = 16 * 1024;
const DEFAULT_STACK_SIZE: usize = 16 * 1024;

#[cfg(feature = "rt")]
fn main() {
    use imxrt_rt::{Family, FlexRamBanks, Memory, RuntimeBuilder};

    let heap_size = env::var("TEENSY4_HEAP_SIZE")
        .map(|val| val.parse::<usize>().unwrap_or(DEFAULT_HEAP_SIZE))
        .unwrap_or(DEFAULT_HEAP_SIZE);
    let stack_size = env::var("TEENSY4_STACK_SIZE")
        .map(|val| val.parse::<usize>().unwrap_or(DEFAULT_STACK_SIZE))
        .unwrap_or(DEFAULT_STACK_SIZE);

    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 1984 * 1024)
        .flexram_banks(FlexRamBanks {
            ocram: 0,
            itcm: 6,
            dtcm: 10,
        })
        .heap(Memory::Ocram)
        .heap_size(heap_size)
        .stack(Memory::Dtcm)
        .stack_size(stack_size)
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
