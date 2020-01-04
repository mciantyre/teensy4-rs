#[doc = "Reader of register ENDPTFLUSH"]
pub type R = crate::R<u32, super::ENDPTFLUSH>;
#[doc = "Writer for register ENDPTFLUSH"]
pub type W = crate::W<u32, super::ENDPTFLUSH>;
#[doc = "Register ENDPTFLUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTFLUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FERB`"]
pub type FERB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FERB`"]
pub struct FERB_W<'a> {
    w: &'a mut W,
}
impl<'a> FERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FETB`"]
pub type FETB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FETB`"]
pub struct FETB_W<'a> {
    w: &'a mut W,
}
impl<'a> FETB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Flush Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub fn ferb(&self) -> FERB_R {
        FERB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Flush Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    pub fn fetb(&self) -> FETB_R {
        FETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flush Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub fn ferb(&mut self) -> FERB_W {
        FERB_W { w: self }
    }
    #[doc = "Bits 16:23 - Flush Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    pub fn fetb(&mut self) -> FETB_W {
        FETB_W { w: self }
    }
}
