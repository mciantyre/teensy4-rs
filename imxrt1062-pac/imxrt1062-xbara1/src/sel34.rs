#[doc = "Reader of register SEL34"]
pub type R = crate::R<u16, super::SEL34>;
#[doc = "Writer for register SEL34"]
pub type W = crate::W<u16, super::SEL34>;
#[doc = "Register SEL34 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL34 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL68`"]
pub type SEL68_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL68`"]
pub struct SEL68_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL68_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL69`"]
pub type SEL69_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL69`"]
pub struct SEL69_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL69_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel68(&self) -> SEL68_R {
        SEL68_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel69(&self) -> SEL69_R {
        SEL69_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel68(&mut self) -> SEL68_W {
        SEL68_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel69(&mut self) -> SEL69_W {
        SEL69_W { w: self }
    }
}
