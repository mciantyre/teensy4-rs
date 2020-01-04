#[doc = "Reader of register SEL29"]
pub type R = crate::R<u16, super::SEL29>;
#[doc = "Writer for register SEL29"]
pub type W = crate::W<u16, super::SEL29>;
#[doc = "Register SEL29 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL29 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL58`"]
pub type SEL58_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL58`"]
pub struct SEL58_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL58_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL59`"]
pub type SEL59_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL59`"]
pub struct SEL59_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL59_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel58(&self) -> SEL58_R {
        SEL58_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel59(&self) -> SEL59_R {
        SEL59_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel58(&mut self) -> SEL58_W {
        SEL58_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel59(&mut self) -> SEL59_W {
        SEL59_W { w: self }
    }
}
