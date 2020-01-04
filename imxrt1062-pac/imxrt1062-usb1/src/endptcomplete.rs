#[doc = "Reader of register ENDPTCOMPLETE"]
pub type R = crate::R<u32, super::ENDPTCOMPLETE>;
#[doc = "Writer for register ENDPTCOMPLETE"]
pub type W = crate::W<u32, super::ENDPTCOMPLETE>;
#[doc = "Register ENDPTCOMPLETE `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTCOMPLETE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERCE`"]
pub type ERCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERCE`"]
pub struct ERCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ETCE`"]
pub type ETCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETCE`"]
pub struct ETCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint Receive Complete Event - RW/C"]
    #[inline(always)]
    pub fn erce(&self) -> ERCE_R {
        ERCE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Endpoint Transmit Complete Event - R/WC"]
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint Receive Complete Event - RW/C"]
    #[inline(always)]
    pub fn erce(&mut self) -> ERCE_W {
        ERCE_W { w: self }
    }
    #[doc = "Bits 16:23 - Endpoint Transmit Complete Event - R/WC"]
    #[inline(always)]
    pub fn etce(&mut self) -> ETCE_W {
        ETCE_W { w: self }
    }
}
