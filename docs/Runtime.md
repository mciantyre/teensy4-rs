`teensy4-bsp` runtime
=====================

This document describes how the `teensy4-bsp` runtime works. Before reading
this document, it helps to know about

- the [`cortex-m-rt` package][cortex-m-rt], which is widely used for embedded
  Rust development on Cortex-M systems
- the default [Teensy 4 memory map](https://www.pjrc.com/store/teensy40.html),
  which is what's used when developing applications with standard tools for the
  Teensy 4.

[cortex-m-rt]: https://github.com/rust-embedded/cortex-m-rt

Linker script
-------------

`t4link.x` describes the memory layout of programs built with the BSP. The
layout is compatible with the `cortex-m-rt` requirements. Specifically, it
includes symbols expected by the crate, such as

- `__sbss` and `__ebss`
- `__sdata`, `__edata`, and `__sidata`

which lets the `cortex-m-rt` zero BSS and initialize data.

The linker script also places the exception and interrupt tables. The exception
table is defined in the `cortex-m-rt` crate. The interrupt table is defined in
the `imxrt-ral` register access layer.

The linker script satisfies the default Teensy 4 memory map. It places
instructions in ITCM, all data in DTCM, and the stack at the top of DTCM.
The linker script computes the ITCM and DTCM partition, ensuring that there's
just enough space for instructions, with the rest of the space for data and
stack.

The boot data structures are placed at the start of flash:

- The FlexSPI configuration block is defined in Rust. It's available in the
  `teensy4-fcb` crate.
- The IVT and remaining boot data is defined in the linker script

The vector table, including the reset handler, is placed in flash. However,
the vector table is copied into DTCM before program initialization. See the
execution flow for more information.

As of this writing, the OCRAM region is allocated for the Teensy 4's USB stack,
written in C. This may change in future versions of the BSP. Sections placed in
OCRAM are considered an implementation detail.

Since the linker script is intended to fully replace the `cortex-m-rt` linker
script, we include other directives necessary for a valid runtime. This support
includes `PROVIDE`s, `EXTERN`s, and additional sections. By referring to this
linker script in your build, you may use the `cortex-m-rt` runtime crate while
still using the advanced memory layout available in i.MX RT microcontrollers.

Execution
---------

On reset, execution begins in `start.s`, a precompiled assembly prelude that
prepares the runtime's advanced features:

1. initialize ITCM and DTCM regions
2. copy all other instructions to ITCM
3. copy the vector table to RAM, and initialize VTOR
4. jump to the next stage

The reset handler executes from flash (XIP). By step 4, after the copy, the MCU
executes all instructions from ITCM.

The second stage is written in Rust. It's defined in the BSP's `rt` runtime
module. The second stage

1. overrides CCM low power behaviors for safer execution
2. jumps to the `cortex-m-rt` reset handler to finish initialization

The second stage must never read anything in a data section, since the memory
is uninitialized.

Execution then relies on the `cortex-m-rt` reset handler to finish system
initialization, and invoke the program's entrypoint. This completes the BSP's
runtime initialization.

This execution lets us prepare TCM for the i.MX RT microcontrollers and meet
the requirements of our memory layout, while still using the `cortex-m-rt` for
complete system initialization.

Discussion
----------

The second stage could have been written in terms of the `cortex-m-rt`'s
`#[pre_init]` feature. However, this approach would mean that a BSP end user
could not implement a `pre_init` function. We therefore implement our behaviors
in Rust, before jumping into the `cortex-m-rt` reset handler.
