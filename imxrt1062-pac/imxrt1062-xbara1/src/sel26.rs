#[doc = "Reader of register SEL26"]
pub type R = crate::R<u16, super::SEL26>;
#[doc = "Writer for register SEL26"]
pub type W = crate::W<u16, super::SEL26>;
#[doc = "Register SEL26 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL26 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL52`"]
pub type SEL52_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL52`"]
pub struct SEL52_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL53`"]
pub type SEL53_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL53`"]
pub struct SEL53_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT52 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel52(&self) -> SEL52_R {
        SEL52_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT53 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel53(&self) -> SEL53_R {
        SEL53_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT52 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel52(&mut self) -> SEL52_W {
        SEL52_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT53 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel53(&mut self) -> SEL53_W {
        SEL53_W { w: self }
    }
}
