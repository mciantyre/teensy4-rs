#[doc = "Reader of register SEL43"]
pub type R = crate::R<u16, super::SEL43>;
#[doc = "Writer for register SEL43"]
pub type W = crate::W<u16, super::SEL43>;
#[doc = "Register SEL43 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL43 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL86`"]
pub type SEL86_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL86`"]
pub struct SEL86_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL86_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL87`"]
pub type SEL87_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL87`"]
pub struct SEL87_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL87_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel86(&self) -> SEL86_R {
        SEL86_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel87(&self) -> SEL87_R {
        SEL87_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel86(&mut self) -> SEL86_W {
        SEL86_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel87(&mut self) -> SEL87_W {
        SEL87_W { w: self }
    }
}
