#[doc = "Reader of register FLSHA2CR0"]
pub type R = crate::R<u32, super::FLSHA2CR0>;
#[doc = "Writer for register FLSHA2CR0"]
pub type W = crate::W<u32, super::FLSHA2CR0>;
#[doc = "Register FLSHA2CR0 `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::FLSHA2CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `FLSHSZ`"]
pub type FLSHSZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLSHSZ`"]
pub struct FLSHSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSHSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&self) -> FLSHSZ_R {
        FLSHSZ_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - Flash Size in KByte."]
    #[inline(always)]
    pub fn flshsz(&mut self) -> FLSHSZ_W {
        FLSHSZ_W { w: self }
    }
}
