#[doc = "Reader of register SEL41"]
pub type R = crate::R<u16, super::SEL41>;
#[doc = "Writer for register SEL41"]
pub type W = crate::W<u16, super::SEL41>;
#[doc = "Register SEL41 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL41 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL82`"]
pub type SEL82_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL82`"]
pub struct SEL82_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL82_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL83`"]
pub type SEL83_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL83`"]
pub struct SEL83_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL83_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel82(&self) -> SEL82_R {
        SEL82_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel83(&self) -> SEL83_R {
        SEL83_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel82(&mut self) -> SEL82_W {
        SEL82_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel83(&mut self) -> SEL83_W {
        SEL83_W { w: self }
    }
}
