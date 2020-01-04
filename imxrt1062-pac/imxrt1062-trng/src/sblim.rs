#[doc = "Reader of register SBLIM"]
pub type R = crate::R<u32, super::SBLIM>;
#[doc = "Writer for register SBLIM"]
pub type W = crate::W<u32, super::SBLIM>;
#[doc = "Register SBLIM `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::SBLIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `SB_LIM`"]
pub type SB_LIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SB_LIM`"]
pub struct SB_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&self) -> SB_LIM_R {
        SB_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&mut self) -> SB_LIM_W {
        SB_LIM_W { w: self }
    }
}
