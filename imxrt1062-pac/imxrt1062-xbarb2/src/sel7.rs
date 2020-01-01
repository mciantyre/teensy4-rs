#[doc = "Reader of register SEL7"]
pub type R = crate::R<u16, super::SEL7>;
#[doc = "Writer for register SEL7"]
pub type W = crate::W<u16, super::SEL7>;
#[doc = "Register SEL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL7 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL14`"]
pub type SEL14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL14`"]
pub struct SEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SEL15`"]
pub type SEL15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL15`"]
pub struct SEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel14(&mut self) -> SEL14_W {
        SEL14_W { w: self }
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel15(&mut self) -> SEL15_W {
        SEL15_W { w: self }
    }
}
