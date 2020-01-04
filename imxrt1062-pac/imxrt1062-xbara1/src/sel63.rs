#[doc = "Reader of register SEL63"]
pub type R = crate::R<u16, super::SEL63>;
#[doc = "Writer for register SEL63"]
pub type W = crate::W<u16, super::SEL63>;
#[doc = "Register SEL63 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL63 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL126`"]
pub type SEL126_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL126`"]
pub struct SEL126_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL126_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL127`"]
pub type SEL127_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL127`"]
pub struct SEL127_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL127_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel126(&self) -> SEL126_R {
        SEL126_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel127(&self) -> SEL127_R {
        SEL127_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel126(&mut self) -> SEL126_W {
        SEL126_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel127(&mut self) -> SEL127_W {
        SEL127_W { w: self }
    }
}
