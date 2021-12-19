INCLUDE device.x

MEMORY
{
    ITCM    (rwx): ORIGIN = 0x00000000, LENGTH = 512K
    DTCM    (rwx): ORIGIN = 0x20000000, LENGTH = 512K
    RAM     (rwx): ORIGIN = 0x20200000, LENGTH = 512K
    FLASH   (rwx): ORIGIN = 0x60000000, LENGTH = 1984K
}

EXTERN(__start);
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
PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
    /* If you add more sections to FLASH, you must add this section here */
    __lflash = SIZEOF(.boot) + SIZEOF(.vector_table) + SIZEOF(.text) + SIZEOF(.data) + SIZEOF(.gnu.sgstubs);

    /* The boot section contains all the special things that allow the IMXRT1062 to boot */
    .boot ORIGIN(FLASH) :
    {
        /* Firmware Configuration Block (FCB) */
        KEEP(*(.fcb));
        FILL(0xFFFFFFFF);
        . = ORIGIN(FLASH) + 0x1000;
        __ivt = .;
        /* ------------------
         * Image Vector Table
         * ------------------
         */
        LONG(0x402000D1);           /* Header, magic number */
        LONG(__sivectors);           /* Address of the vectors table */
        LONG(0x00000000);           /* RESERVED */
        LONG(0x00000000);           /* Device Configuration Data (unused) */
        LONG(__boot_data);          /* Address to boot data */
        LONG(__ivt);                /* Self reference, required by boot ROM */
        LONG(0x00000000);           /* Command Sequence File (unused) */
        LONG(0x00000000);           /* RESERVED */
        /* ---------
         * Boot data
         * ---------
         */
        __boot_data = .;
        LONG(ORIGIN(FLASH));        /* Start of image (origin of flash) */
        LONG(__lflash);             /* Length of flash */
        LONG(0x00000000);           /* Plugin flag (unused) */
        /* --------- */
        KEEP(*(.boot.start))        /* Shim reset handler for TCM init */

        *(.flashmem);               /* Compatibility with USB stack */
        *(.progmem);                /* Compat with USB stack */
    } > FLASH

        /* The following are used to compute the FlexRAM banks for ITCM / DTCM */
    __itcm_block_count = (SIZEOF(.text) + 0x7FFF) >> 15;
    __flexram_bank_config = 0xAAAAAAAA | ((1 << (__itcm_block_count * 2)) - 1);
    PROVIDE(__stack_start = ORIGIN(DTCM) + ((16 - __itcm_block_count) << 15));

    /* ## Sections in FLASH */
    /* ### Vector table */
    .vector_table : ALIGN(1024)
    {
        __sivectors = .;
        /* Initial Stack Pointer (SP) value */
        LONG(__stack_start);

        /* Reset vector */
        LONG(__start);
        __reset_vector = .;

        /* Exceptions */
        KEEP(*(.vector_table.exceptions)); /* this is the `__EXCEPTIONS` symbol */
        __eexceptions = .;

        /* Device specific interrupts */
        KEEP(*(.vector_table.interrupts)); /* this is the `__INTERRUPTS` symbol */
    } > FLASH

    /* ### .text */
    .text :
    {
        __stext = .;
        /* place these 2 close to each other or the `b` instruction will fail to link */
        *(.PreResetTrampoline);
        *(.Reset);

        *(.text .text.*);
        *(.HardFaultTrampoline);
        *(.HardFault.*);
        . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
    } > ITCM AT> FLASH
    . = ALIGN(4); /* Ensure __etext is aligned if something unaligned is inserted after .text */
    __etext = .; /* Define outside of .text to allow using INSERT AFTER .text */
    __sitext = LOADADDR(.text);

    /* ### .gnu.sgstubs
        This section contains the TrustZone-M veneers put there by the Arm GNU linker. */
    . = ALIGN(32); /* Security Attribution Unit blocks must be 32 bytes aligned. */
    __veneer_base = ALIGN(4);
    .gnu.sgstubs : ALIGN(4)
    {
        *(.gnu.sgstubs*)
        . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    } > FLASH
    . = ALIGN(4); /* Ensure __veneer_limit is aligned if something unaligned is inserted after .gnu.sgstubs */
    __veneer_limit = .;

    /* ## Sections in RAM */
    /* ### .data */
    .data : ALIGN(4)
    {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*);
        *(.rodata .rodata.*);
        . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
        __edata = .;
        /* Keep outside of the [__sdata, __edata] range so that it's not overwritten */
        KEEP(*(.vector_table.ram));
    } > DTCM AT>FLASH

    /* LMA of .data */
    __sidata = LOADADDR(.data);

    /* ### .bss */
    . = ALIGN(4);
    __sbss = .; /* Define outside of section to include INSERT BEFORE/AFTER symbols */
    .bss (NOLOAD) : ALIGN(4)
    {
        *(.bss .bss.*);
        *(COMMON); /* Uninitialized C statics */
        . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    } > DTCM
    . = ALIGN(4); /* Ensure __ebss is aligned if something unaligned is inserted after .bss */
    __ebss = .;

    /* ### .uninit */
    .uninit (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.uninit .uninit.*);
        . = ALIGN(4);
    } > DTCM
    . = ALIGN(4); /* Ensure alignment in case of INSERT AFTER */
    __sheap_dtcm = .;

    .dma (NOLOAD) :
    {
        *(.dmabuffers); /* Compat with USB */
        . = ALIGN(16);
    } > RAM

    .heap (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        __sheap = .;
        . = ORIGIN(RAM) + LENGTH(RAM);
        __eheap = .;
    } > RAM

    /* ## .got */
    /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
        the input files and raise an error if relocatable code is found */
    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    /* ## Discarded sections */
    /DISCARD/ :
    {
        /* Unused exception related info that only wastes space */
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}

/* Asserts that check some Rust requirements */
ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(cortex-m-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(cortex-m-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
ERROR(cortex-m-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
ERROR(cortex-m-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
ERROR(cortex-m-rt): .bss is not 4-byte aligned");

ASSERT(__stext % 4 == 0 && __etext % 4 == 0, "
ERROR(cortex-m-rt): .text is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
ERROR(cortex-m-rt): start of heap is not 4-byte aligned");
