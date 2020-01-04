#[doc = "Reader of register REGION1_BOT"]
pub type R = crate::R<u32, super::REGION1_BOT>;
#[doc = "Writer for register REGION1_BOT"]
pub type W = crate::W<u32, super::REGION1_BOT>;
#[doc = "Register REGION1_BOT `reset()`'s with value 0"]
impl crate::ResetValue for super::REGION1_BOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGION1_BOT`"]
pub type REGION1_BOT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGION1_BOT`"]
pub struct REGION1_BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_BOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address lower limit of region1"]
    #[inline(always)]
    pub fn region1_bot(&self) -> REGION1_BOT_R {
        REGION1_BOT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address lower limit of region1"]
    #[inline(always)]
    pub fn region1_bot(&mut self) -> REGION1_BOT_W {
        REGION1_BOT_W { w: self }
    }
}
