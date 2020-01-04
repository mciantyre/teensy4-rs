#[doc = "Reader of register SEL3"]
pub type R = crate::R<u16, super::SEL3>;
#[doc = "Writer for register SEL3"]
pub type W = crate::W<u16, super::SEL3>;
#[doc = "Register SEL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL6`"]
pub type SEL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL6`"]
pub struct SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SEL7`"]
pub type SEL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL7`"]
pub struct SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel6(&mut self) -> SEL6_W {
        SEL6_W { w: self }
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel7(&mut self) -> SEL7_W {
        SEL7_W { w: self }
    }
}
