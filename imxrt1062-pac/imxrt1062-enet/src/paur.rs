#[doc = "Reader of register PAUR"]
pub type R = crate::R<u32, super::PAUR>;
#[doc = "Writer for register PAUR"]
pub type W = crate::W<u32, super::PAUR>;
#[doc = "Register PAUR `reset()`'s with value 0x8808"]
impl crate::ResetValue for super::PAUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8808
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u16, u16>;
#[doc = "Reader of field `PADDR2`"]
pub type PADDR2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PADDR2`"]
pub struct PADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Type Field In PAUSE Frames"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub fn paddr2(&self) -> PADDR2_R {
        PADDR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    #[inline(always)]
    pub fn paddr2(&mut self) -> PADDR2_W {
        PADDR2_W { w: self }
    }
}
