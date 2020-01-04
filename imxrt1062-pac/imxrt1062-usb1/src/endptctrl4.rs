#[doc = "Reader of register ENDPTCTRL4"]
pub type R = crate::R<u32, super::ENDPTCTRL4>;
#[doc = "Writer for register ENDPTCTRL4"]
pub type W = crate::W<u32, super::ENDPTCTRL4>;
#[doc = "Register ENDPTCTRL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTCTRL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXS`"]
pub type RXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXS`"]
pub struct RXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXS_W<'a> {
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
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXD`"]
pub struct RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_W<'a> {
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
#[doc = "Reader of field `RXT`"]
pub type RXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXT`"]
pub struct RXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RXI`"]
pub type RXI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXI`"]
pub struct RXI_W<'a> {
    w: &'a mut W,
}
impl<'a> RXI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RXR`"]
pub type RXR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXR`"]
pub struct RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TXS`"]
pub type TXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXS`"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXD`"]
pub type TXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TXT`"]
pub type TXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXT`"]
pub struct TXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `TXI`"]
pub type TXI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXI`"]
pub struct TXI_W<'a> {
    w: &'a mut W,
}
impl<'a> TXI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TXR`"]
pub type TXR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXR`"]
pub struct TXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
    #[inline(always)]
    pub fn rxi(&self) -> RXI_R {
        RXI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
    #[inline(always)]
    pub fn txi(&self) -> TXI_R {
        TXI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W { w: self }
    }
    #[doc = "Bit 1 - RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[Default\\]
Should always be written as zero"]
    #[inline(always)]
    pub fn rxd(&mut self) -> RXD_W {
        RXD_W { w: self }
    }
    #[doc = "Bits 2:3 - RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn rxt(&mut self) -> RXT_W {
        RXT_W { w: self }
    }
    #[doc = "Bit 5 - RX Data Toggle Inhibit 0 Disabled \\[Default\\]
1 Enabled This bit is only used for test and should always be written as zero"]
    #[inline(always)]
    pub fn rxi(&mut self) -> RXI_W {
        RXI_W { w: self }
    }
    #[doc = "Bit 6 - RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device"]
    #[inline(always)]
    pub fn rxr(&mut self) -> RXR_W {
        RXR_W { w: self }
    }
    #[doc = "Bit 7 - RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 16 - TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
    #[doc = "Bit 17 - TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \\[DEFAULT\\]
Should always be written as 0"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
    #[doc = "Bits 18:19 - TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn txt(&mut self) -> TXT_W {
        TXT_W { w: self }
    }
    #[doc = "Bit 21 - TX Data Toggle Inhibit 0 PID Sequencing Enabled"]
    #[inline(always)]
    pub fn txi(&mut self) -> TXI_W {
        TXI_W { w: self }
    }
    #[doc = "Bit 22 - TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device"]
    #[inline(always)]
    pub fn txr(&mut self) -> TXR_W {
        TXR_W { w: self }
    }
    #[doc = "Bit 23 - TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
}
