#[doc = "Reader of register SEL40"]
pub type R = crate::R<u16, super::SEL40>;
#[doc = "Writer for register SEL40"]
pub type W = crate::W<u16, super::SEL40>;
#[doc = "Register SEL40 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL40 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL80`"]
pub type SEL80_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL80`"]
pub struct SEL80_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL80_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL81`"]
pub type SEL81_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL81`"]
pub struct SEL81_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL81_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT80 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel80(&self) -> SEL80_R {
        SEL80_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT81 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel81(&self) -> SEL81_R {
        SEL81_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT80 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel80(&mut self) -> SEL80_W {
        SEL80_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT81 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel81(&mut self) -> SEL81_W {
        SEL81_W { w: self }
    }
}
