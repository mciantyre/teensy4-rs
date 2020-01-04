#[doc = "Reader of register IPTXFCR"]
pub type R = crate::R<u32, super::IPTXFCR>;
#[doc = "Writer for register IPTXFCR"]
pub type W = crate::W<u32, super::IPTXFCR>;
#[doc = "Register IPTXFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPTXFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRIPTXF`"]
pub type CLRIPTXF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRIPTXF`"]
pub struct CLRIPTXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRIPTXF_W<'a> {
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
#[doc = "IP TX FIFO filling by DMA enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    #[doc = "0: IP TX FIFO would be filled by processor."]
    TXDMAEN_0 = 0,
    #[doc = "1: IP TX FIFO would be filled by DMA."]
    TXDMAEN_1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDMAEN`"]
pub type TXDMAEN_R = crate::R<bool, TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::TXDMAEN_0,
            true => TXDMAEN_A::TXDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_0`"]
    #[inline(always)]
    pub fn is_txdmaen_0(&self) -> bool {
        *self == TXDMAEN_A::TXDMAEN_0
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_1`"]
    #[inline(always)]
    pub fn is_txdmaen_1(&self) -> bool {
        *self == TXDMAEN_A::TXDMAEN_1
    }
}
#[doc = "Write proxy for field `TXDMAEN`"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IP TX FIFO would be filled by processor."]
    #[inline(always)]
    pub fn txdmaen_0(self) -> &'a mut W {
        self.variant(TXDMAEN_A::TXDMAEN_0)
    }
    #[doc = "IP TX FIFO would be filled by DMA."]
    #[inline(always)]
    pub fn txdmaen_1(self) -> &'a mut W {
        self.variant(TXDMAEN_A::TXDMAEN_1)
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
#[doc = "Reader of field `TXWMRK`"]
pub type TXWMRK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXWMRK`"]
pub struct TXWMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn clriptxf(&self) -> CLRIPTXF_R {
        CLRIPTXF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn txwmrk(&self) -> TXWMRK_R {
        TXWMRK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn clriptxf(&mut self) -> CLRIPTXF_W {
        CLRIPTXF_W { w: self }
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bits 2:5 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn txwmrk(&mut self) -> TXWMRK_W {
        TXWMRK_W { w: self }
    }
}
