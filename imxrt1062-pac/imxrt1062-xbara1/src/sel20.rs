#[doc = "Reader of register SEL20"]
pub type R = crate::R<u16, super::SEL20>;
#[doc = "Writer for register SEL20"]
pub type W = crate::W<u16, super::SEL20>;
#[doc = "Register SEL20 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL20 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL40`"]
pub type SEL40_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL40`"]
pub struct SEL40_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL41`"]
pub type SEL41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL41`"]
pub struct SEL41_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel40(&self) -> SEL40_R {
        SEL40_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel41(&self) -> SEL41_R {
        SEL41_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel40(&mut self) -> SEL40_W {
        SEL40_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel41(&mut self) -> SEL41_W {
        SEL41_W { w: self }
    }
}
