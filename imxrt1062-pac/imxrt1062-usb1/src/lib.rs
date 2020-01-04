#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identification register"]
    pub id: ID,
    #[doc = "0x04 - Hardware General"]
    pub hwgeneral: HWGENERAL,
    #[doc = "0x08 - Host Hardware Parameters"]
    pub hwhost: HWHOST,
    #[doc = "0x0c - Device Hardware Parameters"]
    pub hwdevice: HWDEVICE,
    #[doc = "0x10 - TX Buffer Hardware Parameters"]
    pub hwtxbuf: HWTXBUF,
    #[doc = "0x14 - RX Buffer Hardware Parameters"]
    pub hwrxbuf: HWRXBUF,
    _reserved6: [u8; 104usize],
    #[doc = "0x80 - General Purpose Timer #0 Load"]
    pub gptimer0ld: GPTIMER0LD,
    #[doc = "0x84 - General Purpose Timer #0 Controller"]
    pub gptimer0ctrl: GPTIMER0CTRL,
    #[doc = "0x88 - General Purpose Timer #1 Load"]
    pub gptimer1ld: GPTIMER1LD,
    #[doc = "0x8c - General Purpose Timer #1 Controller"]
    pub gptimer1ctrl: GPTIMER1CTRL,
    #[doc = "0x90 - System Bus Config"]
    pub sbuscfg: SBUSCFG,
    _reserved11: [u8; 108usize],
    #[doc = "0x100 - Capability Registers Length"]
    pub caplength: CAPLENGTH,
    _reserved12: [u8; 1usize],
    #[doc = "0x102 - Host Controller Interface Version"]
    pub hciversion: HCIVERSION,
    #[doc = "0x104 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host Controller Capability Parameters"]
    pub hccparams: HCCPARAMS,
    _reserved15: [u8; 20usize],
    #[doc = "0x120 - Device Controller Interface Version"]
    pub dciversion: DCIVERSION,
    _reserved16: [u8; 2usize],
    #[doc = "0x124 - Device Controller Capability Parameters"]
    pub dccparams: DCCPARAMS,
    _reserved17: [u8; 24usize],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - USB Frame Index"]
    pub frindex: FRINDEX,
    _reserved21: [u8; 4usize],
    _reserved_21_deviceaddr: [u8; 4usize],
    _reserved_22_asynclistaddr: [u8; 4usize],
    _reserved23: [u8; 4usize],
    #[doc = "0x160 - Programmable Burst Size"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - TX FIFO Fill Tuning"]
    pub txfilltuning: TXFILLTUNING,
    _reserved25: [u8; 16usize],
    #[doc = "0x178 - Endpoint NAK"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable"]
    pub endptnaken: ENDPTNAKEN,
    #[doc = "0x180 - Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x184 - Port Status & Control"]
    pub portsc1: PORTSC1,
    _reserved29: [u8; 28usize],
    #[doc = "0x1a4 - On-The-Go Status & control"]
    pub otgsc: OTGSC,
    #[doc = "0x1a8 - USB Device Mode"]
    pub usbmode: USBMODE,
    #[doc = "0x1ac - Endpoint Setup Status"]
    pub endptsetupstat: ENDPTSETUPSTAT,
    #[doc = "0x1b0 - Endpoint Prime"]
    pub endptprime: ENDPTPRIME,
    #[doc = "0x1b4 - Endpoint Flush"]
    pub endptflush: ENDPTFLUSH,
    #[doc = "0x1b8 - Endpoint Status"]
    pub endptstat: ENDPTSTAT,
    #[doc = "0x1bc - Endpoint Complete"]
    pub endptcomplete: ENDPTCOMPLETE,
    #[doc = "0x1c0 - Endpoint Control0"]
    pub endptctrl0: ENDPTCTRL0,
    #[doc = "0x1c4 - Endpoint Control 1"]
    pub endptctrl1: ENDPTCTRL1,
    #[doc = "0x1c8 - Endpoint Control 2"]
    pub endptctrl2: ENDPTCTRL2,
    #[doc = "0x1cc - Endpoint Control 3"]
    pub endptctrl3: ENDPTCTRL3,
    #[doc = "0x1d0 - Endpoint Control 4"]
    pub endptctrl4: ENDPTCTRL4,
    #[doc = "0x1d4 - Endpoint Control 5"]
    pub endptctrl5: ENDPTCTRL5,
    #[doc = "0x1d8 - Endpoint Control 6"]
    pub endptctrl6: ENDPTCTRL6,
    #[doc = "0x1dc - Endpoint Control 7"]
    pub endptctrl7: ENDPTCTRL7,
}
impl RegisterBlock {
    #[doc = "0x154 - Frame List Base Address"]
    #[inline(always)]
    pub fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(((self as *const Self) as *const u8).add(340usize) as *const PERIODICLISTBASE) }
    }
    #[doc = "0x154 - Frame List Base Address"]
    #[inline(always)]
    pub fn periodiclistbase_mut(&self) -> &mut PERIODICLISTBASE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(340usize) as *mut PERIODICLISTBASE) }
    }
    #[doc = "0x154 - Device Address"]
    #[inline(always)]
    pub fn deviceaddr(&self) -> &DEVICEADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(340usize) as *const DEVICEADDR) }
    }
    #[doc = "0x154 - Device Address"]
    #[inline(always)]
    pub fn deviceaddr_mut(&self) -> &mut DEVICEADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(340usize) as *mut DEVICEADDR) }
    }
    #[doc = "0x158 - Endpoint List Address"]
    #[inline(always)]
    pub fn endptlistaddr(&self) -> &ENDPTLISTADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(344usize) as *const ENDPTLISTADDR) }
    }
    #[doc = "0x158 - Endpoint List Address"]
    #[inline(always)]
    pub fn endptlistaddr_mut(&self) -> &mut ENDPTLISTADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(344usize) as *mut ENDPTLISTADDR) }
    }
    #[doc = "0x158 - Next Asynch. Address"]
    #[inline(always)]
    pub fn asynclistaddr(&self) -> &ASYNCLISTADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(344usize) as *const ASYNCLISTADDR) }
    }
    #[doc = "0x158 - Next Asynch. Address"]
    #[inline(always)]
    pub fn asynclistaddr_mut(&self) -> &mut ASYNCLISTADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(344usize) as *mut ASYNCLISTADDR) }
    }
}
#[doc = "Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Identification register"]
pub mod id;
#[doc = "Hardware General\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwgeneral](hwgeneral) module"]
pub type HWGENERAL = crate::Reg<u32, _HWGENERAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWGENERAL;
#[doc = "`read()` method returns [hwgeneral::R](hwgeneral::R) reader structure"]
impl crate::Readable for HWGENERAL {}
#[doc = "Hardware General"]
pub mod hwgeneral;
#[doc = "Host Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwhost](hwhost) module"]
pub type HWHOST = crate::Reg<u32, _HWHOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWHOST;
#[doc = "`read()` method returns [hwhost::R](hwhost::R) reader structure"]
impl crate::Readable for HWHOST {}
#[doc = "Host Hardware Parameters"]
pub mod hwhost;
#[doc = "Device Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwdevice](hwdevice) module"]
pub type HWDEVICE = crate::Reg<u32, _HWDEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWDEVICE;
#[doc = "`read()` method returns [hwdevice::R](hwdevice::R) reader structure"]
impl crate::Readable for HWDEVICE {}
#[doc = "Device Hardware Parameters"]
pub mod hwdevice;
#[doc = "TX Buffer Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwtxbuf](hwtxbuf) module"]
pub type HWTXBUF = crate::Reg<u32, _HWTXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWTXBUF;
#[doc = "`read()` method returns [hwtxbuf::R](hwtxbuf::R) reader structure"]
impl crate::Readable for HWTXBUF {}
#[doc = "TX Buffer Hardware Parameters"]
pub mod hwtxbuf;
#[doc = "RX Buffer Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrxbuf](hwrxbuf) module"]
pub type HWRXBUF = crate::Reg<u32, _HWRXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWRXBUF;
#[doc = "`read()` method returns [hwrxbuf::R](hwrxbuf::R) reader structure"]
impl crate::Readable for HWRXBUF {}
#[doc = "RX Buffer Hardware Parameters"]
pub mod hwrxbuf;
#[doc = "General Purpose Timer #0 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer0ld](gptimer0ld) module"]
pub type GPTIMER0LD = crate::Reg<u32, _GPTIMER0LD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMER0LD;
#[doc = "`read()` method returns [gptimer0ld::R](gptimer0ld::R) reader structure"]
impl crate::Readable for GPTIMER0LD {}
#[doc = "`write(|w| ..)` method takes [gptimer0ld::W](gptimer0ld::W) writer structure"]
impl crate::Writable for GPTIMER0LD {}
#[doc = "General Purpose Timer #0 Load"]
pub mod gptimer0ld;
#[doc = "General Purpose Timer #0 Controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer0ctrl](gptimer0ctrl) module"]
pub type GPTIMER0CTRL = crate::Reg<u32, _GPTIMER0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMER0CTRL;
#[doc = "`read()` method returns [gptimer0ctrl::R](gptimer0ctrl::R) reader structure"]
impl crate::Readable for GPTIMER0CTRL {}
#[doc = "`write(|w| ..)` method takes [gptimer0ctrl::W](gptimer0ctrl::W) writer structure"]
impl crate::Writable for GPTIMER0CTRL {}
#[doc = "General Purpose Timer #0 Controller"]
pub mod gptimer0ctrl;
#[doc = "General Purpose Timer #1 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer1ld](gptimer1ld) module"]
pub type GPTIMER1LD = crate::Reg<u32, _GPTIMER1LD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMER1LD;
#[doc = "`read()` method returns [gptimer1ld::R](gptimer1ld::R) reader structure"]
impl crate::Readable for GPTIMER1LD {}
#[doc = "`write(|w| ..)` method takes [gptimer1ld::W](gptimer1ld::W) writer structure"]
impl crate::Writable for GPTIMER1LD {}
#[doc = "General Purpose Timer #1 Load"]
pub mod gptimer1ld;
#[doc = "General Purpose Timer #1 Controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer1ctrl](gptimer1ctrl) module"]
pub type GPTIMER1CTRL = crate::Reg<u32, _GPTIMER1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMER1CTRL;
#[doc = "`read()` method returns [gptimer1ctrl::R](gptimer1ctrl::R) reader structure"]
impl crate::Readable for GPTIMER1CTRL {}
#[doc = "`write(|w| ..)` method takes [gptimer1ctrl::W](gptimer1ctrl::W) writer structure"]
impl crate::Writable for GPTIMER1CTRL {}
#[doc = "General Purpose Timer #1 Controller"]
pub mod gptimer1ctrl;
#[doc = "System Bus Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbuscfg](sbuscfg) module"]
pub type SBUSCFG = crate::Reg<u32, _SBUSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBUSCFG;
#[doc = "`read()` method returns [sbuscfg::R](sbuscfg::R) reader structure"]
impl crate::Readable for SBUSCFG {}
#[doc = "`write(|w| ..)` method takes [sbuscfg::W](sbuscfg::W) writer structure"]
impl crate::Writable for SBUSCFG {}
#[doc = "System Bus Config"]
pub mod sbuscfg;
#[doc = "Capability Registers Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength](caplength) module"]
pub type CAPLENGTH = crate::Reg<u8, _CAPLENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPLENGTH;
#[doc = "`read()` method returns [caplength::R](caplength::R) reader structure"]
impl crate::Readable for CAPLENGTH {}
#[doc = "Capability Registers Length"]
pub mod caplength;
#[doc = "Host Controller Interface Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hciversion](hciversion) module"]
pub type HCIVERSION = crate::Reg<u16, _HCIVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCIVERSION;
#[doc = "`read()` method returns [hciversion::R](hciversion::R) reader structure"]
impl crate::Readable for HCIVERSION {}
#[doc = "Host Controller Interface Version"]
pub mod hciversion;
#[doc = "Host Controller Structural Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](hcsparams) module"]
pub type HCSPARAMS = crate::Reg<u32, _HCSPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPARAMS;
#[doc = "`read()` method returns [hcsparams::R](hcsparams::R) reader structure"]
impl crate::Readable for HCSPARAMS {}
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](hccparams) module"]
pub type HCCPARAMS = crate::Reg<u32, _HCCPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCPARAMS;
#[doc = "`read()` method returns [hccparams::R](hccparams::R) reader structure"]
impl crate::Readable for HCCPARAMS {}
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "Device Controller Interface Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dciversion](dciversion) module"]
pub type DCIVERSION = crate::Reg<u16, _DCIVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIVERSION;
#[doc = "`read()` method returns [dciversion::R](dciversion::R) reader structure"]
impl crate::Readable for DCIVERSION {}
#[doc = "Device Controller Interface Version"]
pub mod dciversion;
#[doc = "Device Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccparams](dccparams) module"]
pub type DCCPARAMS = crate::Reg<u32, _DCCPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCPARAMS;
#[doc = "`read()` method returns [dccparams::R](dccparams::R) reader structure"]
impl crate::Readable for DCCPARAMS {}
#[doc = "Device Controller Capability Parameters"]
pub mod dccparams;
#[doc = "USB Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](usbcmd) module"]
pub type USBCMD = crate::Reg<u32, _USBCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCMD;
#[doc = "`read()` method returns [usbcmd::R](usbcmd::R) reader structure"]
impl crate::Readable for USBCMD {}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](usbcmd::W) writer structure"]
impl crate::Writable for USBCMD {}
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](usbsts) module"]
pub type USBSTS = crate::Reg<u32, _USBSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTS;
#[doc = "`read()` method returns [usbsts::R](usbsts::R) reader structure"]
impl crate::Readable for USBSTS {}
#[doc = "`write(|w| ..)` method takes [usbsts::W](usbsts::W) writer structure"]
impl crate::Writable for USBSTS {}
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](usbintr) module"]
pub type USBINTR = crate::Reg<u32, _USBINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTR;
#[doc = "`read()` method returns [usbintr::R](usbintr::R) reader structure"]
impl crate::Readable for USBINTR {}
#[doc = "`write(|w| ..)` method takes [usbintr::W](usbintr::W) writer structure"]
impl crate::Writable for USBINTR {}
#[doc = "Interrupt Enable Register"]
pub mod usbintr;
#[doc = "USB Frame Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frindex](frindex) module"]
pub type FRINDEX = crate::Reg<u32, _FRINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRINDEX;
#[doc = "`read()` method returns [frindex::R](frindex::R) reader structure"]
impl crate::Readable for FRINDEX {}
#[doc = "`write(|w| ..)` method takes [frindex::W](frindex::W) writer structure"]
impl crate::Writable for FRINDEX {}
#[doc = "USB Frame Index"]
pub mod frindex;
#[doc = "Device Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr](deviceaddr) module"]
pub type DEVICEADDR = crate::Reg<u32, _DEVICEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDR;
#[doc = "`read()` method returns [deviceaddr::R](deviceaddr::R) reader structure"]
impl crate::Readable for DEVICEADDR {}
#[doc = "`write(|w| ..)` method takes [deviceaddr::W](deviceaddr::W) writer structure"]
impl crate::Writable for DEVICEADDR {}
#[doc = "Device Address"]
pub mod deviceaddr;
#[doc = "Frame List Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periodiclistbase](periodiclistbase) module"]
pub type PERIODICLISTBASE = crate::Reg<u32, _PERIODICLISTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIODICLISTBASE;
#[doc = "`read()` method returns [periodiclistbase::R](periodiclistbase::R) reader structure"]
impl crate::Readable for PERIODICLISTBASE {}
#[doc = "`write(|w| ..)` method takes [periodiclistbase::W](periodiclistbase::W) writer structure"]
impl crate::Writable for PERIODICLISTBASE {}
#[doc = "Frame List Base Address"]
pub mod periodiclistbase;
#[doc = "Next Asynch. Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asynclistaddr](asynclistaddr) module"]
pub type ASYNCLISTADDR = crate::Reg<u32, _ASYNCLISTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCLISTADDR;
#[doc = "`read()` method returns [asynclistaddr::R](asynclistaddr::R) reader structure"]
impl crate::Readable for ASYNCLISTADDR {}
#[doc = "`write(|w| ..)` method takes [asynclistaddr::W](asynclistaddr::W) writer structure"]
impl crate::Writable for ASYNCLISTADDR {}
#[doc = "Next Asynch. Address"]
pub mod asynclistaddr;
#[doc = "Endpoint List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptlistaddr](endptlistaddr) module"]
pub type ENDPTLISTADDR = crate::Reg<u32, _ENDPTLISTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTLISTADDR;
#[doc = "`read()` method returns [endptlistaddr::R](endptlistaddr::R) reader structure"]
impl crate::Readable for ENDPTLISTADDR {}
#[doc = "`write(|w| ..)` method takes [endptlistaddr::W](endptlistaddr::W) writer structure"]
impl crate::Writable for ENDPTLISTADDR {}
#[doc = "Endpoint List Address"]
pub mod endptlistaddr;
#[doc = "Programmable Burst Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstsize](burstsize) module"]
pub type BURSTSIZE = crate::Reg<u32, _BURSTSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BURSTSIZE;
#[doc = "`read()` method returns [burstsize::R](burstsize::R) reader structure"]
impl crate::Readable for BURSTSIZE {}
#[doc = "`write(|w| ..)` method takes [burstsize::W](burstsize::W) writer structure"]
impl crate::Writable for BURSTSIZE {}
#[doc = "Programmable Burst Size"]
pub mod burstsize;
#[doc = "TX FIFO Fill Tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfilltuning](txfilltuning) module"]
pub type TXFILLTUNING = crate::Reg<u32, _TXFILLTUNING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFILLTUNING;
#[doc = "`read()` method returns [txfilltuning::R](txfilltuning::R) reader structure"]
impl crate::Readable for TXFILLTUNING {}
#[doc = "`write(|w| ..)` method takes [txfilltuning::W](txfilltuning::W) writer structure"]
impl crate::Writable for TXFILLTUNING {}
#[doc = "TX FIFO Fill Tuning"]
pub mod txfilltuning;
#[doc = "Endpoint NAK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptnak](endptnak) module"]
pub type ENDPTNAK = crate::Reg<u32, _ENDPTNAK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTNAK;
#[doc = "`read()` method returns [endptnak::R](endptnak::R) reader structure"]
impl crate::Readable for ENDPTNAK {}
#[doc = "`write(|w| ..)` method takes [endptnak::W](endptnak::W) writer structure"]
impl crate::Writable for ENDPTNAK {}
#[doc = "Endpoint NAK"]
pub mod endptnak;
#[doc = "Endpoint NAK Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptnaken](endptnaken) module"]
pub type ENDPTNAKEN = crate::Reg<u32, _ENDPTNAKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTNAKEN;
#[doc = "`read()` method returns [endptnaken::R](endptnaken::R) reader structure"]
impl crate::Readable for ENDPTNAKEN {}
#[doc = "`write(|w| ..)` method takes [endptnaken::W](endptnaken::W) writer structure"]
impl crate::Writable for ENDPTNAKEN {}
#[doc = "Endpoint NAK Enable"]
pub mod endptnaken;
#[doc = "Configure Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configflag](configflag) module"]
pub type CONFIGFLAG = crate::Reg<u32, _CONFIGFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIGFLAG;
#[doc = "`read()` method returns [configflag::R](configflag::R) reader structure"]
impl crate::Readable for CONFIGFLAG {}
#[doc = "Configure Flag Register"]
pub mod configflag;
#[doc = "Port Status & Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](portsc1) module"]
pub type PORTSC1 = crate::Reg<u32, _PORTSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTSC1;
#[doc = "`read()` method returns [portsc1::R](portsc1::R) reader structure"]
impl crate::Readable for PORTSC1 {}
#[doc = "`write(|w| ..)` method takes [portsc1::W](portsc1::W) writer structure"]
impl crate::Writable for PORTSC1 {}
#[doc = "Port Status & Control"]
pub mod portsc1;
#[doc = "On-The-Go Status & control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgsc](otgsc) module"]
pub type OTGSC = crate::Reg<u32, _OTGSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGSC;
#[doc = "`read()` method returns [otgsc::R](otgsc::R) reader structure"]
impl crate::Readable for OTGSC {}
#[doc = "`write(|w| ..)` method takes [otgsc::W](otgsc::W) writer structure"]
impl crate::Writable for OTGSC {}
#[doc = "On-The-Go Status & control"]
pub mod otgsc;
#[doc = "USB Device Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmode](usbmode) module"]
pub type USBMODE = crate::Reg<u32, _USBMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBMODE;
#[doc = "`read()` method returns [usbmode::R](usbmode::R) reader structure"]
impl crate::Readable for USBMODE {}
#[doc = "`write(|w| ..)` method takes [usbmode::W](usbmode::W) writer structure"]
impl crate::Writable for USBMODE {}
#[doc = "USB Device Mode"]
pub mod usbmode;
#[doc = "Endpoint Setup Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptsetupstat](endptsetupstat) module"]
pub type ENDPTSETUPSTAT = crate::Reg<u32, _ENDPTSETUPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTSETUPSTAT;
#[doc = "`read()` method returns [endptsetupstat::R](endptsetupstat::R) reader structure"]
impl crate::Readable for ENDPTSETUPSTAT {}
#[doc = "`write(|w| ..)` method takes [endptsetupstat::W](endptsetupstat::W) writer structure"]
impl crate::Writable for ENDPTSETUPSTAT {}
#[doc = "Endpoint Setup Status"]
pub mod endptsetupstat;
#[doc = "Endpoint Prime\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptprime](endptprime) module"]
pub type ENDPTPRIME = crate::Reg<u32, _ENDPTPRIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTPRIME;
#[doc = "`read()` method returns [endptprime::R](endptprime::R) reader structure"]
impl crate::Readable for ENDPTPRIME {}
#[doc = "`write(|w| ..)` method takes [endptprime::W](endptprime::W) writer structure"]
impl crate::Writable for ENDPTPRIME {}
#[doc = "Endpoint Prime"]
pub mod endptprime;
#[doc = "Endpoint Flush\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptflush](endptflush) module"]
pub type ENDPTFLUSH = crate::Reg<u32, _ENDPTFLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTFLUSH;
#[doc = "`read()` method returns [endptflush::R](endptflush::R) reader structure"]
impl crate::Readable for ENDPTFLUSH {}
#[doc = "`write(|w| ..)` method takes [endptflush::W](endptflush::W) writer structure"]
impl crate::Writable for ENDPTFLUSH {}
#[doc = "Endpoint Flush"]
pub mod endptflush;
#[doc = "Endpoint Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptstat](endptstat) module"]
pub type ENDPTSTAT = crate::Reg<u32, _ENDPTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTSTAT;
#[doc = "`read()` method returns [endptstat::R](endptstat::R) reader structure"]
impl crate::Readable for ENDPTSTAT {}
#[doc = "Endpoint Status"]
pub mod endptstat;
#[doc = "Endpoint Complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptcomplete](endptcomplete) module"]
pub type ENDPTCOMPLETE = crate::Reg<u32, _ENDPTCOMPLETE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCOMPLETE;
#[doc = "`read()` method returns [endptcomplete::R](endptcomplete::R) reader structure"]
impl crate::Readable for ENDPTCOMPLETE {}
#[doc = "`write(|w| ..)` method takes [endptcomplete::W](endptcomplete::W) writer structure"]
impl crate::Writable for ENDPTCOMPLETE {}
#[doc = "Endpoint Complete"]
pub mod endptcomplete;
#[doc = "Endpoint Control0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl0](endptctrl0) module"]
pub type ENDPTCTRL0 = crate::Reg<u32, _ENDPTCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL0;
#[doc = "`read()` method returns [endptctrl0::R](endptctrl0::R) reader structure"]
impl crate::Readable for ENDPTCTRL0 {}
#[doc = "`write(|w| ..)` method takes [endptctrl0::W](endptctrl0::W) writer structure"]
impl crate::Writable for ENDPTCTRL0 {}
#[doc = "Endpoint Control0"]
pub mod endptctrl0;
#[doc = "Endpoint Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl1](endptctrl1) module"]
pub type ENDPTCTRL1 = crate::Reg<u32, _ENDPTCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL1;
#[doc = "`read()` method returns [endptctrl1::R](endptctrl1::R) reader structure"]
impl crate::Readable for ENDPTCTRL1 {}
#[doc = "`write(|w| ..)` method takes [endptctrl1::W](endptctrl1::W) writer structure"]
impl crate::Writable for ENDPTCTRL1 {}
#[doc = "Endpoint Control 1"]
pub mod endptctrl1;
#[doc = "Endpoint Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl2](endptctrl2) module"]
pub type ENDPTCTRL2 = crate::Reg<u32, _ENDPTCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL2;
#[doc = "`read()` method returns [endptctrl2::R](endptctrl2::R) reader structure"]
impl crate::Readable for ENDPTCTRL2 {}
#[doc = "`write(|w| ..)` method takes [endptctrl2::W](endptctrl2::W) writer structure"]
impl crate::Writable for ENDPTCTRL2 {}
#[doc = "Endpoint Control 2"]
pub mod endptctrl2;
#[doc = "Endpoint Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl3](endptctrl3) module"]
pub type ENDPTCTRL3 = crate::Reg<u32, _ENDPTCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL3;
#[doc = "`read()` method returns [endptctrl3::R](endptctrl3::R) reader structure"]
impl crate::Readable for ENDPTCTRL3 {}
#[doc = "`write(|w| ..)` method takes [endptctrl3::W](endptctrl3::W) writer structure"]
impl crate::Writable for ENDPTCTRL3 {}
#[doc = "Endpoint Control 3"]
pub mod endptctrl3;
#[doc = "Endpoint Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl4](endptctrl4) module"]
pub type ENDPTCTRL4 = crate::Reg<u32, _ENDPTCTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL4;
#[doc = "`read()` method returns [endptctrl4::R](endptctrl4::R) reader structure"]
impl crate::Readable for ENDPTCTRL4 {}
#[doc = "`write(|w| ..)` method takes [endptctrl4::W](endptctrl4::W) writer structure"]
impl crate::Writable for ENDPTCTRL4 {}
#[doc = "Endpoint Control 4"]
pub mod endptctrl4;
#[doc = "Endpoint Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl5](endptctrl5) module"]
pub type ENDPTCTRL5 = crate::Reg<u32, _ENDPTCTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL5;
#[doc = "`read()` method returns [endptctrl5::R](endptctrl5::R) reader structure"]
impl crate::Readable for ENDPTCTRL5 {}
#[doc = "`write(|w| ..)` method takes [endptctrl5::W](endptctrl5::W) writer structure"]
impl crate::Writable for ENDPTCTRL5 {}
#[doc = "Endpoint Control 5"]
pub mod endptctrl5;
#[doc = "Endpoint Control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl6](endptctrl6) module"]
pub type ENDPTCTRL6 = crate::Reg<u32, _ENDPTCTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL6;
#[doc = "`read()` method returns [endptctrl6::R](endptctrl6::R) reader structure"]
impl crate::Readable for ENDPTCTRL6 {}
#[doc = "`write(|w| ..)` method takes [endptctrl6::W](endptctrl6::W) writer structure"]
impl crate::Writable for ENDPTCTRL6 {}
#[doc = "Endpoint Control 6"]
pub mod endptctrl6;
#[doc = "Endpoint Control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptctrl7](endptctrl7) module"]
pub type ENDPTCTRL7 = crate::Reg<u32, _ENDPTCTRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTCTRL7;
#[doc = "`read()` method returns [endptctrl7::R](endptctrl7::R) reader structure"]
impl crate::Readable for ENDPTCTRL7 {}
#[doc = "`write(|w| ..)` method takes [endptctrl7::W](endptctrl7::W) writer structure"]
impl crate::Writable for ENDPTCTRL7 {}
#[doc = "Endpoint Control 7"]
pub mod endptctrl7;
