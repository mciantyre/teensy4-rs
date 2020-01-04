#[doc = "Reader of register LMOD"]
pub type R = crate::R<u16, super::LMOD>;
#[doc = "Writer for register LMOD"]
pub type W = crate::W<u16, super::LMOD>;
#[doc = "Register LMOD `reset()`'s with value 0"]
impl crate::ResetValue for super::LMOD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register contains the lower (least significant) half of the modulus register"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register contains the lower (least significant) half of the modulus register"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
}
