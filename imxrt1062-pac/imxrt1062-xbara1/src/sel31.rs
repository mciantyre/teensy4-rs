#[doc = "Reader of register SEL31"]
pub type R = crate::R<u16, super::SEL31>;
#[doc = "Writer for register SEL31"]
pub type W = crate::W<u16, super::SEL31>;
#[doc = "Register SEL31 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL31 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL62`"]
pub type SEL62_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL62`"]
pub struct SEL62_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL62_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL63`"]
pub type SEL63_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL63`"]
pub struct SEL63_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel62(&self) -> SEL62_R {
        SEL62_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel63(&self) -> SEL63_R {
        SEL63_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel62(&mut self) -> SEL62_W {
        SEL62_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel63(&mut self) -> SEL63_W {
        SEL63_W { w: self }
    }
}
