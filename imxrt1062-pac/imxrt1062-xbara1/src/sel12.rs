#[doc = "Reader of register SEL12"]
pub type R = crate::R<u16, super::SEL12>;
#[doc = "Writer for register SEL12"]
pub type W = crate::W<u16, super::SEL12>;
#[doc = "Register SEL12 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL12 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL24`"]
pub type SEL24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL24`"]
pub struct SEL24_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL25`"]
pub type SEL25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL25`"]
pub struct SEL25_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel24(&self) -> SEL24_R {
        SEL24_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel25(&self) -> SEL25_R {
        SEL25_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel24(&mut self) -> SEL24_W {
        SEL24_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel25(&mut self) -> SEL25_W {
        SEL25_W { w: self }
    }
}
