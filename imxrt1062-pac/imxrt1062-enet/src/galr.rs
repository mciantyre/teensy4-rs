#[doc = "Reader of register GALR"]
pub type R = crate::R<u32, super::GALR>;
#[doc = "Writer for register GALR"]
pub type W = crate::W<u32, super::GALR>;
#[doc = "Register GALR `reset()`'s with value 0"]
impl crate::ResetValue for super::GALR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GADDR2`"]
pub type GADDR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GADDR2`"]
pub struct GADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> GADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr2(&self) -> GADDR2_R {
        GADDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr2(&mut self) -> GADDR2_W {
        GADDR2_W { w: self }
    }
}
