#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog ARM PLL control Register"]
    pub pll_arm: PLL_ARM,
    #[doc = "0x04 - Analog ARM PLL control Register"]
    pub pll_arm_set: PLL_ARM_SET,
    #[doc = "0x08 - Analog ARM PLL control Register"]
    pub pll_arm_clr: PLL_ARM_CLR,
    #[doc = "0x0c - Analog ARM PLL control Register"]
    pub pll_arm_tog: PLL_ARM_TOG,
    #[doc = "0x10 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1: PLL_USB1,
    #[doc = "0x14 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_set: PLL_USB1_SET,
    #[doc = "0x18 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_clr: PLL_USB1_CLR,
    #[doc = "0x1c - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_tog: PLL_USB1_TOG,
    #[doc = "0x20 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2: PLL_USB2,
    #[doc = "0x24 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_set: PLL_USB2_SET,
    #[doc = "0x28 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_clr: PLL_USB2_CLR,
    #[doc = "0x2c - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_tog: PLL_USB2_TOG,
    #[doc = "0x30 - Analog System PLL Control Register"]
    pub pll_sys: PLL_SYS,
    #[doc = "0x34 - Analog System PLL Control Register"]
    pub pll_sys_set: PLL_SYS_SET,
    #[doc = "0x38 - Analog System PLL Control Register"]
    pub pll_sys_clr: PLL_SYS_CLR,
    #[doc = "0x3c - Analog System PLL Control Register"]
    pub pll_sys_tog: PLL_SYS_TOG,
    #[doc = "0x40 - 528MHz System PLL Spread Spectrum Register"]
    pub pll_sys_ss: PLL_SYS_SS,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_num: PLL_SYS_NUM,
    _reserved18: [u8; 12usize],
    #[doc = "0x60 - Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_denom: PLL_SYS_DENOM,
    _reserved19: [u8; 12usize],
    #[doc = "0x70 - Analog Audio PLL control Register"]
    pub pll_audio: PLL_AUDIO,
    #[doc = "0x74 - Analog Audio PLL control Register"]
    pub pll_audio_set: PLL_AUDIO_SET,
    #[doc = "0x78 - Analog Audio PLL control Register"]
    pub pll_audio_clr: PLL_AUDIO_CLR,
    #[doc = "0x7c - Analog Audio PLL control Register"]
    pub pll_audio_tog: PLL_AUDIO_TOG,
    #[doc = "0x80 - Numerator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_num: PLL_AUDIO_NUM,
    _reserved24: [u8; 12usize],
    #[doc = "0x90 - Denominator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_denom: PLL_AUDIO_DENOM,
    _reserved25: [u8; 12usize],
    #[doc = "0xa0 - Analog Video PLL control Register"]
    pub pll_video: PLL_VIDEO,
    #[doc = "0xa4 - Analog Video PLL control Register"]
    pub pll_video_set: PLL_VIDEO_SET,
    #[doc = "0xa8 - Analog Video PLL control Register"]
    pub pll_video_clr: PLL_VIDEO_CLR,
    #[doc = "0xac - Analog Video PLL control Register"]
    pub pll_video_tog: PLL_VIDEO_TOG,
    #[doc = "0xb0 - Numerator of Video PLL Fractional Loop Divider Register"]
    pub pll_video_num: PLL_VIDEO_NUM,
    _reserved30: [u8; 12usize],
    #[doc = "0xc0 - Denominator of Video PLL Fractional Loop Divider Register"]
    pub pll_video_denom: PLL_VIDEO_DENOM,
    _reserved31: [u8; 28usize],
    #[doc = "0xe0 - Analog ENET PLL Control Register"]
    pub pll_enet: PLL_ENET,
    #[doc = "0xe4 - Analog ENET PLL Control Register"]
    pub pll_enet_set: PLL_ENET_SET,
    #[doc = "0xe8 - Analog ENET PLL Control Register"]
    pub pll_enet_clr: PLL_ENET_CLR,
    #[doc = "0xec - Analog ENET PLL Control Register"]
    pub pll_enet_tog: PLL_ENET_TOG,
    #[doc = "0xf0 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480: PFD_480,
    #[doc = "0xf4 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_set: PFD_480_SET,
    #[doc = "0xf8 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_clr: PFD_480_CLR,
    #[doc = "0xfc - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_tog: PFD_480_TOG,
    #[doc = "0x100 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528: PFD_528,
    #[doc = "0x104 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_set: PFD_528_SET,
    #[doc = "0x108 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_clr: PFD_528_CLR,
    #[doc = "0x10c - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_tog: PFD_528_TOG,
    _reserved43: [u8; 64usize],
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    #[doc = "0x160 - Miscellaneous Register 1"]
    pub misc1: MISC1,
    #[doc = "0x164 - Miscellaneous Register 1"]
    pub misc1_set: MISC1_SET,
    #[doc = "0x168 - Miscellaneous Register 1"]
    pub misc1_clr: MISC1_CLR,
    #[doc = "0x16c - Miscellaneous Register 1"]
    pub misc1_tog: MISC1_TOG,
    #[doc = "0x170 - Miscellaneous Register 2"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Register 2"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Register 2"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Register 2"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "Analog ARM PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_arm](pll_arm) module"]
