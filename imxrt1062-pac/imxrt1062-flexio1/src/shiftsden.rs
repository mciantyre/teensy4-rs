#[doc = "Reader of register SHIFTSDEN"]
pub type R = crate::R<u32, super::SHIFTSDEN>;
#[doc = "Writer for register SHIFTSDEN"]
pub type W = crate::W<u32, super::SHIFTSDEN>;
#[doc = "Register SHIFTSDEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSDEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSDE`"]
pub type SSDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSDE`"]
pub struct SSDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SSDE_R {
        SSDE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&mut self) -> SSDE_W {
        SSDE_W { w: self }
    }
}
