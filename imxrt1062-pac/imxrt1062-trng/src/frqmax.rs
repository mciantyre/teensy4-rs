#[doc = "Reader of register FRQMAX"]
pub type R = crate::R<u32, super::FRQMAX>;
#[doc = "Writer for register FRQMAX"]
pub type W = crate::W<u32, super::FRQMAX>;
#[doc = "Register FRQMAX `reset()`'s with value 0x6400"]
impl crate::ResetValue for super::FRQMAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6400
    }
}
#[doc = "Reader of field `FRQ_MAX`"]
pub type FRQ_MAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRQ_MAX`"]
pub struct FRQ_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&self) -> FRQ_MAX_R {
        FRQ_MAX_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&mut self) -> FRQ_MAX_W {
        FRQ_MAX_W { w: self }
    }
}
