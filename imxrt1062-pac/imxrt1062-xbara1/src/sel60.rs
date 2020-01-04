#[doc = "Reader of register SEL60"]
pub type R = crate::R<u16, super::SEL60>;
#[doc = "Writer for register SEL60"]
pub type W = crate::W<u16, super::SEL60>;
#[doc = "Register SEL60 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL60 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL120`"]
pub type SEL120_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL120`"]
pub struct SEL120_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL120_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL121`"]
pub type SEL121_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL121`"]
pub struct SEL121_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL121_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT120 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel120(&self) -> SEL120_R {
        SEL120_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT121 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel121(&self) -> SEL121_R {
        SEL121_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT120 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel120(&mut self) -> SEL120_W {
        SEL120_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT121 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel121(&mut self) -> SEL121_W {
        SEL121_W { w: self }
    }
}
