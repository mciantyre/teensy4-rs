#[doc = "Reader of register SEL35"]
pub type R = crate::R<u16, super::SEL35>;
#[doc = "Writer for register SEL35"]
pub type W = crate::W<u16, super::SEL35>;
#[doc = "Register SEL35 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL35 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL70`"]
pub type SEL70_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL70`"]
pub struct SEL70_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL70_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL71`"]
pub type SEL71_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL71`"]
pub struct SEL71_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL71_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel70(&self) -> SEL70_R {
        SEL70_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel71(&self) -> SEL71_R {
        SEL71_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel70(&mut self) -> SEL70_W {
        SEL70_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel71(&mut self) -> SEL71_W {
        SEL71_W { w: self }
    }
}
