#[doc = "Reader of register SEL14"]
pub type R = crate::R<u16, super::SEL14>;
#[doc = "Writer for register SEL14"]
pub type W = crate::W<u16, super::SEL14>;
#[doc = "Register SEL14 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL14 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL28`"]
pub type SEL28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL28`"]
pub struct SEL28_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SEL29`"]
pub type SEL29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL29`"]
pub struct SEL29_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u16) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel28(&self) -> SEL28_R {
        SEL28_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel29(&self) -> SEL29_R {
        SEL29_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel28(&mut self) -> SEL28_W {
        SEL28_W { w: self }
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel29(&mut self) -> SEL29_W {
        SEL29_W { w: self }
    }
}
