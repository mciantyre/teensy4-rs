#[doc = "Reader of register CSIDMASA_FB1"]
pub type R = crate::R<u32, super::CSIDMASA_FB1>;
#[doc = "Writer for register CSIDMASA_FB1"]
pub type W = crate::W<u32, super::CSIDMASA_FB1>;
#[doc = "Register CSIDMASA_FB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIDMASA_FB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_START_ADDR_FB1`"]
pub type DMA_START_ADDR_FB1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_START_ADDR_FB1`"]
pub struct DMA_START_ADDR_FB1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_START_ADDR_FB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - DMA Start Address in Frame Buffer1"]
    #[inline(always)]
    pub fn dma_start_addr_fb1(&self) -> DMA_START_ADDR_FB1_R {
        DMA_START_ADDR_FB1_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Start Address in Frame Buffer1"]
    #[inline(always)]
    pub fn dma_start_addr_fb1(&mut self) -> DMA_START_ADDR_FB1_W {
        DMA_START_ADDR_FB1_W { w: self }
    }
}
