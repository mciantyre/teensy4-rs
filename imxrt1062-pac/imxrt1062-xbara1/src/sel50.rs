#[doc = "Reader of register SEL50"]
pub type R = crate::R<u16, super::SEL50>;
#[doc = "Writer for register SEL50"]
pub type W = crate::W<u16, super::SEL50>;
#[doc = "Register SEL50 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL50 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL100`"]
pub type SEL100_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL100`"]
pub struct SEL100_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL101`"]
pub type SEL101_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL101`"]
pub struct SEL101_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL101_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel100(&self) -> SEL100_R {
        SEL100_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel101(&self) -> SEL101_R {
        SEL101_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel100(&mut self) -> SEL100_W {
        SEL100_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel101(&mut self) -> SEL101_W {
        SEL101_W { w: self }
    }
}
