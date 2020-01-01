#[doc = "Reader of register SEL32"]
pub type R = crate::R<u16, super::SEL32>;
#[doc = "Writer for register SEL32"]
pub type W = crate::W<u16, super::SEL32>;
#[doc = "Register SEL32 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL32 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL64`"]
pub type SEL64_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL64`"]
pub struct SEL64_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL64_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL65`"]
pub type SEL65_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL65`"]
pub struct SEL65_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL65_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel64(&self) -> SEL64_R {
        SEL64_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel65(&self) -> SEL65_R {
        SEL65_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel64(&mut self) -> SEL64_W {
        SEL64_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel65(&mut self) -> SEL65_W {
        SEL65_W { w: self }
    }
}
