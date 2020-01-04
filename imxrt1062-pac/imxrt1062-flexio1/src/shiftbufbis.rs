#[doc = "Reader of register SHIFTBUFBIS[%s]"]
pub type R = crate::R<u32, super::SHIFTBUFBIS>;
#[doc = "Writer for register SHIFTBUFBIS[%s]"]
pub type W = crate::W<u32, super::SHIFTBUFBIS>;
#[doc = "Register SHIFTBUFBIS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUFBIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUFBIS`"]
pub type SHIFTBUFBIS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUFBIS`"]
pub struct SHIFTBUFBIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbis(&self) -> SHIFTBUFBIS_R {
        SHIFTBUFBIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbis(&mut self) -> SHIFTBUFBIS_W {
        SHIFTBUFBIS_W { w: self }
    }
}
