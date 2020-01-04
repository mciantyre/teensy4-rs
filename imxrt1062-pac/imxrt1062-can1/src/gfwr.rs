#[doc = "Reader of register GFWR"]
pub type R = crate::R<u32, super::GFWR>;
#[doc = "Writer for register GFWR"]
pub type W = crate::W<u32, super::GFWR>;
#[doc = "Register GFWR `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::GFWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Reader of field `GFWR`"]
pub type GFWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GFWR`"]
pub struct GFWR_W<'a> {
    w: &'a mut W,
}
impl<'a> GFWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - It determines the Glitch Filter Width"]
    #[inline(always)]
    pub fn gfwr(&self) -> GFWR_R {
        GFWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - It determines the Glitch Filter Width"]
    #[inline(always)]
    pub fn gfwr(&mut self) -> GFWR_W {
        GFWR_W { w: self }
    }
}
