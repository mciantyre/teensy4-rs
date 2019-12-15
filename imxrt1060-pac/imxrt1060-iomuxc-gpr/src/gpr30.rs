#[doc = "Reader of register GPR30"]
pub type R = crate::R<u32, super::GPR30>;
#[doc = "Writer for register GPR30"]
pub type W = crate::W<u32, super::GPR30>;
#[doc = "Register GPR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLEXSPI_REMAP_ADDR_START`"]
pub type FLEXSPI_REMAP_ADDR_START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLEXSPI_REMAP_ADDR_START`"]
pub struct FLEXSPI_REMAP_ADDR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI_REMAP_ADDR_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - Start address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub fn flexspi_remap_addr_start(&self) -> FLEXSPI_REMAP_ADDR_START_R {
        FLEXSPI_REMAP_ADDR_START_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Start address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub fn flexspi_remap_addr_start(&mut self) -> FLEXSPI_REMAP_ADDR_START_W {
        FLEXSPI_REMAP_ADDR_START_W { w: self }
    }
}
