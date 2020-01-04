#[doc = "Reader of register FTRL"]
pub type R = crate::R<u32, super::FTRL>;
#[doc = "Writer for register FTRL"]
pub type W = crate::W<u32, super::FTRL>;
#[doc = "Register FTRL `reset()`'s with value 0x07ff"]
impl crate::ResetValue for super::FTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07ff
    }
}
#[doc = "Reader of field `TRUNC_FL`"]
pub type TRUNC_FL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRUNC_FL`"]
pub struct TRUNC_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUNC_FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    pub fn trunc_fl(&self) -> TRUNC_FL_R {
        TRUNC_FL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    pub fn trunc_fl(&mut self) -> TRUNC_FL_W {
        TRUNC_FL_W { w: self }
    }
}
