#[doc = "Reader of register SEL65"]
pub type R = crate::R<u16, super::SEL65>;
#[doc = "Writer for register SEL65"]
pub type W = crate::W<u16, super::SEL65>;
#[doc = "Register SEL65 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL65 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL130`"]
pub type SEL130_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL130`"]
pub struct SEL130_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL130_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL131`"]
pub type SEL131_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL131`"]
pub struct SEL131_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL131_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel130(&self) -> SEL130_R {
        SEL130_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel131(&self) -> SEL131_R {
        SEL131_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel130(&mut self) -> SEL130_W {
        SEL130_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel131(&mut self) -> SEL131_W {
        SEL131_W { w: self }
    }
}
