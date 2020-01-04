#[doc = "Reader of register FRQMIN"]
pub type R = crate::R<u32, super::FRQMIN>;
#[doc = "Writer for register FRQMIN"]
pub type W = crate::W<u32, super::FRQMIN>;
#[doc = "Register FRQMIN `reset()`'s with value 0x0640"]
impl crate::ResetValue for super::FRQMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0640
    }
}
#[doc = "Reader of field `FRQ_MIN`"]
pub type FRQ_MIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRQ_MIN`"]
pub struct FRQ_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&self) -> FRQ_MIN_R {
        FRQ_MIN_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Count Minimum Limit"]
    #[inline(always)]
    pub fn frq_min(&mut self) -> FRQ_MIN_W {
        FRQ_MIN_W { w: self }
    }
}
