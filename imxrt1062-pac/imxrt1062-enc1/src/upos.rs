#[doc = "Reader of register UPOS"]
pub type R = crate::R<u16, super::UPOS>;
#[doc = "Writer for register UPOS"]
pub type W = crate::W<u16, super::UPOS>;
#[doc = "Register UPOS `reset()`'s with value 0"]
impl crate::ResetValue for super::UPOS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POS`"]
pub type POS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POS`"]
pub struct POS_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register contains the upper (most significant) half of the position counter"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register contains the upper (most significant) half of the position counter"]
    #[inline(always)]
    pub fn pos(&mut self) -> POS_W {
        POS_W { w: self }
    }
}
