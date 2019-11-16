// Hard-Fault trampoline
//
// Loads the stack pointer into R0, then calls the
// real hard fault handler. The real hard fault hanlder
// is implemented in Rust.

__attribute__((section(".HardFaultTrampoline"), naked)) void
HardFaultTrampoline(void) {
  __asm__("movs R0, #4 \n"
          "mov R1, LR \n"
          "tst R0, R1 \n"
          "beq _MSP \n"
          "mrs R0, PSP \n"
          "b HardFault \n"
          "_MSP: \n"
          "mrs R0, MSP \n"
          "b HardFault");
}