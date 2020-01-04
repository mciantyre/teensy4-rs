#[doc = "Reader of register SEL10"]
pub type R = crate::R<u16, super::SEL10>;
#[doc = "Writer for register SEL10"]
pub type W = crate::W<u16, super::SEL10>;
#[doc = "Register SEL10 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL10 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL20`"]
pub type SEL20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL20`"]
pub struct SEL20_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL21`"]
pub type SEL21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL21`"]
pub struct SEL21_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel20(&self) -> SEL20_R {
        SEL20_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel21(&self) -> SEL21_R {
        SEL21_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel20(&mut self) -> SEL20_W {
        SEL20_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel21(&mut self) -> SEL21_W {
        SEL21_W { w: self }
    }
}
