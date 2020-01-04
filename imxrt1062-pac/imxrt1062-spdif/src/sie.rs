#[doc = "Reader of register SIE"]
pub type R = crate::R<u32, super::SIE>;
#[doc = "Writer for register SIE"]
pub type W = crate::W<u32, super::SIE>;
#[doc = "Register SIE `reset()`'s with value 0"]
impl crate::ResetValue for super::SIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RxFIFOFul`"]
pub type RXFIFOFUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxFIFOFul`"]
pub struct RXFIFOFUL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFUL_W<'a> {
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
#[doc = "Reader of field `TxEm`"]
pub type TXEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxEm`"]
pub struct TXEM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEM_W<'a> {
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
#[doc = "Reader of field `LockLoss`"]
pub type LOCKLOSS_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `RxFIFOResyn`"]
pub type RXFIFORESYN_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `RxFIFOUnOv`"]
pub type RXFIFOUNOV_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `UQErr`"]
pub type UQERR_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `UQSync`"]
pub type UQSYNC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `QRxOv`"]
pub type QRXOV_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `QRxFul`"]
pub type QRXFUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QRxFul`"]
pub struct QRXFUL_W<'a> {
    w: &'a mut W,
}
impl<'a> QRXFUL_W<'a> {
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
#[doc = "Reader of field `URxOv`"]
pub type URXOV_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `URxFul`"]
pub type URXFUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URxFul`"]
pub struct URXFUL_W<'a> {
    w: &'a mut W,
}
impl<'a> URXFUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BitErr`"]
pub type BITERR_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `SymErr`"]
pub type SYMERR_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `ValNoGood`"]
pub type VALNOGOOD_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `CNew`"]
pub type CNEW_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `TxResyn`"]
pub type TXRESYN_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `TxUnOv`"]
pub type TXUNOV_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `Lock`"]
pub type LOCK_R = crate::R<bool, bool>;
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
impl R {
    #[doc = "Bit 0 - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifoful(&self) -> RXFIFOFUL_R {
        RXFIFOFUL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    pub fn tx_em(&self) -> TXEM_R {
        TXEM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPDIF receiver loss of lock"]
    #[inline(always)]
    pub fn lock_loss(&self) -> LOCKLOSS_R {
        LOCKLOSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO resync"]
    #[inline(always)]
    pub fn rx_fiforesyn(&self) -> RXFIFORESYN_R {
        RXFIFORESYN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO underrun/overrun"]
    #[inline(always)]
    pub fn rx_fifoun_ov(&self) -> RXFIFOUNOV_R {
        RXFIFOUNOV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - U/Q Channel framing error"]
    #[inline(always)]
    pub fn uqerr(&self) -> UQERR_R {
        UQERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - U/Q Channel sync found"]
    #[inline(always)]
    pub fn uqsync(&self) -> UQSYNC_R {
        UQSYNC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Q Channel receive register overrun"]
    #[inline(always)]
    pub fn qrx_ov(&self) -> QRXOV_R {
        QRXOV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn qrx_ful(&self) -> QRXFUL_R {
        QRXFUL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    pub fn urx_ov(&self) -> URXOV_R {
        URXOV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn urx_ful(&self) -> URXFUL_R {
        URXFUL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPDIF receiver found parity bit error"]
    #[inline(always)]
    pub fn bit_err(&self) -> BITERR_R {
        BITERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPDIF receiver found illegal symbol"]
    #[inline(always)]
    pub fn sym_err(&self) -> SYMERR_R {
        SYMERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPDIF validity flag no good"]
    #[inline(always)]
    pub fn val_no_good(&self) -> VALNOGOOD_R {
        VALNOGOOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SPDIF receive change in value of control channel"]
    #[inline(always)]
    pub fn cnew(&self) -> CNEW_R {
        CNEW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SPDIF Tx FIFO resync"]
    #[inline(always)]
    pub fn tx_resyn(&self) -> TXRESYN_R {
        TXRESYN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SPDIF Tx FIFO under/overrun"]
    #[inline(always)]
    pub fn tx_un_ov(&self) -> TXUNOV_R {
        TXUNOV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SPDIF receiver's DPLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifoful(&mut self) -> RXFIFOFUL_W {
        RXFIFOFUL_W { w: self }
    }
    #[doc = "Bit 1 - SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TXEM_W {
        TXEM_W { w: self }
    }
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
    #[doc = "Bit 8 - Q Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn qrx_ful(&mut self) -> QRXFUL_W {
        QRXFUL_W { w: self }
    }
    #[doc = "Bit 9 - U Channel receive register overrun"]
    #[inline(always)]
    pub fn urx_ov(&mut self) -> URXOV_W {
        URXOV_W { w: self }
    }
    #[doc = "Bit 10 - U Channel receive register full, can't be cleared with reg"]
    #[inline(always)]
    pub fn urx_ful(&mut self) -> URXFUL_W {
        URXFUL_W { w: self }
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
