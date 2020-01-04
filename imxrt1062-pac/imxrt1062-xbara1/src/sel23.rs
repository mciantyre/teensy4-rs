#[doc = "Reader of register SEL23"]
pub type R = crate::R<u16, super::SEL23>;
#[doc = "Writer for register SEL23"]
pub type W = crate::W<u16, super::SEL23>;
#[doc = "Register SEL23 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL23 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL46`"]
pub type SEL46_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL46`"]
pub struct SEL46_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL47`"]
pub type SEL47_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL47`"]
pub struct SEL47_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel46(&self) -> SEL46_R {
        SEL46_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel47(&self) -> SEL47_R {
        SEL47_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel46(&mut self) -> SEL46_W {
        SEL46_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel47(&mut self) -> SEL47_W {
        SEL47_W { w: self }
    }
}
