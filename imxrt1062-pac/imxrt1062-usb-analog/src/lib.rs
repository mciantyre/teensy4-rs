#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 416usize],
    #[doc = "0x1a0 - USB VBUS Detect Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0x1a4 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0x1a8 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0x1ac - USB VBUS Detect Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0x1b0 - USB Charger Detect Register"]
    pub usb1_chrg_detect: USB1_CHRG_DETECT,
    #[doc = "0x1b4 - USB Charger Detect Register"]
    pub usb1_chrg_detect_set: USB1_CHRG_DETECT_SET,
    #[doc = "0x1b8 - USB Charger Detect Register"]
    pub usb1_chrg_detect_clr: USB1_CHRG_DETECT_CLR,
    #[doc = "0x1bc - USB Charger Detect Register"]
    pub usb1_chrg_detect_tog: USB1_CHRG_DETECT_TOG,
    #[doc = "0x1c0 - USB VBUS Detect Status Register"]
    pub usb1_vbus_detect_stat: USB1_VBUS_DETECT_STAT,
    _reserved9: [u8; 12usize],
    #[doc = "0x1d0 - USB Charger Detect Status Register"]
    pub usb1_chrg_detect_stat: USB1_CHRG_DETECT_STAT,
    _reserved10: [u8; 28usize],
    #[doc = "0x1f0 - USB Misc Register"]
    pub usb1_misc: USB1_MISC,
    #[doc = "0x1f4 - USB Misc Register"]
    pub usb1_misc_set: USB1_MISC_SET,
    #[doc = "0x1f8 - USB Misc Register"]
    pub usb1_misc_clr: USB1_MISC_CLR,
    #[doc = "0x1fc - USB Misc Register"]
    pub usb1_misc_tog: USB1_MISC_TOG,
    #[doc = "0x200 - USB VBUS Detect Register"]
    pub usb2_vbus_detect: USB2_VBUS_DETECT,
    #[doc = "0x204 - USB VBUS Detect Register"]
    pub usb2_vbus_detect_set: USB2_VBUS_DETECT_SET,
    #[doc = "0x208 - USB VBUS Detect Register"]
    pub usb2_vbus_detect_clr: USB2_VBUS_DETECT_CLR,
    #[doc = "0x20c - USB VBUS Detect Register"]
    pub usb2_vbus_detect_tog: USB2_VBUS_DETECT_TOG,
    #[doc = "0x210 - USB Charger Detect Register"]
    pub usb2_chrg_detect: USB2_CHRG_DETECT,
    #[doc = "0x214 - USB Charger Detect Register"]
    pub usb2_chrg_detect_set: USB2_CHRG_DETECT_SET,
    #[doc = "0x218 - USB Charger Detect Register"]
    pub usb2_chrg_detect_clr: USB2_CHRG_DETECT_CLR,
    #[doc = "0x21c - USB Charger Detect Register"]
    pub usb2_chrg_detect_tog: USB2_CHRG_DETECT_TOG,
    #[doc = "0x220 - USB VBUS Detect Status Register"]
    pub usb2_vbus_detect_stat: USB2_VBUS_DETECT_STAT,
    _reserved23: [u8; 12usize],
    #[doc = "0x230 - USB Charger Detect Status Register"]
    pub usb2_chrg_detect_stat: USB2_CHRG_DETECT_STAT,
    _reserved24: [u8; 28usize],
    #[doc = "0x250 - USB Misc Register"]
    pub usb2_misc: USB2_MISC,
    #[doc = "0x254 - USB Misc Register"]
    pub usb2_misc_set: USB2_MISC_SET,
    #[doc = "0x258 - USB Misc Register"]
    pub usb2_misc_clr: USB2_MISC_CLR,
    #[doc = "0x25c - USB Misc Register"]
    pub usb2_misc_tog: USB2_MISC_TOG,
    #[doc = "0x260 - Chip Silicon Version"]
    pub digprog: DIGPROG,
}
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect](usb1_vbus_detect) module"]
pub type USB1_VBUS_DETECT = crate::Reg<u32, _USB1_VBUS_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT;
#[doc = "`read()` method returns [usb1_vbus_detect::R](usb1_vbus_detect::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect::W](usb1_vbus_detect::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT {}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_set](usb1_vbus_detect_set) module"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<u32, _USB1_VBUS_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_SET;
#[doc = "`read()` method returns [usb1_vbus_detect_set::R](usb1_vbus_detect_set::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_set::W](usb1_vbus_detect_set::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SET {}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_clr](usb1_vbus_detect_clr) module"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<u32, _USB1_VBUS_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_CLR;
#[doc = "`read()` method returns [usb1_vbus_detect_clr::R](usb1_vbus_detect_clr::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_clr::W](usb1_vbus_detect_clr::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_CLR {}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_tog](usb1_vbus_detect_tog) module"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<u32, _USB1_VBUS_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_TOG;
#[doc = "`read()` method returns [usb1_vbus_detect_tog::R](usb1_vbus_detect_tog::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_tog::W](usb1_vbus_detect_tog::W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_TOG {}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect](usb1_chrg_detect) module"]
pub type USB1_CHRG_DETECT = crate::Reg<u32, _USB1_CHRG_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT;
#[doc = "`read()` method returns [usb1_chrg_detect::R](usb1_chrg_detect::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect::W](usb1_chrg_detect::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT {}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_set](usb1_chrg_detect_set) module"]
pub type USB1_CHRG_DETECT_SET = crate::Reg<u32, _USB1_CHRG_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_SET;
#[doc = "`read()` method returns [usb1_chrg_detect_set::R](usb1_chrg_detect_set::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_set::W](usb1_chrg_detect_set::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_SET {}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_set;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_clr](usb1_chrg_detect_clr) module"]
pub type USB1_CHRG_DETECT_CLR = crate::Reg<u32, _USB1_CHRG_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_CLR;
#[doc = "`read()` method returns [usb1_chrg_detect_clr::R](usb1_chrg_detect_clr::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_clr::W](usb1_chrg_detect_clr::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_CLR {}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_tog](usb1_chrg_detect_tog) module"]
pub type USB1_CHRG_DETECT_TOG = crate::Reg<u32, _USB1_CHRG_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_TOG;
#[doc = "`read()` method returns [usb1_chrg_detect_tog::R](usb1_chrg_detect_tog::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_tog::W](usb1_chrg_detect_tog::W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_TOG {}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB VBUS Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_stat](usb1_vbus_detect_stat) module"]
pub type USB1_VBUS_DETECT_STAT = crate::Reg<u32, _USB1_VBUS_DETECT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_VBUS_DETECT_STAT;
#[doc = "`read()` method returns [usb1_vbus_detect_stat::R](usb1_vbus_detect_stat::R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_STAT {}
#[doc = "USB VBUS Detect Status Register"]
pub mod usb1_vbus_detect_stat;
#[doc = "USB Charger Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_stat](usb1_chrg_detect_stat) module"]
pub type USB1_CHRG_DETECT_STAT = crate::Reg<u32, _USB1_CHRG_DETECT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_CHRG_DETECT_STAT;
#[doc = "`read()` method returns [usb1_chrg_detect_stat::R](usb1_chrg_detect_stat::R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_STAT {}
#[doc = "USB Charger Detect Status Register"]
pub mod usb1_chrg_detect_stat;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_misc](usb1_misc) module"]
pub type USB1_MISC = crate::Reg<u32, _USB1_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_MISC;
#[doc = "`read()` method returns [usb1_misc::R](usb1_misc::R) reader structure"]
impl crate::Readable for USB1_MISC {}
#[doc = "`write(|w| ..)` method takes [usb1_misc::W](usb1_misc::W) writer structure"]
impl crate::Writable for USB1_MISC {}
#[doc = "USB Misc Register"]
pub mod usb1_misc;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_misc_set](usb1_misc_set) module"]
pub type USB1_MISC_SET = crate::Reg<u32, _USB1_MISC_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_MISC_SET;
#[doc = "`read()` method returns [usb1_misc_set::R](usb1_misc_set::R) reader structure"]
impl crate::Readable for USB1_MISC_SET {}
#[doc = "`write(|w| ..)` method takes [usb1_misc_set::W](usb1_misc_set::W) writer structure"]
impl crate::Writable for USB1_MISC_SET {}
#[doc = "USB Misc Register"]
pub mod usb1_misc_set;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_misc_clr](usb1_misc_clr) module"]
pub type USB1_MISC_CLR = crate::Reg<u32, _USB1_MISC_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_MISC_CLR;
#[doc = "`read()` method returns [usb1_misc_clr::R](usb1_misc_clr::R) reader structure"]
impl crate::Readable for USB1_MISC_CLR {}
#[doc = "`write(|w| ..)` method takes [usb1_misc_clr::W](usb1_misc_clr::W) writer structure"]
impl crate::Writable for USB1_MISC_CLR {}
#[doc = "USB Misc Register"]
pub mod usb1_misc_clr;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_misc_tog](usb1_misc_tog) module"]
pub type USB1_MISC_TOG = crate::Reg<u32, _USB1_MISC_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1_MISC_TOG;
#[doc = "`read()` method returns [usb1_misc_tog::R](usb1_misc_tog::R) reader structure"]
impl crate::Readable for USB1_MISC_TOG {}
#[doc = "`write(|w| ..)` method takes [usb1_misc_tog::W](usb1_misc_tog::W) writer structure"]
impl crate::Writable for USB1_MISC_TOG {}
#[doc = "USB Misc Register"]
pub mod usb1_misc_tog;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_vbus_detect](usb2_vbus_detect) module"]
pub type USB2_VBUS_DETECT = crate::Reg<u32, _USB2_VBUS_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_VBUS_DETECT;
#[doc = "`read()` method returns [usb2_vbus_detect::R](usb2_vbus_detect::R) reader structure"]
impl crate::Readable for USB2_VBUS_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb2_vbus_detect::W](usb2_vbus_detect::W) writer structure"]
impl crate::Writable for USB2_VBUS_DETECT {}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_vbus_detect_set](usb2_vbus_detect_set) module"]
pub type USB2_VBUS_DETECT_SET = crate::Reg<u32, _USB2_VBUS_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_VBUS_DETECT_SET;
#[doc = "`read()` method returns [usb2_vbus_detect_set::R](usb2_vbus_detect_set::R) reader structure"]
impl crate::Readable for USB2_VBUS_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb2_vbus_detect_set::W](usb2_vbus_detect_set::W) writer structure"]
impl crate::Writable for USB2_VBUS_DETECT_SET {}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_set;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_vbus_detect_clr](usb2_vbus_detect_clr) module"]
pub type USB2_VBUS_DETECT_CLR = crate::Reg<u32, _USB2_VBUS_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_VBUS_DETECT_CLR;
#[doc = "`read()` method returns [usb2_vbus_detect_clr::R](usb2_vbus_detect_clr::R) reader structure"]
impl crate::Readable for USB2_VBUS_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb2_vbus_detect_clr::W](usb2_vbus_detect_clr::W) writer structure"]
impl crate::Writable for USB2_VBUS_DETECT_CLR {}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_clr;
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_vbus_detect_tog](usb2_vbus_detect_tog) module"]
pub type USB2_VBUS_DETECT_TOG = crate::Reg<u32, _USB2_VBUS_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_VBUS_DETECT_TOG;
#[doc = "`read()` method returns [usb2_vbus_detect_tog::R](usb2_vbus_detect_tog::R) reader structure"]
impl crate::Readable for USB2_VBUS_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb2_vbus_detect_tog::W](usb2_vbus_detect_tog::W) writer structure"]
impl crate::Writable for USB2_VBUS_DETECT_TOG {}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_tog;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_chrg_detect](usb2_chrg_detect) module"]
pub type USB2_CHRG_DETECT = crate::Reg<u32, _USB2_CHRG_DETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_CHRG_DETECT;
#[doc = "`read()` method returns [usb2_chrg_detect::R](usb2_chrg_detect::R) reader structure"]
impl crate::Readable for USB2_CHRG_DETECT {}
#[doc = "`write(|w| ..)` method takes [usb2_chrg_detect::W](usb2_chrg_detect::W) writer structure"]
impl crate::Writable for USB2_CHRG_DETECT {}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_chrg_detect_set](usb2_chrg_detect_set) module"]
pub type USB2_CHRG_DETECT_SET = crate::Reg<u32, _USB2_CHRG_DETECT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_CHRG_DETECT_SET;
#[doc = "`read()` method returns [usb2_chrg_detect_set::R](usb2_chrg_detect_set::R) reader structure"]
impl crate::Readable for USB2_CHRG_DETECT_SET {}
#[doc = "`write(|w| ..)` method takes [usb2_chrg_detect_set::W](usb2_chrg_detect_set::W) writer structure"]
impl crate::Writable for USB2_CHRG_DETECT_SET {}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_set;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_chrg_detect_clr](usb2_chrg_detect_clr) module"]
pub type USB2_CHRG_DETECT_CLR = crate::Reg<u32, _USB2_CHRG_DETECT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_CHRG_DETECT_CLR;
#[doc = "`read()` method returns [usb2_chrg_detect_clr::R](usb2_chrg_detect_clr::R) reader structure"]
impl crate::Readable for USB2_CHRG_DETECT_CLR {}
#[doc = "`write(|w| ..)` method takes [usb2_chrg_detect_clr::W](usb2_chrg_detect_clr::W) writer structure"]
impl crate::Writable for USB2_CHRG_DETECT_CLR {}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_clr;
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_chrg_detect_tog](usb2_chrg_detect_tog) module"]
pub type USB2_CHRG_DETECT_TOG = crate::Reg<u32, _USB2_CHRG_DETECT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_CHRG_DETECT_TOG;
#[doc = "`read()` method returns [usb2_chrg_detect_tog::R](usb2_chrg_detect_tog::R) reader structure"]
impl crate::Readable for USB2_CHRG_DETECT_TOG {}
#[doc = "`write(|w| ..)` method takes [usb2_chrg_detect_tog::W](usb2_chrg_detect_tog::W) writer structure"]
impl crate::Writable for USB2_CHRG_DETECT_TOG {}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_tog;
#[doc = "USB VBUS Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_vbus_detect_stat](usb2_vbus_detect_stat) module"]
pub type USB2_VBUS_DETECT_STAT = crate::Reg<u32, _USB2_VBUS_DETECT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_VBUS_DETECT_STAT;
#[doc = "`read()` method returns [usb2_vbus_detect_stat::R](usb2_vbus_detect_stat::R) reader structure"]
impl crate::Readable for USB2_VBUS_DETECT_STAT {}
#[doc = "USB VBUS Detect Status Register"]
pub mod usb2_vbus_detect_stat;
#[doc = "USB Charger Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_chrg_detect_stat](usb2_chrg_detect_stat) module"]
pub type USB2_CHRG_DETECT_STAT = crate::Reg<u32, _USB2_CHRG_DETECT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_CHRG_DETECT_STAT;
#[doc = "`read()` method returns [usb2_chrg_detect_stat::R](usb2_chrg_detect_stat::R) reader structure"]
impl crate::Readable for USB2_CHRG_DETECT_STAT {}
#[doc = "USB Charger Detect Status Register"]
pub mod usb2_chrg_detect_stat;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_misc](usb2_misc) module"]
pub type USB2_MISC = crate::Reg<u32, _USB2_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_MISC;
#[doc = "`read()` method returns [usb2_misc::R](usb2_misc::R) reader structure"]
impl crate::Readable for USB2_MISC {}
#[doc = "`write(|w| ..)` method takes [usb2_misc::W](usb2_misc::W) writer structure"]
impl crate::Writable for USB2_MISC {}
#[doc = "USB Misc Register"]
pub mod usb2_misc;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_misc_set](usb2_misc_set) module"]
pub type USB2_MISC_SET = crate::Reg<u32, _USB2_MISC_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_MISC_SET;
#[doc = "`read()` method returns [usb2_misc_set::R](usb2_misc_set::R) reader structure"]
impl crate::Readable for USB2_MISC_SET {}
#[doc = "`write(|w| ..)` method takes [usb2_misc_set::W](usb2_misc_set::W) writer structure"]
impl crate::Writable for USB2_MISC_SET {}
#[doc = "USB Misc Register"]
pub mod usb2_misc_set;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_misc_clr](usb2_misc_clr) module"]
pub type USB2_MISC_CLR = crate::Reg<u32, _USB2_MISC_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_MISC_CLR;
#[doc = "`read()` method returns [usb2_misc_clr::R](usb2_misc_clr::R) reader structure"]
impl crate::Readable for USB2_MISC_CLR {}
#[doc = "`write(|w| ..)` method takes [usb2_misc_clr::W](usb2_misc_clr::W) writer structure"]
impl crate::Writable for USB2_MISC_CLR {}
#[doc = "USB Misc Register"]
pub mod usb2_misc_clr;
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb2_misc_tog](usb2_misc_tog) module"]
pub type USB2_MISC_TOG = crate::Reg<u32, _USB2_MISC_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB2_MISC_TOG;
#[doc = "`read()` method returns [usb2_misc_tog::R](usb2_misc_tog::R) reader structure"]
impl crate::Readable for USB2_MISC_TOG {}
#[doc = "`write(|w| ..)` method takes [usb2_misc_tog::W](usb2_misc_tog::W) writer structure"]
impl crate::Writable for USB2_MISC_TOG {}
#[doc = "USB Misc Register"]
pub mod usb2_misc_tog;
#[doc = "Chip Silicon Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digprog](digprog) module"]
pub type DIGPROG = crate::Reg<u32, _DIGPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIGPROG;
#[doc = "`read()` method returns [digprog::R](digprog::R) reader structure"]
impl crate::Readable for DIGPROG {}
#[doc = "Chip Silicon Version"]
pub mod digprog;
