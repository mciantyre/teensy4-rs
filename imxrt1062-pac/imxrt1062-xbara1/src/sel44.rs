#[doc = "Reader of register SEL44"]
pub type R = crate::R<u16, super::SEL44>;
#[doc = "Writer for register SEL44"]
pub type W = crate::W<u16, super::SEL44>;
#[doc = "Register SEL44 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL44 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL88`"]
pub type SEL88_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL88`"]
pub struct SEL88_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL88_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL89`"]
pub type SEL89_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL89`"]
pub struct SEL89_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL89_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel88(&self) -> SEL88_R {
        SEL88_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel89(&self) -> SEL89_R {
        SEL89_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel88(&mut self) -> SEL88_W {
        SEL88_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel89(&mut self) -> SEL89_W {
        SEL89_W { w: self }
    }
}
