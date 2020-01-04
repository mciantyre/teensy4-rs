#[doc = "Reader of register TCD14_DADDR"]
pub type R = crate::R<u32, super::TCD14_DADDR>;
#[doc = "Writer for register TCD14_DADDR"]
pub type W = crate::W<u32, super::TCD14_DADDR>;
#[doc = "Register TCD14_DADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD14_DADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DADDR`"]
pub type DADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DADDR`"]
pub struct DADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Address"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DADDR_W {
        DADDR_W { w: self }
    }
}
