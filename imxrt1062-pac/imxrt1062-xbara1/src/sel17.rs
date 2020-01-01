#[doc = "Reader of register SEL17"]
pub type R = crate::R<u16, super::SEL17>;
#[doc = "Writer for register SEL17"]
pub type W = crate::W<u16, super::SEL17>;
#[doc = "Register SEL17 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL17 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL34`"]
pub type SEL34_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL34`"]
pub struct SEL34_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL35`"]
pub type SEL35_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL35`"]
pub struct SEL35_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel34(&self) -> SEL34_R {
        SEL34_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel35(&self) -> SEL35_R {
        SEL35_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel34(&mut self) -> SEL34_W {
        SEL34_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel35(&mut self) -> SEL35_W {
        SEL35_W { w: self }
    }
}
