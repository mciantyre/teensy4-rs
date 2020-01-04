#[doc = "Reader of register SEL59"]
pub type R = crate::R<u16, super::SEL59>;
#[doc = "Writer for register SEL59"]
pub type W = crate::W<u16, super::SEL59>;
#[doc = "Register SEL59 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL59 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL118`"]
pub type SEL118_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL118`"]
pub struct SEL118_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL118_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL119`"]
pub type SEL119_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL119`"]
pub struct SEL119_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL119_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel118(&self) -> SEL118_R {
        SEL118_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel119(&self) -> SEL119_R {
        SEL119_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel118(&mut self) -> SEL118_W {
        SEL118_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel119(&mut self) -> SEL119_W {
        SEL119_W { w: self }
    }
}
