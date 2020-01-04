#[doc = "Reader of register SEL4"]
pub type R = crate::R<u16, super::SEL4>;
#[doc = "Writer for register SEL4"]
pub type W = crate::W<u16, super::SEL4>;
#[doc = "Register SEL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL8`"]
pub type SEL8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL8`"]
pub struct SEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL9`"]
pub type SEL9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL9`"]
pub struct SEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W {
        SEL8_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel9(&mut self) -> SEL9_W {
        SEL9_W { w: self }
    }
}
