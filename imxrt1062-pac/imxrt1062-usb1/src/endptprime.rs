#[doc = "Reader of register ENDPTPRIME"]
pub type R = crate::R<u32, super::ENDPTPRIME>;
#[doc = "Writer for register ENDPTPRIME"]
pub type W = crate::W<u32, super::ENDPTPRIME>;
#[doc = "Register ENDPTPRIME `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTPRIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERB`"]
pub type PERB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERB`"]
pub struct PERB_W<'a> {
    w: &'a mut W,
}
impl<'a> PERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PETB`"]
pub type PETB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PETB`"]
pub struct PETB_W<'a> {
    w: &'a mut W,
}
impl<'a> PETB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Prime Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Prime Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    pub fn petb(&self) -> PETB_R {
        PETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prime Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W {
        PERB_W { w: self }
    }
    #[doc = "Bits 16:23 - Prime Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    pub fn petb(&mut self) -> PETB_W {
        PETB_W { w: self }
    }
}
