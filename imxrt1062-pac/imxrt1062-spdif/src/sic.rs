#[doc = "Reader of register SIC"]
pub type R = crate::R<u32, super::SIC>;
#[doc = "Writer for register SIC"]
pub type W = crate::W<u32, super::SIC>;
#[doc = "Register SIC `reset()`'s with value 0"]
impl crate::ResetValue for super::SIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LockLoss`"]
pub struct LOCKLOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLOSS_W<'a> {
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
#[doc = "Write proxy for field `RxFIFOResyn`"]
pub struct RXFIFORESYN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFORESYN_W<'a> {
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
#[doc = "Write proxy for field `RxFIFOUnOv`"]
pub struct RXFIFOUNOV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOUNOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `UQErr`"]
pub struct UQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UQERR_W<'a> {
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
#[doc = "Write proxy for field `UQSync`"]
pub struct UQSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> UQSYNC_W<'a> {
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
#[doc = "Write proxy for field `QRxOv`"]
pub struct QRXOV_W<'a> {
    w: &'a mut W,
}
impl<'a> QRXOV_W<'a> {
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
#[doc = "Write proxy for field `URxOv`"]
pub struct URXOV_W<'a> {
    w: &'a mut W,
}
impl<'a> URXOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `BitErr`"]
pub struct BITERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BITERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `SymErr`"]
pub struct SYMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYMERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `ValNoGood`"]
pub struct VALNOGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> VALNOGOOD_W<'a> {
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
#[doc = "Write proxy for field `CNew`"]
pub struct CNEW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNEW_W<'a> {
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
#[doc = "Write proxy for field `TxResyn`"]
pub struct TXRESYN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRESYN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `TxUnOv`"]
pub struct TXUNOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `Lock`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 2 - SPDIF receiver loss of lock"]
    #[inline(always)]
    pub fn lock_loss(&mut self) -> LOCKLOSS_W {
        LOCKLOSS_W { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO resync"]
    #[inline(always)]
    pub fn rx_fiforesyn(&mut self) -> RXFIFORESYN_W {
        RXFIFORESYN_W { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub fn rx_fifoun_ov(&mut self) -> RXFIFOUNOV_W {
        RXFIFOUNOV_W { w: self }
    }
    #[doc = "Bit 5 - U/Q Channel framing error"]
    #[inline(always)]
    pub fn uqerr(&mut self) -> UQERR_W {
        UQERR_W { w: self }
    }
    #[doc = "Bit 6 - U/Q Channel sync found"]
    #[inline(always)]
    pub fn uqsync(&mut self) -> UQSYNC_W {
        UQSYNC_W { w: self }
    }
    #[doc = "Bit 7 - Q Channel receive register overrun"]
    #[inline(always)]
    pub fn qrx_ov(&mut self) -> QRXOV_W {
        QRXOV_W { w: self }
    }
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    pub fn urx_ov(&mut self) -> URXOV_W {
        URXOV_W { w: self }
    }
    #[doc = "Bit 14 - SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub fn bit_err(&mut self) -> BITERR_W {
        BITERR_W { w: self }
    }
    #[doc = "Bit 15 - SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub fn sym_err(&mut self) -> SYMERR_W {
        SYMERR_W { w: self }
    }
    #[doc = "Bit 16 - SPDIF validity flag no good"]
    #[inline(always)]
    pub fn val_no_good(&mut self) -> VALNOGOOD_W {
        VALNOGOOD_W { w: self }
    }
    #[doc = "Bit 17 - SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub fn cnew(&mut self) -> CNEW_W {
        CNEW_W { w: self }
    }
    #[doc = "Bit 18 - SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub fn tx_resyn(&mut self) -> TXRESYN_W {
        TXRESYN_W { w: self }
    }
    #[doc = "Bit 19 - SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub fn tx_un_ov(&mut self) -> TXUNOV_W {
        TXUNOV_W { w: self }
    }
    #[doc = "Bit 20 - SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
