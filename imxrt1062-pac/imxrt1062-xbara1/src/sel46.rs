#[doc = "Reader of register SEL46"]
pub type R = crate::R<u16, super::SEL46>;
#[doc = "Writer for register SEL46"]
pub type W = crate::W<u16, super::SEL46>;
#[doc = "Register SEL46 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL46 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL92`"]
pub type SEL92_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL92`"]
pub struct SEL92_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL92_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL93`"]
pub type SEL93_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL93`"]
pub struct SEL93_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL93_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel92(&self) -> SEL92_R {
        SEL92_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel93(&self) -> SEL93_R {
        SEL93_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel92(&mut self) -> SEL92_W {
        SEL92_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel93(&mut self) -> SEL93_W {
        SEL93_W { w: self }
    }
}
