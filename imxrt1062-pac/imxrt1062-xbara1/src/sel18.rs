#[doc = "Reader of register SEL18"]
pub type R = crate::R<u16, super::SEL18>;
#[doc = "Writer for register SEL18"]
pub type W = crate::W<u16, super::SEL18>;
#[doc = "Register SEL18 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL18 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL36`"]
pub type SEL36_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL36`"]
pub struct SEL36_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL37`"]
pub type SEL37_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL37`"]
pub struct SEL37_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel36(&self) -> SEL36_R {
        SEL36_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel37(&self) -> SEL37_R {
        SEL37_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel36(&mut self) -> SEL36_W {
        SEL36_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel37(&mut self) -> SEL37_W {
        SEL37_W { w: self }
    }
}
