#[doc = "Reader of register IAUR"]
pub type R = crate::R<u32, super::IAUR>;
#[doc = "Writer for register IAUR"]
pub type W = crate::W<u32, super::IAUR>;
#[doc = "Register IAUR `reset()`'s with value 0"]
impl crate::ResetValue for super::IAUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IADDR1`"]
pub type IADDR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IADDR1`"]
pub struct IADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> IADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr1(&self) -> IADDR1_R {
        IADDR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr1(&mut self) -> IADDR1_W {
        IADDR1_W { w: self }
    }
}
