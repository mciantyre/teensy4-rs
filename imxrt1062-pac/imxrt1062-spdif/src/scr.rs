#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USRC_SEL_A {
    #[doc = "0: No embedded U channel"]
    USRC_SEL_0 = 0,
    #[doc = "1: U channel from SPDIF receive block (CD mode)"]
    USRC_SEL_1 = 1,
    #[doc = "3: U channel from on chip transmitter"]
    USRC_SEL_3 = 3,
}
impl From<USRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USrc_Sel`"]
pub type USRC_SEL_R = crate::R<u8, USRC_SEL_A>;
impl USRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USRC_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USRC_SEL_A::USRC_SEL_0),
            1 => Val(USRC_SEL_A::USRC_SEL_1),
            3 => Val(USRC_SEL_A::USRC_SEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_0`"]
    #[inline(always)]
    pub fn is_usrc_sel_0(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_1`"]
    #[inline(always)]
    pub fn is_usrc_sel_1(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `USRC_SEL_3`"]
    #[inline(always)]
    pub fn is_usrc_sel_3(&self) -> bool {
        *self == USRC_SEL_A::USRC_SEL_3
    }
}
#[doc = "Write proxy for field `USrc_Sel`"]
pub struct USRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No embedded U channel"]
    #[inline(always)]
    pub fn usrc_sel_0(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_0)
    }
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    #[inline(always)]
    pub fn usrc_sel_1(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_1)
    }
    #[doc = "U channel from on chip transmitter"]
    #[inline(always)]
    pub fn usrc_sel_3(self) -> &'a mut W {
        self.variant(USRC_SEL_A::USRC_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXSEL_A {
    #[doc = "0: Off and output 0"]
    TXSEL_0 = 0,
    #[doc = "1: Feed-through SPDIFIN"]
    TXSEL_1 = 1,
    #[doc = "5: Tx Normal operation"]
    TXSEL_5 = 5,
}
impl From<TXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TxSel`"]
pub type TXSEL_R = crate::R<u8, TXSEL_A>;
impl TXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXSEL_A::TXSEL_0),
            1 => Val(TXSEL_A::TXSEL_1),
            5 => Val(TXSEL_A::TXSEL_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXSEL_0`"]
    #[inline(always)]
    pub fn is_tx_sel_0(&self) -> bool {
        *self == TXSEL_A::TXSEL_0
    }
    #[doc = "Checks if the value of the field is `TXSEL_1`"]
    #[inline(always)]
    pub fn is_tx_sel_1(&self) -> bool {
        *self == TXSEL_A::TXSEL_1
    }
    #[doc = "Checks if the value of the field is `TXSEL_5`"]
    #[inline(always)]
    pub fn is_tx_sel_5(&self) -> bool {
        *self == TXSEL_A::TXSEL_5
    }
}
#[doc = "Write proxy for field `TxSel`"]
pub struct TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Off and output 0"]
    #[inline(always)]
    pub fn tx_sel_0(self) -> &'a mut W {
        self.variant(TXSEL_A::TXSEL_0)
    }
    #[doc = "Feed-through SPDIFIN"]
    #[inline(always)]
    pub fn tx_sel_1(self) -> &'a mut W {
        self.variant(TXSEL_A::TXSEL_1)
    }
    #[doc = "Tx Normal operation"]
    #[inline(always)]
    pub fn tx_sel_5(self) -> &'a mut W {
        self.variant(TXSEL_A::TXSEL_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALCTRL_A {
    #[doc = "0: Outgoing Validity always set"]
    VALCTRL_0 = 0,
    #[doc = "1: Outgoing Validity always clear"]
    VALCTRL_1 = 1,
}
impl From<VALCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VALCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ValCtrl`"]
pub type VALCTRL_R = crate::R<bool, VALCTRL_A>;
impl VALCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALCTRL_A {
        match self.bits {
            false => VALCTRL_A::VALCTRL_0,
            true => VALCTRL_A::VALCTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALCTRL_0`"]
    #[inline(always)]
    pub fn is_val_ctrl_0(&self) -> bool {
        *self == VALCTRL_A::VALCTRL_0
    }
    #[doc = "Checks if the value of the field is `VALCTRL_1`"]
    #[inline(always)]
    pub fn is_val_ctrl_1(&self) -> bool {
        *self == VALCTRL_A::VALCTRL_1
    }
}
#[doc = "Write proxy for field `ValCtrl`"]
pub struct VALCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> VALCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Outgoing Validity always set"]
    #[inline(always)]
    pub fn val_ctrl_0(self) -> &'a mut W {
        self.variant(VALCTRL_A::VALCTRL_0)
    }
    #[doc = "Outgoing Validity always clear"]
    #[inline(always)]
    pub fn val_ctrl_1(self) -> &'a mut W {
        self.variant(VALCTRL_A::VALCTRL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DMA_TX_En`"]
pub type DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TX_En`"]
pub struct DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_EN_W<'a> {
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
#[doc = "Reader of field `DMA_Rx_En`"]
pub type DMA_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_Rx_En`"]
pub struct DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_EN_W<'a> {
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
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFO_CTRL_A {
    #[doc = "0: Send out digital zero on SPDIF Tx"]
    TXFIFO_CTRL_0 = 0,
    #[doc = "1: Tx Normal operation"]
    TXFIFO_CTRL_1 = 1,
    #[doc = "2: Reset to 1 sample remaining"]
    TXFIFO_CTRL_2 = 2,
}
impl From<TXFIFO_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFO_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TxFIFO_Ctrl`"]
pub type TXFIFO_CTRL_R = crate::R<u8, TXFIFO_CTRL_A>;
impl TXFIFO_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXFIFO_CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXFIFO_CTRL_A::TXFIFO_CTRL_0),
            1 => Val(TXFIFO_CTRL_A::TXFIFO_CTRL_1),
            2 => Val(TXFIFO_CTRL_A::TXFIFO_CTRL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_0`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_0(&self) -> bool {
        *self == TXFIFO_CTRL_A::TXFIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_1`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_1(&self) -> bool {
        *self == TXFIFO_CTRL_A::TXFIFO_CTRL_1
    }
    #[doc = "Checks if the value of the field is `TXFIFO_CTRL_2`"]
    #[inline(always)]
    pub fn is_tx_fifo_ctrl_2(&self) -> bool {
        *self == TXFIFO_CTRL_A::TXFIFO_CTRL_2
    }
}
#[doc = "Write proxy for field `TxFIFO_Ctrl`"]
pub struct TXFIFO_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFO_CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Send out digital zero on SPDIF Tx"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(TXFIFO_CTRL_A::TXFIFO_CTRL_0)
    }
    #[doc = "Tx Normal operation"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(TXFIFO_CTRL_A::TXFIFO_CTRL_1)
    }
    #[doc = "Reset to 1 sample remaining"]
    #[inline(always)]
    pub fn tx_fifo_ctrl_2(self) -> &'a mut W {
        self.variant(TXFIFO_CTRL_A::TXFIFO_CTRL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `soft_reset`"]
pub type SOFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_reset`"]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LOW_POWER`"]
pub type LOW_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOW_POWER`"]
pub struct LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_POWER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFOEMPTY_SEL_A {
    #[doc = "0: Empty interrupt if 0 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_0 = 0,
    #[doc = "1: Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_1 = 1,
    #[doc = "2: Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_2 = 2,
    #[doc = "3: Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    TXFIFOEMPTY_SEL_3 = 3,
}
impl From<TXFIFOEMPTY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOEMPTY_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TxFIFOEmpty_Sel`"]
pub type TXFIFOEMPTY_SEL_R = crate::R<u8, TXFIFOEMPTY_SEL_A>;
impl TXFIFOEMPTY_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOEMPTY_SEL_A {
        match self.bits {
            0 => TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_0,
            1 => TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_1,
            2 => TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_2,
            3 => TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_0`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_0(&self) -> bool {
        *self == TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_0
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_1`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_1(&self) -> bool {
        *self == TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_1
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_2`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_2(&self) -> bool {
        *self == TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_2
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY_SEL_3`"]
    #[inline(always)]
    pub fn is_tx_fifoempty_sel_3(&self) -> bool {
        *self == TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_3
    }
}
#[doc = "Write proxy for field `TxFIFOEmpty_Sel`"]
pub struct TXFIFOEMPTY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOEMPTY_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFOEMPTY_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_0(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_0)
    }
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_1(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_1)
    }
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_2(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_2)
    }
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    #[inline(always)]
    pub fn tx_fifoempty_sel_3(self) -> &'a mut W {
        self.variant(TXFIFOEMPTY_SEL_A::TXFIFOEMPTY_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXAUTOSYNC_A {
    #[doc = "0: Tx FIFO auto sync off"]
    TXAUTOSYNC_0 = 0,
    #[doc = "1: Tx FIFO auto sync on"]
    TXAUTOSYNC_1 = 1,
}
impl From<TXAUTOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TXAUTOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TxAutoSync`"]
pub type TXAUTOSYNC_R = crate::R<bool, TXAUTOSYNC_A>;
impl TXAUTOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXAUTOSYNC_A {
        match self.bits {
            false => TXAUTOSYNC_A::TXAUTOSYNC_0,
            true => TXAUTOSYNC_A::TXAUTOSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXAUTOSYNC_0`"]
    #[inline(always)]
    pub fn is_tx_auto_sync_0(&self) -> bool {
        *self == TXAUTOSYNC_A::TXAUTOSYNC_0
    }
    #[doc = "Checks if the value of the field is `TXAUTOSYNC_1`"]
    #[inline(always)]
    pub fn is_tx_auto_sync_1(&self) -> bool {
        *self == TXAUTOSYNC_A::TXAUTOSYNC_1
    }
}
#[doc = "Write proxy for field `TxAutoSync`"]
pub struct TXAUTOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXAUTOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXAUTOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx FIFO auto sync off"]
    #[inline(always)]
    pub fn tx_auto_sync_0(self) -> &'a mut W {
        self.variant(TXAUTOSYNC_A::TXAUTOSYNC_0)
    }
    #[doc = "Tx FIFO auto sync on"]
    #[inline(always)]
    pub fn tx_auto_sync_1(self) -> &'a mut W {
        self.variant(TXAUTOSYNC_A::TXAUTOSYNC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXAUTOSYNC_A {
    #[doc = "0: Rx FIFO auto sync off"]
    RXAUTOSYNC_0 = 0,
    #[doc = "1: RxFIFO auto sync on"]
    RXAUTOSYNC_1 = 1,
}
impl From<RXAUTOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RXAUTOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxAutoSync`"]
pub type RXAUTOSYNC_R = crate::R<bool, RXAUTOSYNC_A>;
impl RXAUTOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXAUTOSYNC_A {
        match self.bits {
            false => RXAUTOSYNC_A::RXAUTOSYNC_0,
            true => RXAUTOSYNC_A::RXAUTOSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXAUTOSYNC_0`"]
    #[inline(always)]
    pub fn is_rx_auto_sync_0(&self) -> bool {
        *self == RXAUTOSYNC_A::RXAUTOSYNC_0
    }
    #[doc = "Checks if the value of the field is `RXAUTOSYNC_1`"]
    #[inline(always)]
    pub fn is_rx_auto_sync_1(&self) -> bool {
        *self == RXAUTOSYNC_A::RXAUTOSYNC_1
    }
}
#[doc = "Write proxy for field `RxAutoSync`"]
pub struct RXAUTOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXAUTOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXAUTOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO auto sync off"]
    #[inline(always)]
    pub fn rx_auto_sync_0(self) -> &'a mut W {
        self.variant(RXAUTOSYNC_A::RXAUTOSYNC_0)
    }
    #[doc = "RxFIFO auto sync on"]
    #[inline(always)]
    pub fn rx_auto_sync_1(self) -> &'a mut W {
        self.variant(RXAUTOSYNC_A::RXAUTOSYNC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIFOFULL_SEL_A {
    #[doc = "0: Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_0 = 0,
    #[doc = "1: Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_1 = 1,
    #[doc = "2: Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    RXFIFOFULL_SEL_2 = 2,
    #[doc = "3: Full interrupt if at least 16 sample in Rx left and right FIFO"]
    RXFIFOFULL_SEL_3 = 3,
}
impl From<RXFIFOFULL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOFULL_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RxFIFOFull_Sel`"]
pub type RXFIFOFULL_SEL_R = crate::R<u8, RXFIFOFULL_SEL_A>;
impl RXFIFOFULL_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOFULL_SEL_A {
        match self.bits {
            0 => RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_0,
            1 => RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_1,
            2 => RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_2,
            3 => RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_0`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_0(&self) -> bool {
        *self == RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_0
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_1`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_1(&self) -> bool {
        *self == RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_1
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_2`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_2(&self) -> bool {
        *self == RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_2
    }
    #[doc = "Checks if the value of the field is `RXFIFOFULL_SEL_3`"]
    #[inline(always)]
    pub fn is_rx_fifofull_sel_3(&self) -> bool {
        *self == RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_3
    }
}
#[doc = "Write proxy for field `RxFIFOFull_Sel`"]
pub struct RXFIFOFULL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFULL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFOFULL_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_0(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_0)
    }
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_1(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_1)
    }
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    #[inline(always)]
    pub fn rx_fifofull_sel_2(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_2)
    }
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    #[inline(always)]
    pub fn rx_fifofull_sel_3(self) -> &'a mut W {
        self.variant(RXFIFOFULL_SEL_A::RXFIFOFULL_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_RST_A {
    #[doc = "0: Normal operation"]
    RXFIFO_RST_0 = 0,
    #[doc = "1: Reset register to 1 sample remaining"]
    RXFIFO_RST_1 = 1,
}
impl From<RXFIFO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFO_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxFIFO_Rst`"]
pub type RXFIFO_RST_R = crate::R<bool, RXFIFO_RST_A>;
impl RXFIFO_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFO_RST_A {
        match self.bits {
            false => RXFIFO_RST_A::RXFIFO_RST_0,
            true => RXFIFO_RST_A::RXFIFO_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_RST_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_rst_0(&self) -> bool {
        *self == RXFIFO_RST_A::RXFIFO_RST_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_RST_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_rst_1(&self) -> bool {
        *self == RXFIFO_RST_A::RXFIFO_RST_1
    }
}
#[doc = "Write proxy for field `RxFIFO_Rst`"]
pub struct RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFO_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn rx_fifo_rst_0(self) -> &'a mut W {
        self.variant(RXFIFO_RST_A::RXFIFO_RST_0)
    }
    #[doc = "Reset register to 1 sample remaining"]
    #[inline(always)]
    pub fn rx_fifo_rst_1(self) -> &'a mut W {
        self.variant(RXFIFO_RST_A::RXFIFO_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_OFF_ON_A {
    #[doc = "0: SPDIF Rx FIFO is on"]
    RXFIFO_OFF_ON_0 = 0,
    #[doc = "1: SPDIF Rx FIFO is off. Does not accept data from interface"]
    RXFIFO_OFF_ON_1 = 1,
}
impl From<RXFIFO_OFF_ON_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFO_OFF_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxFIFO_Off_On`"]
pub type RXFIFO_OFF_ON_R = crate::R<bool, RXFIFO_OFF_ON_A>;
impl RXFIFO_OFF_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFO_OFF_ON_A {
        match self.bits {
            false => RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_0,
            true => RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_OFF_ON_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_off_on_0(&self) -> bool {
        *self == RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_OFF_ON_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_off_on_1(&self) -> bool {
        *self == RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_1
    }
}
#[doc = "Write proxy for field `RxFIFO_Off_On`"]
pub struct RXFIFO_OFF_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OFF_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFO_OFF_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPDIF Rx FIFO is on"]
    #[inline(always)]
    pub fn rx_fifo_off_on_0(self) -> &'a mut W {
        self.variant(RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_0)
    }
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    #[inline(always)]
    pub fn rx_fifo_off_on_1(self) -> &'a mut W {
        self.variant(RXFIFO_OFF_ON_A::RXFIFO_OFF_ON_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFO_CTRL_A {
    #[doc = "0: Normal operation"]
    RXFIFO_CTRL_0 = 0,
    #[doc = "1: Always read zero from Rx data register"]
    RXFIFO_CTRL_1 = 1,
}
impl From<RXFIFO_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFO_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxFIFO_Ctrl`"]
pub type RXFIFO_CTRL_R = crate::R<bool, RXFIFO_CTRL_A>;
impl RXFIFO_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFO_CTRL_A {
        match self.bits {
            false => RXFIFO_CTRL_A::RXFIFO_CTRL_0,
            true => RXFIFO_CTRL_A::RXFIFO_CTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_CTRL_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_ctrl_0(&self) -> bool {
        *self == RXFIFO_CTRL_A::RXFIFO_CTRL_0
    }
    #[doc = "Checks if the value of the field is `RXFIFO_CTRL_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_ctrl_1(&self) -> bool {
        *self == RXFIFO_CTRL_A::RXFIFO_CTRL_1
    }
}
#[doc = "Write proxy for field `RxFIFO_Ctrl`"]
pub struct RXFIFO_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFO_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn rx_fifo_ctrl_0(self) -> &'a mut W {
        self.variant(RXFIFO_CTRL_A::RXFIFO_CTRL_0)
    }
    #[doc = "Always read zero from Rx data register"]
    #[inline(always)]
    pub fn rx_fifo_ctrl_1(self) -> &'a mut W {
        self.variant(RXFIFO_CTRL_A::RXFIFO_CTRL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn usrc_sel(&self) -> USRC_SEL_R {
        USRC_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline(always)]
    pub fn tx_sel(&self) -> TXSEL_R {
        TXSEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn val_ctrl(&self) -> VALCTRL_R {
        VALCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline(always)]
    pub fn tx_fifo_ctrl(&self) -> TXFIFO_CTRL_R {
        TXFIFO_CTRL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline(always)]
    pub fn low_power(&self) -> LOW_POWER_R {
        LOW_POWER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline(always)]
    pub fn tx_fifoempty_sel(&self) -> TXFIFOEMPTY_SEL_R {
        TXFIFOEMPTY_SEL_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn tx_auto_sync(&self) -> TXAUTOSYNC_R {
        TXAUTOSYNC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn rx_auto_sync(&self) -> RXAUTOSYNC_R {
        RXAUTOSYNC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline(always)]
    pub fn rx_fifofull_sel(&self) -> RXFIFOFULL_SEL_R {
        RXFIFOFULL_SEL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_off_on(&self) -> RXFIFO_OFF_ON_R {
        RXFIFO_OFF_ON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_ctrl(&self) -> RXFIFO_CTRL_R {
        RXFIFO_CTRL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn usrc_sel(&mut self) -> USRC_SEL_W {
        USRC_SEL_W { w: self }
    }
    #[doc = "Bits 2:4 - no description available"]
    #[inline(always)]
    pub fn tx_sel(&mut self) -> TXSEL_W {
        TXSEL_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn val_ctrl(&mut self) -> VALCTRL_W {
        VALCTRL_W { w: self }
    }
    #[doc = "Bit 8 - DMA Transmit Request Enable (Tx FIFO empty)"]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W {
        DMA_TX_EN_W { w: self }
    }
    #[doc = "Bit 9 - DMA Receive Request Enable (RX FIFO full)"]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W {
        DMA_RX_EN_W { w: self }
    }
    #[doc = "Bits 10:11 - no description available"]
    #[inline(always)]
    pub fn tx_fifo_ctrl(&mut self) -> TXFIFO_CTRL_W {
        TXFIFO_CTRL_W { w: self }
    }
    #[doc = "Bit 12 - When write 1 to this bit, it will cause SPDIF software reset"]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 13 - When write 1 to this bit, it will cause SPDIF enter low-power mode"]
    #[inline(always)]
    pub fn low_power(&mut self) -> LOW_POWER_W {
        LOW_POWER_W { w: self }
    }
    #[doc = "Bits 15:16 - no description available"]
    #[inline(always)]
    pub fn tx_fifoempty_sel(&mut self) -> TXFIFOEMPTY_SEL_W {
        TXFIFOEMPTY_SEL_W { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn tx_auto_sync(&mut self) -> TXAUTOSYNC_W {
        TXAUTOSYNC_W { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn rx_auto_sync(&mut self) -> RXAUTOSYNC_W {
        RXAUTOSYNC_W { w: self }
    }
    #[doc = "Bits 19:20 - no description available"]
    #[inline(always)]
    pub fn rx_fifofull_sel(&mut self) -> RXFIFOFULL_SEL_W {
        RXFIFOFULL_SEL_W { w: self }
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RXFIFO_RST_W {
        RXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_off_on(&mut self) -> RXFIFO_OFF_ON_W {
        RXFIFO_OFF_ON_W { w: self }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn rx_fifo_ctrl(&mut self) -> RXFIFO_CTRL_W {
        RXFIFO_CTRL_W { w: self }
    }
}
