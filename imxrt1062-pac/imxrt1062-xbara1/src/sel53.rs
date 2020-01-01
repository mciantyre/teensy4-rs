#[doc = "Reader of register SEL53"]
pub type R = crate::R<u16, super::SEL53>;
#[doc = "Writer for register SEL53"]
pub type W = crate::W<u16, super::SEL53>;
#[doc = "Register SEL53 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL53 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL106`"]
pub type SEL106_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL106`"]
pub struct SEL106_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL106_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL107`"]
pub type SEL107_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL107`"]
pub struct SEL107_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL107_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel106(&self) -> SEL106_R {
        SEL106_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel107(&self) -> SEL107_R {
        SEL107_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel106(&mut self) -> SEL106_W {
        SEL106_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel107(&mut self) -> SEL107_W {
        SEL107_W { w: self }
    }
}
