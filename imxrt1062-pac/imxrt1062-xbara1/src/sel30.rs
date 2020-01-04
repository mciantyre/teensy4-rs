#[doc = "Reader of register SEL30"]
pub type R = crate::R<u16, super::SEL30>;
#[doc = "Writer for register SEL30"]
pub type W = crate::W<u16, super::SEL30>;
#[doc = "Register SEL30 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL30 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL60`"]
pub type SEL60_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL60`"]
pub struct SEL60_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL60_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL61`"]
pub type SEL61_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL61`"]
pub struct SEL61_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL61_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel60(&self) -> SEL60_R {
        SEL60_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel61(&self) -> SEL61_R {
        SEL61_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel60(&mut self) -> SEL60_W {
        SEL60_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel61(&mut self) -> SEL61_W {
        SEL61_W { w: self }
    }
}
