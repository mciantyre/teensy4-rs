#[doc = "Reader of register SEL5"]
pub type R = crate::R<u16, super::SEL5>;
#[doc = "Writer for register SEL5"]
pub type W = crate::W<u16, super::SEL5>;
#[doc = "Register SEL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL10`"]
pub type SEL10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL10`"]
pub struct SEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SEL11`"]
pub type SEL11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL11`"]
pub struct SEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel10(&mut self) -> SEL10_W {
        SEL10_W { w: self }
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel11(&mut self) -> SEL11_W {
        SEL11_W { w: self }
    }
}
