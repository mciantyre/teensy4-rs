#[doc = "Reader of register SEL24"]
pub type R = crate::R<u16, super::SEL24>;
#[doc = "Writer for register SEL24"]
pub type W = crate::W<u16, super::SEL24>;
#[doc = "Register SEL24 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL24 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL48`"]
pub type SEL48_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL48`"]
pub struct SEL48_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL49`"]
pub type SEL49_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL49`"]
pub struct SEL49_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel48(&self) -> SEL48_R {
        SEL48_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel49(&self) -> SEL49_R {
        SEL49_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel48(&mut self) -> SEL48_W {
        SEL48_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel49(&mut self) -> SEL49_W {
        SEL49_W { w: self }
    }
}
