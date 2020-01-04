#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Auxiliary Control Register,"]
    pub actlr: ACTLR,
    _reserved1: [u8; 3316usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status register"]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: DFSR,
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    _reserved16: [u8; 4usize],
    #[doc = "0xd40 - Processor Feature Register 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0xd44 - Processor Feature Register 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0xd48 - Debug Feature Register"]
    pub id_dfr0: ID_DFR0,
    #[doc = "0xd4c - Auxiliary Feature Register"]
    pub id_afr0: ID_AFR0,
    #[doc = "0xd50 - Memory Model Feature Register 0"]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0xd54 - Memory Model Feature Register 1"]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0xd58 - Memory Model Feature Register 2"]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0xd5c - Memory Model Feature Register 3"]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0xd60 - Instruction Set Attributes Register 0"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0xd64 - Instruction Set Attributes Register 1"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0xd68 - Instruction Set Attributes Register 2"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0xd6c - Instruction Set Attributes Register 3"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0xd70 - Instruction Set Attributes Register 4"]
    pub id_isar4: ID_ISAR4,
    _reserved29: [u8; 4usize],
    #[doc = "0xd78 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0xd7c - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0xd80 - Cache Size ID Register"]
    pub ccsidr: CCSIDR,
    #[doc = "0xd84 - Cache Size Selection Register"]
    pub csselr: CSSELR,
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved34: [u8; 372usize],
    #[doc = "0xf00 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub stir: STIR,
    _reserved35: [u8; 76usize],
    #[doc = "0xf50 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub iciallu: ICIALLU,
    _reserved36: [u8; 4usize],
    #[doc = "0xf58 - Instruction cache invalidate by address to PoU"]
    pub icimvau: ICIMVAU,
    #[doc = "0xf5c - Data cache invalidate by address to Point of Coherency (PoC)"]
    pub dcimvac: DCIMVAC,
    #[doc = "0xf60 - Data cache invalidate by set/way"]
    pub dcisw: DCISW,
    #[doc = "0xf64 - Data cache by address to PoU"]
    pub dccmvau: DCCMVAU,
    #[doc = "0xf68 - Data cache clean by address to PoC"]
    pub dccmvac: DCCMVAC,
    #[doc = "0xf6c - Data cache clean by set/way"]
    pub dccsw: DCCSW,
    #[doc = "0xf70 - Data cache clean and invalidate by address to PoC"]
    pub dccimvac: DCCIMVAC,
    #[doc = "0xf74 - Data cache clean and invalidate by set/way"]
    pub dccisw: DCCISW,
    _reserved44: [u8; 24usize],
    #[doc = "0xf90 - Instruction Tightly-Coupled Memory Control Register"]
    pub cm7_itcmcr: CM7_ITCMCR,
    #[doc = "0xf94 - Data Tightly-Coupled Memory Control Register"]
    pub cm7_dtcmcr: CM7_DTCMCR,
    #[doc = "0xf98 - AHBP Control Register"]
    pub cm7_ahbpcr: CM7_AHBPCR,
    #[doc = "0xf9c - L1 Cache Control Register"]
    pub cm7_cacr: CM7_CACR,
    #[doc = "0xfa0 - AHB Slave Control Register"]
    pub cm7_ahbscr: CM7_AHBSCR,
    _reserved49: [u8; 4usize],
    #[doc = "0xfa8 - Auxiliary Bus Fault Status Register"]
    pub cm7_abfsr: CM7_ABFSR,
}
#[doc = "Auxiliary Control Register,\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](actlr) module"]
pub type ACTLR = crate::Reg<u32, _ACTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTLR;
#[doc = "`read()` method returns [actlr::R](actlr::R) reader structure"]
impl crate::Readable for ACTLR {}
#[doc = "`write(|w| ..)` method takes [actlr::W](actlr::W) writer structure"]
impl crate::Writable for ACTLR {}
#[doc = "Auxiliary Control Register,"]
pub mod actlr;
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Vector Table Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtor](vtor) module"]
pub type VTOR = crate::Reg<u32, _VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTOR;
#[doc = "`read()` method returns [vtor::R](vtor::R) reader structure"]
impl crate::Readable for VTOR {}
#[doc = "`write(|w| ..)` method takes [vtor::W](vtor::W) writer structure"]
impl crate::Writable for VTOR {}
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr1](shpr1) module"]
pub type SHPR1 = crate::Reg<u32, _SHPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR1;
#[doc = "`read()` method returns [shpr1::R](shpr1::R) reader structure"]
impl crate::Readable for SHPR1 {}
#[doc = "`write(|w| ..)` method takes [shpr1::W](shpr1::W) writer structure"]
impl crate::Writable for SHPR1 {}
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "Configurable Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](cfsr) module"]
pub type CFSR = crate::Reg<u32, _CFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFSR;
#[doc = "`read()` method returns [cfsr::R](cfsr::R) reader structure"]
impl crate::Readable for CFSR {}
#[doc = "`write(|w| ..)` method takes [cfsr::W](cfsr::W) writer structure"]
impl crate::Writable for CFSR {}
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HardFault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](hfsr) module"]
pub type HFSR = crate::Reg<u32, _HFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFSR;
#[doc = "`read()` method returns [hfsr::R](hfsr::R) reader structure"]
impl crate::Readable for HFSR {}
#[doc = "`write(|w| ..)` method takes [hfsr::W](hfsr::W) writer structure"]
impl crate::Writable for HFSR {}
#[doc = "HardFault Status register"]
pub mod hfsr;
#[doc = "Debug Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](dfsr) module"]
pub type DFSR = crate::Reg<u32, _DFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSR;
#[doc = "`read()` method returns [dfsr::R](dfsr::R) reader structure"]
impl crate::Readable for DFSR {}
#[doc = "`write(|w| ..)` method takes [dfsr::W](dfsr::W) writer structure"]
impl crate::Writable for DFSR {}
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MemManage Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfar](mmfar) module"]
pub type MMFAR = crate::Reg<u32, _MMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFAR;
#[doc = "`read()` method returns [mmfar::R](mmfar::R) reader structure"]
impl crate::Readable for MMFAR {}
#[doc = "`write(|w| ..)` method takes [mmfar::W](mmfar::W) writer structure"]
impl crate::Writable for MMFAR {}
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BusFault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfar](bfar) module"]
pub type BFAR = crate::Reg<u32, _BFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFAR;
#[doc = "`read()` method returns [bfar::R](bfar::R) reader structure"]
impl crate::Readable for BFAR {}
#[doc = "`write(|w| ..)` method takes [bfar::W](bfar::W) writer structure"]
impl crate::Writable for BFAR {}
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "Processor Feature Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr0](id_pfr0) module"]
pub type ID_PFR0 = crate::Reg<u32, _ID_PFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_PFR0;
#[doc = "`read()` method returns [id_pfr0::R](id_pfr0::R) reader structure"]
impl crate::Readable for ID_PFR0 {}
#[doc = "Processor Feature Register 0"]
pub mod id_pfr0;
#[doc = "Processor Feature Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr1](id_pfr1) module"]
pub type ID_PFR1 = crate::Reg<u32, _ID_PFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_PFR1;
#[doc = "`read()` method returns [id_pfr1::R](id_pfr1::R) reader structure"]
impl crate::Readable for ID_PFR1 {}
#[doc = "Processor Feature Register 1"]
pub mod id_pfr1;
#[doc = "Debug Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_dfr0](id_dfr0) module"]
pub type ID_DFR0 = crate::Reg<u32, _ID_DFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_DFR0;
#[doc = "`read()` method returns [id_dfr0::R](id_dfr0::R) reader structure"]
impl crate::Readable for ID_DFR0 {}
#[doc = "Debug Feature Register"]
pub mod id_dfr0;
#[doc = "Auxiliary Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_afr0](id_afr0) module"]
pub type ID_AFR0 = crate::Reg<u32, _ID_AFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_AFR0;
#[doc = "`read()` method returns [id_afr0::R](id_afr0::R) reader structure"]
impl crate::Readable for ID_AFR0 {}
#[doc = "Auxiliary Feature Register"]
pub mod id_afr0;
#[doc = "Memory Model Feature Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr0](id_mmfr0) module"]
pub type ID_MMFR0 = crate::Reg<u32, _ID_MMFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR0;
#[doc = "`read()` method returns [id_mmfr0::R](id_mmfr0::R) reader structure"]
impl crate::Readable for ID_MMFR0 {}
#[doc = "Memory Model Feature Register 0"]
pub mod id_mmfr0;
#[doc = "Memory Model Feature Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr1](id_mmfr1) module"]
pub type ID_MMFR1 = crate::Reg<u32, _ID_MMFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR1;
#[doc = "`read()` method returns [id_mmfr1::R](id_mmfr1::R) reader structure"]
impl crate::Readable for ID_MMFR1 {}
#[doc = "Memory Model Feature Register 1"]
pub mod id_mmfr1;
#[doc = "Memory Model Feature Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr2](id_mmfr2) module"]
pub type ID_MMFR2 = crate::Reg<u32, _ID_MMFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR2;
#[doc = "`read()` method returns [id_mmfr2::R](id_mmfr2::R) reader structure"]
impl crate::Readable for ID_MMFR2 {}
#[doc = "Memory Model Feature Register 2"]
pub mod id_mmfr2;
#[doc = "Memory Model Feature Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr3](id_mmfr3) module"]
pub type ID_MMFR3 = crate::Reg<u32, _ID_MMFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR3;
#[doc = "`read()` method returns [id_mmfr3::R](id_mmfr3::R) reader structure"]
impl crate::Readable for ID_MMFR3 {}
#[doc = "Memory Model Feature Register 3"]
pub mod id_mmfr3;
#[doc = "Instruction Set Attributes Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar0](id_isar0) module"]
pub type ID_ISAR0 = crate::Reg<u32, _ID_ISAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR0;
#[doc = "`read()` method returns [id_isar0::R](id_isar0::R) reader structure"]
impl crate::Readable for ID_ISAR0 {}
#[doc = "Instruction Set Attributes Register 0"]
pub mod id_isar0;
#[doc = "Instruction Set Attributes Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar1](id_isar1) module"]
pub type ID_ISAR1 = crate::Reg<u32, _ID_ISAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR1;
#[doc = "`read()` method returns [id_isar1::R](id_isar1::R) reader structure"]
impl crate::Readable for ID_ISAR1 {}
#[doc = "Instruction Set Attributes Register 1"]
pub mod id_isar1;
#[doc = "Instruction Set Attributes Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar2](id_isar2) module"]
pub type ID_ISAR2 = crate::Reg<u32, _ID_ISAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR2;
#[doc = "`read()` method returns [id_isar2::R](id_isar2::R) reader structure"]
impl crate::Readable for ID_ISAR2 {}
#[doc = "Instruction Set Attributes Register 2"]
pub mod id_isar2;
#[doc = "Instruction Set Attributes Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar3](id_isar3) module"]
pub type ID_ISAR3 = crate::Reg<u32, _ID_ISAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR3;
#[doc = "`read()` method returns [id_isar3::R](id_isar3::R) reader structure"]
impl crate::Readable for ID_ISAR3 {}
#[doc = "Instruction Set Attributes Register 3"]
pub mod id_isar3;
#[doc = "Instruction Set Attributes Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar4](id_isar4) module"]
pub type ID_ISAR4 = crate::Reg<u32, _ID_ISAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR4;
#[doc = "`read()` method returns [id_isar4::R](id_isar4::R) reader structure"]
impl crate::Readable for ID_ISAR4 {}
#[doc = "Instruction Set Attributes Register 4"]
pub mod id_isar4;
#[doc = "Cache Level ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clidr](clidr) module"]
pub type CLIDR = crate::Reg<u32, _CLIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLIDR;
#[doc = "`read()` method returns [clidr::R](clidr::R) reader structure"]
impl crate::Readable for CLIDR {}
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "Cache Type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "Cache Size ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsidr](ccsidr) module"]
pub type CCSIDR = crate::Reg<u32, _CCSIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCSIDR;
#[doc = "`read()` method returns [ccsidr::R](ccsidr::R) reader structure"]
impl crate::Readable for CCSIDR {}
#[doc = "Cache Size ID Register"]
pub mod ccsidr;
#[doc = "Cache Size Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csselr](csselr) module"]
pub type CSSELR = crate::Reg<u32, _CSSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSSELR;
#[doc = "`read()` method returns [csselr::R](csselr::R) reader structure"]
impl crate::Readable for CSSELR {}
#[doc = "`write(|w| ..)` method takes [csselr::W](csselr::W) writer structure"]
impl crate::Writable for CSSELR {}
#[doc = "Cache Size Selection Register"]
pub mod csselr;
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](cpacr) module"]
pub type CPACR = crate::Reg<u32, _CPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPACR;
#[doc = "`read()` method returns [cpacr::R](cpacr::R) reader structure"]
impl crate::Readable for CPACR {}
#[doc = "`write(|w| ..)` method takes [cpacr::W](cpacr::W) writer structure"]
impl crate::Writable for CPACR {}
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stir](stir) module"]
pub type STIR = crate::Reg<u32, _STIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIR;
#[doc = "`read()` method returns [stir::R](stir::R) reader structure"]
impl crate::Readable for STIR {}
#[doc = "`write(|w| ..)` method takes [stir::W](stir::W) writer structure"]
impl crate::Writable for STIR {}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod stir;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iciallu](iciallu) module"]
pub type ICIALLU = crate::Reg<u32, _ICIALLU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICIALLU;
#[doc = "`write(|w| ..)` method takes [iciallu::W](iciallu::W) writer structure"]
impl crate::Writable for ICIALLU {}
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod iciallu;
#[doc = "Instruction cache invalidate by address to PoU\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icimvau](icimvau) module"]
pub type ICIMVAU = crate::Reg<u32, _ICIMVAU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICIMVAU;
#[doc = "`write(|w| ..)` method takes [icimvau::W](icimvau::W) writer structure"]
impl crate::Writable for ICIMVAU {}
#[doc = "Instruction cache invalidate by address to PoU"]
pub mod icimvau;
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcimvac](dcimvac) module"]
pub type DCIMVAC = crate::Reg<u32, _DCIMVAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIMVAC;
#[doc = "`write(|w| ..)` method takes [dcimvac::W](dcimvac::W) writer structure"]
impl crate::Writable for DCIMVAC {}
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
pub mod dcimvac;
#[doc = "Data cache invalidate by set/way\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcisw](dcisw) module"]
pub type DCISW = crate::Reg<u32, _DCISW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCISW;
#[doc = "`write(|w| ..)` method takes [dcisw::W](dcisw::W) writer structure"]
impl crate::Writable for DCISW {}
#[doc = "Data cache invalidate by set/way"]
pub mod dcisw;
#[doc = "Data cache by address to PoU\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmvau](dccmvau) module"]
pub type DCCMVAU = crate::Reg<u32, _DCCMVAU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMVAU;
#[doc = "`write(|w| ..)` method takes [dccmvau::W](dccmvau::W) writer structure"]
impl crate::Writable for DCCMVAU {}
#[doc = "Data cache by address to PoU"]
pub mod dccmvau;
#[doc = "Data cache clean by address to PoC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmvac](dccmvac) module"]
pub type DCCMVAC = crate::Reg<u32, _DCCMVAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCMVAC;
#[doc = "`write(|w| ..)` method takes [dccmvac::W](dccmvac::W) writer structure"]
impl crate::Writable for DCCMVAC {}
#[doc = "Data cache clean by address to PoC"]
pub mod dccmvac;
#[doc = "Data cache clean by set/way\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccsw](dccsw) module"]
pub type DCCSW = crate::Reg<u32, _DCCSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCSW;
#[doc = "`write(|w| ..)` method takes [dccsw::W](dccsw::W) writer structure"]
impl crate::Writable for DCCSW {}
#[doc = "Data cache clean by set/way"]
pub mod dccsw;
#[doc = "Data cache clean and invalidate by address to PoC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccimvac](dccimvac) module"]
pub type DCCIMVAC = crate::Reg<u32, _DCCIMVAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCIMVAC;
#[doc = "`write(|w| ..)` method takes [dccimvac::W](dccimvac::W) writer structure"]
impl crate::Writable for DCCIMVAC {}
#[doc = "Data cache clean and invalidate by address to PoC"]
pub mod dccimvac;
#[doc = "Data cache clean and invalidate by set/way\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccisw](dccisw) module"]
pub type DCCISW = crate::Reg<u32, _DCCISW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCISW;
#[doc = "`write(|w| ..)` method takes [dccisw::W](dccisw::W) writer structure"]
impl crate::Writable for DCCISW {}
#[doc = "Data cache clean and invalidate by set/way"]
pub mod dccisw;
#[doc = "Instruction Tightly-Coupled Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_itcmcr](cm7_itcmcr) module"]
pub type CM7_ITCMCR = crate::Reg<u32, _CM7_ITCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_ITCMCR;
#[doc = "`read()` method returns [cm7_itcmcr::R](cm7_itcmcr::R) reader structure"]
impl crate::Readable for CM7_ITCMCR {}
#[doc = "`write(|w| ..)` method takes [cm7_itcmcr::W](cm7_itcmcr::W) writer structure"]
impl crate::Writable for CM7_ITCMCR {}
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
pub mod cm7_itcmcr;
#[doc = "Data Tightly-Coupled Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_dtcmcr](cm7_dtcmcr) module"]
pub type CM7_DTCMCR = crate::Reg<u32, _CM7_DTCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_DTCMCR;
#[doc = "`read()` method returns [cm7_dtcmcr::R](cm7_dtcmcr::R) reader structure"]
impl crate::Readable for CM7_DTCMCR {}
#[doc = "`write(|w| ..)` method takes [cm7_dtcmcr::W](cm7_dtcmcr::W) writer structure"]
impl crate::Writable for CM7_DTCMCR {}
#[doc = "Data Tightly-Coupled Memory Control Register"]
pub mod cm7_dtcmcr;
#[doc = "AHBP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_ahbpcr](cm7_ahbpcr) module"]
pub type CM7_AHBPCR = crate::Reg<u32, _CM7_AHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_AHBPCR;
#[doc = "`read()` method returns [cm7_ahbpcr::R](cm7_ahbpcr::R) reader structure"]
impl crate::Readable for CM7_AHBPCR {}
#[doc = "`write(|w| ..)` method takes [cm7_ahbpcr::W](cm7_ahbpcr::W) writer structure"]
impl crate::Writable for CM7_AHBPCR {}
#[doc = "AHBP Control Register"]
pub mod cm7_ahbpcr;
#[doc = "L1 Cache Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_cacr](cm7_cacr) module"]
pub type CM7_CACR = crate::Reg<u32, _CM7_CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_CACR;
#[doc = "`read()` method returns [cm7_cacr::R](cm7_cacr::R) reader structure"]
impl crate::Readable for CM7_CACR {}
#[doc = "`write(|w| ..)` method takes [cm7_cacr::W](cm7_cacr::W) writer structure"]
impl crate::Writable for CM7_CACR {}
#[doc = "L1 Cache Control Register"]
pub mod cm7_cacr;
#[doc = "AHB Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_ahbscr](cm7_ahbscr) module"]
pub type CM7_AHBSCR = crate::Reg<u32, _CM7_AHBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_AHBSCR;
#[doc = "`read()` method returns [cm7_ahbscr::R](cm7_ahbscr::R) reader structure"]
impl crate::Readable for CM7_AHBSCR {}
#[doc = "`write(|w| ..)` method takes [cm7_ahbscr::W](cm7_ahbscr::W) writer structure"]
impl crate::Writable for CM7_AHBSCR {}
#[doc = "AHB Slave Control Register"]
pub mod cm7_ahbscr;
#[doc = "Auxiliary Bus Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_abfsr](cm7_abfsr) module"]
pub type CM7_ABFSR = crate::Reg<u32, _CM7_ABFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM7_ABFSR;
#[doc = "`read()` method returns [cm7_abfsr::R](cm7_abfsr::R) reader structure"]
impl crate::Readable for CM7_ABFSR {}
#[doc = "`write(|w| ..)` method takes [cm7_abfsr::W](cm7_abfsr::W) writer structure"]
impl crate::Writable for CM7_ABFSR {}
#[doc = "Auxiliary Bus Fault Status Register"]
pub mod cm7_abfsr;
