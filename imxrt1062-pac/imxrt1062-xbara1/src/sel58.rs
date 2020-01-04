#[doc = "Reader of register SEL58"]
pub type R = crate::R<u16, super::SEL58>;
#[doc = "Writer for register SEL58"]
pub type W = crate::W<u16, super::SEL58>;
#[doc = "Register SEL58 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL58 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL116`"]
pub type SEL116_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL116`"]
pub struct SEL116_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL116_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL117`"]
pub type SEL117_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL117`"]
pub struct SEL117_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL117_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel116(&self) -> SEL116_R {
        SEL116_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel117(&self) -> SEL117_R {
        SEL117_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel116(&mut self) -> SEL116_W {
        SEL116_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel117(&mut self) -> SEL117_W {
        SEL117_W { w: self }
    }
}
