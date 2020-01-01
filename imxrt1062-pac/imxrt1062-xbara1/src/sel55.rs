#[doc = "Reader of register SEL55"]
pub type R = crate::R<u16, super::SEL55>;
#[doc = "Writer for register SEL55"]
pub type W = crate::W<u16, super::SEL55>;
#[doc = "Register SEL55 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL55 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL110`"]
pub type SEL110_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL110`"]
pub struct SEL110_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL110_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL111`"]
pub type SEL111_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL111`"]
pub struct SEL111_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL111_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel110(&self) -> SEL110_R {
        SEL110_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel111(&self) -> SEL111_R {
        SEL111_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel110(&mut self) -> SEL110_W {
        SEL110_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel111(&mut self) -> SEL111_W {
        SEL111_W { w: self }
    }
}
