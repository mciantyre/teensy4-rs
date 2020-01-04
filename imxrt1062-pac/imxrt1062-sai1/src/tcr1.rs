#[doc = "Reader of register TCR1"]
pub type R = crate::R<u32, super::TCR1>;
#[doc = "Writer for register TCR1"]
pub type W = crate::W<u32, super::TCR1>;
#[doc = "Register TCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TFW`"]
pub type TFW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFW`"]
pub struct TFW_W<'a> {
    w: &'a mut W,
}
impl<'a> TFW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Watermark"]
    #[inline(always)]
    pub fn tfw(&self) -> TFW_R {
        TFW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Watermark"]
    #[inline(always)]
    pub fn tfw(&mut self) -> TFW_W {
        TFW_W { w: self }
    }
}
