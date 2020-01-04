#[doc = "Reader of register ADMA_SYS_ADDR"]
pub type R = crate::R<u32, super::ADMA_SYS_ADDR>;
#[doc = "Writer for register ADMA_SYS_ADDR"]
pub type W = crate::W<u32, super::ADMA_SYS_ADDR>;
#[doc = "Register ADMA_SYS_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADMA_SYS_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADS_ADDR`"]
pub type ADS_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADS_ADDR`"]
pub struct ADS_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADS_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn ads_addr(&self) -> ADS_ADDR_R {
        ADS_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn ads_addr(&mut self) -> ADS_ADDR_W {
        ADS_ADDR_W { w: self }
    }
}
