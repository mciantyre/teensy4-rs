#[doc = "Reader of register IPRXFCR"]
pub type R = crate::R<u32, super::IPRXFCR>;
#[doc = "Writer for register IPRXFCR"]
pub type W = crate::W<u32, super::IPRXFCR>;
#[doc = "Register IPRXFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPRXFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRIPRXF`"]
pub type CLRIPRXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRIPRXF`"]
pub struct CLRIPRXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRIPRXF_W<'a> {
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
#[doc = "IP RX FIFO reading by DMA enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    #[doc = "0: IP RX FIFO would be read by processor."]
    RXDMAEN_0 = 0,
    #[doc = "1: IP RX FIFO would be read by DMA."]
    RXDMAEN_1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDMAEN`"]
pub type RXDMAEN_R = crate::R<bool, RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::RXDMAEN_0,
            true => RXDMAEN_A::RXDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDMAEN_0`"]
    #[inline(always)]
    pub fn is_rxdmaen_0(&self) -> bool {
        *self == RXDMAEN_A::RXDMAEN_0
    }
    #[doc = "Checks if the value of the field is `RXDMAEN_1`"]
    #[inline(always)]
    pub fn is_rxdmaen_1(&self) -> bool {
        *self == RXDMAEN_A::RXDMAEN_1
    }
}
#[doc = "Write proxy for field `RXDMAEN`"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IP RX FIFO would be read by processor."]
    #[inline(always)]
    pub fn rxdmaen_0(self) -> &'a mut W {
        self.variant(RXDMAEN_A::RXDMAEN_0)
    }
    #[doc = "IP RX FIFO would be read by DMA."]
    #[inline(always)]
    pub fn rxdmaen_1(self) -> &'a mut W {
        self.variant(RXDMAEN_A::RXDMAEN_1)
    }
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
#[doc = "Reader of field `RXWMRK`"]
pub type RXWMRK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXWMRK`"]
pub struct RXWMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn clriprxf(&self) -> CLRIPRXF_R {
        CLRIPRXF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn rxwmrk(&self) -> RXWMRK_R {
        RXWMRK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn clriprxf(&mut self) -> CLRIPRXF_W {
        CLRIPRXF_W { w: self }
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bits 2:5 - Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn rxwmrk(&mut self) -> RXWMRK_W {
        RXWMRK_W { w: self }
    }
}
