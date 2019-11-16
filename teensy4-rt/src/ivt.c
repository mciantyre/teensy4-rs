// Image Vector Table (IVT)
//
// TODO figure out if there's a way to define
// this in Rust / the linker script. Current
// preceived limitations include
//
// - the inability to turn a Rust function pointer
//   into a u32 in a static array (on stable compiler)
// - self-referential IVT array cannot be expressed in Rust?
// - unfamiliarity with linker script and how to express
//   the interdependencies between addresses (if that's
//   at all possible.)

#include <stdint.h>

extern uint32_t __lflash;
extern void _reset(void);

__attribute((section(".vector_table"), used)) const uint32_t vectors[2] = {
    // 64KiB offset from the DTCM origin. We'll re-compute the stack
    // pointer after we know how much memory is for DTCM / ITCM.
    0x20010000,
    (uint32_t)&_reset // Reset handler
};

// See section 8.7.1.2 "Boot data structure" of the IMXRT1060 reference manual
__attribute__((section(".boot.data"), used)) const uint32_t boot_data[3] = {
    0x60000000,          // Start of image (address of FLASH)
    (uint32_t)&__lflash, // Length of image
    0,                   // Plugin flag: unused
};

// See section 8.7.1.1 "Image vector table structure" of the IMXRT1060 reference
// manual
__attribute__((section(".boot.ivt"), used))
const uint32_t image_vector_table[8] = {
    0x402000D1,                   // Header
    (uint32_t)vectors,            // Address to the vectors table
    0,                            // RESERVED
    0,                            // Device Configuration Data (DCD): unused
    (uint32_t)boot_data,          // Address to boot data
    (uint32_t)image_vector_table, // Address to self
    0,                            // Command Sequence File (CSF): unused
    0                             // RESERVED
};