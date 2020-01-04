#[doc = "Reader of register SEL27"]
pub type R = crate::R<u16, super::SEL27>;
#[doc = "Writer for register SEL27"]
pub type W = crate::W<u16, super::SEL27>;
#[doc = "Register SEL27 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL27 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL54`"]
pub type SEL54_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL54`"]
pub struct SEL54_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL55`"]
pub type SEL55_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL55`"]
pub struct SEL55_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel54(&self) -> SEL54_R {
        SEL54_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel55(&self) -> SEL55_R {
        SEL55_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel54(&mut self) -> SEL54_W {
        SEL54_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel55(&mut self) -> SEL55_W {
        SEL55_W { w: self }
    }
}
