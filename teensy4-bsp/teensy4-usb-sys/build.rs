static C_SRCS: &[&str] = &[
    "./src/nonstd.c",
    "./src/usb_desc.c",
    "./src/usb_serial.c",
    "./src/usb_startup.c",
    "./src/usb.c",
];

/// The C compiler
static CC: &str = "arm-none-eabi-gcc";
/// The archiver
static AR: &str = "arm-none-eabi-gcc-ar";
/// Compiler flags
static CFLAGS: &[&str] = &[
    "-c",
    "-Wall",
    "-MMD",
    "-g",
    "-O2",
    "-ffunction-sections",
    "-fdata-sections",
    "-mcpu=cortex-m7",
    "-mthumb",
    "-mfloat-abi=hard",
    "-mfpu=fpv5-d16",
    "-std=gnu11",
];
/// Preprocessor flags
static CPPFLAGS: &[&str] = &[
    "-DUSB_SERIAL",
    "-D__IMXRT1062__",
    // TODO figure out how to handle / alias these
    "-DFLASHMEM=__attribute__((section(\".flashmem\")))",
    "-DPROGMEM=__attribute__((section(\".progmem\")))",
    "-DDMAMEM=__attribute__ ((section(\".dmabuffers\"), used))",
];

fn main() {
    for &rerun_if_changed in C_SRCS.iter() {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    let mut builder = cc::Build::new();
    builder.compiler(CC);
    builder.archiver(AR);
    builder.no_default_flags(true);
    builder.files(C_SRCS);
    builder.include("./src/");
    // Add all flags
    for &flag in CFLAGS.iter().chain(CPPFLAGS.iter()) {
        builder.flag(flag);
    }
    builder.compile("usbsys");
}
