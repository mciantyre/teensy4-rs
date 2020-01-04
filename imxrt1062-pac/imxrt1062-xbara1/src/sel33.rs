#[doc = "Reader of register SEL33"]
pub type R = crate::R<u16, super::SEL33>;
#[doc = "Writer for register SEL33"]
pub type W = crate::W<u16, super::SEL33>;
#[doc = "Register SEL33 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL33 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL66`"]
pub type SEL66_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL66`"]
pub struct SEL66_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL66_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL67`"]
pub type SEL67_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL67`"]
pub struct SEL67_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL67_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel66(&self) -> SEL66_R {
        SEL66_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel67(&self) -> SEL67_R {
        SEL67_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel66(&mut self) -> SEL66_W {
        SEL66_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel67(&mut self) -> SEL67_W {
        SEL67_W { w: self }
    }
}
