#[doc = "Reader of register RSFL"]
pub type R = crate::R<u32, super::RSFL>;
#[doc = "Writer for register RSFL"]
pub type W = crate::W<u32, super::RSFL>;
#[doc = "Register RSFL `reset()`'s with value 0"]
impl crate::ResetValue for super::RSFL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_SECTION_FULL`"]
pub type RX_SECTION_FULL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SECTION_FULL`"]
pub struct RX_SECTION_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SECTION_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub fn rx_section_full(&self) -> RX_SECTION_FULL_R {
        RX_SECTION_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub fn rx_section_full(&mut self) -> RX_SECTION_FULL_W {
        RX_SECTION_FULL_W { w: self }
    }
}
