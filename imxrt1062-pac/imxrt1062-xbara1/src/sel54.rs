#[doc = "Reader of register SEL54"]
pub type R = crate::R<u16, super::SEL54>;
#[doc = "Writer for register SEL54"]
pub type W = crate::W<u16, super::SEL54>;
#[doc = "Register SEL54 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL54 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL108`"]
pub type SEL108_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL108`"]
pub struct SEL108_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL108_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL109`"]
pub type SEL109_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL109`"]
pub struct SEL109_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL109_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel108(&self) -> SEL108_R {
        SEL108_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel109(&self) -> SEL109_R {
        SEL109_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel108(&mut self) -> SEL108_W {
        SEL108_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel109(&mut self) -> SEL109_W {
        SEL109_W { w: self }
    }
}
