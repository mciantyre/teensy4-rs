#[doc = "Reader of register SEL39"]
pub type R = crate::R<u16, super::SEL39>;
#[doc = "Writer for register SEL39"]
pub type W = crate::W<u16, super::SEL39>;
#[doc = "Register SEL39 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL39 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL78`"]
pub type SEL78_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL78`"]
pub struct SEL78_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL78_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL79`"]
pub type SEL79_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL79`"]
pub struct SEL79_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL79_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel78(&self) -> SEL78_R {
        SEL78_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel79(&self) -> SEL79_R {
        SEL79_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel78(&mut self) -> SEL78_W {
        SEL78_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel79(&mut self) -> SEL79_W {
        SEL79_W { w: self }
    }
}
