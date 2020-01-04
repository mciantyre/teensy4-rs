#[doc = "Reader of register GPTIMER1LD"]
pub type R = crate::R<u32, super::GPTIMER1LD>;
#[doc = "Writer for register GPTIMER1LD"]
pub type W = crate::W<u32, super::GPTIMER1LD>;
#[doc = "Register GPTIMER1LD `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTIMER1LD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTLD`"]
pub type GPTLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPTLD`"]
pub struct GPTLD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
    #[inline(always)]
    pub fn gptld(&self) -> GPTLD_R {
        GPTLD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
    #[inline(always)]
    pub fn gptld(&mut self) -> GPTLD_W {
        GPTLD_W { w: self }
    }
}
