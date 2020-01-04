#[doc = "Reader of register SEL42"]
pub type R = crate::R<u16, super::SEL42>;
#[doc = "Writer for register SEL42"]
pub type W = crate::W<u16, super::SEL42>;
#[doc = "Register SEL42 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL42 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL84`"]
pub type SEL84_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL84`"]
pub struct SEL84_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL84_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL85`"]
pub type SEL85_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL85`"]
pub struct SEL85_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL85_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel84(&self) -> SEL84_R {
        SEL84_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel85(&self) -> SEL85_R {
        SEL85_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel84(&mut self) -> SEL84_W {
        SEL84_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel85(&mut self) -> SEL85_W {
        SEL85_W { w: self }
    }
}
