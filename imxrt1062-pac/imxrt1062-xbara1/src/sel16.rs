#[doc = "Reader of register SEL16"]
pub type R = crate::R<u16, super::SEL16>;
#[doc = "Writer for register SEL16"]
pub type W = crate::W<u16, super::SEL16>;
#[doc = "Register SEL16 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL16 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL32`"]
pub type SEL32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL32`"]
pub struct SEL32_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL33`"]
pub type SEL33_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL33`"]
pub struct SEL33_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel32(&self) -> SEL32_R {
        SEL32_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel33(&self) -> SEL33_R {
        SEL33_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel32(&mut self) -> SEL32_W {
        SEL32_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel33(&mut self) -> SEL33_W {
        SEL33_W { w: self }
    }
}
