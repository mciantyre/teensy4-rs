#[doc = "Reader of register SEL19"]
pub type R = crate::R<u16, super::SEL19>;
#[doc = "Writer for register SEL19"]
pub type W = crate::W<u16, super::SEL19>;
#[doc = "Register SEL19 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL19 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL38`"]
pub type SEL38_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL38`"]
pub struct SEL38_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL39`"]
pub type SEL39_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL39`"]
pub struct SEL39_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel38(&self) -> SEL38_R {
        SEL38_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel39(&self) -> SEL39_R {
        SEL39_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel38(&mut self) -> SEL38_W {
        SEL38_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel39(&mut self) -> SEL39_W {
        SEL39_W { w: self }
    }
}
