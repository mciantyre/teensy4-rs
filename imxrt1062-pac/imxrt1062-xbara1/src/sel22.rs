#[doc = "Reader of register SEL22"]
pub type R = crate::R<u16, super::SEL22>;
#[doc = "Writer for register SEL22"]
pub type W = crate::W<u16, super::SEL22>;
#[doc = "Register SEL22 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL22 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL44`"]
pub type SEL44_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL44`"]
pub struct SEL44_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL44_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL45`"]
pub type SEL45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL45`"]
pub struct SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel44(&self) -> SEL44_R {
        SEL44_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel45(&self) -> SEL45_R {
        SEL45_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel44(&mut self) -> SEL44_W {
        SEL44_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel45(&mut self) -> SEL45_W {
        SEL45_W { w: self }
    }
}
