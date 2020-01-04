#[doc = "Reader of register SHIFTBUFNBS[%s]"]
pub type R = crate::R<u32, super::SHIFTBUFNBS>;
#[doc = "Writer for register SHIFTBUFNBS[%s]"]
pub type W = crate::W<u32, super::SHIFTBUFNBS>;
#[doc = "Register SHIFTBUFNBS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUFNBS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUFNBS`"]
pub type SHIFTBUFNBS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUFNBS`"]
pub struct SHIFTBUFNBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFNBS_W<'a> {
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
    pub fn shiftbufnbs(&self) -> SHIFTBUFNBS_R {
        SHIFTBUFNBS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufnbs(&mut self) -> SHIFTBUFNBS_W {
        SHIFTBUFNBS_W { w: self }
    }
}
