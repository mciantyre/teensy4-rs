#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - USB OTG1 Control Register"]
    pub usb_otg1_ctrl: USB_OTG1_CTRL,
    _reserved1: [u8; 20usize],
    #[doc = "0x818 - OTG1 UTMI PHY Control 0 Register"]
    pub usb_otg1_phy_ctrl_0: USB_OTG1_PHY_CTRL_0,
}
#[doc = "USB OTG1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg1_ctrl](usb_otg1_ctrl) module"]
pub type USB_OTG1_CTRL = crate::Reg<u32, _USB_OTG1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_OTG1_CTRL;
#[doc = "`read()` method returns [usb_otg1_ctrl::R](usb_otg1_ctrl::R) reader structure"]
impl crate::Readable for USB_OTG1_CTRL {}
#[doc = "`write(|w| ..)` method takes [usb_otg1_ctrl::W](usb_otg1_ctrl::W) writer structure"]
impl crate::Writable for USB_OTG1_CTRL {}
#[doc = "USB OTG1 Control Register"]
pub mod usb_otg1_ctrl;
#[doc = "OTG1 UTMI PHY Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg1_phy_ctrl_0](usb_otg1_phy_ctrl_0) module"]
pub type USB_OTG1_PHY_CTRL_0 = crate::Reg<u32, _USB_OTG1_PHY_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_OTG1_PHY_CTRL_0;
#[doc = "`read()` method returns [usb_otg1_phy_ctrl_0::R](usb_otg1_phy_ctrl_0::R) reader structure"]
impl crate::Readable for USB_OTG1_PHY_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [usb_otg1_phy_ctrl_0::W](usb_otg1_phy_ctrl_0::W) writer structure"]
impl crate::Writable for USB_OTG1_PHY_CTRL_0 {}
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub mod usb_otg1_phy_ctrl_0;
