#[doc = "Reader of register GDIR"]
pub type R = crate::R<u32, super::GDIR>;
#[doc = "Writer for register GDIR"]
pub type W = crate::W<u32, super::GDIR>;
#[doc = "Register GDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::GDIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GDIR`"]
pub type GDIR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GDIR`"]
pub struct GDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> GDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GDIR"]
    #[inline(always)]
    pub fn gdir(&self) -> GDIR_R {
        GDIR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GDIR"]
    #[inline(always)]
    pub fn gdir(&mut self) -> GDIR_W {
        GDIR_W { w: self }
    }
}
