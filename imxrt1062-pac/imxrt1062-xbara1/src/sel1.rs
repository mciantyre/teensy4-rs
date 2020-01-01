#[doc = "Reader of register SEL1"]
pub type R = crate::R<u16, super::SEL1>;
#[doc = "Writer for register SEL1"]
pub type W = crate::W<u16, super::SEL1>;
#[doc = "Register SEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL2`"]
pub type SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL2`"]
pub struct SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL3`"]
pub type SEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL3`"]
pub struct SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel2(&mut self) -> SEL2_W {
        SEL2_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel3(&mut self) -> SEL3_W {
        SEL3_W { w: self }
    }
}
