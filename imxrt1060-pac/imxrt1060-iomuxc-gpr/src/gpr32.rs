#[doc = "Reader of register GPR32"]
pub type R = crate::R<u32, super::GPR32>;
#[doc = "Writer for register GPR32"]
pub type W = crate::W<u32, super::GPR32>;
#[doc = "Register GPR32 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLEXSPI_REMAP_ADDR_OFFSET`"]
pub type FLEXSPI_REMAP_ADDR_OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLEXSPI_REMAP_ADDR_OFFSET`"]
pub struct FLEXSPI_REMAP_ADDR_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI_REMAP_ADDR_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - Offset address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub fn flexspi_remap_addr_offset(&self) -> FLEXSPI_REMAP_ADDR_OFFSET_R {
        FLEXSPI_REMAP_ADDR_OFFSET_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Offset address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub fn flexspi_remap_addr_offset(&mut self) -> FLEXSPI_REMAP_ADDR_OFFSET_W {
        FLEXSPI_REMAP_ADDR_OFFSET_W { w: self }
    }
}
