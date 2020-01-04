#[doc = "Reader of register RSEM"]
pub type R = crate::R<u32, super::RSEM>;
#[doc = "Writer for register RSEM"]
pub type W = crate::W<u32, super::RSEM>;
#[doc = "Register RSEM `reset()`'s with value 0"]
impl crate::ResetValue for super::RSEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_SECTION_EMPTY`"]
pub type RX_SECTION_EMPTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SECTION_EMPTY`"]
pub struct RX_SECTION_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SECTION_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `STAT_SECTION_EMPTY`"]
pub type STAT_SECTION_EMPTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STAT_SECTION_EMPTY`"]
pub struct STAT_SECTION_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_SECTION_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn rx_section_empty(&self) -> RX_SECTION_EMPTY_R {
        RX_SECTION_EMPTY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn stat_section_empty(&self) -> STAT_SECTION_EMPTY_R {
        STAT_SECTION_EMPTY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn rx_section_empty(&mut self) -> RX_SECTION_EMPTY_W {
        RX_SECTION_EMPTY_W { w: self }
    }
    #[doc = "Bits 16:20 - RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn stat_section_empty(&mut self) -> STAT_SECTION_EMPTY_W {
        STAT_SECTION_EMPTY_W { w: self }
    }
}
