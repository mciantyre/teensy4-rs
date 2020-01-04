#[doc = "Reader of register SEL56"]
pub type R = crate::R<u16, super::SEL56>;
#[doc = "Writer for register SEL56"]
pub type W = crate::W<u16, super::SEL56>;
#[doc = "Register SEL56 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL56 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL112`"]
pub type SEL112_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL112`"]
pub struct SEL112_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL112_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL113`"]
pub type SEL113_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL113`"]
pub struct SEL113_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL113_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel112(&self) -> SEL112_R {
        SEL112_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel113(&self) -> SEL113_R {
        SEL113_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel112(&mut self) -> SEL112_W {
        SEL112_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel113(&mut self) -> SEL113_W {
        SEL113_W { w: self }
    }
}
