#[doc = "Reader of register OCR1"]
pub type R = crate::R<u32, super::OCR1>;
#[doc = "Writer for register OCR1"]
pub type W = crate::W<u32, super::OCR1>;
#[doc = "Register OCR1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::OCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
}
