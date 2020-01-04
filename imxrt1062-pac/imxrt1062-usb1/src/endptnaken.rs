#[doc = "Reader of register ENDPTNAKEN"]
pub type R = crate::R<u32, super::ENDPTNAKEN>;
#[doc = "Writer for register ENDPTNAKEN"]
pub type W = crate::W<u32, super::ENDPTNAKEN>;
#[doc = "Register ENDPTNAKEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTNAKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPRNE`"]
pub type EPRNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPRNE`"]
pub struct EPRNE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EPTNE`"]
pub type EPTNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPTNE`"]
pub struct EPTNE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RX Endpoint NAK Enable - R/W"]
    #[inline(always)]
    pub fn eprne(&self) -> EPRNE_R {
        EPRNE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX Endpoint NAK Enable - R/W"]
    #[inline(always)]
    pub fn eptne(&self) -> EPTNE_R {
        EPTNE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX Endpoint NAK Enable - R/W"]
    #[inline(always)]
    pub fn eprne(&mut self) -> EPRNE_W {
        EPRNE_W { w: self }
    }
    #[doc = "Bits 16:23 - TX Endpoint NAK Enable - R/W"]
    #[inline(always)]
    pub fn eptne(&mut self) -> EPTNE_W {
        EPTNE_W { w: self }
    }
}
