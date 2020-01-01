#[doc = "Reader of register PKRRNG"]
pub type R = crate::R<u32, super::PKRRNG>;
#[doc = "Writer for register PKRRNG"]
pub type W = crate::W<u32, super::PKRRNG>;
#[doc = "Register PKRRNG `reset()`'s with value 0x09a3"]
impl crate::ResetValue for super::PKRRNG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x09a3
    }
}
#[doc = "Reader of field `PKR_RNG`"]
pub type PKR_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKR_RNG`"]
pub struct PKR_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PKR_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&self) -> PKR_RNG_R {
        PKR_RNG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&mut self) -> PKR_RNG_W {
        PKR_RNG_W { w: self }
    }
}
