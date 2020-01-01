#[doc = "Reader of register SEL6"]
pub type R = crate::R<u16, super::SEL6>;
#[doc = "Writer for register SEL6"]
pub type W = crate::W<u16, super::SEL6>;
#[doc = "Register SEL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL6 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL12`"]
pub type SEL12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL12`"]
pub struct SEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SEL13`"]
pub type SEL13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL13`"]
pub struct SEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel12(&mut self) -> SEL12_W {
        SEL12_W { w: self }
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel13(&mut self) -> SEL13_W {
        SEL13_W { w: self }
    }
}
