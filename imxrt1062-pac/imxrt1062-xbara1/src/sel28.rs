#[doc = "Reader of register SEL28"]
pub type R = crate::R<u16, super::SEL28>;
#[doc = "Writer for register SEL28"]
pub type W = crate::W<u16, super::SEL28>;
#[doc = "Register SEL28 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL28 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL56`"]
pub type SEL56_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL56`"]
pub struct SEL56_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL56_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL57`"]
pub type SEL57_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL57`"]
pub struct SEL57_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL57_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel56(&self) -> SEL56_R {
        SEL56_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel57(&self) -> SEL57_R {
        SEL57_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel56(&mut self) -> SEL56_W {
        SEL56_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel57(&mut self) -> SEL57_W {
        SEL57_W { w: self }
    }
}
