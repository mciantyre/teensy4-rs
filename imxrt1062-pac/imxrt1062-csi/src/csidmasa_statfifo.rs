#[doc = "Reader of register CSIDMASA_STATFIFO"]
pub type R = crate::R<u32, super::CSIDMASA_STATFIFO>;
#[doc = "Writer for register CSIDMASA_STATFIFO"]
pub type W = crate::W<u32, super::CSIDMASA_STATFIFO>;
#[doc = "Register CSIDMASA_STATFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIDMASA_STATFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_START_ADDR_SFF`"]
pub type DMA_START_ADDR_SFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_START_ADDR_SFF`"]
pub struct DMA_START_ADDR_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_START_ADDR_SFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - DMA Start Address for STATFIFO"]
    #[inline(always)]
    pub fn dma_start_addr_sff(&self) -> DMA_START_ADDR_SFF_R {
        DMA_START_ADDR_SFF_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Start Address for STATFIFO"]
    #[inline(always)]
    pub fn dma_start_addr_sff(&mut self) -> DMA_START_ADDR_SFF_W {
        DMA_START_ADDR_SFF_W { w: self }
    }
}
