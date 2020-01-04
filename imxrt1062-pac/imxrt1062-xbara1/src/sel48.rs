#[doc = "Reader of register SEL48"]
pub type R = crate::R<u16, super::SEL48>;
#[doc = "Writer for register SEL48"]
pub type W = crate::W<u16, super::SEL48>;
#[doc = "Register SEL48 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL48 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL96`"]
pub type SEL96_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL96`"]
pub struct SEL96_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL96_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL97`"]
pub type SEL97_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL97`"]
pub struct SEL97_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL97_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel96(&self) -> SEL96_R {
        SEL96_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel97(&self) -> SEL97_R {
        SEL97_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel96(&mut self) -> SEL96_W {
        SEL96_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel97(&mut self) -> SEL97_W {
        SEL97_W { w: self }
    }
}
