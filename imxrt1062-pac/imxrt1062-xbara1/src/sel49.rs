#[doc = "Reader of register SEL49"]
pub type R = crate::R<u16, super::SEL49>;
#[doc = "Writer for register SEL49"]
pub type W = crate::W<u16, super::SEL49>;
#[doc = "Register SEL49 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL49 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL98`"]
pub type SEL98_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL98`"]
pub struct SEL98_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL98_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL99`"]
pub type SEL99_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL99`"]
pub struct SEL99_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL99_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel98(&self) -> SEL98_R {
        SEL98_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel99(&self) -> SEL99_R {
        SEL99_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel98(&mut self) -> SEL98_W {
        SEL98_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel99(&mut self) -> SEL99_W {
        SEL99_W { w: self }
    }
}
