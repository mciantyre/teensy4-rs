#[doc = "Reader of register SEL47"]
pub type R = crate::R<u16, super::SEL47>;
#[doc = "Writer for register SEL47"]
pub type W = crate::W<u16, super::SEL47>;
#[doc = "Register SEL47 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL47 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL94`"]
pub type SEL94_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL94`"]
pub struct SEL94_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL94_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL95`"]
pub type SEL95_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL95`"]
pub struct SEL95_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL95_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel94(&self) -> SEL94_R {
        SEL94_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel95(&self) -> SEL95_R {
        SEL95_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel94(&mut self) -> SEL94_W {
        SEL94_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel95(&mut self) -> SEL95_W {
        SEL95_W { w: self }
    }
}
