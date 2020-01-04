#[doc = "Reader of register LCOMP"]
pub type R = crate::R<u16, super::LCOMP>;
#[doc = "Writer for register LCOMP"]
pub type W = crate::W<u16, super::LCOMP>;
#[doc = "Register LCOMP `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::LCOMP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register contains the lower (least significant) half of the position compare register"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register contains the lower (least significant) half of the position compare register"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
}
