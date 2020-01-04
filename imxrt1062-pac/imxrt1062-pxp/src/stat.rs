#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ`"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `AXI_WRITE_ERROR`"]
pub type AXI_WRITE_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_WRITE_ERROR`"]
pub struct AXI_WRITE_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_WRITE_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AXI_READ_ERROR`"]
pub type AXI_READ_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_READ_ERROR`"]
pub struct AXI_READ_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_READ_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `NEXT_IRQ`"]
pub type NEXT_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEXT_IRQ`"]
pub struct NEXT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_IRQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AXI_ERROR_ID`"]
pub type AXI_ERROR_ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `LUT_DMA_LOAD_DONE_IRQ`"]
pub type LUT_DMA_LOAD_DONE_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LUT_DMA_LOAD_DONE_IRQ`"]
pub struct LUT_DMA_LOAD_DONE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_DMA_LOAD_DONE_IRQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `BLOCKY`"]
pub type BLOCKY_R = crate::R<u8, u8>;
#[doc = "Reader of field `BLOCKX`"]
pub type BLOCKX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Indicates current PXP interrupt status"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub fn axi_write_error(&self) -> AXI_WRITE_ERROR_R {
        AXI_WRITE_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub fn axi_read_error(&self) -> AXI_READ_ERROR_R {
        AXI_READ_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub fn next_irq(&self) -> NEXT_IRQ_R {
        NEXT_IRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Indicates the AXI ID of the failing bus operation."]
    #[inline(always)]
    pub fn axi_error_id(&self) -> AXI_ERROR_ID_R {
        AXI_ERROR_ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub fn lut_dma_load_done_irq(&self) -> LUT_DMA_LOAD_DONE_IRQ_R {
        LUT_DMA_LOAD_DONE_IRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub fn blocky(&self) -> BLOCKY_R {
        BLOCKY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub fn blockx(&self) -> BLOCKX_R {
        BLOCKX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates current PXP interrupt status"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Bit 1 - Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub fn axi_write_error(&mut self) -> AXI_WRITE_ERROR_W {
        AXI_WRITE_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub fn axi_read_error(&mut self) -> AXI_READ_ERROR_W {
        AXI_READ_ERROR_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub fn next_irq(&mut self) -> NEXT_IRQ_W {
        NEXT_IRQ_W { w: self }
    }
    #[doc = "Bit 8 - Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub fn lut_dma_load_done_irq(&mut self) -> LUT_DMA_LOAD_DONE_IRQ_W {
        LUT_DMA_LOAD_DONE_IRQ_W { w: self }
    }
}
