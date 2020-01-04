#[doc = "Reader of register CRC_STAT"]
pub type R = crate::R<u32, super::CRC_STAT>;
#[doc = "Writer for register CRC_STAT"]
pub type W = crate::W<u32, super::CRC_STAT>;
#[doc = "Register CRC_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC_VALUE`"]
pub type CRC_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRC_VALUE`"]
pub struct CRC_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Calculated CRC value."]
    #[inline(always)]
    pub fn crc_value(&self) -> CRC_VALUE_R {
        CRC_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculated CRC value."]
    #[inline(always)]
    pub fn crc_value(&mut self) -> CRC_VALUE_W {
        CRC_VALUE_W { w: self }
    }
}