pub type PLL_ARM = crate::Reg<u32, _PLL_ARM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ARM;
#[doc = "`read()` method returns [pll_arm::R](pll_arm::R) reader structure"]
impl crate::Readable for PLL_ARM {}
#[doc = "`write(|w| ..)` method takes [pll_arm::W](pll_arm::W) writer structure"]
impl crate::Writable for PLL_ARM {}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm;
#[doc = "Analog ARM PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_arm_set](pll_arm_set) module"]
pub type PLL_ARM_SET = crate::Reg<u32, _PLL_ARM_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ARM_SET;
#[doc = "`read()` method returns [pll_arm_set::R](pll_arm_set::R) reader structure"]
impl crate::Readable for PLL_ARM_SET {}
#[doc = "`write(|w| ..)` method takes [pll_arm_set::W](pll_arm_set::W) writer structure"]
impl crate::Writable for PLL_ARM_SET {}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_set;
#[doc = "Analog ARM PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_arm_clr](pll_arm_clr) module"]
pub type PLL_ARM_CLR = crate::Reg<u32, _PLL_ARM_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ARM_CLR;
#[doc = "`read()` method returns [pll_arm_clr::R](pll_arm_clr::R) reader structure"]
impl crate::Readable for PLL_ARM_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_arm_clr::W](pll_arm_clr::W) writer structure"]
impl crate::Writable for PLL_ARM_CLR {}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_clr;
#[doc = "Analog ARM PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_arm_tog](pll_arm_tog) module"]
pub type PLL_ARM_TOG = crate::Reg<u32, _PLL_ARM_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ARM_TOG;
#[doc = "`read()` method returns [pll_arm_tog::R](pll_arm_tog::R) reader structure"]
impl crate::Readable for PLL_ARM_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_arm_tog::W](pll_arm_tog::W) writer structure"]
impl crate::Writable for PLL_ARM_TOG {}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_tog;
#[doc = "Analog USB1 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb1](pll_usb1) module"]
pub type PLL_USB1 = crate::Reg<u32, _PLL_USB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB1;
#[doc = "`read()` method returns [pll_usb1::R](pll_usb1::R) reader structure"]
impl crate::Readable for PLL_USB1 {}
#[doc = "`write(|w| ..)` method takes [pll_usb1::W](pll_usb1::W) writer structure"]
impl crate::Writable for PLL_USB1 {}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1;
#[doc = "Analog USB1 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb1_set](pll_usb1_set) module"]
pub type PLL_USB1_SET = crate::Reg<u32, _PLL_USB1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB1_SET;
#[doc = "`read()` method returns [pll_usb1_set::R](pll_usb1_set::R) reader structure"]
impl crate::Readable for PLL_USB1_SET {}
#[doc = "`write(|w| ..)` method takes [pll_usb1_set::W](pll_usb1_set::W) writer structure"]
impl crate::Writable for PLL_USB1_SET {}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_set;
#[doc = "Analog USB1 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb1_clr](pll_usb1_clr) module"]
pub type PLL_USB1_CLR = crate::Reg<u32, _PLL_USB1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB1_CLR;
#[doc = "`read()` method returns [pll_usb1_clr::R](pll_usb1_clr::R) reader structure"]
impl crate::Readable for PLL_USB1_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_usb1_clr::W](pll_usb1_clr::W) writer structure"]
impl crate::Writable for PLL_USB1_CLR {}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_clr;
#[doc = "Analog USB1 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb1_tog](pll_usb1_tog) module"]
pub type PLL_USB1_TOG = crate::Reg<u32, _PLL_USB1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB1_TOG;
#[doc = "`read()` method returns [pll_usb1_tog::R](pll_usb1_tog::R) reader structure"]
impl crate::Readable for PLL_USB1_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_usb1_tog::W](pll_usb1_tog::W) writer structure"]
impl crate::Writable for PLL_USB1_TOG {}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_tog;
#[doc = "Analog USB2 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb2](pll_usb2) module"]
pub type PLL_USB2 = crate::Reg<u32, _PLL_USB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB2;
#[doc = "`read()` method returns [pll_usb2::R](pll_usb2::R) reader structure"]
impl crate::Readable for PLL_USB2 {}
#[doc = "`write(|w| ..)` method takes [pll_usb2::W](pll_usb2::W) writer structure"]
impl crate::Writable for PLL_USB2 {}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2;
#[doc = "Analog USB2 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb2_set](pll_usb2_set) module"]
pub type PLL_USB2_SET = crate::Reg<u32, _PLL_USB2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB2_SET;
#[doc = "`read()` method returns [pll_usb2_set::R](pll_usb2_set::R) reader structure"]
impl crate::Readable for PLL_USB2_SET {}
#[doc = "`write(|w| ..)` method takes [pll_usb2_set::W](pll_usb2_set::W) writer structure"]
impl crate::Writable for PLL_USB2_SET {}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_set;
#[doc = "Analog USB2 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb2_clr](pll_usb2_clr) module"]
pub type PLL_USB2_CLR = crate::Reg<u32, _PLL_USB2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB2_CLR;
#[doc = "`read()` method returns [pll_usb2_clr::R](pll_usb2_clr::R) reader structure"]
impl crate::Readable for PLL_USB2_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_usb2_clr::W](pll_usb2_clr::W) writer structure"]
impl crate::Writable for PLL_USB2_CLR {}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_clr;
#[doc = "Analog USB2 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_usb2_tog](pll_usb2_tog) module"]
pub type PLL_USB2_TOG = crate::Reg<u32, _PLL_USB2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_USB2_TOG;
#[doc = "`read()` method returns [pll_usb2_tog::R](pll_usb2_tog::R) reader structure"]
impl crate::Readable for PLL_USB2_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_usb2_tog::W](pll_usb2_tog::W) writer structure"]
impl crate::Writable for PLL_USB2_TOG {}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_tog;
#[doc = "Analog System PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys](pll_sys) module"]
pub type PLL_SYS = crate::Reg<u32, _PLL_SYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS;
#[doc = "`read()` method returns [pll_sys::R](pll_sys::R) reader structure"]
impl crate::Readable for PLL_SYS {}
#[doc = "`write(|w| ..)` method takes [pll_sys::W](pll_sys::W) writer structure"]
impl crate::Writable for PLL_SYS {}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys;
#[doc = "Analog System PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_set](pll_sys_set) module"]
pub type PLL_SYS_SET = crate::Reg<u32, _PLL_SYS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_SET;
#[doc = "`read()` method returns [pll_sys_set::R](pll_sys_set::R) reader structure"]
impl crate::Readable for PLL_SYS_SET {}
#[doc = "`write(|w| ..)` method takes [pll_sys_set::W](pll_sys_set::W) writer structure"]
impl crate::Writable for PLL_SYS_SET {}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_set;
#[doc = "Analog System PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_clr](pll_sys_clr) module"]
pub type PLL_SYS_CLR = crate::Reg<u32, _PLL_SYS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_CLR;
#[doc = "`read()` method returns [pll_sys_clr::R](pll_sys_clr::R) reader structure"]
impl crate::Readable for PLL_SYS_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_sys_clr::W](pll_sys_clr::W) writer structure"]
impl crate::Writable for PLL_SYS_CLR {}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_clr;
#[doc = "Analog System PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_tog](pll_sys_tog) module"]
pub type PLL_SYS_TOG = crate::Reg<u32, _PLL_SYS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_TOG;
#[doc = "`read()` method returns [pll_sys_tog::R](pll_sys_tog::R) reader structure"]
impl crate::Readable for PLL_SYS_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_sys_tog::W](pll_sys_tog::W) writer structure"]
impl crate::Writable for PLL_SYS_TOG {}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_tog;
#[doc = "528MHz System PLL Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_ss](pll_sys_ss) module"]
pub type PLL_SYS_SS = crate::Reg<u32, _PLL_SYS_SS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_SS;
#[doc = "`read()` method returns [pll_sys_ss::R](pll_sys_ss::R) reader structure"]
impl crate::Readable for PLL_SYS_SS {}
#[doc = "`write(|w| ..)` method takes [pll_sys_ss::W](pll_sys_ss::W) writer structure"]
impl crate::Writable for PLL_SYS_SS {}
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub mod pll_sys_ss;
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_num](pll_sys_num) module"]
pub type PLL_SYS_NUM = crate::Reg<u32, _PLL_SYS_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_NUM;
#[doc = "`read()` method returns [pll_sys_num::R](pll_sys_num::R) reader structure"]
impl crate::Readable for PLL_SYS_NUM {}
#[doc = "`write(|w| ..)` method takes [pll_sys_num::W](pll_sys_num::W) writer structure"]
impl crate::Writable for PLL_SYS_NUM {}
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_num;
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_sys_denom](pll_sys_denom) module"]
pub type PLL_SYS_DENOM = crate::Reg<u32, _PLL_SYS_DENOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_SYS_DENOM;
#[doc = "`read()` method returns [pll_sys_denom::R](pll_sys_denom::R) reader structure"]
impl crate::Readable for PLL_SYS_DENOM {}
#[doc = "`write(|w| ..)` method takes [pll_sys_denom::W](pll_sys_denom::W) writer structure"]
impl crate::Writable for PLL_SYS_DENOM {}
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_denom;
#[doc = "Analog Audio PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio](pll_audio) module"]
pub type PLL_AUDIO = crate::Reg<u32, _PLL_AUDIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO;
#[doc = "`read()` method returns [pll_audio::R](pll_audio::R) reader structure"]
impl crate::Readable for PLL_AUDIO {}
#[doc = "`write(|w| ..)` method takes [pll_audio::W](pll_audio::W) writer structure"]
impl crate::Writable for PLL_AUDIO {}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio;
#[doc = "Analog Audio PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio_set](pll_audio_set) module"]
pub type PLL_AUDIO_SET = crate::Reg<u32, _PLL_AUDIO_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO_SET;
#[doc = "`read()` method returns [pll_audio_set::R](pll_audio_set::R) reader structure"]
impl crate::Readable for PLL_AUDIO_SET {}
#[doc = "`write(|w| ..)` method takes [pll_audio_set::W](pll_audio_set::W) writer structure"]
impl crate::Writable for PLL_AUDIO_SET {}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_set;
#[doc = "Analog Audio PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio_clr](pll_audio_clr) module"]
pub type PLL_AUDIO_CLR = crate::Reg<u32, _PLL_AUDIO_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO_CLR;
#[doc = "`read()` method returns [pll_audio_clr::R](pll_audio_clr::R) reader structure"]
impl crate::Readable for PLL_AUDIO_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_audio_clr::W](pll_audio_clr::W) writer structure"]
impl crate::Writable for PLL_AUDIO_CLR {}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_clr;
#[doc = "Analog Audio PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio_tog](pll_audio_tog) module"]
pub type PLL_AUDIO_TOG = crate::Reg<u32, _PLL_AUDIO_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO_TOG;
#[doc = "`read()` method returns [pll_audio_tog::R](pll_audio_tog::R) reader structure"]
impl crate::Readable for PLL_AUDIO_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_audio_tog::W](pll_audio_tog::W) writer structure"]
impl crate::Writable for PLL_AUDIO_TOG {}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_tog;
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio_num](pll_audio_num) module"]
pub type PLL_AUDIO_NUM = crate::Reg<u32, _PLL_AUDIO_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO_NUM;
#[doc = "`read()` method returns [pll_audio_num::R](pll_audio_num::R) reader structure"]
impl crate::Readable for PLL_AUDIO_NUM {}
#[doc = "`write(|w| ..)` method takes [pll_audio_num::W](pll_audio_num::W) writer structure"]
impl crate::Writable for PLL_AUDIO_NUM {}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_num;
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_audio_denom](pll_audio_denom) module"]
pub type PLL_AUDIO_DENOM = crate::Reg<u32, _PLL_AUDIO_DENOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_AUDIO_DENOM;
#[doc = "`read()` method returns [pll_audio_denom::R](pll_audio_denom::R) reader structure"]
impl crate::Readable for PLL_AUDIO_DENOM {}
#[doc = "`write(|w| ..)` method takes [pll_audio_denom::W](pll_audio_denom::W) writer structure"]
impl crate::Writable for PLL_AUDIO_DENOM {}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_denom;
#[doc = "Analog Video PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video](pll_video) module"]
pub type PLL_VIDEO = crate::Reg<u32, _PLL_VIDEO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO;
#[doc = "`read()` method returns [pll_video::R](pll_video::R) reader structure"]
impl crate::Readable for PLL_VIDEO {}
#[doc = "`write(|w| ..)` method takes [pll_video::W](pll_video::W) writer structure"]
impl crate::Writable for PLL_VIDEO {}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video;
#[doc = "Analog Video PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video_set](pll_video_set) module"]
pub type PLL_VIDEO_SET = crate::Reg<u32, _PLL_VIDEO_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO_SET;
#[doc = "`read()` method returns [pll_video_set::R](pll_video_set::R) reader structure"]
impl crate::Readable for PLL_VIDEO_SET {}
#[doc = "`write(|w| ..)` method takes [pll_video_set::W](pll_video_set::W) writer structure"]
impl crate::Writable for PLL_VIDEO_SET {}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_set;
#[doc = "Analog Video PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video_clr](pll_video_clr) module"]
pub type PLL_VIDEO_CLR = crate::Reg<u32, _PLL_VIDEO_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO_CLR;
#[doc = "`read()` method returns [pll_video_clr::R](pll_video_clr::R) reader structure"]
impl crate::Readable for PLL_VIDEO_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_video_clr::W](pll_video_clr::W) writer structure"]
impl crate::Writable for PLL_VIDEO_CLR {}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_clr;
#[doc = "Analog Video PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video_tog](pll_video_tog) module"]
pub type PLL_VIDEO_TOG = crate::Reg<u32, _PLL_VIDEO_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO_TOG;
#[doc = "`read()` method returns [pll_video_tog::R](pll_video_tog::R) reader structure"]
impl crate::Readable for PLL_VIDEO_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_video_tog::W](pll_video_tog::W) writer structure"]
impl crate::Writable for PLL_VIDEO_TOG {}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_tog;
#[doc = "Numerator of Video PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video_num](pll_video_num) module"]
pub type PLL_VIDEO_NUM = crate::Reg<u32, _PLL_VIDEO_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO_NUM;
#[doc = "`read()` method returns [pll_video_num::R](pll_video_num::R) reader structure"]
impl crate::Readable for PLL_VIDEO_NUM {}
#[doc = "`write(|w| ..)` method takes [pll_video_num::W](pll_video_num::W) writer structure"]
impl crate::Writable for PLL_VIDEO_NUM {}
#[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
pub mod pll_video_num;
#[doc = "Denominator of Video PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_video_denom](pll_video_denom) module"]
pub type PLL_VIDEO_DENOM = crate::Reg<u32, _PLL_VIDEO_DENOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_VIDEO_DENOM;
#[doc = "`read()` method returns [pll_video_denom::R](pll_video_denom::R) reader structure"]
impl crate::Readable for PLL_VIDEO_DENOM {}
#[doc = "`write(|w| ..)` method takes [pll_video_denom::W](pll_video_denom::W) writer structure"]
impl crate::Writable for PLL_VIDEO_DENOM {}
#[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
pub mod pll_video_denom;
#[doc = "Analog ENET PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_enet](pll_enet) module"]
pub type PLL_ENET = crate::Reg<u32, _PLL_ENET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ENET;
#[doc = "`read()` method returns [pll_enet::R](pll_enet::R) reader structure"]
impl crate::Readable for PLL_ENET {}
#[doc = "`write(|w| ..)` method takes [pll_enet::W](pll_enet::W) writer structure"]
impl crate::Writable for PLL_ENET {}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet;
#[doc = "Analog ENET PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_enet_set](pll_enet_set) module"]
pub type PLL_ENET_SET = crate::Reg<u32, _PLL_ENET_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ENET_SET;
#[doc = "`read()` method returns [pll_enet_set::R](pll_enet_set::R) reader structure"]
impl crate::Readable for PLL_ENET_SET {}
#[doc = "`write(|w| ..)` method takes [pll_enet_set::W](pll_enet_set::W) writer structure"]
impl crate::Writable for PLL_ENET_SET {}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_set;
#[doc = "Analog ENET PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_enet_clr](pll_enet_clr) module"]
pub type PLL_ENET_CLR = crate::Reg<u32, _PLL_ENET_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ENET_CLR;
#[doc = "`read()` method returns [pll_enet_clr::R](pll_enet_clr::R) reader structure"]
impl crate::Readable for PLL_ENET_CLR {}
#[doc = "`write(|w| ..)` method takes [pll_enet_clr::W](pll_enet_clr::W) writer structure"]
impl crate::Writable for PLL_ENET_CLR {}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_clr;
#[doc = "Analog ENET PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_enet_tog](pll_enet_tog) module"]
pub type PLL_ENET_TOG = crate::Reg<u32, _PLL_ENET_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_ENET_TOG;
#[doc = "`read()` method returns [pll_enet_tog::R](pll_enet_tog::R) reader structure"]
impl crate::Readable for PLL_ENET_TOG {}
#[doc = "`write(|w| ..)` method takes [pll_enet_tog::W](pll_enet_tog::W) writer structure"]
impl crate::Writable for PLL_ENET_TOG {}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_tog;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_480](pfd_480) module"]
pub type PFD_480 = crate::Reg<u32, _PFD_480>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_480;
#[doc = "`read()` method returns [pfd_480::R](pfd_480::R) reader structure"]
impl crate::Readable for PFD_480 {}
#[doc = "`write(|w| ..)` method takes [pfd_480::W](pfd_480::W) writer structure"]
impl crate::Writable for PFD_480 {}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_480_set](pfd_480_set) module"]
pub type PFD_480_SET = crate::Reg<u32, _PFD_480_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_480_SET;
#[doc = "`read()` method returns [pfd_480_set::R](pfd_480_set::R) reader structure"]
impl crate::Readable for PFD_480_SET {}
#[doc = "`write(|w| ..)` method takes [pfd_480_set::W](pfd_480_set::W) writer structure"]
impl crate::Writable for PFD_480_SET {}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_set;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_480_clr](pfd_480_clr) module"]
pub type PFD_480_CLR = crate::Reg<u32, _PFD_480_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_480_CLR;
#[doc = "`read()` method returns [pfd_480_clr::R](pfd_480_clr::R) reader structure"]
impl crate::Readable for PFD_480_CLR {}
#[doc = "`write(|w| ..)` method takes [pfd_480_clr::W](pfd_480_clr::W) writer structure"]
impl crate::Writable for PFD_480_CLR {}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_clr;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_480_tog](pfd_480_tog) module"]
pub type PFD_480_TOG = crate::Reg<u32, _PFD_480_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_480_TOG;
#[doc = "`read()` method returns [pfd_480_tog::R](pfd_480_tog::R) reader structure"]
impl crate::Readable for PFD_480_TOG {}
#[doc = "`write(|w| ..)` method takes [pfd_480_tog::W](pfd_480_tog::W) writer structure"]
impl crate::Writable for PFD_480_TOG {}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_tog;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_528](pfd_528) module"]
pub type PFD_528 = crate::Reg<u32, _PFD_528>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_528;
#[doc = "`read()` method returns [pfd_528::R](pfd_528::R) reader structure"]
impl crate::Readable for PFD_528 {}
#[doc = "`write(|w| ..)` method takes [pfd_528::W](pfd_528::W) writer structure"]
impl crate::Writable for PFD_528 {}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_528_set](pfd_528_set) module"]
pub type PFD_528_SET = crate::Reg<u32, _PFD_528_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_528_SET;
#[doc = "`read()` method returns [pfd_528_set::R](pfd_528_set::R) reader structure"]
impl crate::Readable for PFD_528_SET {}
#[doc = "`write(|w| ..)` method takes [pfd_528_set::W](pfd_528_set::W) writer structure"]
impl crate::Writable for PFD_528_SET {}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_set;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_528_clr](pfd_528_clr) module"]
pub type PFD_528_CLR = crate::Reg<u32, _PFD_528_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_528_CLR;
#[doc = "`read()` method returns [pfd_528_clr::R](pfd_528_clr::R) reader structure"]
impl crate::Readable for PFD_528_CLR {}
#[doc = "`write(|w| ..)` method takes [pfd_528_clr::W](pfd_528_clr::W) writer structure"]
impl crate::Writable for PFD_528_CLR {}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_clr;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfd_528_tog](pfd_528_tog) module"]
pub type PFD_528_TOG = crate::Reg<u32, _PFD_528_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFD_528_TOG;
#[doc = "`read()` method returns [pfd_528_tog::R](pfd_528_tog::R) reader structure"]
impl crate::Readable for PFD_528_TOG {}
#[doc = "`write(|w| ..)` method takes [pfd_528_tog::W](pfd_528_tog::W) writer structure"]
impl crate::Writable for PFD_528_TOG {}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_tog;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc0](misc0) module"]
pub type MISC0 = crate::Reg<u32, _MISC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0;
#[doc = "`read()` method returns [misc0::R](misc0::R) reader structure"]
impl crate::Readable for MISC0 {}
#[doc = "`write(|w| ..)` method takes [misc0::W](misc0::W) writer structure"]
impl crate::Writable for MISC0 {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc0_set](misc0_set) module"]
pub type MISC0_SET = crate::Reg<u32, _MISC0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_SET;
#[doc = "`read()` method returns [misc0_set::R](misc0_set::R) reader structure"]
impl crate::Readable for MISC0_SET {}
#[doc = "`write(|w| ..)` method takes [misc0_set::W](misc0_set::W) writer structure"]
impl crate::Writable for MISC0_SET {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc0_clr](misc0_clr) module"]
pub type MISC0_CLR = crate::Reg<u32, _MISC0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_CLR;
#[doc = "`read()` method returns [misc0_clr::R](misc0_clr::R) reader structure"]
impl crate::Readable for MISC0_CLR {}
#[doc = "`write(|w| ..)` method takes [misc0_clr::W](misc0_clr::W) writer structure"]
impl crate::Writable for MISC0_CLR {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc0_tog](misc0_tog) module"]
pub type MISC0_TOG = crate::Reg<u32, _MISC0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_TOG;
#[doc = "`read()` method returns [misc0_tog::R](misc0_tog::R) reader structure"]
impl crate::Readable for MISC0_TOG {}
#[doc = "`write(|w| ..)` method takes [misc0_tog::W](misc0_tog::W) writer structure"]
impl crate::Writable for MISC0_TOG {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc1](misc1) module"]
pub type MISC1 = crate::Reg<u32, _MISC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1;
#[doc = "`read()` method returns [misc1::R](misc1::R) reader structure"]
impl crate::Readable for MISC1 {}
#[doc = "`write(|w| ..)` method takes [misc1::W](misc1::W) writer structure"]
impl crate::Writable for MISC1 {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc1_set](misc1_set) module"]
pub type MISC1_SET = crate::Reg<u32, _MISC1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_SET;
#[doc = "`read()` method returns [misc1_set::R](misc1_set::R) reader structure"]
impl crate::Readable for MISC1_SET {}
#[doc = "`write(|w| ..)` method takes [misc1_set::W](misc1_set::W) writer structure"]
impl crate::Writable for MISC1_SET {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_set;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc1_clr](misc1_clr) module"]
pub type MISC1_CLR = crate::Reg<u32, _MISC1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_CLR;
#[doc = "`read()` method returns [misc1_clr::R](misc1_clr::R) reader structure"]
impl crate::Readable for MISC1_CLR {}
#[doc = "`write(|w| ..)` method takes [misc1_clr::W](misc1_clr::W) writer structure"]
impl crate::Writable for MISC1_CLR {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_clr;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc1_tog](misc1_tog) module"]
pub type MISC1_TOG = crate::Reg<u32, _MISC1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_TOG;
#[doc = "`read()` method returns [misc1_tog::R](misc1_tog::R) reader structure"]
impl crate::Readable for MISC1_TOG {}
#[doc = "`write(|w| ..)` method takes [misc1_tog::W](misc1_tog::W) writer structure"]
impl crate::Writable for MISC1_TOG {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_tog;
#[doc = "Miscellaneous Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc2](misc2) module"]
pub type MISC2 = crate::Reg<u32, _MISC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2;
#[doc = "`read()` method returns [misc2::R](misc2::R) reader structure"]
impl crate::Readable for MISC2 {}
#[doc = "`write(|w| ..)` method takes [misc2::W](misc2::W) writer structure"]
impl crate::Writable for MISC2 {}
#[doc = "Miscellaneous Register 2"]
pub mod misc2;
#[doc = "Miscellaneous Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc2_set](misc2_set) module"]
pub type MISC2_SET = crate::Reg<u32, _MISC2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_SET;
#[doc = "`read()` method returns [misc2_set::R](misc2_set::R) reader structure"]
impl crate::Readable for MISC2_SET {}
#[doc = "`write(|w| ..)` method takes [misc2_set::W](misc2_set::W) writer structure"]
impl crate::Writable for MISC2_SET {}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_set;
#[doc = "Miscellaneous Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc2_clr](misc2_clr) module"]
pub type MISC2_CLR = crate::Reg<u32, _MISC2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_CLR;
#[doc = "`read()` method returns [misc2_clr::R](misc2_clr::R) reader structure"]
impl crate::Readable for MISC2_CLR {}
#[doc = "`write(|w| ..)` method takes [misc2_clr::W](misc2_clr::W) writer structure"]
impl crate::Writable for MISC2_CLR {}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_clr;
#[doc = "Miscellaneous Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc2_tog](misc2_tog) module"]
pub type MISC2_TOG = crate::Reg<u32, _MISC2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_TOG;
#[doc = "`read()` method returns [misc2_tog::R](misc2_tog::R) reader structure"]
impl crate::Readable for MISC2_TOG {}
#[doc = "`write(|w| ..)` method takes [misc2_tog::W](misc2_tog::W) writer structure"]
impl crate::Writable for MISC2_TOG {}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_tog;
