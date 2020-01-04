#[doc = "Reader of register RX"]
pub type R = crate::R<u32, super::RX>;
#[doc = "Writer for register RX"]
pub type W = crate::W<u32, super::RX>;
#[doc = "Register RX `reset()`'s with value 0"]
impl crate::ResetValue for super::RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENVADJ`"]
pub type ENVADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENVADJ`"]
pub struct ENVADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DISCONADJ`"]
pub type DISCONADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DISCONADJ`"]
pub struct DISCONADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u16, u16>;
#[doc = "Reader of field `RXDBYPASS`"]
pub type RXDBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDBYPASS`"]
pub struct RXDBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:21 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 7) & 0x7fff) as u16)
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> ENVADJ_W {
        ENVADJ_W { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DISCONADJ_W {
        DISCONADJ_W { w: self }
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W {
        RXDBYPASS_W { w: self }
    }
}
