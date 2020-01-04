#[doc = "Reader of register SEL64"]
pub type R = crate::R<u16, super::SEL64>;
#[doc = "Writer for register SEL64"]
pub type W = crate::W<u16, super::SEL64>;
#[doc = "Register SEL64 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL64 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL128`"]
pub type SEL128_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL128`"]
pub struct SEL128_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL128_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL129`"]
pub type SEL129_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL129`"]
pub struct SEL129_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL129_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel128(&self) -> SEL128_R {
        SEL128_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel129(&self) -> SEL129_R {
        SEL129_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel128(&mut self) -> SEL128_W {
        SEL128_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel129(&mut self) -> SEL129_W {
        SEL129_W { w: self }
    }
}
