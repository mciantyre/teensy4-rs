#[doc = "Reader of register TOVAL"]
pub type R = crate::R<u32, super::TOVAL>;
#[doc = "Writer for register TOVAL"]
pub type W = crate::W<u32, super::TOVAL>;
#[doc = "Register TOVAL `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::TOVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `TOVALLOW`"]
pub type TOVALLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOVALLOW`"]
pub struct TOVALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TOVALHIGH`"]
pub type TOVALHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOVALHIGH`"]
pub struct TOVALHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&mut self) -> TOVALLOW_W {
        TOVALLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W {
        TOVALHIGH_W { w: self }
    }
}
