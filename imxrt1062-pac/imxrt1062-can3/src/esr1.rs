#[doc = "Reader of register ESR1"]
pub type R = crate::R<u32, super::ESR1>;
#[doc = "Writer for register ESR1"]
pub type W = crate::W<u32, super::ESR1>;
#[doc = "Register ESR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wake-Up Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKINT_A {
    #[doc = "0: No such occurrence."]
    WAKINT_0 = 0,
    #[doc = "1: Indicates a recessive to dominant transition was received on the CAN bus."]
    WAKINT_1 = 1,
}
impl From<WAKINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKINT`"]
pub type WAKINT_R = crate::R<bool, WAKINT_A>;
impl WAKINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKINT_A {
        match self.bits {
            false => WAKINT_A::WAKINT_0,
            true => WAKINT_A::WAKINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKINT_0`"]
    #[inline(always)]
    pub fn is_wakint_0(&self) -> bool {
        *self == WAKINT_A::WAKINT_0
    }
    #[doc = "Checks if the value of the field is `WAKINT_1`"]
    #[inline(always)]
    pub fn is_wakint_1(&self) -> bool {
        *self == WAKINT_A::WAKINT_1
    }
}
#[doc = "Write proxy for field `WAKINT`"]
pub struct WAKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn wakint_0(self) -> &'a mut W {
        self.variant(WAKINT_A::WAKINT_0)
    }
    #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
    #[inline(always)]
    pub fn wakint_1(self) -> &'a mut W {
        self.variant(WAKINT_A::WAKINT_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_A {
    #[doc = "0: No such occurrence."]
    ERRINT_0 = 0,
    #[doc = "1: Indicates setting of any Error Bit in the Error and Status Register."]
    ERRINT_1 = 1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRINT`"]
pub type ERRINT_R = crate::R<bool, ERRINT_A>;
impl ERRINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::ERRINT_0,
            true => ERRINT_A::ERRINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRINT_0`"]
    #[inline(always)]
    pub fn is_errint_0(&self) -> bool {
        *self == ERRINT_A::ERRINT_0
    }
    #[doc = "Checks if the value of the field is `ERRINT_1`"]
    #[inline(always)]
    pub fn is_errint_1(&self) -> bool {
        *self == ERRINT_A::ERRINT_1
    }
}
#[doc = "Write proxy for field `ERRINT`"]
pub struct ERRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn errint_0(self) -> &'a mut W {
        self.variant(ERRINT_A::ERRINT_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    #[inline(always)]
    pub fn errint_1(self) -> &'a mut W {
        self.variant(ERRINT_A::ERRINT_1)
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
#[doc = "Bus Off Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFINT_A {
    #[doc = "0: No such occurrence."]
    BOFFINT_0 = 0,
    #[doc = "1: FlexCAN module entered Bus Off state."]
    BOFFINT_1 = 1,
}
impl From<BOFFINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOFFINT`"]
pub type BOFFINT_R = crate::R<bool, BOFFINT_A>;
impl BOFFINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFINT_A {
        match self.bits {
            false => BOFFINT_A::BOFFINT_0,
            true => BOFFINT_A::BOFFINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFINT_0`"]
    #[inline(always)]
    pub fn is_boffint_0(&self) -> bool {
        *self == BOFFINT_A::BOFFINT_0
    }
    #[doc = "Checks if the value of the field is `BOFFINT_1`"]
    #[inline(always)]
    pub fn is_boffint_1(&self) -> bool {
        *self == BOFFINT_A::BOFFINT_1
    }
}
#[doc = "Write proxy for field `BOFFINT`"]
pub struct BOFFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn boffint_0(self) -> &'a mut W {
        self.variant(BOFFINT_A::BOFFINT_0)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn boffint_1(self) -> &'a mut W {
        self.variant(BOFFINT_A::BOFFINT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "FlexCAN In Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: FlexCAN is not receiving a message."]
    RX_0 = 0,
    #[doc = "1: FlexCAN is receiving a message."]
    RX_1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::RX_0,
            true => RX_A::RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_0`"]
    #[inline(always)]
    pub fn is_rx_0(&self) -> bool {
        *self == RX_A::RX_0
    }
    #[doc = "Checks if the value of the field is `RX_1`"]
    #[inline(always)]
    pub fn is_rx_1(&self) -> bool {
        *self == RX_A::RX_1
    }
}
#[doc = "Fault Confinement State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTCONF_A {
    #[doc = "0: Error Active"]
    FLTCONF_0 = 0,
    #[doc = "1: Error Passive"]
    FLTCONF_1 = 1,
    #[doc = "2: Bus Off"]
    FLTCONF_2 = 2,
}
impl From<FLTCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTCONF`"]
pub type FLTCONF_R = crate::R<u8, FLTCONF_A>;
impl FLTCONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLTCONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLTCONF_A::FLTCONF_0),
            1 => Val(FLTCONF_A::FLTCONF_1),
            2 => Val(FLTCONF_A::FLTCONF_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLTCONF_0`"]
    #[inline(always)]
    pub fn is_fltconf_0(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_0
    }
    #[doc = "Checks if the value of the field is `FLTCONF_1`"]
    #[inline(always)]
    pub fn is_fltconf_1(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_1
    }
    #[doc = "Checks if the value of the field is `FLTCONF_2`"]
    #[inline(always)]
    pub fn is_fltconf_2(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_2
    }
}
#[doc = "FlexCAN In Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: FlexCAN is not transmitting a message."]
    TX_0 = 0,
    #[doc = "1: FlexCAN is transmitting a message."]
    TX_1 = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::TX_0,
            true => TX_A::TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_0`"]
    #[inline(always)]
    pub fn is_tx_0(&self) -> bool {
        *self == TX_A::TX_0
    }
    #[doc = "Checks if the value of the field is `TX_1`"]
    #[inline(always)]
    pub fn is_tx_1(&self) -> bool {
        *self == TX_A::TX_1
    }
}
#[doc = "IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: No such occurrence."]
    IDLE_0 = 0,
    #[doc = "1: CAN bus is now IDLE."]
    IDLE_1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::IDLE_0,
            true => IDLE_A::IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_0`"]
    #[inline(always)]
    pub fn is_idle_0(&self) -> bool {
        *self == IDLE_A::IDLE_0
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline(always)]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLE_A::IDLE_1
    }
}
#[doc = "Rx Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWRN_A {
    #[doc = "0: No such occurrence."]
    RXWRN_0 = 0,
    #[doc = "1: RXERRCNT is greater than or equal to 96."]
    RXWRN_1 = 1,
}
impl From<RXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: RXWRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXWRN`"]
pub type RXWRN_R = crate::R<bool, RXWRN_A>;
impl RXWRN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWRN_A {
        match self.bits {
            false => RXWRN_A::RXWRN_0,
            true => RXWRN_A::RXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXWRN_0`"]
    #[inline(always)]
    pub fn is_rxwrn_0(&self) -> bool {
        *self == RXWRN_A::RXWRN_0
    }
    #[doc = "Checks if the value of the field is `RXWRN_1`"]
    #[inline(always)]
    pub fn is_rxwrn_1(&self) -> bool {
        *self == RXWRN_A::RXWRN_1
    }
}
#[doc = "TX Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXWRN_A {
    #[doc = "0: No such occurrence."]
    TXWRN_0 = 0,
    #[doc = "1: TXERRCNT is greater than or equal to 96."]
    TXWRN_1 = 1,
}
impl From<TXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: TXWRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXWRN`"]
pub type TXWRN_R = crate::R<bool, TXWRN_A>;
impl TXWRN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWRN_A {
        match self.bits {
            false => TXWRN_A::TXWRN_0,
            true => TXWRN_A::TXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXWRN_0`"]
    #[inline(always)]
    pub fn is_txwrn_0(&self) -> bool {
        *self == TXWRN_A::TXWRN_0
    }
    #[doc = "Checks if the value of the field is `TXWRN_1`"]
    #[inline(always)]
    pub fn is_txwrn_1(&self) -> bool {
        *self == TXWRN_A::TXWRN_1
    }
}
#[doc = "Stuffing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERR_A {
    #[doc = "0: No such occurrence."]
    STFERR_0 = 0,
    #[doc = "1: A Stuffing Error occurred since last read of this register."]
    STFERR_1 = 1,
}
impl From<STFERR_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STFERR`"]
pub type STFERR_R = crate::R<bool, STFERR_A>;
impl STFERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_A {
        match self.bits {
            false => STFERR_A::STFERR_0,
            true => STFERR_A::STFERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `STFERR_0`"]
    #[inline(always)]
    pub fn is_stferr_0(&self) -> bool {
        *self == STFERR_A::STFERR_0
    }
    #[doc = "Checks if the value of the field is `STFERR_1`"]
    #[inline(always)]
    pub fn is_stferr_1(&self) -> bool {
        *self == STFERR_A::STFERR_1
    }
}
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_A {
    #[doc = "0: No such occurrence."]
    FRMERR_0 = 0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    FRMERR_1 = 1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRMERR`"]
pub type FRMERR_R = crate::R<bool, FRMERR_A>;
impl FRMERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::FRMERR_0,
            true => FRMERR_A::FRMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMERR_0`"]
    #[inline(always)]
    pub fn is_frmerr_0(&self) -> bool {
        *self == FRMERR_A::FRMERR_0
    }
    #[doc = "Checks if the value of the field is `FRMERR_1`"]
    #[inline(always)]
    pub fn is_frmerr_1(&self) -> bool {
        *self == FRMERR_A::FRMERR_1
    }
}
#[doc = "Cyclic Redundancy Check Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: No such occurrence."]
    CRCERR_0 = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CRCERR_1 = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, CRCERR_A>;
impl CRCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::CRCERR_0,
            true => CRCERR_A::CRCERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERR_0`"]
    #[inline(always)]
    pub fn is_crcerr_0(&self) -> bool {
        *self == CRCERR_A::CRCERR_0
    }
    #[doc = "Checks if the value of the field is `CRCERR_1`"]
    #[inline(always)]
    pub fn is_crcerr_1(&self) -> bool {
        *self == CRCERR_A::CRCERR_1
    }
}
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKERR_A {
    #[doc = "0: No such occurrence."]
    ACKERR_0 = 0,
    #[doc = "1: An ACK error occurred since last read of this register."]
    ACKERR_1 = 1,
}
impl From<ACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACKERR`"]
pub type ACKERR_R = crate::R<bool, ACKERR_A>;
impl ACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKERR_A {
        match self.bits {
            false => ACKERR_A::ACKERR_0,
            true => ACKERR_A::ACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACKERR_0`"]
    #[inline(always)]
    pub fn is_ackerr_0(&self) -> bool {
        *self == ACKERR_A::ACKERR_0
    }
    #[doc = "Checks if the value of the field is `ACKERR_1`"]
    #[inline(always)]
    pub fn is_ackerr_1(&self) -> bool {
        *self == ACKERR_A::ACKERR_1
    }
}
#[doc = "Bit0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERR_A {
    #[doc = "0: No such occurrence."]
    BIT0ERR_0 = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    BIT0ERR_1 = 1,
}
impl From<BIT0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT0ERR`"]
pub type BIT0ERR_R = crate::R<bool, BIT0ERR_A>;
impl BIT0ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_A {
        match self.bits {
            false => BIT0ERR_A::BIT0ERR_0,
            true => BIT0ERR_A::BIT0ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_0`"]
    #[inline(always)]
    pub fn is_bit0err_0(&self) -> bool {
        *self == BIT0ERR_A::BIT0ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_1`"]
    #[inline(always)]
    pub fn is_bit0err_1(&self) -> bool {
        *self == BIT0ERR_A::BIT0ERR_1
    }
}
#[doc = "Bit1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERR_A {
    #[doc = "0: No such occurrence."]
    BIT1ERR_0 = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    BIT1ERR_1 = 1,
}
impl From<BIT1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT1ERR`"]
pub type BIT1ERR_R = crate::R<bool, BIT1ERR_A>;
impl BIT1ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_A {
        match self.bits {
            false => BIT1ERR_A::BIT1ERR_0,
            true => BIT1ERR_A::BIT1ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_0`"]
    #[inline(always)]
    pub fn is_bit1err_0(&self) -> bool {
        *self == BIT1ERR_A::BIT1ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_1`"]
    #[inline(always)]
    pub fn is_bit1err_1(&self) -> bool {
        *self == BIT1ERR_A::BIT1ERR_1
    }
}
#[doc = "Rx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNINT_A {
    #[doc = "0: No such occurrence."]
    RWRNINT_0 = 0,
    #[doc = "1: The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    RWRNINT_1 = 1,
}
impl From<RWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWRNINT`"]
pub type RWRNINT_R = crate::R<bool, RWRNINT_A>;
impl RWRNINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNINT_A {
        match self.bits {
            false => RWRNINT_A::RWRNINT_0,
            true => RWRNINT_A::RWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWRNINT_0`"]
    #[inline(always)]
    pub fn is_rwrnint_0(&self) -> bool {
        *self == RWRNINT_A::RWRNINT_0
    }
    #[doc = "Checks if the value of the field is `RWRNINT_1`"]
    #[inline(always)]
    pub fn is_rwrnint_1(&self) -> bool {
        *self == RWRNINT_A::RWRNINT_1
    }
}
#[doc = "Write proxy for field `RWRNINT`"]
pub struct RWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWRNINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn rwrnint_0(self) -> &'a mut W {
        self.variant(RWRNINT_A::RWRNINT_0)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn rwrnint_1(self) -> &'a mut W {
        self.variant(RWRNINT_A::RWRNINT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Tx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNINT_A {
    #[doc = "0: No such occurrence."]
    TWRNINT_0 = 0,
    #[doc = "1: The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    TWRNINT_1 = 1,
}
impl From<TWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TWRNINT`"]
pub type TWRNINT_R = crate::R<bool, TWRNINT_A>;
impl TWRNINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNINT_A {
        match self.bits {
            false => TWRNINT_A::TWRNINT_0,
            true => TWRNINT_A::TWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNINT_0`"]
    #[inline(always)]
    pub fn is_twrnint_0(&self) -> bool {
        *self == TWRNINT_A::TWRNINT_0
    }
    #[doc = "Checks if the value of the field is `TWRNINT_1`"]
    #[inline(always)]
    pub fn is_twrnint_1(&self) -> bool {
        *self == TWRNINT_A::TWRNINT_1
    }
}
#[doc = "Write proxy for field `TWRNINT`"]
pub struct TWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWRNINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn twrnint_0(self) -> &'a mut W {
        self.variant(TWRNINT_A::TWRNINT_0)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn twrnint_1(self) -> &'a mut W {
        self.variant(TWRNINT_A::TWRNINT_1)
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
#[doc = "CAN Synchronization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCH_A {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus."]
    SYNCH_0 = 0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus."]
    SYNCH_1 = 1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCH`"]
pub type SYNCH_R = crate::R<bool, SYNCH_A>;
impl SYNCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::SYNCH_0,
            true => SYNCH_A::SYNCH_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCH_0`"]
    #[inline(always)]
    pub fn is_synch_0(&self) -> bool {
        *self == SYNCH_A::SYNCH_0
    }
    #[doc = "Checks if the value of the field is `SYNCH_1`"]
    #[inline(always)]
    pub fn is_synch_1(&self) -> bool {
        *self == SYNCH_A::SYNCH_1
    }
}
#[doc = "Bus Off Done Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFDONEINT_A {
    #[doc = "0: No such occurrence."]
    BOFFDONEINT_0 = 0,
    #[doc = "1: FlexCAN module has completed Bus Off process."]
    BOFFDONEINT_1 = 1,
}
impl From<BOFFDONEINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFDONEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOFFDONEINT`"]
pub type BOFFDONEINT_R = crate::R<bool, BOFFDONEINT_A>;
impl BOFFDONEINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFDONEINT_A {
        match self.bits {
            false => BOFFDONEINT_A::BOFFDONEINT_0,
            true => BOFFDONEINT_A::BOFFDONEINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFDONEINT_0`"]
    #[inline(always)]
    pub fn is_boffdoneint_0(&self) -> bool {
        *self == BOFFDONEINT_A::BOFFDONEINT_0
    }
    #[doc = "Checks if the value of the field is `BOFFDONEINT_1`"]
    #[inline(always)]
    pub fn is_boffdoneint_1(&self) -> bool {
        *self == BOFFDONEINT_A::BOFFDONEINT_1
    }
}
#[doc = "Write proxy for field `BOFFDONEINT`"]
pub struct BOFFDONEINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFDONEINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFDONEINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn boffdoneint_0(self) -> &'a mut W {
        self.variant(BOFFDONEINT_A::BOFFDONEINT_0)
    }
    #[doc = "FlexCAN module has completed Bus Off process."]
    #[inline(always)]
    pub fn boffdoneint_1(self) -> &'a mut W {
        self.variant(BOFFDONEINT_A::BOFFDONEINT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Error Interrupt for errors detected in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_FAST_A {
    #[doc = "0: No such occurrence."]
    ERRINT_FAST_0 = 0,
    #[doc = "1: Indicates setting of any Error Bit detected in the Data Phase of CAN FD frames with the BRS bit set."]
    ERRINT_FAST_1 = 1,
}
impl From<ERRINT_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRINT_FAST`"]
pub type ERRINT_FAST_R = crate::R<bool, ERRINT_FAST_A>;
impl ERRINT_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_FAST_A {
        match self.bits {
            false => ERRINT_FAST_A::ERRINT_FAST_0,
            true => ERRINT_FAST_A::ERRINT_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRINT_FAST_0`"]
    #[inline(always)]
    pub fn is_errint_fast_0(&self) -> bool {
        *self == ERRINT_FAST_A::ERRINT_FAST_0
    }
    #[doc = "Checks if the value of the field is `ERRINT_FAST_1`"]
    #[inline(always)]
    pub fn is_errint_fast_1(&self) -> bool {
        *self == ERRINT_FAST_A::ERRINT_FAST_1
    }
}
#[doc = "Write proxy for field `ERRINT_FAST`"]
pub struct ERRINT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn errint_fast_0(self) -> &'a mut W {
        self.variant(ERRINT_FAST_A::ERRINT_FAST_0)
    }
    #[doc = "Indicates setting of any Error Bit detected in the Data Phase of CAN FD frames with the BRS bit set."]
    #[inline(always)]
    pub fn errint_fast_1(self) -> &'a mut W {
        self.variant(ERRINT_FAST_A::ERRINT_FAST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Error Overrun bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROVR_A {
    #[doc = "0: Overrun has not occurred."]
    ERROVR_0 = 0,
    #[doc = "1: Overrun has occurred."]
    ERROVR_1 = 1,
}
impl From<ERROVR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROVR`"]
pub type ERROVR_R = crate::R<bool, ERROVR_A>;
impl ERROVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROVR_A {
        match self.bits {
            false => ERROVR_A::ERROVR_0,
            true => ERROVR_A::ERROVR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERROVR_0`"]
    #[inline(always)]
    pub fn is_errovr_0(&self) -> bool {
        *self == ERROVR_A::ERROVR_0
    }
    #[doc = "Checks if the value of the field is `ERROVR_1`"]
    #[inline(always)]
    pub fn is_errovr_1(&self) -> bool {
        *self == ERROVR_A::ERROVR_1
    }
}
#[doc = "Write proxy for field `ERROVR`"]
pub struct ERROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overrun has not occurred."]
    #[inline(always)]
    pub fn errovr_0(self) -> &'a mut W {
        self.variant(ERROVR_A::ERROVR_0)
    }
    #[doc = "Overrun has occurred."]
    #[inline(always)]
    pub fn errovr_1(self) -> &'a mut W {
        self.variant(ERROVR_A::ERROVR_1)
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
#[doc = "Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERR_FAST_A {
    #[doc = "0: No such occurrence."]
    STFERR_FAST_0 = 0,
    #[doc = "1: A Stuffing Error occurred since last read of this register."]
    STFERR_FAST_1 = 1,
}
impl From<STFERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STFERR_FAST`"]
pub type STFERR_FAST_R = crate::R<bool, STFERR_FAST_A>;
impl STFERR_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_FAST_A {
        match self.bits {
            false => STFERR_FAST_A::STFERR_FAST_0,
            true => STFERR_FAST_A::STFERR_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `STFERR_FAST_0`"]
    #[inline(always)]
    pub fn is_stferr_fast_0(&self) -> bool {
        *self == STFERR_FAST_A::STFERR_FAST_0
    }
    #[doc = "Checks if the value of the field is `STFERR_FAST_1`"]
    #[inline(always)]
    pub fn is_stferr_fast_1(&self) -> bool {
        *self == STFERR_FAST_A::STFERR_FAST_1
    }
}
#[doc = "Form Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_FAST_A {
    #[doc = "0: No such occurrence."]
    FRMERR_FAST_0 = 0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    FRMERR_FAST_1 = 1,
}
impl From<FRMERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRMERR_FAST`"]
pub type FRMERR_FAST_R = crate::R<bool, FRMERR_FAST_A>;
impl FRMERR_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_FAST_A {
        match self.bits {
            false => FRMERR_FAST_A::FRMERR_FAST_0,
            true => FRMERR_FAST_A::FRMERR_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMERR_FAST_0`"]
    #[inline(always)]
    pub fn is_frmerr_fast_0(&self) -> bool {
        *self == FRMERR_FAST_A::FRMERR_FAST_0
    }
    #[doc = "Checks if the value of the field is `FRMERR_FAST_1`"]
    #[inline(always)]
    pub fn is_frmerr_fast_1(&self) -> bool {
        *self == FRMERR_FAST_A::FRMERR_FAST_1
    }
}
#[doc = "Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_FAST_A {
    #[doc = "0: No such occurrence."]
    CRCERR_FAST_0 = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CRCERR_FAST_1 = 1,
}
impl From<CRCERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERR_FAST`"]
pub type CRCERR_FAST_R = crate::R<bool, CRCERR_FAST_A>;
impl CRCERR_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_FAST_A {
        match self.bits {
            false => CRCERR_FAST_A::CRCERR_FAST_0,
            true => CRCERR_FAST_A::CRCERR_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERR_FAST_0`"]
    #[inline(always)]
    pub fn is_crcerr_fast_0(&self) -> bool {
        *self == CRCERR_FAST_A::CRCERR_FAST_0
    }
    #[doc = "Checks if the value of the field is `CRCERR_FAST_1`"]
    #[inline(always)]
    pub fn is_crcerr_fast_1(&self) -> bool {
        *self == CRCERR_FAST_A::CRCERR_FAST_1
    }
}
#[doc = "Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERR_FAST_A {
    #[doc = "0: No such occurrence."]
    BIT0ERR_FAST_0 = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    BIT0ERR_FAST_1 = 1,
}
impl From<BIT0ERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT0ERR_FAST`"]
pub type BIT0ERR_FAST_R = crate::R<bool, BIT0ERR_FAST_A>;
impl BIT0ERR_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_FAST_A {
        match self.bits {
            false => BIT0ERR_FAST_A::BIT0ERR_FAST_0,
            true => BIT0ERR_FAST_A::BIT0ERR_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_FAST_0`"]
    #[inline(always)]
    pub fn is_bit0err_fast_0(&self) -> bool {
        *self == BIT0ERR_FAST_A::BIT0ERR_FAST_0
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_FAST_1`"]
    #[inline(always)]
    pub fn is_bit0err_fast_1(&self) -> bool {
        *self == BIT0ERR_FAST_A::BIT0ERR_FAST_1
    }
}
#[doc = "Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERR_FAST_A {
    #[doc = "0: No such occurrence."]
    BIT1ERR_FAST_0 = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    BIT1ERR_FAST_1 = 1,
}
impl From<BIT1ERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT1ERR_FAST`"]
pub type BIT1ERR_FAST_R = crate::R<bool, BIT1ERR_FAST_A>;
impl BIT1ERR_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_FAST_A {
        match self.bits {
            false => BIT1ERR_FAST_A::BIT1ERR_FAST_0,
            true => BIT1ERR_FAST_A::BIT1ERR_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_FAST_0`"]
    #[inline(always)]
    pub fn is_bit1err_fast_0(&self) -> bool {
        *self == BIT1ERR_FAST_A::BIT1ERR_FAST_0
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_FAST_1`"]
    #[inline(always)]
    pub fn is_bit1err_fast_1(&self) -> bool {
        *self == BIT1ERR_FAST_A::BIT1ERR_FAST_1
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakint(&self) -> WAKINT_R {
        WAKINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&self) -> BOFFINT_R {
        BOFFINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline(always)]
    pub fn fltconf(&self) -> FLTCONF_R {
        FLTCONF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline(always)]
    pub fn rxwrn(&self) -> RXWRN_R {
        RXWRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline(always)]
    pub fn txwrn(&self) -> TXWRN_R {
        TXWRN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline(always)]
    pub fn stferr(&self) -> STFERR_R {
        STFERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline(always)]
    pub fn ackerr(&self) -> ACKERR_R {
        ACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline(always)]
    pub fn bit0err(&self) -> BIT0ERR_R {
        BIT0ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline(always)]
    pub fn bit1err(&self) -> BIT1ERR_R {
        BIT1ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RWRNINT_R {
        RWRNINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&self) -> TWRNINT_R {
        TWRNINT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&self) -> BOFFDONEINT_R {
        BOFFDONEINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Error Interrupt for errors detected in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&self) -> ERRINT_FAST_R {
        ERRINT_FAST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Error Overrun bit"]
    #[inline(always)]
    pub fn errovr(&self) -> ERROVR_R {
        ERROVR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn stferr_fast(&self) -> STFERR_FAST_R {
        STFERR_FAST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn frmerr_fast(&self) -> FRMERR_FAST_R {
        FRMERR_FAST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn crcerr_fast(&self) -> CRCERR_FAST_R {
        CRCERR_FAST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit0err_fast(&self) -> BIT0ERR_FAST_R {
        BIT0ERR_FAST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit1err_fast(&self) -> BIT1ERR_FAST_R {
        BIT1ERR_FAST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakint(&mut self) -> WAKINT_W {
        WAKINT_W { w: self }
    }
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ERRINT_W {
        ERRINT_W { w: self }
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&mut self) -> BOFFINT_W {
        BOFFINT_W { w: self }
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&mut self) -> RWRNINT_W {
        RWRNINT_W { w: self }
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&mut self) -> TWRNINT_W {
        TWRNINT_W { w: self }
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&mut self) -> BOFFDONEINT_W {
        BOFFDONEINT_W { w: self }
    }
    #[doc = "Bit 20 - Error Interrupt for errors detected in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&mut self) -> ERRINT_FAST_W {
        ERRINT_FAST_W { w: self }
    }
    #[doc = "Bit 21 - Error Overrun bit"]
    #[inline(always)]
    pub fn errovr(&mut self) -> ERROVR_W {
        ERROVR_W { w: self }
    }
}
