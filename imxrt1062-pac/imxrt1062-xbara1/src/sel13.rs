#[doc = "Reader of register SEL13"]
pub type R = crate::R<u16, super::SEL13>;
#[doc = "Writer for register SEL13"]
pub type W = crate::W<u16, super::SEL13>;
#[doc = "Register SEL13 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL13 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL26`"]
pub type SEL26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL26`"]
pub struct SEL26_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL27`"]
pub type SEL27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL27`"]
pub struct SEL27_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel26(&self) -> SEL26_R {
        SEL26_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel27(&self) -> SEL27_R {
        SEL27_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel26(&mut self) -> SEL26_W {
        SEL26_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel27(&mut self) -> SEL27_W {
        SEL27_W { w: self }
    }
}
