#[doc = "Reader of register SEL8"]
pub type R = crate::R<u16, super::SEL8>;
#[doc = "Writer for register SEL8"]
pub type W = crate::W<u16, super::SEL8>;
#[doc = "Register SEL8 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL8 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL16`"]
pub type SEL16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL16`"]
pub struct SEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL17`"]
pub type SEL17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL17`"]
pub struct SEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel16(&self) -> SEL16_R {
        SEL16_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel17(&self) -> SEL17_R {
        SEL17_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel16(&mut self) -> SEL16_W {
        SEL16_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel17(&mut self) -> SEL17_W {
        SEL17_W { w: self }
    }
}
