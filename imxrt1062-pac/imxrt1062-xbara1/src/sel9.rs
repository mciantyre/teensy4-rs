#[doc = "Reader of register SEL9"]
pub type R = crate::R<u16, super::SEL9>;
#[doc = "Writer for register SEL9"]
pub type W = crate::W<u16, super::SEL9>;
#[doc = "Register SEL9 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL9 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL18`"]
pub type SEL18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL18`"]
pub struct SEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL19`"]
pub type SEL19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL19`"]
pub struct SEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel18(&self) -> SEL18_R {
        SEL18_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel19(&self) -> SEL19_R {
        SEL19_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel18(&mut self) -> SEL18_W {
        SEL18_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel19(&mut self) -> SEL19_W {
        SEL19_W { w: self }
    }
}
