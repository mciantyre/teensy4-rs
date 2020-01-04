#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `LFIFO_COUNT`"]
pub type LFIFO_COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `TXFIFO_EMPTY`"]
pub type TXFIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_FULL`"]
pub type TXFIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFIFO_EMPTY`"]
pub type LFIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFIFO_FULL`"]
pub type LFIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_REQ`"]
pub type DMA_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - Read only view of the current count in Latency buffer (LFIFO)."]
    #[inline(always)]
    pub fn lfifo_count(&self) -> LFIFO_COUNT_R {
        LFIFO_COUNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Read only view of the signals that indicates LCD TXFIFO is empty."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Read only view of the signals that indicates LCD TXFIFO is full."]
    #[inline(always)]
    pub fn txfifo_full(&self) -> TXFIFO_FULL_R {
        TXFIFO_FULL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Read only view of the signals that indicates LCD LFIFO is empty."]
    #[inline(always)]
    pub fn lfifo_empty(&self) -> LFIFO_EMPTY_R {
        LFIFO_EMPTY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Read only view of the signals that indicates LCD LFIFO is full."]
    #[inline(always)]
    pub fn lfifo_full(&self) -> LFIFO_FULL_R {
        LFIFO_FULL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reflects the current state of the DMA Request line for the LCDIF"]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0: LCDIF not present on this product 1: LCDIF is present."]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
