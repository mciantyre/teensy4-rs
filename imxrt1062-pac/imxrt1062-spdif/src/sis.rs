#[doc = "Reader of register SIS"]
pub type R = crate::R<u32, super::SIS>;
#[doc = "Reader of field `RxFIFOFul`"]
pub type RXFIFOFUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TxEm`"]
pub type TXEM_R = crate::R<bool, bool>;
#[doc = "Reader of field `LockLoss`"]
pub type LOCKLOSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RxFIFOResyn`"]
pub type RXFIFORESYN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RxFIFOUnOv`"]
pub type RXFIFOUNOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `UQErr`"]
pub type UQERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `UQSync`"]
pub type UQSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `QRxOv`"]
pub type QRXOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `QRxFul`"]
pub type QRXFUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `URxOv`"]
pub type URXOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `URxFul`"]
pub type URXFUL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BitErr`"]
pub type BITERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SymErr`"]
pub type SYMERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ValNoGood`"]
pub type VALNOGOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNew`"]
pub type CNEW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TxResyn`"]
pub type TXRESYN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TxUnOv`"]
pub type TXUNOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `Lock`"]
pub type LOCK_R = crate::R<bool, bool>;
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
