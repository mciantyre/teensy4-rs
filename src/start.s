.thumb
.section .boot.start, "ax"
.global __start
.type	__start,%function
.thumb_func
__start:
    # Prepare TCM memory regions
    # ---------------------------
    # We're currently sitting at the top of DTCM.
    # But, the TCM regions aren't initialized! We
    # need to set up the TCM regions based on the
    # ITCM / DTCM separation, then enable the memory
    # regions. If you touch the stack before DTCM
    # is ready, the processor faults.

    ldr r0, = 0x400AC000            @ IMXRT_IOMUXC_GPR base address
    ldr r1, =__flexram_bank_config  @ Value for GPR17, computed in linker script
    ldr r2, =0x00200007             @ Value for GPR16
    ldr r3, =0x00AA0000             @ Value for GPR14

    str r1, [r0, #68]               @ *(IMXRT_IOMUXC_GPR + 17) = (uint32_t)&__flexram_bank_config;
    str r2, [r0, #64]               @ *(IMXRT_IOMUXC_GPR + 16) = 0x00200007;
    str r3, [r0, #56]               @ *(IMXRT_IOMUXC_GPR + 14) = 0x00AA0000;

    # We can now use the stack!

    # Copy text into ITCM
    # -------------------
    # We've placed nearly all of the program's instructions
    # into ITCM. We now need to copy them from FLASH into
    # ITCM, so that the rest of the program works.

    ldr r0, =__stext
    ldr r1, =__etext
    ldr r2, =__sitext
0:
    cmp r1, r0
    beq 1f
    ldm r2!, {{r3}}
    stm r0!, {{r3}}
    b 0b
1:
    # At this point, all text is in ITCM.

    # Copy vector table into RAM
    # --------------------------

    ldr r0, =__svectors
    ldr r1, =__evectors
    ldr r2, =__sivectors
    ldr r3, =0xE000ED08             @ SCB_VTOR address
    str r0, [r3]                    @ *SCB_VTOR = (uint32_t)&__svectors
2:
    cmp r1, r0
    beq 3f
    ldm r2!, {{r3-r5}}              @ NUM_VECTORS % 3 == 0
    stm r0!, {{r3-r5}}
    b 2b
3:
    # Call into Rust, and finish the rest of the
    # Teensy 4 initialization.
    bl t4_init

.equ NUM_VECTORS, 16 + 158
.section .vector_table.ram, "aw"
.align 10
__svectors:
.skip 4 * NUM_VECTORS
__evectors:
