// Initialization routines
//
// TODO this should be written in assembly. It's much
// too sensitive to compiler optimizations.

#include <stdint.h>

#define IMXRT_IOMUXC_GPR ((volatile uint32_t *)0x400AC000)
#define IOMUXC_GPR_GPR(N) (*(IMXRT_IOMUXC_GPR + (N)))

extern uint32_t __flexram_bank_config;
extern uint32_t __stext;
extern uint32_t __etext;
extern uint32_t __sitext;
extern void Reset(void);

__attribute__((always_inline)) inline static void
init_data(uint32_t *sdata, const uint32_t *const edata,
          const uint32_t *sidata) {
  while (sdata < edata) {
    *sdata++ = *sidata++;
  }
}

__attribute__((section(".boot.start"), optimize("no-tree-loop-distribute-patterns"), naked)) void __start(void) {
  // Initialize TCM regions
  IOMUXC_GPR_GPR(17) = (uint32_t)&__flexram_bank_config;
  IOMUXC_GPR_GPR(16) = 0x00200007;
  IOMUXC_GPR_GPR(14) = 0x00AA0000;

  // Copy text and data into TCM regions
  init_data(&__stext, &__etext, &__sitext);

  // Jump into Rust
  Reset();
}
