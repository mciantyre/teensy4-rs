#[doc = "Reader of register ATCOR"]
pub type R = crate::R<u32, super::ATCOR>;
#[doc = "Writer for register ATCOR"]
pub type W = crate::W<u32, super::ATCOR>;
#[doc = "Register ATCOR `reset()`'s with value 0"]
impl crate::ResetValue for super::ATCOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COR`"]
pub type COR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COR`"]
pub struct COR_W<'a> {
    w: &'a mut W,
}
impl<'a> COR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub fn cor(&self) -> COR_R {
        COR_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub fn cor(&mut self) -> COR_W {
        COR_W { w: self }
    }
}
