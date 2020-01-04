#[doc = "Reader of register SEL2"]
pub type R = crate::R<u16, super::SEL2>;
#[doc = "Writer for register SEL2"]
pub type W = crate::W<u16, super::SEL2>;
#[doc = "Register SEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL4`"]
pub type SEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL4`"]
pub struct SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL5`"]
pub type SEL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL5`"]
pub struct SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL4_W {
        SEL4_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel5(&mut self) -> SEL5_W {
        SEL5_W { w: self }
    }
}
