#[doc = "Reader of register SEL37"]
pub type R = crate::R<u16, super::SEL37>;
#[doc = "Writer for register SEL37"]
pub type W = crate::W<u16, super::SEL37>;
#[doc = "Register SEL37 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL37 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL74`"]
pub type SEL74_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL74`"]
pub struct SEL74_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL74_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL75`"]
pub type SEL75_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL75`"]
pub struct SEL75_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL75_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT74 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel74(&self) -> SEL74_R {
        SEL74_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT75 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel75(&self) -> SEL75_R {
        SEL75_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT74 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel74(&mut self) -> SEL74_W {
        SEL74_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT75 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel75(&mut self) -> SEL75_W {
        SEL75_W { w: self }
    }
}
