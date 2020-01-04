#[doc = "Reader of register ENDPTCTRL0"]
pub type R = crate::R<u32, super::ENDPTCTRL0>;
#[doc = "Writer for register ENDPTCTRL0"]
pub type W = crate::W<u32, super::ENDPTCTRL0>;
#[doc = "Register ENDPTCTRL0 `reset()`'s with value 0x0080_0080"]
impl crate::ResetValue for super::ENDPTCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0080
    }
}
#[doc = "Reader of field `RXS`"]
pub type RXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXS`"]
pub struct RXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXS_W<'a> {
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
#[doc = "Reader of field `RXT`"]
pub type RXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXT`"]
pub struct RXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
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
#[doc = "Reader of field `TXS`"]
pub type TXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXS`"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
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
#[doc = "Reader of field `TXT`"]
pub type TXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXT`"]
pub struct TXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W { w: self }
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn rxt(&mut self) -> RXT_W {
        RXT_W { w: self }
    }
    #[doc = "Bit 7 - RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\]
1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub fn txt(&mut self) -> TXT_W {
        TXT_W { w: self }
    }
    #[doc = "Bit 23 - TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
}
