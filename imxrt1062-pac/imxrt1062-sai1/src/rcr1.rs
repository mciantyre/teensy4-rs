#[doc = "Reader of register RCR1"]
pub type R = crate::R<u32, super::RCR1>;
#[doc = "Writer for register RCR1"]
pub type W = crate::W<u32, super::RCR1>;
#[doc = "Register RCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFW`"]
pub type RFW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFW`"]
pub struct RFW_W<'a> {
    w: &'a mut W,
}
impl<'a> RFW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rfw(&mut self) -> RFW_W {
        RFW_W { w: self }
    }
}
