#[doc = "Reader of register DEBUG_CLR"]
pub type R = crate::R<u32, super::DEBUG_CLR>;
#[doc = "Writer for register DEBUG_CLR"]
pub type W = crate::W<u32, super::DEBUG_CLR>;
#[doc = "Register DEBUG_CLR `reset()`'s with value 0x7f18_0000"]
impl crate::ResetValue for super::DEBUG_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f18_0000
    }
}
#[doc = "Reader of field `OTGIDPIOLOCK`"]
pub type OTGIDPIOLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGIDPIOLOCK`"]
pub struct OTGIDPIOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGIDPIOLOCK_W<'a> {
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
#[doc = "Reader of field `DEBUG_INTERFACE_HOLD`"]
pub type DEBUG_INTERFACE_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUG_INTERFACE_HOLD`"]
pub struct DEBUG_INTERFACE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_INTERFACE_HOLD_W<'a> {
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
#[doc = "Reader of field `HSTPULLDOWN`"]
pub type HSTPULLDOWN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTPULLDOWN`"]
pub struct HSTPULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTPULLDOWN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ENHSTPULLDOWN`"]
pub type ENHSTPULLDOWN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENHSTPULLDOWN`"]
pub struct ENHSTPULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHSTPULLDOWN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX2RXCOUNT`"]
pub type TX2RXCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX2RXCOUNT`"]
pub struct TX2RXCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX2RXCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENTX2RXCOUNT`"]
pub type ENTX2RXCOUNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTX2RXCOUNT`"]
pub struct ENTX2RXCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTX2RXCOUNT_W<'a> {
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
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SQUELCHRESETCOUNT`"]
pub type SQUELCHRESETCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQUELCHRESETCOUNT`"]
pub struct SQUELCHRESETCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SQUELCHRESETCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `ENSQUELCHRESET`"]
pub type ENSQUELCHRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSQUELCHRESET`"]
pub struct ENSQUELCHRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSQUELCHRESET_W<'a> {
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
#[doc = "Reader of field `SQUELCHRESETLENGTH`"]
pub type SQUELCHRESETLENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQUELCHRESETLENGTH`"]
pub struct SQUELCHRESETLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SQUELCHRESETLENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "Reader of field `HOST_RESUME_DEBUG`"]
pub type HOST_RESUME_DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_RESUME_DEBUG`"]
pub struct HOST_RESUME_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_RESUME_DEBUG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RSVD3`"]
pub type RSVD3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub fn otgidpiolock(&self) -> OTGIDPIOLOCK_R {
        OTGIDPIOLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub fn debug_interface_hold(&self) -> DEBUG_INTERFACE_HOLD_R {
        DEBUG_INTERFACE_HOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub fn hstpulldown(&self) -> HSTPULLDOWN_R {
        HSTPULLDOWN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub fn enhstpulldown(&self) -> ENHSTPULLDOWN_R {
        ENHSTPULLDOWN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub fn tx2rxcount(&self) -> TX2RXCOUNT_R {
        TX2RXCOUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub fn entx2rxcount(&self) -> ENTX2RXCOUNT_R {
        ENTX2RXCOUNT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub fn squelchresetcount(&self) -> SQUELCHRESETCOUNT_R {
        SQUELCHRESETCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub fn ensquelchreset(&self) -> ENSQUELCHRESET_R {
        ENSQUELCHRESET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub fn squelchresetlength(&self) -> SQUELCHRESETLENGTH_R {
        SQUELCHRESETLENGTH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub fn host_resume_debug(&self) -> HOST_RESUME_DEBUG_R {
        HOST_RESUME_DEBUG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> RSVD3_R {
        RSVD3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub fn otgidpiolock(&mut self) -> OTGIDPIOLOCK_W {
        OTGIDPIOLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub fn debug_interface_hold(&mut self) -> DEBUG_INTERFACE_HOLD_W {
        DEBUG_INTERFACE_HOLD_W { w: self }
    }
    #[doc = "Bits 2:3 - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub fn hstpulldown(&mut self) -> HSTPULLDOWN_W {
        HSTPULLDOWN_W { w: self }
    }
    #[doc = "Bits 4:5 - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub fn enhstpulldown(&mut self) -> ENHSTPULLDOWN_W {
        ENHSTPULLDOWN_W { w: self }
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub fn tx2rxcount(&mut self) -> TX2RXCOUNT_W {
        TX2RXCOUNT_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub fn entx2rxcount(&mut self) -> ENTX2RXCOUNT_W {
        ENTX2RXCOUNT_W { w: self }
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub fn squelchresetcount(&mut self) -> SQUELCHRESETCOUNT_W {
        SQUELCHRESETCOUNT_W { w: self }
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub fn ensquelchreset(&mut self) -> ENSQUELCHRESET_W {
        ENSQUELCHRESET_W { w: self }
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub fn squelchresetlength(&mut self) -> SQUELCHRESETLENGTH_W {
        SQUELCHRESETLENGTH_W { w: self }
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub fn host_resume_debug(&mut self) -> HOST_RESUME_DEBUG_W {
        HOST_RESUME_DEBUG_W { w: self }
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
}
