#[doc = "Reader of register ATPER"]
pub type R = crate::R<u32, super::ATPER>;
#[doc = "Writer for register ATPER"]
pub type W = crate::W<u32, super::ATPER>;
#[doc = "Register ATPER `reset()`'s with value 0x3b9a_ca00"]
impl crate::ResetValue for super::ATPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3b9a_ca00
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value for generating periodic events"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value for generating periodic events"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}
