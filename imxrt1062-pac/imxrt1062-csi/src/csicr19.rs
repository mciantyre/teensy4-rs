#[doc = "Reader of register CSICR19"]
pub type R = crate::R<u32, super::CSICR19>;
#[doc = "Writer for register CSICR19"]
pub type W = crate::W<u32, super::CSICR19>;
#[doc = "Register CSICR19 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSICR19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_RFIFO_HIGHEST_FIFO_LEVEL`"]
pub type DMA_RFIFO_HIGHEST_FIFO_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_RFIFO_HIGHEST_FIFO_LEVEL`"]
pub struct DMA_RFIFO_HIGHEST_FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RFIFO_HIGHEST_FIFO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it"]
    #[inline(always)]
    pub fn dma_rfifo_highest_fifo_level(&self) -> DMA_RFIFO_HIGHEST_FIFO_LEVEL_R {
        DMA_RFIFO_HIGHEST_FIFO_LEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it"]
    #[inline(always)]
    pub fn dma_rfifo_highest_fifo_level(&mut self) -> DMA_RFIFO_HIGHEST_FIFO_LEVEL_W {
        DMA_RFIFO_HIGHEST_FIFO_LEVEL_W { w: self }
    }
}
