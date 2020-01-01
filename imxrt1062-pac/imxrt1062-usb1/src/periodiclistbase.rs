#[doc = "Reader of register PERIODICLISTBASE"]
pub type R = crate::R<u32, super::PERIODICLISTBASE>;
#[doc = "Writer for register PERIODICLISTBASE"]
pub type W = crate::W<u32, super::PERIODICLISTBASE>;
#[doc = "Register PERIODICLISTBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIODICLISTBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASEADR`"]
pub type BASEADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASEADR`"]
pub struct BASEADR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - Base Address (Low)"]
    #[inline(always)]
    pub fn baseadr(&self) -> BASEADR_R {
        BASEADR_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base Address (Low)"]
    #[inline(always)]
    pub fn baseadr(&mut self) -> BASEADR_W {
        BASEADR_W { w: self }
    }
}
