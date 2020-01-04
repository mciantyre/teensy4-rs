#[doc = "Reader of register SEL38"]
pub type R = crate::R<u16, super::SEL38>;
#[doc = "Writer for register SEL38"]
pub type W = crate::W<u16, super::SEL38>;
#[doc = "Register SEL38 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL38 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL76`"]
pub type SEL76_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL76`"]
pub struct SEL76_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL76_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL77`"]
pub type SEL77_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL77`"]
pub struct SEL77_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL77_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT76 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel76(&self) -> SEL76_R {
        SEL76_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT77 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel77(&self) -> SEL77_R {
        SEL77_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT76 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel76(&mut self) -> SEL76_W {
        SEL76_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT77 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel77(&mut self) -> SEL77_W {
        SEL77_W { w: self }
    }
}
