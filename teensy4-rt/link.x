INCLUDE device.x

MEMORY
{
    ITCM    (rwx): ORIGIN = 0x00000000, LENGTH = 512K
    DTCM    (rwx): ORIGIN = 0x20000000, LENGTH = 512K
    RAM     (rwx): ORIGIN = 0x20200000, LENGTH = 512K
    FLASH   (rwx): ORIGIN = 0x60000000, LENGTH = 1984K
}

/* Symbol provided by Rust */
EXTERN(_reset);
/* This might get stripped out in dependent crates, but it's important to keep around. */
/* It's put into the FCB block below. */
EXTERN(FLEXSPI_CONFIGURATION_BLOCK);

EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);

EXTERN(DefaultHandler);
PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(HardFault = HardFault_);
PROVIDE(DefaultHandler = DefaultHandler_);

SECTIONS
{
    /* The boot section contains all the special things that allow the IMXRT1062 to boot */
    .boot :
    {
        /* Firmware Configuration Block (FCB) */
        KEEP(*(.fcb));
        FILL(0xFFFFFFFF);
        . = ORIGIN(FLASH) + 0x1000;
        _ivt = .;
        /* ------------------
         * Image Vector Table
         * ------------------
         */
        LONG(0x402000D1);           /* Header, magic number */
        LONG(__svectors);           /* Address of the vectors table */
        LONG(0x00000000);           /* RESERVED */
        LONG(0x00000000);           /* Device Configuration Data (unused) */
        LONG(_boot_data);           /* Address to boot data */
        LONG(_ivt);                 /* Self reference, required by boot ROM */
        LONG(0x00000000);           /* Command Sequence File (unused) */
        LONG(0x00000000);           /* RESERVED */
        /* ---------
         * Boot data
         * ---------
         */
        _boot_data = .;
        LONG(ORIGIN(FLASH));        /* Start of image (origin of flash) */
        LONG(_lflash);              /* Length of flash */
        LONG(0x00000000);           /* Plugin flag (unused) */
        /* --------- */
        KEEP(*(.boot.reset));
        KEEP(*(.boot.tcm));
        KEEP(*(.HardFaultTrampoline));
        KEEP(*(.HardFault.*));
        /* Contains the reset vectors, exceptions, and interrupts. */
        /* It must be 1024-byte aligned */
        . = ALIGN(1024);
        __svectors = .;
        LONG(__stack); /* Initial stack pointer */
        LONG(_reset);     /* Pointer to the reset handler */
        KEEP(*(.vector_table.exceptions));
        KEEP(*(.vector_table.interrupts));
        *(.flashmem); /* Compatibility with USB stack */
        *(.progmem); /* Compat with USB stack */
        . = ALIGN(16);
    } > FLASH

    .text :
    {
        __stext = .;
        . = . + 32; /* MPU TRAP TODO */
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > ITCM AT> FLASH

    .padding (NOLOAD) :
    {
        . = ALIGN(32768);
    } > ITCM

    __sitext = LOADADDR(.text);

    .data :
    {
        __sdata = .;
        *(.rodata .rodata.*);
        *(.data .data.*);
        . = ALIGN(16);
        __edata = .;
    } > DTCM AT> FLASH

    __sidata = LOADADDR(.data);

    .bss ALIGN(4) :
    {
        __sbss = .;
        *(.bss .bss.*);
        *(COMMON);
        . = ALIGN(32);
        . = . + 32; /* MPU TRAP TODO */
        __ebss = .;
    } > DTCM

    .dma (NOLOAD) :
    {
        *(.dmabuffers); /* Compat with USB */
        . = ALIGN(16);
    } > RAM

    /DISCARD/ :
    {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }

    /* The length of flash is required for the boot data */
    _lflash = SIZEOF(.boot) + SIZEOF(.text) + SIZEOF(.data);

    /* The following are used to compute the FlexRAM banks for ITCM / DTCM */
    _itcm_block_count = (SIZEOF(.text) + 0x7FFF) >> 15;
    __flexram_bank_config = 0xAAAAAAAA | ((1 << (_itcm_block_count * 2)) - 1);
    /* We reconfigure the stack pointer based on the ITCM / DTCM separation */
    __stack = ORIGIN(DTCM) + ((16 - _itcm_block_count) << 15);
}

/* Asserts that check some Rust requirements */
ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(teensy4-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(teensy4-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
ERROR(teensy4-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
ERROR(teensy4-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
ERROR(teensy4-rt): .bss is not 4-byte aligned");

ASSERT(__stext % 4 == 0 && __etext % 4 == 0, "
ERROR(teensy4-rt): .text is not 4-byte aligned");

ENTRY(_ivt);