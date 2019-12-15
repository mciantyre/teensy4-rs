#[doc = "Reader of register SM3CAPTCOMPB"]
pub type R = crate::R<u16, super::SM3CAPTCOMPB>;
#[doc = "Writer for register SM3CAPTCOMPB"]
pub type W = crate::W<u16, super::SM3CAPTCOMPB>;
#[doc = "Register SM3CAPTCOMPB `reset()`'s with value 0"]
impl crate::ResetValue for super::SM3CAPTCOMPB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDGCMPB`"]
pub type EDGCMPB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGCMPB`"]
pub struct EDGCMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EDGCNTB`"]
pub type EDGCNTB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare B"]
    #[inline(always)]
    pub fn edgcmpb(&self) -> EDGCMPB_R {
        EDGCMPB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter B"]
    #[inline(always)]
    pub fn edgcntb(&self) -> EDGCNTB_R {
        EDGCNTB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare B"]
    #[inline(always)]
    pub fn edgcmpb(&mut self) -> EDGCMPB_W {
        EDGCMPB_W { w: self }
    }
}
