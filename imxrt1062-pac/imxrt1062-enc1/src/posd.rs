#[doc = "Reader of register POSD"]
pub type R = crate::R<u16, super::POSD>;
#[doc = "Writer for register POSD"]
pub type W = crate::W<u16, super::POSD>;
#[doc = "Register POSD `reset()`'s with value 0"]
impl crate::ResetValue for super::POSD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POSD`"]
pub type POSD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POSD`"]
pub struct POSD_W<'a> {
    w: &'a mut W,
}
impl<'a> POSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register contains the position change in value occurring between each read of the position register"]
    #[inline(always)]
    pub fn posd(&self) -> POSD_R {
        POSD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register contains the position change in value occurring between each read of the position register"]
    #[inline(always)]
    pub fn posd(&mut self) -> POSD_W {
        POSD_W { w: self }
    }
}
