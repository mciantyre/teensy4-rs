#[doc = "Reader of register SEL62"]
pub type R = crate::R<u16, super::SEL62>;
#[doc = "Writer for register SEL62"]
pub type W = crate::W<u16, super::SEL62>;
#[doc = "Register SEL62 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL62 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL124`"]
pub type SEL124_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL124`"]
pub struct SEL124_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL124_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL125`"]
pub type SEL125_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL125`"]
pub struct SEL125_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL125_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel124(&self) -> SEL124_R {
        SEL124_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel125(&self) -> SEL125_R {
        SEL125_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel124(&mut self) -> SEL124_W {
        SEL124_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel125(&mut self) -> SEL125_W {
        SEL125_W { w: self }
    }
}
