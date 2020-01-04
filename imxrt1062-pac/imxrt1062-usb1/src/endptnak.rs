#[doc = "Reader of register ENDPTNAK"]
pub type R = crate::R<u32, super::ENDPTNAK>;
#[doc = "Writer for register ENDPTNAK"]
pub type W = crate::W<u32, super::ENDPTNAK>;
#[doc = "Register ENDPTNAK `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTNAK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPRN`"]
pub type EPRN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPRN`"]
pub struct EPRN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EPTN`"]
pub type EPTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPTN`"]
pub struct EPTN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RX Endpoint NAK - R/WC"]
    #[inline(always)]
    pub fn eprn(&self) -> EPRN_R {
        EPRN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX Endpoint NAK - R/WC"]
    #[inline(always)]
    pub fn eptn(&self) -> EPTN_R {
        EPTN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX Endpoint NAK - R/WC"]
    #[inline(always)]
    pub fn eprn(&mut self) -> EPRN_W {
        EPRN_W { w: self }
    }
    #[doc = "Bits 16:23 - TX Endpoint NAK - R/WC"]
    #[inline(always)]
    pub fn eptn(&mut self) -> EPTN_W {
        EPTN_W { w: self }
    }
}
