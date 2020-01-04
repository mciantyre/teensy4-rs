#[doc = "Reader of register IALR"]
pub type R = crate::R<u32, super::IALR>;
#[doc = "Writer for register IALR"]
pub type W = crate::W<u32, super::IALR>;
#[doc = "Register IALR `reset()`'s with value 0"]
impl crate::ResetValue for super::IALR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IADDR2`"]
pub type IADDR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IADDR2`"]
pub struct IADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> IADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr2(&self) -> IADDR2_R {
        IADDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr2(&mut self) -> IADDR2_W {
        IADDR2_W { w: self }
    }
}
