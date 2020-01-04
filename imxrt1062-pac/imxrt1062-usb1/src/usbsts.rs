#[doc = "Reader of register USBSTS"]
pub type R = crate::R<u32, super::USBSTS>;
#[doc = "Writer for register USBSTS"]
pub type W = crate::W<u32, super::USBSTS>;
#[doc = "Register USBSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::USBSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UI`"]
pub type UI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UI`"]
pub struct UI_W<'a> {
    w: &'a mut W,
}
impl<'a> UI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UEI`"]
pub type UEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEI`"]
pub struct UEI_W<'a> {
    w: &'a mut W,
}
impl<'a> UEI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PCI`"]
pub type PCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCI`"]
pub struct PCI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FRI`"]
pub type FRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRI`"]
pub struct FRI_W<'a> {
    w: &'a mut W,
}
impl<'a> FRI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SEI`"]
pub type SEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEI`"]
pub struct SEI_W<'a> {
    w: &'a mut W,
}
impl<'a> SEI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AAI`"]
pub type AAI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAI`"]
pub struct AAI_W<'a> {
    w: &'a mut W,
}
impl<'a> AAI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `URI`"]
pub type URI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URI`"]
pub struct URI_W<'a> {
    w: &'a mut W,
}
impl<'a> URI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SRI`"]
pub type SRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRI`"]
pub struct SRI_W<'a> {
    w: &'a mut W,
}
impl<'a> SRI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SLI`"]
pub type SLI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLI`"]
pub struct SLI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ULPII`"]
pub type ULPII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULPII`"]
pub struct ULPII_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPII_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `HCH`"]
pub type HCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCH`"]
pub struct HCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RCL`"]
pub type RCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCL`"]
pub struct RCL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `AS`"]
pub type AS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AS`"]
pub struct AS_W<'a> {
    w: &'a mut W,
}
impl<'a> AS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `NAKI`"]
pub type NAKI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TI0`"]
pub type TI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI0`"]
pub struct TI0_W<'a> {
    w: &'a mut W,
}
impl<'a> TI0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TI1`"]
pub type TI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI1`"]
pub struct TI1_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB Interrupt (USBINT) - R/WC"]
    #[inline(always)]
    pub fn ui(&self) -> UI_R {
        UI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt (USBERRINT) - R/WC"]
    #[inline(always)]
    pub fn uei(&self) -> UEI_R {
        UEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect - R/WC"]
    #[inline(always)]
    pub fn pci(&self) -> PCI_R {
        PCI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover - R/WC"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System Error- R/WC"]
    #[inline(always)]
    pub fn sei(&self) -> SEI_R {
        SEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance - R/WC"]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB Reset Received - R/WC"]
    #[inline(always)]
    pub fn uri(&self) -> URI_R {
        URI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SOF Received - R/WC"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCSuspend - R/WC"]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ULPI Interrupt - R/WC"]
    #[inline(always)]
    pub fn ulpii(&self) -> ULPII_R {
        ULPII_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HCHaIted - Read Only"]
    #[inline(always)]
    pub fn hch(&self) -> HCH_R {
        HCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reclamation - Read Only"]
    #[inline(always)]
    pub fn rcl(&self) -> RCL_R {
        RCL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status - Read Only"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status - Read Only"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK Interrupt Bit--RO"]
    #[inline(always)]
    pub fn naki(&self) -> NAKI_R {
        NAKI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[inline(always)]
    pub fn ti0(&self) -> TI0_R {
        TI0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (USBINT) - R/WC"]
    #[inline(always)]
    pub fn ui(&mut self) -> UI_W {
        UI_W { w: self }
    }
    #[doc = "Bit 1 - USB Error Interrupt (USBERRINT) - R/WC"]
    #[inline(always)]
    pub fn uei(&mut self) -> UEI_W {
        UEI_W { w: self }
    }
    #[doc = "Bit 2 - Port Change Detect - R/WC"]
    #[inline(always)]
    pub fn pci(&mut self) -> PCI_W {
        PCI_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover - R/WC"]
    #[inline(always)]
    pub fn fri(&mut self) -> FRI_W {
        FRI_W { w: self }
    }
    #[doc = "Bit 4 - System Error- R/WC"]
    #[inline(always)]
    pub fn sei(&mut self) -> SEI_W {
        SEI_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt on Async Advance - R/WC"]
    #[inline(always)]
    pub fn aai(&mut self) -> AAI_W {
        AAI_W { w: self }
    }
    #[doc = "Bit 6 - USB Reset Received - R/WC"]
    #[inline(always)]
    pub fn uri(&mut self) -> URI_W {
        URI_W { w: self }
    }
    #[doc = "Bit 7 - SOF Received - R/WC"]
    #[inline(always)]
    pub fn sri(&mut self) -> SRI_W {
        SRI_W { w: self }
    }
    #[doc = "Bit 8 - DCSuspend - R/WC"]
    #[inline(always)]
    pub fn sli(&mut self) -> SLI_W {
        SLI_W { w: self }
    }
    #[doc = "Bit 10 - ULPI Interrupt - R/WC"]
    #[inline(always)]
    pub fn ulpii(&mut self) -> ULPII_W {
        ULPII_W { w: self }
    }
    #[doc = "Bit 12 - HCHaIted - Read Only"]
    #[inline(always)]
    pub fn hch(&mut self) -> HCH_W {
        HCH_W { w: self }
    }
    #[doc = "Bit 13 - Reclamation - Read Only"]
    #[inline(always)]
    pub fn rcl(&mut self) -> RCL_W {
        RCL_W { w: self }
    }
    #[doc = "Bit 14 - Periodic Schedule Status - Read Only"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status - Read Only"]
    #[inline(always)]
    pub fn as_(&mut self) -> AS_W {
        AS_W { w: self }
    }
    #[doc = "Bit 24 - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[inline(always)]
    pub fn ti0(&mut self) -> TI0_W {
        TI0_W { w: self }
    }
    #[doc = "Bit 25 - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    #[inline(always)]
    pub fn ti1(&mut self) -> TI1_W {
        TI1_W { w: self }
    }
}
