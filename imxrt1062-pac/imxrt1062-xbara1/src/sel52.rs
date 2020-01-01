#[doc = "Reader of register SEL52"]
pub type R = crate::R<u16, super::SEL52>;
#[doc = "Writer for register SEL52"]
pub type W = crate::W<u16, super::SEL52>;
#[doc = "Register SEL52 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL52 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL104`"]
pub type SEL104_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL104`"]
pub struct SEL104_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL104_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL105`"]
pub type SEL105_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL105`"]
pub struct SEL105_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL105_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel104(&self) -> SEL104_R {
        SEL104_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel105(&self) -> SEL105_R {
        SEL105_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel104(&mut self) -> SEL104_W {
        SEL104_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel105(&mut self) -> SEL105_W {
        SEL105_W { w: self }
    }
}
