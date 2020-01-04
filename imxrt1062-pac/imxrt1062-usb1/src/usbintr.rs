#[doc = "Reader of register USBINTR"]
pub type R = crate::R<u32, super::USBINTR>;
#[doc = "Writer for register USBINTR"]
pub type W = crate::W<u32, super::USBINTR>;
#[doc = "Register USBINTR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBINTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UE`"]
pub type UE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UE`"]
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
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
#[doc = "Reader of field `UEE`"]
pub type UEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEE`"]
pub struct UEE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEE_W<'a> {
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
#[doc = "Reader of field `PCE`"]
pub type PCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCE`"]
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
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
#[doc = "Reader of field `FRE`"]
pub type FRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRE`"]
pub struct FRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRE_W<'a> {
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
#[doc = "Reader of field `SEE`"]
pub type SEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEE`"]
pub struct SEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEE_W<'a> {
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
#[doc = "Reader of field `AAE`"]
pub type AAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAE`"]
pub struct AAE_W<'a> {
    w: &'a mut W,
}
impl<'a> AAE_W<'a> {
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
#[doc = "Reader of field `URE`"]
pub type URE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URE`"]
pub struct URE_W<'a> {
    w: &'a mut W,
}
impl<'a> URE_W<'a> {
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
#[doc = "Reader of field `SRE`"]
pub type SRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRE`"]
pub struct SRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRE_W<'a> {
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
#[doc = "Reader of field `SLE`"]
pub type SLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLE`"]
pub struct SLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLE_W<'a> {
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
#[doc = "Reader of field `ULPIE`"]
pub type ULPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULPIE`"]
pub struct ULPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPIE_W<'a> {
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
#[doc = "Reader of field `NAKE`"]
pub type NAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKE`"]
pub struct NAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UAIE`"]
pub type UAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UAIE`"]
pub struct UAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UAIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `UPIE`"]
pub type UPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPIE`"]
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TIE0`"]
pub type TIE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE0`"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
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
#[doc = "Reader of field `TIE1`"]
pub type TIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE1`"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
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
    #[doc = "Bit 0 - USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn uee(&self) -> UEE_R {
        UEE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn see(&self) -> SEE_R {
        SEE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn aae(&self) -> AAE_R {
        AAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ure(&self) -> URE_R {
        URE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn sle(&self) -> SLE_R {
        SLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ulpie(&self) -> ULPIE_R {
        ULPIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn nake(&self) -> NAKE_R {
        NAKE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub fn uaie(&self) -> UAIE_R {
        UAIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn uee(&mut self) -> UEE_W {
        UEE_W { w: self }
    }
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn fre(&mut self) -> FRE_W {
        FRE_W { w: self }
    }
    #[doc = "Bit 4 - System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn see(&mut self) -> SEE_W {
        SEE_W { w: self }
    }
    #[doc = "Bit 5 - Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn aae(&mut self) -> AAE_W {
        AAE_W { w: self }
    }
    #[doc = "Bit 6 - USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ure(&mut self) -> URE_W {
        URE_W { w: self }
    }
    #[doc = "Bit 7 - SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn sre(&mut self) -> SRE_W {
        SRE_W { w: self }
    }
    #[doc = "Bit 8 - Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn sle(&mut self) -> SLE_W {
        SLE_W { w: self }
    }
    #[doc = "Bit 10 - ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn ulpie(&mut self) -> ULPIE_W {
        ULPIE_W { w: self }
    }
    #[doc = "Bit 16 - NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn nake(&mut self) -> NAKE_W {
        NAKE_W { w: self }
    }
    #[doc = "Bit 18 - USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub fn uaie(&mut self) -> UAIE_W {
        UAIE_W { w: self }
    }
    #[doc = "Bit 19 - USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
    #[doc = "Bit 24 - General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 25 - General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
}
