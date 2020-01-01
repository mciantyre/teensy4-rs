#[doc = "Reader of register SEL51"]
pub type R = crate::R<u16, super::SEL51>;
#[doc = "Writer for register SEL51"]
pub type W = crate::W<u16, super::SEL51>;
#[doc = "Register SEL51 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL51 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL102`"]
pub type SEL102_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL102`"]
pub struct SEL102_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL102_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL103`"]
pub type SEL103_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL103`"]
pub struct SEL103_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL103_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel102(&self) -> SEL102_R {
        SEL102_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel103(&self) -> SEL103_R {
        SEL103_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel102(&mut self) -> SEL102_W {
        SEL102_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel103(&mut self) -> SEL103_W {
        SEL103_W { w: self }
    }
}
