#[doc = "Reader of register SEL21"]
pub type R = crate::R<u16, super::SEL21>;
#[doc = "Writer for register SEL21"]
pub type W = crate::W<u16, super::SEL21>;
#[doc = "Register SEL21 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL21 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL42`"]
pub type SEL42_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL42`"]
pub struct SEL42_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL43`"]
pub type SEL43_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL43`"]
pub struct SEL43_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel42(&self) -> SEL42_R {
        SEL42_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel43(&self) -> SEL43_R {
        SEL43_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel42(&mut self) -> SEL42_W {
        SEL42_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel43(&mut self) -> SEL43_W {
        SEL43_W { w: self }
    }
}
