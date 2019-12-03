// Initialization routines
//
// Ideally, we would use the r0 Rust crate. However,
// the functions aren't inlined when compiling in release
// mode. Even if they were inlined, the functions that
// the r0 functions end up calling might not be inlined.
//
// We need more control, and we don't want to drop into
// assembly... So write them here.

#include <stdint.h>

#define IMXRT_IOMUXC_GPR ((volatile uint32_t *)0x400AC000)
#define IOMUXC_GPR_GPR(N) (*(IMXRT_IOMUXC_GPR + (N)))

extern uint32_t __flexram_bank_config;
extern uint32_t __sbss;
extern uint32_t __ebss;
extern uint32_t __stext;
extern uint32_t __etext;
extern uint32_t __sitext;
extern uint32_t __sdata;
extern uint32_t __edata;
extern uint32_t __sidata;
extern uint32_t __estack;
extern void _start(void);

__attribute__((always_inline)) inline static void
init_data(uint32_t *sdata, const uint32_t *const edata,
          const uint32_t *sidata) {
  while (sdata < edata) {
    *sdata++ = *sidata++;
  }
}

__attribute__((always_inline)) inline static void
zero_bss(uint32_t *sbss, const uint32_t *const ebss) {
  while (sbss < ebss) {
    *sbss++ = 0;
  }
}

__attribute__((section(".boot.reset"), naked)) void _reset(void) {
  // Initialize TCM regions
  IOMUXC_GPR_GPR(17) = (uint32_t)&__flexram_bank_config;
  IOMUXC_GPR_GPR(16) = 0x00000007;
  IOMUXC_GPR_GPR(14) = 0x00AA0000;

  // Reconfigure the stack pointer(s) based on the DTCM / ITCM separation
  __asm__ volatile("msr MSP, %0" : : "r"((uint32_t)&__estack) :);
  __asm__ volatile("msr PSP, %0" : : "r"((uint32_t)&__estack) :);

  // Copy text and data into TCM regions
  init_data(&__stext, &__etext, &__sitext);
  init_data(&__sdata, &__edata, &__sidata);
  zero_bss(&__sbss, &__ebss);

  // Jump into Rust
  _start();
}