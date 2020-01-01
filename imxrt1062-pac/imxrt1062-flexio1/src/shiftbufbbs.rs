#[doc = "Reader of register SHIFTBUFBBS[%s]"]
pub type R = crate::R<u32, super::SHIFTBUFBBS>;
#[doc = "Writer for register SHIFTBUFBBS[%s]"]
pub type W = crate::W<u32, super::SHIFTBUFBBS>;
#[doc = "Register SHIFTBUFBBS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUFBBS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUFBBS`"]
pub type SHIFTBUFBBS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUFBBS`"]
pub struct SHIFTBUFBBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBBS_W<'a> {
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
    pub fn shiftbufbbs(&self) -> SHIFTBUFBBS_R {
        SHIFTBUFBBS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbbs(&mut self) -> SHIFTBUFBBS_W {
        SHIFTBUFBBS_W { w: self }
    }
}
