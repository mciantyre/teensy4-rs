#[doc = "Reader of register SEL45"]
pub type R = crate::R<u16, super::SEL45>;
#[doc = "Writer for register SEL45"]
pub type W = crate::W<u16, super::SEL45>;
#[doc = "Register SEL45 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL45 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL90`"]
pub type SEL90_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL90`"]
pub struct SEL90_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL90_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL91`"]
pub type SEL91_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL91`"]
pub struct SEL91_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL91_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel90(&self) -> SEL90_R {
        SEL90_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel91(&self) -> SEL91_R {
        SEL91_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel90(&mut self) -> SEL90_W {
        SEL90_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel91(&mut self) -> SEL91_W {
        SEL91_W { w: self }
    }
}
