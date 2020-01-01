#[doc = "Reader of register IPCR0"]
pub type R = crate::R<u32, super::IPCR0>;
#[doc = "Writer for register IPCR0"]
pub type W = crate::W<u32, super::IPCR0>;
#[doc = "Register IPCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFAR`"]
pub type SFAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SFAR`"]
pub struct SFAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SFAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    pub fn sfar(&self) -> SFAR_R {
        SFAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Serial Flash Address for IP command."]
    #[inline(always)]
    pub fn sfar(&mut self) -> SFAR_W {
        SFAR_W { w: self }
    }
}
