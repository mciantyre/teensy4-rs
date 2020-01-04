#[doc = "Reader of register CSIDMATS_STATFIFO"]
pub type R = crate::R<u32, super::CSIDMATS_STATFIFO>;
#[doc = "Writer for register CSIDMATS_STATFIFO"]
pub type W = crate::W<u32, super::CSIDMATS_STATFIFO>;
#[doc = "Register CSIDMATS_STATFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIDMATS_STATFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_TSF_SIZE_SFF`"]
pub type DMA_TSF_SIZE_SFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_TSF_SIZE_SFF`"]
pub struct DMA_TSF_SIZE_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TSF_SIZE_SFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Transfer Size for STATFIFO"]
    #[inline(always)]
    pub fn dma_tsf_size_sff(&self) -> DMA_TSF_SIZE_SFF_R {
        DMA_TSF_SIZE_SFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Transfer Size for STATFIFO"]
    #[inline(always)]
    pub fn dma_tsf_size_sff(&mut self) -> DMA_TSF_SIZE_SFF_W {
        DMA_TSF_SIZE_SFF_W { w: self }
    }
}
