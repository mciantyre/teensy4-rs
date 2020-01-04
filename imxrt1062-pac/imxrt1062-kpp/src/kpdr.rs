#[doc = "Reader of register KPDR"]
pub type R = crate::R<u16, super::KPDR>;
#[doc = "Writer for register KPDR"]
pub type W = crate::W<u16, super::KPDR>;
#[doc = "Register KPDR `reset()`'s with value 0"]
impl crate::ResetValue for super::KPDR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KRD`"]
pub type KRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KRD`"]
pub struct KRD_W<'a> {
    w: &'a mut W,
}
impl<'a> KRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `KCD`"]
pub type KCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KCD`"]
pub struct KCD_W<'a> {
    w: &'a mut W,
}
impl<'a> KCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Keypad Row Data"]
    #[inline(always)]
    pub fn krd(&self) -> KRD_R {
        KRD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Data"]
    #[inline(always)]
    pub fn kcd(&self) -> KCD_R {
        KCD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Data"]
    #[inline(always)]
    pub fn krd(&mut self) -> KRD_W {
        KRD_W { w: self }
    }
    #[doc = "Bits 8:15 - Keypad Column Data"]
    #[inline(always)]
    pub fn kcd(&mut self) -> KCD_W {
        KCD_W { w: self }
    }
}
