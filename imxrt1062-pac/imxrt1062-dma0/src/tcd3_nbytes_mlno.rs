#[doc = "Reader of register TCD3_NBYTES_MLNO"]
pub type R = crate::R<u32, super::TCD3_NBYTES_MLNO>;
#[doc = "Writer for register TCD3_NBYTES_MLNO"]
pub type W = crate::W<u32, super::TCD3_NBYTES_MLNO>;
#[doc = "Register TCD3_NBYTES_MLNO `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD3_NBYTES_MLNO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBYTES`"]
pub type NBYTES_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NBYTES`"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
}
