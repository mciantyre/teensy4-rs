#[cfg(feature = "rt")]
use std::env;

#[cfg(feature = "rt")]
fn parse_size_string(val: Result<String, env::VarError>, default: usize) -> usize {
    val.map(|val| {
        if val.ends_with('k') || val.ends_with('K') {
            val[.. val.len() - 1].parse::<usize>().unwrap() * 1024
        } else {
            val.parse::<usize>().unwrap()
        }
    }).unwrap_or(default)
}

#[cfg(feature = "rt")]
fn main() {
    use imxrt_rt::{Family, FlexRamBanks, Memory, RuntimeBuilder};

    let heap_size = parse_size_string(env::var("TEENSY4_HEAP_SIZE"), 16 * 1024);
    let stack_size = parse_size_string(env::var("TEENSY4_STACK_SIZE"), 16 * 1024);

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
