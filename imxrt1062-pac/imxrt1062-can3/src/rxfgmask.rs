#[doc = "Reader of register RXFGMASK"]
pub type R = crate::R<u32, super::RXFGMASK>;
#[doc = "Writer for register RXFGMASK"]
pub type W = crate::W<u32, super::RXFGMASK>;
#[doc = "Register RXFGMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FGM`"]
pub type FGM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FGM`"]
pub struct FGM_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Legacy Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm(&self) -> FGM_R {
        FGM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Legacy Rx FIFO Global Mask Bits"]
    #[inline(always)]
    pub fn fgm(&mut self) -> FGM_W {
        FGM_W { w: self }
    }
}
