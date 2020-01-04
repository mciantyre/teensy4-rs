#[doc = "Reader of register CSIRXCNT"]
pub type R = crate::R<u32, super::CSIRXCNT>;
#[doc = "Writer for register CSIRXCNT"]
pub type W = crate::W<u32, super::CSIRXCNT>;
#[doc = "Register CSIRXCNT `reset()`'s with value 0x9600"]
impl crate::ResetValue for super::CSIRXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9600
    }
}
#[doc = "Reader of field `RXCNT`"]
pub type RXCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXCNT`"]
pub struct RXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - RxFIFO Count"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - RxFIFO Count"]
    #[inline(always)]
    pub fn rxcnt(&mut self) -> RXCNT_W {
        RXCNT_W { w: self }
    }
}
