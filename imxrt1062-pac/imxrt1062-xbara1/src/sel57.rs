#[doc = "Reader of register SEL57"]
pub type R = crate::R<u16, super::SEL57>;
#[doc = "Writer for register SEL57"]
pub type W = crate::W<u16, super::SEL57>;
#[doc = "Register SEL57 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL57 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL114`"]
pub type SEL114_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL114`"]
pub struct SEL114_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL114_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL115`"]
pub type SEL115_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL115`"]
pub struct SEL115_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL115_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel114(&self) -> SEL114_R {
        SEL114_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel115(&self) -> SEL115_R {
        SEL115_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel114(&mut self) -> SEL114_W {
        SEL114_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel115(&mut self) -> SEL115_W {
        SEL115_W { w: self }
    }
}
