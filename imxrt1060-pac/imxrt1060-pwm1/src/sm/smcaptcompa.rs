#[doc = "Reader of register SMCAPTCOMPA"]
pub type R = crate::R<u16, super::SMCAPTCOMPA>;
#[doc = "Writer for register SMCAPTCOMPA"]
pub type W = crate::W<u16, super::SMCAPTCOMPA>;
#[doc = "Register SMCAPTCOMPA `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCAPTCOMPA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDGCMPA`"]
pub type EDGCMPA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGCMPA`"]
pub struct EDGCMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EDGCNTA`"]
pub type EDGCNTA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare A"]
    #[inline(always)]
    pub fn edgcmpa(&self) -> EDGCMPA_R {
        EDGCMPA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter A"]
    #[inline(always)]
    pub fn edgcnta(&self) -> EDGCNTA_R {
        EDGCNTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare A"]
    #[inline(always)]
    pub fn edgcmpa(&mut self) -> EDGCMPA_W {
        EDGCMPA_W { w: self }
    }
}
