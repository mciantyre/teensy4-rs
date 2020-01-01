#[doc = "Reader of register SHIFTSIEN"]
pub type R = crate::R<u32, super::SHIFTSIEN>;
#[doc = "Writer for register SHIFTSIEN"]
pub type W = crate::W<u32, super::SHIFTSIEN>;
#[doc = "Register SHIFTSIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSIE`"]
pub type SSIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSIE`"]
pub struct SSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W {
        SSIE_W { w: self }
    }
}
