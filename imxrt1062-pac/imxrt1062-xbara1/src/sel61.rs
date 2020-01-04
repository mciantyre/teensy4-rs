#[doc = "Reader of register SEL61"]
pub type R = crate::R<u16, super::SEL61>;
#[doc = "Writer for register SEL61"]
pub type W = crate::W<u16, super::SEL61>;
#[doc = "Register SEL61 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL61 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL122`"]
pub type SEL122_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL122`"]
pub struct SEL122_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL122_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL123`"]
pub type SEL123_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL123`"]
pub struct SEL123_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL123_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel122(&self) -> SEL122_R {
        SEL122_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel123(&self) -> SEL123_R {
        SEL123_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel122(&mut self) -> SEL122_W {
        SEL122_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel123(&mut self) -> SEL123_W {
        SEL123_W { w: self }
    }
}
