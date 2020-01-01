#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTP Controller Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - OTP Controller Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - OTP Controller Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - OTP Controller Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - OTP Controller Timing Register"]
    pub timing: TIMING,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - OTP Controller Write Data Register"]
    pub data: DATA,
    _reserved6: [u8; 12usize],
    #[doc = "0x30 - OTP Controller Write Data Register"]
    pub read_ctrl: READ_CTRL,
    _reserved7: [u8; 12usize],
    #[doc = "0x40 - OTP Controller Read Data Register"]
    pub read_fuse_data: READ_FUSE_DATA,
    _reserved8: [u8; 12usize],
    #[doc = "0x50 - Sticky bit Register"]
    pub sw_sticky: SW_STICKY,
    _reserved9: [u8; 12usize],
    #[doc = "0x60 - Software Controllable Signals Register"]
    pub scs: SCS,
    #[doc = "0x64 - Software Controllable Signals Register"]
    pub scs_set: SCS_SET,
    #[doc = "0x68 - Software Controllable Signals Register"]
    pub scs_clr: SCS_CLR,
    #[doc = "0x6c - Software Controllable Signals Register"]
    pub scs_tog: SCS_TOG,
    #[doc = "0x70 - OTP Controller CRC test address"]
    pub crc_addr: CRC_ADDR,
    _reserved14: [u8; 12usize],
    #[doc = "0x80 - OTP Controller CRC Value Register"]
    pub crc_value: CRC_VALUE,
    _reserved15: [u8; 12usize],
    #[doc = "0x90 - OTP Controller Version Register"]
    pub version: VERSION,
    _reserved16: [u8; 108usize],
    #[doc = "0x100 - OTP Controller Timing Register"]
    pub timing2: TIMING2,
    _reserved17: [u8; 764usize],
    #[doc = "0x400 - Value of OTP Bank0 Word0 (Lock controls)"]
    pub lock: LOCK,
    _reserved18: [u8; 12usize],
    #[doc = "0x410 - Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    pub cfg0: CFG0,
    _reserved19: [u8; 12usize],
    #[doc = "0x420 - Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    pub cfg1: CFG1,
    _reserved20: [u8; 12usize],
    #[doc = "0x430 - Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    pub cfg2: CFG2,
    _reserved21: [u8; 12usize],
    #[doc = "0x440 - Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    pub cfg3: CFG3,
    _reserved22: [u8; 12usize],
    #[doc = "0x450 - Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    pub cfg4: CFG4,
    _reserved23: [u8; 12usize],
    #[doc = "0x460 - Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    pub cfg5: CFG5,
    _reserved24: [u8; 12usize],
    #[doc = "0x470 - Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    pub cfg6: CFG6,
    _reserved25: [u8; 12usize],
    #[doc = "0x480 - Value of OTP Bank1 Word0 (Memory Related Info.)"]
    pub mem0: MEM0,
    _reserved26: [u8; 12usize],
    #[doc = "0x490 - Value of OTP Bank1 Word1 (Memory Related Info.)"]
    pub mem1: MEM1,
    _reserved27: [u8; 12usize],
    #[doc = "0x4a0 - Value of OTP Bank1 Word2 (Memory Related Info.)"]
    pub mem2: MEM2,
    _reserved28: [u8; 12usize],
    #[doc = "0x4b0 - Value of OTP Bank1 Word3 (Memory Related Info.)"]
    pub mem3: MEM3,
    _reserved29: [u8; 12usize],
    #[doc = "0x4c0 - Value of OTP Bank1 Word4 (Memory Related Info.)"]
    pub mem4: MEM4,
    _reserved30: [u8; 12usize],
    #[doc = "0x4d0 - Value of OTP Bank1 Word5 (Memory Related Info.)"]
    pub ana0: ANA0,
    _reserved31: [u8; 12usize],
    #[doc = "0x4e0 - Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)"]
    pub ana1: ANA1,
    _reserved32: [u8; 12usize],
    #[doc = "0x4f0 - Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)"]
    pub ana2: ANA2,
    _reserved33: [u8; 12usize],
    #[doc = "0x500 - Value of OTP Bank2 Word0 (OTPMK Key)"]
    pub otpmk0: OTPMK0,
    _reserved34: [u8; 12usize],
    #[doc = "0x510 - Value of OTP Bank2 Word1 (OTPMK Key)"]
    pub otpmk1: OTPMK1,
    _reserved35: [u8; 12usize],
    #[doc = "0x520 - Value of OTP Bank2 Word2 (OTPMK Key)"]
    pub otpmk2: OTPMK2,
    _reserved36: [u8; 12usize],
    #[doc = "0x530 - Value of OTP Bank2 Word3 (OTPMK Key)"]
    pub otpmk3: OTPMK3,
    _reserved37: [u8; 12usize],
    #[doc = "0x540 - Value of OTP Bank2 Word4 (OTPMK Key)"]
    pub otpmk4: OTPMK4,
    _reserved38: [u8; 12usize],
    #[doc = "0x550 - Value of OTP Bank2 Word5 (OTPMK Key)"]
    pub otpmk5: OTPMK5,
    _reserved39: [u8; 12usize],
    #[doc = "0x560 - Value of OTP Bank2 Word6 (OTPMK Key)"]
    pub otpmk6: OTPMK6,
    _reserved40: [u8; 12usize],
    #[doc = "0x570 - Value of OTP Bank2 Word7 (OTPMK Key)"]
    pub otpmk7: OTPMK7,
    _reserved41: [u8; 12usize],
    #[doc = "0x580 - Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    pub srk0: SRK0,
    _reserved42: [u8; 12usize],
    #[doc = "0x590 - Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    pub srk1: SRK1,
    _reserved43: [u8; 12usize],
    #[doc = "0x5a0 - Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    pub srk2: SRK2,
    _reserved44: [u8; 12usize],
    #[doc = "0x5b0 - Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    pub srk3: SRK3,
    _reserved45: [u8; 12usize],
    #[doc = "0x5c0 - Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    pub srk4: SRK4,
    _reserved46: [u8; 12usize],
    #[doc = "0x5d0 - Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    pub srk5: SRK5,
    _reserved47: [u8; 12usize],
    #[doc = "0x5e0 - Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    pub srk6: SRK6,
    _reserved48: [u8; 12usize],
    #[doc = "0x5f0 - Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    pub srk7: SRK7,
    _reserved49: [u8; 12usize],
    #[doc = "0x600 - Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    pub sjc_resp0: SJC_RESP0,
    _reserved50: [u8; 12usize],
    #[doc = "0x610 - Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    pub sjc_resp1: SJC_RESP1,
    _reserved51: [u8; 12usize],
    #[doc = "0x620 - Value of OTP Bank4 Word2 (MAC Address)"]
    pub mac0: MAC0,
    _reserved52: [u8; 12usize],
    #[doc = "0x630 - Value of OTP Bank4 Word3 (MAC Address)"]
    pub mac1: MAC1,
    _reserved53: [u8; 12usize],
    #[doc = "0x640 - Value of OTP Bank4 Word4 (MAC2 Address)"]
    pub mac2: MAC2,
    _reserved54: [u8; 12usize],
    #[doc = "0x650 - Value of OTP Bank4 Word5 (CRC Key)"]
    pub otpmk_crc32: OTPMK_CRC32,
    _reserved55: [u8; 12usize],
    #[doc = "0x660 - Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    pub gp1: GP1,
    _reserved56: [u8; 12usize],
    #[doc = "0x670 - Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    pub gp2: GP2,
    _reserved57: [u8; 12usize],
    #[doc = "0x680 - Value of OTP Bank5 Word0 (SW GP1)"]
    pub sw_gp1: SW_GP1,
    _reserved58: [u8; 12usize],
    #[doc = "0x690 - Value of OTP Bank5 Word1 (SW GP2)"]
    pub sw_gp20: SW_GP20,
    _reserved59: [u8; 12usize],
    #[doc = "0x6a0 - Value of OTP Bank5 Word2 (SW GP2)"]
    pub sw_gp21: SW_GP21,
    _reserved60: [u8; 12usize],
    #[doc = "0x6b0 - Value of OTP Bank5 Word3 (SW GP2)"]
    pub sw_gp22: SW_GP22,
    _reserved61: [u8; 12usize],
    #[doc = "0x6c0 - Value of OTP Bank5 Word4 (SW GP2)"]
    pub sw_gp23: SW_GP23,
    _reserved62: [u8; 12usize],
    #[doc = "0x6d0 - Value of OTP Bank5 Word5 (Misc Conf)"]
    pub misc_conf0: MISC_CONF0,
    _reserved63: [u8; 12usize],
    #[doc = "0x6e0 - Value of OTP Bank5 Word6 (Misc Conf)"]
    pub misc_conf1: MISC_CONF1,
    _reserved64: [u8; 12usize],
    #[doc = "0x6f0 - Value of OTP Bank5 Word7 (SRK Revoke)"]
    pub srk_revoke: SRK_REVOKE,
    _reserved65: [u8; 268usize],
    #[doc = "0x800 - Value of OTP Bank6 Word0 (ROM Patch)"]
    pub rom_patch0: ROM_PATCH0,
    _reserved66: [u8; 12usize],
    #[doc = "0x810 - Value of OTP Bank6 Word1 (ROM Patch)"]
    pub rom_patch1: ROM_PATCH1,
    _reserved67: [u8; 12usize],
    #[doc = "0x820 - Value of OTP Bank6 Word2 (ROM Patch)"]
    pub rom_patch2: ROM_PATCH2,
    _reserved68: [u8; 12usize],
    #[doc = "0x830 - Value of OTP Bank6 Word3 (ROM Patch)"]
    pub rom_patch3: ROM_PATCH3,
    _reserved69: [u8; 12usize],
    #[doc = "0x840 - Value of OTP Bank6 Word4 (ROM Patch)"]
    pub rom_patch4: ROM_PATCH4,
    _reserved70: [u8; 12usize],
    #[doc = "0x850 - Value of OTP Bank6 Word5 (ROM Patch)"]
    pub rom_patch5: ROM_PATCH5,
    _reserved71: [u8; 12usize],
    #[doc = "0x860 - Value of OTP Bank6 Word6 (ROM Patch)"]
    pub rom_patch6: ROM_PATCH6,
    _reserved72: [u8; 12usize],
    #[doc = "0x870 - Value of OTP Bank6 Word7 (ROM Patch)"]
    pub rom_patch7: ROM_PATCH7,
    _reserved73: [u8; 12usize],
    #[doc = "0x880 - Value of OTP Bank7 Word0 (GP3)"]
    pub gp30: GP30,
    _reserved74: [u8; 12usize],
    #[doc = "0x890 - Value of OTP Bank7 Word1 (GP3)"]
    pub gp31: GP31,
    _reserved75: [u8; 12usize],
    #[doc = "0x8a0 - Value of OTP Bank7 Word2 (GP3)"]
    pub gp32: GP32,
    _reserved76: [u8; 12usize],
    #[doc = "0x8b0 - Value of OTP Bank7 Word3 (GP3)"]
    pub gp33: GP33,
    _reserved77: [u8; 12usize],
    #[doc = "0x8c0 - Value of OTP Bank7 Word4 (GP4)"]
    pub gp40: GP40,
    _reserved78: [u8; 12usize],
    #[doc = "0x8d0 - Value of OTP Bank7 Word5 (GP4)"]
    pub gp41: GP41,
    _reserved79: [u8; 12usize],
    #[doc = "0x8e0 - Value of OTP Bank7 Word6 (GP4)"]
    pub gp42: GP42,
    _reserved80: [u8; 12usize],
    #[doc = "0x8f0 - Value of OTP Bank7 Word7 (GP4)"]
    pub gp43: GP43,
}
#[doc = "OTP Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "OTP Controller Control Register"]
pub mod ctrl;
#[doc = "OTP Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "OTP Controller Control Register"]
pub mod ctrl_set;
#[doc = "OTP Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "OTP Controller Control Register"]
pub mod ctrl_clr;
#[doc = "OTP Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tog](ctrl_tog) module"]
pub type CTRL_TOG = crate::Reg<u32, _CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_TOG;
#[doc = "`read()` method returns [ctrl_tog::R](ctrl_tog::R) reader structure"]
impl crate::Readable for CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](ctrl_tog::W) writer structure"]
impl crate::Writable for CTRL_TOG {}
#[doc = "OTP Controller Control Register"]
pub mod ctrl_tog;
#[doc = "OTP Controller Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](timing) module"]
pub type TIMING = crate::Reg<u32, _TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMING;
#[doc = "`read()` method returns [timing::R](timing::R) reader structure"]
impl crate::Readable for TIMING {}
#[doc = "`write(|w| ..)` method takes [timing::W](timing::W) writer structure"]
impl crate::Writable for TIMING {}
#[doc = "OTP Controller Timing Register"]
pub mod timing;
#[doc = "OTP Controller Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "OTP Controller Write Data Register"]
pub mod data;
#[doc = "OTP Controller Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [read_ctrl](read_ctrl) module"]
pub type READ_CTRL = crate::Reg<u32, _READ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READ_CTRL;
#[doc = "`read()` method returns [read_ctrl::R](read_ctrl::R) reader structure"]
impl crate::Readable for READ_CTRL {}
#[doc = "`write(|w| ..)` method takes [read_ctrl::W](read_ctrl::W) writer structure"]
impl crate::Writable for READ_CTRL {}
#[doc = "OTP Controller Write Data Register"]
pub mod read_ctrl;
#[doc = "OTP Controller Read Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [read_fuse_data](read_fuse_data) module"]
pub type READ_FUSE_DATA = crate::Reg<u32, _READ_FUSE_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READ_FUSE_DATA;
#[doc = "`read()` method returns [read_fuse_data::R](read_fuse_data::R) reader structure"]
impl crate::Readable for READ_FUSE_DATA {}
#[doc = "`write(|w| ..)` method takes [read_fuse_data::W](read_fuse_data::W) writer structure"]
impl crate::Writable for READ_FUSE_DATA {}
#[doc = "OTP Controller Read Data Register"]
pub mod read_fuse_data;
#[doc = "Sticky bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_sticky](sw_sticky) module"]
pub type SW_STICKY = crate::Reg<u32, _SW_STICKY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_STICKY;
#[doc = "`read()` method returns [sw_sticky::R](sw_sticky::R) reader structure"]
impl crate::Readable for SW_STICKY {}
#[doc = "`write(|w| ..)` method takes [sw_sticky::W](sw_sticky::W) writer structure"]
impl crate::Writable for SW_STICKY {}
#[doc = "Sticky bit Register"]
pub mod sw_sticky;
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs](scs) module"]
pub type SCS = crate::Reg<u32, _SCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS;
#[doc = "`read()` method returns [scs::R](scs::R) reader structure"]
impl crate::Readable for SCS {}
#[doc = "`write(|w| ..)` method takes [scs::W](scs::W) writer structure"]
impl crate::Writable for SCS {}
#[doc = "Software Controllable Signals Register"]
pub mod scs;
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs_set](scs_set) module"]
pub type SCS_SET = crate::Reg<u32, _SCS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS_SET;
#[doc = "`read()` method returns [scs_set::R](scs_set::R) reader structure"]
impl crate::Readable for SCS_SET {}
#[doc = "`write(|w| ..)` method takes [scs_set::W](scs_set::W) writer structure"]
impl crate::Writable for SCS_SET {}
#[doc = "Software Controllable Signals Register"]
pub mod scs_set;
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs_clr](scs_clr) module"]
pub type SCS_CLR = crate::Reg<u32, _SCS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS_CLR;
#[doc = "`read()` method returns [scs_clr::R](scs_clr::R) reader structure"]
impl crate::Readable for SCS_CLR {}
#[doc = "`write(|w| ..)` method takes [scs_clr::W](scs_clr::W) writer structure"]
impl crate::Writable for SCS_CLR {}
#[doc = "Software Controllable Signals Register"]
pub mod scs_clr;
#[doc = "Software Controllable Signals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs_tog](scs_tog) module"]
pub type SCS_TOG = crate::Reg<u32, _SCS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS_TOG;
#[doc = "`read()` method returns [scs_tog::R](scs_tog::R) reader structure"]
impl crate::Readable for SCS_TOG {}
#[doc = "`write(|w| ..)` method takes [scs_tog::W](scs_tog::W) writer structure"]
impl crate::Writable for SCS_TOG {}
#[doc = "Software Controllable Signals Register"]
pub mod scs_tog;
#[doc = "OTP Controller CRC test address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_addr](crc_addr) module"]
pub type CRC_ADDR = crate::Reg<u32, _CRC_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADDR;
#[doc = "`read()` method returns [crc_addr::R](crc_addr::R) reader structure"]
impl crate::Readable for CRC_ADDR {}
#[doc = "`write(|w| ..)` method takes [crc_addr::W](crc_addr::W) writer structure"]
impl crate::Writable for CRC_ADDR {}
#[doc = "OTP Controller CRC test address"]
pub mod crc_addr;
#[doc = "OTP Controller CRC Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_value](crc_value) module"]
pub type CRC_VALUE = crate::Reg<u32, _CRC_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_VALUE;
#[doc = "`read()` method returns [crc_value::R](crc_value::R) reader structure"]
impl crate::Readable for CRC_VALUE {}
#[doc = "`write(|w| ..)` method takes [crc_value::W](crc_value::W) writer structure"]
impl crate::Writable for CRC_VALUE {}
#[doc = "OTP Controller CRC Value Register"]
pub mod crc_value;
#[doc = "OTP Controller Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "OTP Controller Version Register"]
pub mod version;
#[doc = "OTP Controller Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing2](timing2) module"]
pub type TIMING2 = crate::Reg<u32, _TIMING2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMING2;
#[doc = "`read()` method returns [timing2::R](timing2::R) reader structure"]
impl crate::Readable for TIMING2 {}
#[doc = "`write(|w| ..)` method takes [timing2::W](timing2::W) writer structure"]
impl crate::Writable for TIMING2 {}
#[doc = "OTP Controller Timing Register"]
pub mod timing2;
#[doc = "Value of OTP Bank0 Word0 (Lock controls)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod lock;
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub mod cfg0;
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
pub mod cfg1;
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
pub mod cfg2;
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg3](cfg3) module"]
pub type CFG3 = crate::Reg<u32, _CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG3;
#[doc = "`read()` method returns [cfg3::R](cfg3::R) reader structure"]
impl crate::Readable for CFG3 {}
#[doc = "`write(|w| ..)` method takes [cfg3::W](cfg3::W) writer structure"]
impl crate::Writable for CFG3 {}
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
pub mod cfg3;
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg4](cfg4) module"]
pub type CFG4 = crate::Reg<u32, _CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG4;
#[doc = "`read()` method returns [cfg4::R](cfg4::R) reader structure"]
impl crate::Readable for CFG4 {}
#[doc = "`write(|w| ..)` method takes [cfg4::W](cfg4::W) writer structure"]
impl crate::Writable for CFG4 {}
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
pub mod cfg4;
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg5](cfg5) module"]
pub type CFG5 = crate::Reg<u32, _CFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG5;
#[doc = "`read()` method returns [cfg5::R](cfg5::R) reader structure"]
impl crate::Readable for CFG5 {}
#[doc = "`write(|w| ..)` method takes [cfg5::W](cfg5::W) writer structure"]
impl crate::Writable for CFG5 {}
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
pub mod cfg5;
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg6](cfg6) module"]
pub type CFG6 = crate::Reg<u32, _CFG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG6;
#[doc = "`read()` method returns [cfg6::R](cfg6::R) reader structure"]
impl crate::Readable for CFG6 {}
#[doc = "`write(|w| ..)` method takes [cfg6::W](cfg6::W) writer structure"]
impl crate::Writable for CFG6 {}
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
pub mod cfg6;
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem0](mem0) module"]
pub type MEM0 = crate::Reg<u32, _MEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM0;
#[doc = "`read()` method returns [mem0::R](mem0::R) reader structure"]
impl crate::Readable for MEM0 {}
#[doc = "`write(|w| ..)` method takes [mem0::W](mem0::W) writer structure"]
impl crate::Writable for MEM0 {}
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
pub mod mem0;
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem1](mem1) module"]
pub type MEM1 = crate::Reg<u32, _MEM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM1;
#[doc = "`read()` method returns [mem1::R](mem1::R) reader structure"]
impl crate::Readable for MEM1 {}
#[doc = "`write(|w| ..)` method takes [mem1::W](mem1::W) writer structure"]
impl crate::Writable for MEM1 {}
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
pub mod mem1;
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem2](mem2) module"]
pub type MEM2 = crate::Reg<u32, _MEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM2;
#[doc = "`read()` method returns [mem2::R](mem2::R) reader structure"]
impl crate::Readable for MEM2 {}
#[doc = "`write(|w| ..)` method takes [mem2::W](mem2::W) writer structure"]
impl crate::Writable for MEM2 {}
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
pub mod mem2;
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem3](mem3) module"]
pub type MEM3 = crate::Reg<u32, _MEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM3;
#[doc = "`read()` method returns [mem3::R](mem3::R) reader structure"]
impl crate::Readable for MEM3 {}
#[doc = "`write(|w| ..)` method takes [mem3::W](mem3::W) writer structure"]
impl crate::Writable for MEM3 {}
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
pub mod mem3;
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem4](mem4) module"]
pub type MEM4 = crate::Reg<u32, _MEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEM4;
#[doc = "`read()` method returns [mem4::R](mem4::R) reader structure"]
impl crate::Readable for MEM4 {}
#[doc = "`write(|w| ..)` method takes [mem4::W](mem4::W) writer structure"]
impl crate::Writable for MEM4 {}
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
pub mod mem4;
#[doc = "Value of OTP Bank1 Word5 (Memory Related Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana0](ana0) module"]
pub type ANA0 = crate::Reg<u32, _ANA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA0;
#[doc = "`read()` method returns [ana0::R](ana0::R) reader structure"]
impl crate::Readable for ANA0 {}
#[doc = "`write(|w| ..)` method takes [ana0::W](ana0::W) writer structure"]
impl crate::Writable for ANA0 {}
#[doc = "Value of OTP Bank1 Word5 (Memory Related Info.)"]
pub mod ana0;
#[doc = "Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana1](ana1) module"]
pub type ANA1 = crate::Reg<u32, _ANA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA1;
#[doc = "`read()` method returns [ana1::R](ana1::R) reader structure"]
impl crate::Readable for ANA1 {}
#[doc = "`write(|w| ..)` method takes [ana1::W](ana1::W) writer structure"]
impl crate::Writable for ANA1 {}
#[doc = "Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)"]
pub mod ana1;
#[doc = "Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana2](ana2) module"]
pub type ANA2 = crate::Reg<u32, _ANA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA2;
#[doc = "`read()` method returns [ana2::R](ana2::R) reader structure"]
impl crate::Readable for ANA2 {}
#[doc = "`write(|w| ..)` method takes [ana2::W](ana2::W) writer structure"]
impl crate::Writable for ANA2 {}
#[doc = "Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)"]
pub mod ana2;
#[doc = "Value of OTP Bank2 Word0 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk0](otpmk0) module"]
pub type OTPMK0 = crate::Reg<u32, _OTPMK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK0;
#[doc = "`read()` method returns [otpmk0::R](otpmk0::R) reader structure"]
impl crate::Readable for OTPMK0 {}
#[doc = "`write(|w| ..)` method takes [otpmk0::W](otpmk0::W) writer structure"]
impl crate::Writable for OTPMK0 {}
#[doc = "Value of OTP Bank2 Word0 (OTPMK Key)"]
pub mod otpmk0;
#[doc = "Value of OTP Bank2 Word1 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk1](otpmk1) module"]
pub type OTPMK1 = crate::Reg<u32, _OTPMK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK1;
#[doc = "`read()` method returns [otpmk1::R](otpmk1::R) reader structure"]
impl crate::Readable for OTPMK1 {}
#[doc = "`write(|w| ..)` method takes [otpmk1::W](otpmk1::W) writer structure"]
impl crate::Writable for OTPMK1 {}
#[doc = "Value of OTP Bank2 Word1 (OTPMK Key)"]
pub mod otpmk1;
#[doc = "Value of OTP Bank2 Word2 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk2](otpmk2) module"]
pub type OTPMK2 = crate::Reg<u32, _OTPMK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK2;
#[doc = "`read()` method returns [otpmk2::R](otpmk2::R) reader structure"]
impl crate::Readable for OTPMK2 {}
#[doc = "`write(|w| ..)` method takes [otpmk2::W](otpmk2::W) writer structure"]
impl crate::Writable for OTPMK2 {}
#[doc = "Value of OTP Bank2 Word2 (OTPMK Key)"]
pub mod otpmk2;
#[doc = "Value of OTP Bank2 Word3 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk3](otpmk3) module"]
pub type OTPMK3 = crate::Reg<u32, _OTPMK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK3;
#[doc = "`read()` method returns [otpmk3::R](otpmk3::R) reader structure"]
impl crate::Readable for OTPMK3 {}
#[doc = "`write(|w| ..)` method takes [otpmk3::W](otpmk3::W) writer structure"]
impl crate::Writable for OTPMK3 {}
#[doc = "Value of OTP Bank2 Word3 (OTPMK Key)"]
pub mod otpmk3;
#[doc = "Value of OTP Bank2 Word4 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk4](otpmk4) module"]
pub type OTPMK4 = crate::Reg<u32, _OTPMK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK4;
#[doc = "`read()` method returns [otpmk4::R](otpmk4::R) reader structure"]
impl crate::Readable for OTPMK4 {}
#[doc = "`write(|w| ..)` method takes [otpmk4::W](otpmk4::W) writer structure"]
impl crate::Writable for OTPMK4 {}
#[doc = "Value of OTP Bank2 Word4 (OTPMK Key)"]
pub mod otpmk4;
#[doc = "Value of OTP Bank2 Word5 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk5](otpmk5) module"]
pub type OTPMK5 = crate::Reg<u32, _OTPMK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK5;
#[doc = "`read()` method returns [otpmk5::R](otpmk5::R) reader structure"]
impl crate::Readable for OTPMK5 {}
#[doc = "`write(|w| ..)` method takes [otpmk5::W](otpmk5::W) writer structure"]
impl crate::Writable for OTPMK5 {}
#[doc = "Value of OTP Bank2 Word5 (OTPMK Key)"]
pub mod otpmk5;
#[doc = "Value of OTP Bank2 Word6 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk6](otpmk6) module"]
pub type OTPMK6 = crate::Reg<u32, _OTPMK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK6;
#[doc = "`read()` method returns [otpmk6::R](otpmk6::R) reader structure"]
impl crate::Readable for OTPMK6 {}
#[doc = "`write(|w| ..)` method takes [otpmk6::W](otpmk6::W) writer structure"]
impl crate::Writable for OTPMK6 {}
#[doc = "Value of OTP Bank2 Word6 (OTPMK Key)"]
pub mod otpmk6;
#[doc = "Value of OTP Bank2 Word7 (OTPMK Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk7](otpmk7) module"]
pub type OTPMK7 = crate::Reg<u32, _OTPMK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK7;
#[doc = "`read()` method returns [otpmk7::R](otpmk7::R) reader structure"]
impl crate::Readable for OTPMK7 {}
#[doc = "`write(|w| ..)` method takes [otpmk7::W](otpmk7::W) writer structure"]
impl crate::Writable for OTPMK7 {}
#[doc = "Value of OTP Bank2 Word7 (OTPMK Key)"]
pub mod otpmk7;
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk0](srk0) module"]
pub type SRK0 = crate::Reg<u32, _SRK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK0;
#[doc = "`read()` method returns [srk0::R](srk0::R) reader structure"]
impl crate::Readable for SRK0 {}
#[doc = "`write(|w| ..)` method takes [srk0::W](srk0::W) writer structure"]
impl crate::Writable for SRK0 {}
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
pub mod srk0;
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk1](srk1) module"]
pub type SRK1 = crate::Reg<u32, _SRK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK1;
#[doc = "`read()` method returns [srk1::R](srk1::R) reader structure"]
impl crate::Readable for SRK1 {}
#[doc = "`write(|w| ..)` method takes [srk1::W](srk1::W) writer structure"]
impl crate::Writable for SRK1 {}
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
pub mod srk1;
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk2](srk2) module"]
pub type SRK2 = crate::Reg<u32, _SRK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK2;
#[doc = "`read()` method returns [srk2::R](srk2::R) reader structure"]
impl crate::Readable for SRK2 {}
#[doc = "`write(|w| ..)` method takes [srk2::W](srk2::W) writer structure"]
impl crate::Writable for SRK2 {}
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
pub mod srk2;
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk3](srk3) module"]
pub type SRK3 = crate::Reg<u32, _SRK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK3;
#[doc = "`read()` method returns [srk3::R](srk3::R) reader structure"]
impl crate::Readable for SRK3 {}
#[doc = "`write(|w| ..)` method takes [srk3::W](srk3::W) writer structure"]
impl crate::Writable for SRK3 {}
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
pub mod srk3;
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk4](srk4) module"]
pub type SRK4 = crate::Reg<u32, _SRK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK4;
#[doc = "`read()` method returns [srk4::R](srk4::R) reader structure"]
impl crate::Readable for SRK4 {}
#[doc = "`write(|w| ..)` method takes [srk4::W](srk4::W) writer structure"]
impl crate::Writable for SRK4 {}
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
pub mod srk4;
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk5](srk5) module"]
pub type SRK5 = crate::Reg<u32, _SRK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK5;
#[doc = "`read()` method returns [srk5::R](srk5::R) reader structure"]
impl crate::Readable for SRK5 {}
#[doc = "`write(|w| ..)` method takes [srk5::W](srk5::W) writer structure"]
impl crate::Writable for SRK5 {}
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
pub mod srk5;
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk6](srk6) module"]
pub type SRK6 = crate::Reg<u32, _SRK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK6;
#[doc = "`read()` method returns [srk6::R](srk6::R) reader structure"]
impl crate::Readable for SRK6 {}
#[doc = "`write(|w| ..)` method takes [srk6::W](srk6::W) writer structure"]
impl crate::Writable for SRK6 {}
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
pub mod srk6;
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk7](srk7) module"]
pub type SRK7 = crate::Reg<u32, _SRK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK7;
#[doc = "`read()` method returns [srk7::R](srk7::R) reader structure"]
impl crate::Readable for SRK7 {}
#[doc = "`write(|w| ..)` method takes [srk7::W](srk7::W) writer structure"]
impl crate::Writable for SRK7 {}
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
pub mod srk7;
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sjc_resp0](sjc_resp0) module"]
pub type SJC_RESP0 = crate::Reg<u32, _SJC_RESP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SJC_RESP0;
#[doc = "`read()` method returns [sjc_resp0::R](sjc_resp0::R) reader structure"]
impl crate::Readable for SJC_RESP0 {}
#[doc = "`write(|w| ..)` method takes [sjc_resp0::W](sjc_resp0::W) writer structure"]
impl crate::Writable for SJC_RESP0 {}
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
pub mod sjc_resp0;
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sjc_resp1](sjc_resp1) module"]
pub type SJC_RESP1 = crate::Reg<u32, _SJC_RESP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SJC_RESP1;
#[doc = "`read()` method returns [sjc_resp1::R](sjc_resp1::R) reader structure"]
impl crate::Readable for SJC_RESP1 {}
#[doc = "`write(|w| ..)` method takes [sjc_resp1::W](sjc_resp1::W) writer structure"]
impl crate::Writable for SJC_RESP1 {}
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
pub mod sjc_resp1;
#[doc = "Value of OTP Bank4 Word2 (MAC Address)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac0](mac0) module"]
pub type MAC0 = crate::Reg<u32, _MAC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC0;
#[doc = "`read()` method returns [mac0::R](mac0::R) reader structure"]
impl crate::Readable for MAC0 {}
#[doc = "`write(|w| ..)` method takes [mac0::W](mac0::W) writer structure"]
impl crate::Writable for MAC0 {}
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
pub mod mac0;
#[doc = "Value of OTP Bank4 Word3 (MAC Address)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac1](mac1) module"]
pub type MAC1 = crate::Reg<u32, _MAC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC1;
#[doc = "`read()` method returns [mac1::R](mac1::R) reader structure"]
impl crate::Readable for MAC1 {}
#[doc = "`write(|w| ..)` method takes [mac1::W](mac1::W) writer structure"]
impl crate::Writable for MAC1 {}
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
pub mod mac1;
#[doc = "Value of OTP Bank4 Word4 (MAC2 Address)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac2](mac2) module"]
pub type MAC2 = crate::Reg<u32, _MAC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC2;
#[doc = "`read()` method returns [mac2::R](mac2::R) reader structure"]
impl crate::Readable for MAC2 {}
#[doc = "`write(|w| ..)` method takes [mac2::W](mac2::W) writer structure"]
impl crate::Writable for MAC2 {}
#[doc = "Value of OTP Bank4 Word4 (MAC2 Address)"]
pub mod mac2;
#[doc = "Value of OTP Bank4 Word5 (CRC Key)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpmk_crc32](otpmk_crc32) module"]
pub type OTPMK_CRC32 = crate::Reg<u32, _OTPMK_CRC32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPMK_CRC32;
#[doc = "`read()` method returns [otpmk_crc32::R](otpmk_crc32::R) reader structure"]
impl crate::Readable for OTPMK_CRC32 {}
#[doc = "`write(|w| ..)` method takes [otpmk_crc32::W](otpmk_crc32::W) writer structure"]
impl crate::Writable for OTPMK_CRC32 {}
#[doc = "Value of OTP Bank4 Word5 (CRC Key)"]
pub mod otpmk_crc32;
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp1](gp1) module"]
pub type GP1 = crate::Reg<u32, _GP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP1;
#[doc = "`read()` method returns [gp1::R](gp1::R) reader structure"]
impl crate::Readable for GP1 {}
#[doc = "`write(|w| ..)` method takes [gp1::W](gp1::W) writer structure"]
impl crate::Writable for GP1 {}
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
pub mod gp1;
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp2](gp2) module"]
pub type GP2 = crate::Reg<u32, _GP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP2;
#[doc = "`read()` method returns [gp2::R](gp2::R) reader structure"]
impl crate::Readable for GP2 {}
#[doc = "`write(|w| ..)` method takes [gp2::W](gp2::W) writer structure"]
impl crate::Writable for GP2 {}
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
pub mod gp2;
#[doc = "Value of OTP Bank5 Word0 (SW GP1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_gp1](sw_gp1) module"]
pub type SW_GP1 = crate::Reg<u32, _SW_GP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_GP1;
#[doc = "`read()` method returns [sw_gp1::R](sw_gp1::R) reader structure"]
impl crate::Readable for SW_GP1 {}
#[doc = "`write(|w| ..)` method takes [sw_gp1::W](sw_gp1::W) writer structure"]
impl crate::Writable for SW_GP1 {}
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
pub mod sw_gp1;
#[doc = "Value of OTP Bank5 Word1 (SW GP2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_gp20](sw_gp20) module"]
pub type SW_GP20 = crate::Reg<u32, _SW_GP20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_GP20;
#[doc = "`read()` method returns [sw_gp20::R](sw_gp20::R) reader structure"]
impl crate::Readable for SW_GP20 {}
#[doc = "`write(|w| ..)` method takes [sw_gp20::W](sw_gp20::W) writer structure"]
impl crate::Writable for SW_GP20 {}
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
pub mod sw_gp20;
#[doc = "Value of OTP Bank5 Word2 (SW GP2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_gp21](sw_gp21) module"]
pub type SW_GP21 = crate::Reg<u32, _SW_GP21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_GP21;
#[doc = "`read()` method returns [sw_gp21::R](sw_gp21::R) reader structure"]
impl crate::Readable for SW_GP21 {}
#[doc = "`write(|w| ..)` method takes [sw_gp21::W](sw_gp21::W) writer structure"]
impl crate::Writable for SW_GP21 {}
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
pub mod sw_gp21;
#[doc = "Value of OTP Bank5 Word3 (SW GP2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_gp22](sw_gp22) module"]
pub type SW_GP22 = crate::Reg<u32, _SW_GP22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_GP22;
#[doc = "`read()` method returns [sw_gp22::R](sw_gp22::R) reader structure"]
impl crate::Readable for SW_GP22 {}
#[doc = "`write(|w| ..)` method takes [sw_gp22::W](sw_gp22::W) writer structure"]
impl crate::Writable for SW_GP22 {}
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
pub mod sw_gp22;
#[doc = "Value of OTP Bank5 Word4 (SW GP2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_gp23](sw_gp23) module"]
pub type SW_GP23 = crate::Reg<u32, _SW_GP23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_GP23;
#[doc = "`read()` method returns [sw_gp23::R](sw_gp23::R) reader structure"]
impl crate::Readable for SW_GP23 {}
#[doc = "`write(|w| ..)` method takes [sw_gp23::W](sw_gp23::W) writer structure"]
impl crate::Writable for SW_GP23 {}
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
pub mod sw_gp23;
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf0](misc_conf0) module"]
pub type MISC_CONF0 = crate::Reg<u32, _MISC_CONF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CONF0;
#[doc = "`read()` method returns [misc_conf0::R](misc_conf0::R) reader structure"]
impl crate::Readable for MISC_CONF0 {}
#[doc = "`write(|w| ..)` method takes [misc_conf0::W](misc_conf0::W) writer structure"]
impl crate::Writable for MISC_CONF0 {}
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
pub mod misc_conf0;
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf1](misc_conf1) module"]
pub type MISC_CONF1 = crate::Reg<u32, _MISC_CONF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CONF1;
#[doc = "`read()` method returns [misc_conf1::R](misc_conf1::R) reader structure"]
impl crate::Readable for MISC_CONF1 {}
#[doc = "`write(|w| ..)` method takes [misc_conf1::W](misc_conf1::W) writer structure"]
impl crate::Writable for MISC_CONF1 {}
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
pub mod misc_conf1;
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srk_revoke](srk_revoke) module"]
pub type SRK_REVOKE = crate::Reg<u32, _SRK_REVOKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRK_REVOKE;
#[doc = "`read()` method returns [srk_revoke::R](srk_revoke::R) reader structure"]
impl crate::Readable for SRK_REVOKE {}
#[doc = "`write(|w| ..)` method takes [srk_revoke::W](srk_revoke::W) writer structure"]
impl crate::Writable for SRK_REVOKE {}
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
pub mod srk_revoke;
#[doc = "Value of OTP Bank6 Word0 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch0](rom_patch0) module"]
pub type ROM_PATCH0 = crate::Reg<u32, _ROM_PATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH0;
#[doc = "`read()` method returns [rom_patch0::R](rom_patch0::R) reader structure"]
impl crate::Readable for ROM_PATCH0 {}
#[doc = "`write(|w| ..)` method takes [rom_patch0::W](rom_patch0::W) writer structure"]
impl crate::Writable for ROM_PATCH0 {}
#[doc = "Value of OTP Bank6 Word0 (ROM Patch)"]
pub mod rom_patch0;
#[doc = "Value of OTP Bank6 Word1 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch1](rom_patch1) module"]
pub type ROM_PATCH1 = crate::Reg<u32, _ROM_PATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH1;
#[doc = "`read()` method returns [rom_patch1::R](rom_patch1::R) reader structure"]
impl crate::Readable for ROM_PATCH1 {}
#[doc = "`write(|w| ..)` method takes [rom_patch1::W](rom_patch1::W) writer structure"]
impl crate::Writable for ROM_PATCH1 {}
#[doc = "Value of OTP Bank6 Word1 (ROM Patch)"]
pub mod rom_patch1;
#[doc = "Value of OTP Bank6 Word2 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch2](rom_patch2) module"]
pub type ROM_PATCH2 = crate::Reg<u32, _ROM_PATCH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH2;
#[doc = "`read()` method returns [rom_patch2::R](rom_patch2::R) reader structure"]
impl crate::Readable for ROM_PATCH2 {}
#[doc = "`write(|w| ..)` method takes [rom_patch2::W](rom_patch2::W) writer structure"]
impl crate::Writable for ROM_PATCH2 {}
#[doc = "Value of OTP Bank6 Word2 (ROM Patch)"]
pub mod rom_patch2;
#[doc = "Value of OTP Bank6 Word3 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch3](rom_patch3) module"]
pub type ROM_PATCH3 = crate::Reg<u32, _ROM_PATCH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH3;
#[doc = "`read()` method returns [rom_patch3::R](rom_patch3::R) reader structure"]
impl crate::Readable for ROM_PATCH3 {}
#[doc = "`write(|w| ..)` method takes [rom_patch3::W](rom_patch3::W) writer structure"]
impl crate::Writable for ROM_PATCH3 {}
#[doc = "Value of OTP Bank6 Word3 (ROM Patch)"]
pub mod rom_patch3;
#[doc = "Value of OTP Bank6 Word4 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch4](rom_patch4) module"]
pub type ROM_PATCH4 = crate::Reg<u32, _ROM_PATCH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH4;
#[doc = "`read()` method returns [rom_patch4::R](rom_patch4::R) reader structure"]
impl crate::Readable for ROM_PATCH4 {}
#[doc = "`write(|w| ..)` method takes [rom_patch4::W](rom_patch4::W) writer structure"]
impl crate::Writable for ROM_PATCH4 {}
#[doc = "Value of OTP Bank6 Word4 (ROM Patch)"]
pub mod rom_patch4;
#[doc = "Value of OTP Bank6 Word5 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch5](rom_patch5) module"]
pub type ROM_PATCH5 = crate::Reg<u32, _ROM_PATCH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH5;
#[doc = "`read()` method returns [rom_patch5::R](rom_patch5::R) reader structure"]
impl crate::Readable for ROM_PATCH5 {}
#[doc = "`write(|w| ..)` method takes [rom_patch5::W](rom_patch5::W) writer structure"]
impl crate::Writable for ROM_PATCH5 {}
#[doc = "Value of OTP Bank6 Word5 (ROM Patch)"]
pub mod rom_patch5;
#[doc = "Value of OTP Bank6 Word6 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch6](rom_patch6) module"]
pub type ROM_PATCH6 = crate::Reg<u32, _ROM_PATCH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH6;
#[doc = "`read()` method returns [rom_patch6::R](rom_patch6::R) reader structure"]
impl crate::Readable for ROM_PATCH6 {}
#[doc = "`write(|w| ..)` method takes [rom_patch6::W](rom_patch6::W) writer structure"]
impl crate::Writable for ROM_PATCH6 {}
#[doc = "Value of OTP Bank6 Word6 (ROM Patch)"]
pub mod rom_patch6;
#[doc = "Value of OTP Bank6 Word7 (ROM Patch)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_patch7](rom_patch7) module"]
pub type ROM_PATCH7 = crate::Reg<u32, _ROM_PATCH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_PATCH7;
#[doc = "`read()` method returns [rom_patch7::R](rom_patch7::R) reader structure"]
impl crate::Readable for ROM_PATCH7 {}
#[doc = "`write(|w| ..)` method takes [rom_patch7::W](rom_patch7::W) writer structure"]
impl crate::Writable for ROM_PATCH7 {}
#[doc = "Value of OTP Bank6 Word7 (ROM Patch)"]
pub mod rom_patch7;
#[doc = "Value of OTP Bank7 Word0 (GP3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp30](gp30) module"]
pub type GP30 = crate::Reg<u32, _GP30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP30;
#[doc = "`read()` method returns [gp30::R](gp30::R) reader structure"]
impl crate::Readable for GP30 {}
#[doc = "`write(|w| ..)` method takes [gp30::W](gp30::W) writer structure"]
impl crate::Writable for GP30 {}
#[doc = "Value of OTP Bank7 Word0 (GP3)"]
pub mod gp30;
#[doc = "Value of OTP Bank7 Word1 (GP3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp31](gp31) module"]
pub type GP31 = crate::Reg<u32, _GP31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP31;
#[doc = "`read()` method returns [gp31::R](gp31::R) reader structure"]
impl crate::Readable for GP31 {}
#[doc = "`write(|w| ..)` method takes [gp31::W](gp31::W) writer structure"]
impl crate::Writable for GP31 {}
#[doc = "Value of OTP Bank7 Word1 (GP3)"]
pub mod gp31;
#[doc = "Value of OTP Bank7 Word2 (GP3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp32](gp32) module"]
pub type GP32 = crate::Reg<u32, _GP32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP32;
#[doc = "`read()` method returns [gp32::R](gp32::R) reader structure"]
impl crate::Readable for GP32 {}
#[doc = "`write(|w| ..)` method takes [gp32::W](gp32::W) writer structure"]
impl crate::Writable for GP32 {}
#[doc = "Value of OTP Bank7 Word2 (GP3)"]
pub mod gp32;
#[doc = "Value of OTP Bank7 Word3 (GP3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp33](gp33) module"]
pub type GP33 = crate::Reg<u32, _GP33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP33;
#[doc = "`read()` method returns [gp33::R](gp33::R) reader structure"]
impl crate::Readable for GP33 {}
#[doc = "`write(|w| ..)` method takes [gp33::W](gp33::W) writer structure"]
impl crate::Writable for GP33 {}
#[doc = "Value of OTP Bank7 Word3 (GP3)"]
pub mod gp33;
#[doc = "Value of OTP Bank7 Word4 (GP4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp40](gp40) module"]
pub type GP40 = crate::Reg<u32, _GP40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP40;
#[doc = "`read()` method returns [gp40::R](gp40::R) reader structure"]
impl crate::Readable for GP40 {}
#[doc = "`write(|w| ..)` method takes [gp40::W](gp40::W) writer structure"]
impl crate::Writable for GP40 {}
#[doc = "Value of OTP Bank7 Word4 (GP4)"]
pub mod gp40;
#[doc = "Value of OTP Bank7 Word5 (GP4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp41](gp41) module"]
pub type GP41 = crate::Reg<u32, _GP41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP41;
#[doc = "`read()` method returns [gp41::R](gp41::R) reader structure"]
impl crate::Readable for GP41 {}
#[doc = "`write(|w| ..)` method takes [gp41::W](gp41::W) writer structure"]
impl crate::Writable for GP41 {}
#[doc = "Value of OTP Bank7 Word5 (GP4)"]
pub mod gp41;
#[doc = "Value of OTP Bank7 Word6 (GP4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp42](gp42) module"]
pub type GP42 = crate::Reg<u32, _GP42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP42;
#[doc = "`read()` method returns [gp42::R](gp42::R) reader structure"]
impl crate::Readable for GP42 {}
#[doc = "`write(|w| ..)` method takes [gp42::W](gp42::W) writer structure"]
impl crate::Writable for GP42 {}
#[doc = "Value of OTP Bank7 Word6 (GP4)"]
pub mod gp42;
#[doc = "Value of OTP Bank7 Word7 (GP4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp43](gp43) module"]
pub type GP43 = crate::Reg<u32, _GP43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP43;
#[doc = "`read()` method returns [gp43::R](gp43::R) reader structure"]
impl crate::Readable for GP43 {}
#[doc = "`write(|w| ..)` method takes [gp43::W](gp43::W) writer structure"]
impl crate::Writable for GP43 {}
#[doc = "Value of OTP Bank7 Word7 (GP4)"]
pub mod gp43;
