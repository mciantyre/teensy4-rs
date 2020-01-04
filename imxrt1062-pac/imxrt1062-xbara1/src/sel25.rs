#[doc = "Reader of register SEL25"]
pub type R = crate::R<u16, super::SEL25>;
#[doc = "Writer for register SEL25"]
pub type W = crate::W<u16, super::SEL25>;
#[doc = "Register SEL25 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL25 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL50`"]
pub type SEL50_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL50`"]
pub struct SEL50_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL51`"]
pub type SEL51_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL51`"]
pub struct SEL51_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT50 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel50(&self) -> SEL50_R {
        SEL50_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT51 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel51(&self) -> SEL51_R {
        SEL51_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT50 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel50(&mut self) -> SEL50_W {
        SEL50_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT51 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel51(&mut self) -> SEL51_W {
        SEL51_W { w: self }
    }
}
