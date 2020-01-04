#[doc = "Reader of register SHIFTEIEN"]
pub type R = crate::R<u32, super::SHIFTEIEN>;
#[doc = "Writer for register SHIFTEIEN"]
pub type W = crate::W<u32, super::SHIFTEIEN>;
#[doc = "Register SHIFTEIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTEIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEIE`"]
pub type SEIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEIE`"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
}
