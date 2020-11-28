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
    ldm r2!, {r3}
    stm r0!, {r3}
    b 0b
1:
    # At this point, all text is in ITCM.
    # This branch, and all calls, will find
    # executable code. t4_init is implemented
    # in Rust.
    bl t4_init
