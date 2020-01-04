#[doc = "Reader of register SEL11"]
pub type R = crate::R<u16, super::SEL11>;
#[doc = "Writer for register SEL11"]
pub type W = crate::W<u16, super::SEL11>;
#[doc = "Register SEL11 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL11 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL22`"]
pub type SEL22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL22`"]
pub struct SEL22_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL23`"]
pub type SEL23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL23`"]
pub struct SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT22 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel22(&self) -> SEL22_R {
        SEL22_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT23 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel23(&self) -> SEL23_R {
        SEL23_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT22 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel22(&mut self) -> SEL22_W {
        SEL22_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT23 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel23(&mut self) -> SEL23_W {
        SEL23_W { w: self }
    }
}
