#[doc = "Reader of register LDVAL"]
pub type R = crate::R<u32, super::LDVAL>;
#[doc = "Writer for register LDVAL"]
pub type W = crate::W<u32, super::LDVAL>;
#[doc = "Register LDVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::LDVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSV`"]
pub type TSV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSV`"]
pub struct TSV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Start Value"]
    #[inline(always)]
    pub fn tsv(&self) -> TSV_R {
        TSV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Start Value"]
    #[inline(always)]
    pub fn tsv(&mut self) -> TSV_W {
        TSV_W { w: self }
    }
}
