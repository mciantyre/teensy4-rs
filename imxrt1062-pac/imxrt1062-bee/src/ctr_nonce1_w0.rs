#[doc = "Writer for register CTR_NONCE1_W0"]
pub type W = crate::W<u32, super::CTR_NONCE1_W0>;
#[doc = "Register CTR_NONCE1_W0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR_NONCE1_W0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NONCE10`"]
pub struct NONCE10_W<'a> {
    w: &'a mut W,
}
impl<'a> NONCE10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    #[inline(always)]
    pub fn nonce10(&mut self) -> NONCE10_W {
        NONCE10_W { w: self }
    }
}
