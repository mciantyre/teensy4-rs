#[doc = "Reader of register SEL15"]
pub type R = crate::R<u16, super::SEL15>;
#[doc = "Writer for register SEL15"]
pub type W = crate::W<u16, super::SEL15>;
#[doc = "Register SEL15 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL15 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL30`"]
pub type SEL30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL30`"]
pub struct SEL30_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL31`"]
pub type SEL31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL31`"]
pub struct SEL31_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel30(&self) -> SEL30_R {
        SEL30_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel31(&self) -> SEL31_R {
        SEL31_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel30(&mut self) -> SEL30_W {
        SEL30_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel31(&mut self) -> SEL31_W {
        SEL31_W { w: self }
    }
}
