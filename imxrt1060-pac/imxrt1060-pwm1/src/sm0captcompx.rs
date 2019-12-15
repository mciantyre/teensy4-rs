#[doc = "Reader of register SM0CAPTCOMPX"]
pub type R = crate::R<u16, super::SM0CAPTCOMPX>;
#[doc = "Writer for register SM0CAPTCOMPX"]
pub type W = crate::W<u16, super::SM0CAPTCOMPX>;
#[doc = "Register SM0CAPTCOMPX `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0CAPTCOMPX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDGCMPX`"]
pub type EDGCMPX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGCMPX`"]
pub struct EDGCMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCMPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EDGCNTX`"]
pub type EDGCNTX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare X"]
    #[inline(always)]
    pub fn edgcmpx(&self) -> EDGCMPX_R {
        EDGCMPX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter X"]
    #[inline(always)]
    pub fn edgcntx(&self) -> EDGCNTX_R {
        EDGCNTX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare X"]
    #[inline(always)]
    pub fn edgcmpx(&mut self) -> EDGCMPX_W {
        EDGCMPX_W { w: self }
    }
}
