#[doc = "Reader of register TSEM"]
pub type R = crate::R<u32, super::TSEM>;
#[doc = "Writer for register TSEM"]
pub type W = crate::W<u32, super::TSEM>;
#[doc = "Register TSEM `reset()`'s with value 0"]
impl crate::ResetValue for super::TSEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_SECTION_EMPTY`"]
pub type TX_SECTION_EMPTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_SECTION_EMPTY`"]
pub struct TX_SECTION_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SECTION_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn tx_section_empty(&self) -> TX_SECTION_EMPTY_R {
        TX_SECTION_EMPTY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn tx_section_empty(&mut self) -> TX_SECTION_EMPTY_W {
        TX_SECTION_EMPTY_W { w: self }
    }
}
