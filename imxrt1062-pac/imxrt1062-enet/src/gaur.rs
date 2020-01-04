#[doc = "Reader of register GAUR"]
pub type R = crate::R<u32, super::GAUR>;
#[doc = "Writer for register GAUR"]
pub type W = crate::W<u32, super::GAUR>;
#[doc = "Register GAUR `reset()`'s with value 0"]
impl crate::ResetValue for super::GAUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GADDR1`"]
pub type GADDR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GADDR1`"]
pub struct GADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> GADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr1(&self) -> GADDR1_R {
        GADDR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr1(&mut self) -> GADDR1_W {
        GADDR1_W { w: self }
    }
}
