# Rust initialization code: it better be inlined

"It works in *release* mode, but not in *debug* mode," I repeated and repeated, staring at the disassembly from a small Rust program. The binary boots the Teensy 4 by preparing the tightly-coupled memory (TCM) regions, setting up the interrupt table, and twiddling some floating-point unit registers before jumping to `main()` and enabling the LED. Optimizations make everything work; the output code looks great. But, so does the unoptimized version; there's less inlined code, but that should be ok, right?

It wasn't OK. Code placement matters, and it better be inlined if it needs to configure TCM.

## Tightly-Coupled Memory on the Teensy 4

Tightly-coupled memory (TCM) is low-latency memory, allowing for faster access without the unpredictability of a cache. On the Teensy 4, I can configure one TCM region for instructions (ITCM), and another TCM region for data (DTCM). Each region can be sized up to 512KiB. The [`teensy4-rt` crate] in the `teensy4-rs` project provides the memory layout for the Teensy 4, and it decides to use the maximum size for each region. Given its ideal latency properties, it makes sense to use the ITCM and DTCM regions as much as possible. I designed the runtime crate to place all `.text` into ITCM, and all `.data` into DTCM.

The implications of "puts all `.text` into ITCM" weren't initially apparent. I had been compiling all my startup and application code in `release` mode, which allowed the Rust compiler to agressively inline functions. As soon as I switched to debug mode, the compiler didn't inline all functions. I knew that I was relying on inlined functions for my own start-up routines. But, what I hadn't realized is that I was also relying on inlining of *Rust* functions, like [`ptr::offset`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset-1) and [`ptr::write`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write). My startup code could never work if it were written in Rust.

## System startup

Although it might vary from system to system, the startup of an embedded system typically includes a zero of the `.bss` section and a copy of data from flash to RAM. In fact, these are two of the first things that happen in the `Reset()` handler provided by the [`cortex-m-rt` crate], the Rust crate for ARM Cortex-M startup.

```rust
#[link_section = ".reset"]
#[no_mangle]
unsafe extern "C" Reset() -> ! {
    extern "C" { // Symbols specified in a linker script
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;
    }

    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);

    // ...
}
```

Zeroing `.bss` and copying data from flash to RAM is provided by two functions in the [`r0` crate]:

```rust
use core::{mem, ptr};

pub unsafe fn zero_bss<T>(mut sbss: *mut T, ebss: *mut T)
where
    T: Copy,
{
    while sbss < ebss {
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }
}

pub unsafe fn init_data<T>(
    mut sdata: *mut T,
    edata: *mut T,
    mut sidata: *const T,
) where
    T: Copy,
{
    while sdata < edata {
        ptr::write(sdata, ptr::read(sidata));
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}
```

The first function, `zero_bss`, sets the memory between pointers `sbss` and `ebss` to zero using a combination of `ptr::write_volatile` and `mem::zeroed`. `init_data` copies data from `sidata` into `sdata` by the number of `T`-sized elements between `sdata` and `edata`. The pair of `ptr::read` and `ptr::write` functions comprise the copy, then `ptr::offset` advances to the next `T` element.

For my Teensy 4 system with TCM, I have the same setup, but with one more step:

1. Zero `.bss` section
2. Copy data from FLASH to DTCM
3. Copy instructions from FLASH to ITCM

Step 2 is effectively the same, only I'm copying into DTCM instead of RAM. Step 3 is new. I have to copy instructions into the ITCM region.

```rust
#[link_section = ".reset"]
#[no_mangle]
unsafe extern "C" Reset() -> ! {
    extern "C" { // Symbols specified in a linker script
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;

        // === NEW ===
        static mut __stext: u32;
        static mut __etext: u32;
        static __sitext: u32;
    }

    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);
    // === NEW ===
    r0::init_data(&mut __stext, &mut __etext, &__sitext);
    // ...
}
```

The new `__stext` and `__etext` delimit the ITCM region that will eventually hold instructions. Although the function is called `init_data`, I can use it to initialize the `.text` region, since the behavior is the same: copy values from one region to another region.

The first issues I noticed: `r0::zero_bss` and `r0::init_data` aren't decorated with `#[inline(always)]` in the `r0` crate. When compiling without optimizations, these functions might not be inlined. And, since they are normal functions without any `#[link_section]` placement, they will end up in ITCM with all the other functions. It's a catch 22: I need the two functions to prepare ITCM, but the functions are executed from ITCM.

Inlining those two functions is not enough, because there's a bigger problem: the `ptr` and `mem` functions that the `r0` functions use, like `ptr::write`, `ptr::offset`, and `mem::zeroed`,  may not be inlined! Moreover, I can't control their placement in memory, since they're part of the Rust core library. By requiring functions just to increment a pointer, Rust prevents me from writing the code I require to boot my system. This wouldn't be an issue in C...

## Write the startup in C

And that was my workaround. I wrote the system initialization in C, where I don't need functions to work with memory:

```c
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
  // Copy text and data into TCM regions
  zero_bss(&__sbss, &__ebss);
  init_data(&__sdata, &__edata, &__sidata);
  init_data(&__stext, &__etext, &__sitext);
  
  // ...

  // Jump into Rust
  _start();
}
```

The forced inline places the initialization code directly into the `_reset` reset handler. More importantly, my ability to increment addressess, load from memory, and store into memory isn't affected by the compiler's ability to inline code. The code is truly zero-cost and behaves consistently across optimization settings.

[`teensy4-rt` crate]: https://github.com/mciantyre/teensy4-rs
[`cortex-m-rt` crate]: https://crates.io/crates/cortex-m-rt
[`r0` crate]: https://crates.io/crates/r0