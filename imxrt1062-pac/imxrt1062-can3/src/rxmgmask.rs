#[doc = "Reader of register RXMGMASK"]
pub type R = crate::R<u32, super::RXMGMASK>;
#[doc = "Writer for register RXMGMASK"]
pub type W = crate::W<u32, super::RXMGMASK>;
#[doc = "Register RXMGMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::RXMGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MG`"]
pub type MG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MG`"]
pub struct MG_W<'a> {
    w: &'a mut W,
}
impl<'a> MG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg(&mut self) -> MG_W {
        MG_W { w: self }
    }
}
