#[doc = "Reader of register TCD12_DLASTSGA"]
pub type R = crate::R<u32, super::TCD12_DLASTSGA>;
#[doc = "Writer for register TCD12_DLASTSGA"]
pub type W = crate::W<u32, super::TCD12_DLASTSGA>;
#[doc = "Register TCD12_DLASTSGA `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD12_DLASTSGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLASTSGA`"]
pub type DLASTSGA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DLASTSGA`"]
pub struct DLASTSGA_W<'a> {
    w: &'a mut W,
}
impl<'a> DLASTSGA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DLASTSGA"]
    #[inline(always)]
    pub fn dlastsga(&self) -> DLASTSGA_R {
        DLASTSGA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DLASTSGA"]
    #[inline(always)]
    pub fn dlastsga(&mut self) -> DLASTSGA_W {
        DLASTSGA_W { w: self }
    }
}
