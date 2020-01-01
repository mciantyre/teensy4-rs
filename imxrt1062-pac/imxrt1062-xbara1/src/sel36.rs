#[doc = "Reader of register SEL36"]
pub type R = crate::R<u16, super::SEL36>;
#[doc = "Writer for register SEL36"]
pub type W = crate::W<u16, super::SEL36>;
#[doc = "Register SEL36 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL36 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL72`"]
pub type SEL72_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL72`"]
pub struct SEL72_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL72_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL73`"]
pub type SEL73_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL73`"]
pub struct SEL73_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL73_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel72(&self) -> SEL72_R {
        SEL72_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel73(&self) -> SEL73_R {
        SEL73_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel72(&mut self) -> SEL72_W {
        SEL72_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel73(&mut self) -> SEL73_W {
        SEL73_W { w: self }
    }
}
