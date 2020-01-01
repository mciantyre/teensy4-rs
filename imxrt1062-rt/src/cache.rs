//! Cache configuration

// Visually appealing constructs like (1 << 0) or (0x0000_0000 | [non-zero number])
#![allow(clippy::identity_op)]

use core::ptr;
use cortex_m::asm;

const SCB_CCR: *mut u32 = 0xE000_ED14 as *mut u32;
const SCB_MPU_CTRL: *mut u32 = 0xE000_ED94 as *mut u32;
const SCB_MPU_RBAR: *mut u32 = 0xE000_ED9C as *mut u32;
const SCB_MPU_RASR: *mut u32 = 0xE000_EDA0 as *mut u32;
const SCB_CACHE_ICIALLU: *mut u32 = 0xE000_EF50 as *mut u32;
const SCB_CCR_IC: u32 = 1 << 17;
const SCB_CCR_DC: u32 = 1 << 16;

const SCB_MPU_RBAR_VALID: u32 = 1 << 4;
const SCB_MPU_RASR_ENABLE: u32 = 1 << 0;
const SCB_MPU_RASR_XN: u32 = 1 << 28;
const fn scb_mpu_rasr_ap(n: u32) -> u32 {
    (n & 7) << 24
}
const fn scb_mpu_rasr_tex(n: u32) -> u32 {
    (n & 7) << 19
}
const SCB_MPU_RASR_C: u32 = 1 << 17;
const SCB_MPU_RASR_B: u32 = 1 << 16;
const SCB_MPU_CTRL_ENABLE: u32 = 1 << 0;
const fn scb_mpu_rasr_size(n: u32) -> u32 {
    (n & 31) << 1
}
const fn scb_mpu_rbar_region(n: u32) -> u32 {
    n & 15
}

const NOEXEC: u32 = SCB_MPU_RASR_XN;
const READONLY: u32 = scb_mpu_rasr_ap(7);
const READWRITE: u32 = scb_mpu_rasr_ap(3);
const _NOACCESS: u32 = scb_mpu_rasr_ap(0);
const MEM_CACHE_WT: u32 = scb_mpu_rasr_tex(0) | SCB_MPU_RASR_C;
const _MEM_CACHE_WB: u32 = scb_mpu_rasr_tex(0) | SCB_MPU_RASR_C | SCB_MPU_RASR_B;
const MEM_CACHE_WBWA: u32 = scb_mpu_rasr_tex(1) | SCB_MPU_RASR_C | SCB_MPU_RASR_B;
const MEM_NOCACHE: u32 = scb_mpu_rasr_tex(1);
const DEV_NOCACHE: u32 = scb_mpu_rasr_tex(2);
const SIZE_128K: u32 = scb_mpu_rasr_size(16) | SCB_MPU_RASR_ENABLE;
const _SIZE_256K: u32 = scb_mpu_rasr_size(17) | SCB_MPU_RASR_ENABLE;
const SIZE_512K: u32 = scb_mpu_rasr_size(18) | SCB_MPU_RASR_ENABLE;
const SIZE_1M: u32 = scb_mpu_rasr_size(19) | SCB_MPU_RASR_ENABLE;
const _SIZE_2M: u32 = scb_mpu_rasr_size(20) | SCB_MPU_RASR_ENABLE;
const _SIZE_4M: u32 = scb_mpu_rasr_size(21) | SCB_MPU_RASR_ENABLE;
const _SIZE_8M: u32 = scb_mpu_rasr_size(22) | SCB_MPU_RASR_ENABLE;
const SIZE_16M: u32 = scb_mpu_rasr_size(23) | SCB_MPU_RASR_ENABLE;
const _SIZE_32M: u32 = scb_mpu_rasr_size(24) | SCB_MPU_RASR_ENABLE;
const SIZE_64M: u32 = scb_mpu_rasr_size(25) | SCB_MPU_RASR_ENABLE;
const fn region(n: u32) -> u32 {
    scb_mpu_rbar_region(n) | SCB_MPU_RBAR_VALID
}

#[inline(always)]
pub unsafe fn init() {
    ptr::write_volatile(SCB_MPU_CTRL, 0);

    ptr::write_volatile(SCB_MPU_RBAR, 0x0000_0000 | region(0)); // ITCM
    ptr::write_volatile(SCB_MPU_RASR, MEM_NOCACHE | READWRITE | SIZE_512K);

    ptr::write_volatile(SCB_MPU_RBAR, 0x0020_0000 | region(1)); // Boot ROM
    ptr::write_volatile(SCB_MPU_RASR, MEM_CACHE_WT | READONLY | SIZE_128K);

    ptr::write_volatile(SCB_MPU_RBAR, 0x2000_0000 | region(2)); // DTCM
    ptr::write_volatile(SCB_MPU_RASR, MEM_NOCACHE | READWRITE | NOEXEC | SIZE_512K);

    ptr::write_volatile(SCB_MPU_RBAR, 0x2020_0000 | region(3)); // RAM (AXI bus)
    ptr::write_volatile(SCB_MPU_RASR, MEM_CACHE_WBWA | READWRITE | NOEXEC | SIZE_1M);

    ptr::write_volatile(SCB_MPU_RBAR, 0x4000_0000 | region(4)); // Peripherals
    ptr::write_volatile(SCB_MPU_RASR, DEV_NOCACHE | READWRITE | NOEXEC | SIZE_64M);

    ptr::write_volatile(SCB_MPU_RBAR, 0x6000_0000 | region(5)); // QSPI Flash
    ptr::write_volatile(SCB_MPU_RASR, MEM_CACHE_WBWA | READONLY | SIZE_16M);

    ptr::write_volatile(SCB_MPU_CTRL, SCB_MPU_CTRL_ENABLE);

    asm::dsb();
    asm::isb();
    ptr::write_volatile(SCB_CACHE_ICIALLU, 0);

    asm::dsb();
    asm::isb();
    ptr::write_volatile(SCB_CCR, SCB_CCR_IC | SCB_CCR_DC);
}
