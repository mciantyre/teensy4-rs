#[doc = "Reader of register TX_CLR"]
pub type R = crate::R<u32, super::TX_CLR>;
#[doc = "Writer for register TX_CLR"]
pub type W = crate::W<u32, super::TX_CLR>;
#[doc = "Register TX_CLR `reset()`'s with value 0x1006_0607"]
impl crate::ResetValue for super::TX_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1006_0607
    }
}
#[doc = "Reader of field `D_CAL`"]
pub type D_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D_CAL`"]
pub struct D_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD0`"]
pub struct RSVD0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DN`"]
pub type TXCAL45DN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DN`"]
pub struct TXCAL45DN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD1`"]
pub struct RSVD1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DP`"]
pub type TXCAL45DP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DP`"]
pub struct TXCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `USBPHY_TX_EDGECTRL`"]
pub type USBPHY_TX_EDGECTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBPHY_TX_EDGECTRL`"]
pub struct USBPHY_TX_EDGECTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHY_TX_EDGECTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Reader of field `RSVD5`"]
pub type RSVD5_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub fn txcal45dn(&self) -> TXCAL45DN_R {
        TXCAL45DN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub fn usbphy_tx_edgectrl(&self) -> USBPHY_TX_EDGECTRL_R {
        USBPHY_TX_EDGECTRL_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd5(&self) -> RSVD5_R {
        RSVD5_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub fn d_cal(&mut self) -> D_CAL_W {
        D_CAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&mut self) -> RSVD0_W {
        RSVD0_W { w: self }
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub fn txcal45dn(&mut self) -> TXCAL45DN_W {
        TXCAL45DN_W { w: self }
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd1(&mut self) -> RSVD1_W {
        RSVD1_W { w: self }
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W {
        TXCAL45DP_W { w: self }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub fn usbphy_tx_edgectrl(&mut self) -> USBPHY_TX_EDGECTRL_W {
        USBPHY_TX_EDGECTRL_W { w: self }
    }
}
