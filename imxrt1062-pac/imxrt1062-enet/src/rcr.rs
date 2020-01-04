#[doc = "Reader of register RCR"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register RCR"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register RCR `reset()`'s with value 0x05ee_0001"]
impl crate::ResetValue for super::RCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05ee_0001
    }
}
#[doc = "Internal Loopback\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_A {
    #[doc = "0: Loopback disabled."]
    LOOP_0 = 0,
    #[doc = "1: Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    LOOP_1 = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOP`"]
pub type LOOP_R = crate::R<bool, LOOP_A>;
impl LOOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::LOOP_0,
            true => LOOP_A::LOOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline(always)]
    pub fn is_loop_0(&self) -> bool {
        *self == LOOP_A::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline(always)]
    pub fn is_loop_1(&self) -> bool {
        *self == LOOP_A::LOOP_1
    }
}
#[doc = "Write proxy for field `LOOP`"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loopback disabled."]
    #[inline(always)]
    pub fn loop_0(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_0)
    }
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    #[inline(always)]
    pub fn loop_1(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_1)
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
#[doc = "Disable Receive On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRT_A {
    #[doc = "0: Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    DRT_0 = 0,
    #[doc = "1: Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    DRT_1 = 1,
}
impl From<DRT_A> for bool {
    #[inline(always)]
    fn from(variant: DRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRT`"]
pub type DRT_R = crate::R<bool, DRT_A>;
impl DRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRT_A {
        match self.bits {
            false => DRT_A::DRT_0,
            true => DRT_A::DRT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRT_0`"]
    #[inline(always)]
    pub fn is_drt_0(&self) -> bool {
        *self == DRT_A::DRT_0
    }
    #[doc = "Checks if the value of the field is `DRT_1`"]
    #[inline(always)]
    pub fn is_drt_1(&self) -> bool {
        *self == DRT_A::DRT_1
    }
}
#[doc = "Write proxy for field `DRT`"]
pub struct DRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    #[inline(always)]
    pub fn drt_0(self) -> &'a mut W {
        self.variant(DRT_A::DRT_0)
    }
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    #[inline(always)]
    pub fn drt_1(self) -> &'a mut W {
        self.variant(DRT_A::DRT_1)
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
#[doc = "Media Independent Interface Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MII_MODE_A {
    #[doc = "1: MII or RMII mode, as indicated by the RMII_MODE field."]
    MII_MODE_1 = 1,
}
impl From<MII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MII_MODE`"]
pub type MII_MODE_R = crate::R<bool, MII_MODE_A>;
impl MII_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MII_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MII_MODE_A::MII_MODE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MII_MODE_1`"]
    #[inline(always)]
    pub fn is_mii_mode_1(&self) -> bool {
        *self == MII_MODE_A::MII_MODE_1
    }
}
#[doc = "Write proxy for field `MII_MODE`"]
pub struct MII_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MII_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    #[inline(always)]
    pub fn mii_mode_1(self) -> &'a mut W {
        self.variant(MII_MODE_A::MII_MODE_1)
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
#[doc = "Promiscuous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROM_A {
    #[doc = "0: Disabled."]
    PROM_0 = 0,
    #[doc = "1: Enabled."]
    PROM_1 = 1,
}
impl From<PROM_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROM`"]
pub type PROM_R = crate::R<bool, PROM_A>;
impl PROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_A {
        match self.bits {
            false => PROM_A::PROM_0,
            true => PROM_A::PROM_1,
        }
    }
    #[doc = "Checks if the value of the field is `PROM_0`"]
    #[inline(always)]
    pub fn is_prom_0(&self) -> bool {
        *self == PROM_A::PROM_0
    }
    #[doc = "Checks if the value of the field is `PROM_1`"]
    #[inline(always)]
    pub fn is_prom_1(&self) -> bool {
        *self == PROM_A::PROM_1
    }
}
#[doc = "Write proxy for field `PROM`"]
pub struct PROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn prom_0(self) -> &'a mut W {
        self.variant(PROM_A::PROM_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn prom_1(self) -> &'a mut W {
        self.variant(PROM_A::PROM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BC_REJ`"]
pub type BC_REJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BC_REJ`"]
pub struct BC_REJ_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_REJ_W<'a> {
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
#[doc = "Reader of field `FCE`"]
pub type FCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCE`"]
pub struct FCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCE_W<'a> {
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
#[doc = "RMII Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_MODE_A {
    #[doc = "0: MAC configured for MII mode."]
    RMII_MODE_0 = 0,
    #[doc = "1: MAC configured for RMII operation."]
    RMII_MODE_1 = 1,
}
impl From<RMII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RMII_MODE`"]
pub type RMII_MODE_R = crate::R<bool, RMII_MODE_A>;
impl RMII_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_MODE_A {
        match self.bits {
            false => RMII_MODE_A::RMII_MODE_0,
            true => RMII_MODE_A::RMII_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMII_MODE_0`"]
    #[inline(always)]
    pub fn is_rmii_mode_0(&self) -> bool {
        *self == RMII_MODE_A::RMII_MODE_0
    }
    #[doc = "Checks if the value of the field is `RMII_MODE_1`"]
    #[inline(always)]
    pub fn is_rmii_mode_1(&self) -> bool {
        *self == RMII_MODE_A::RMII_MODE_1
    }
}
#[doc = "Write proxy for field `RMII_MODE`"]
pub struct RMII_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMII_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MAC configured for MII mode."]
    #[inline(always)]
    pub fn rmii_mode_0(self) -> &'a mut W {
        self.variant(RMII_MODE_A::RMII_MODE_0)
    }
    #[doc = "MAC configured for RMII operation."]
    #[inline(always)]
    pub fn rmii_mode_1(self) -> &'a mut W {
        self.variant(RMII_MODE_A::RMII_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enables 10-Mbit/s mode of the RMII .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_10T_A {
    #[doc = "0: 100-Mbit/s operation."]
    RMII_10T_0 = 0,
    #[doc = "1: 10-Mbit/s operation."]
    RMII_10T_1 = 1,
}
impl From<RMII_10T_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_10T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RMII_10T`"]
pub type RMII_10T_R = crate::R<bool, RMII_10T_A>;
impl RMII_10T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_10T_A {
        match self.bits {
            false => RMII_10T_A::RMII_10T_0,
            true => RMII_10T_A::RMII_10T_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMII_10T_0`"]
    #[inline(always)]
    pub fn is_rmii_10t_0(&self) -> bool {
        *self == RMII_10T_A::RMII_10T_0
    }
    #[doc = "Checks if the value of the field is `RMII_10T_1`"]
    #[inline(always)]
    pub fn is_rmii_10t_1(&self) -> bool {
        *self == RMII_10T_A::RMII_10T_1
    }
}
#[doc = "Write proxy for field `RMII_10T`"]
pub struct RMII_10T_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_10T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMII_10T_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "100-Mbit/s operation."]
    #[inline(always)]
    pub fn rmii_10t_0(self) -> &'a mut W {
        self.variant(RMII_10T_A::RMII_10T_0)
    }
    #[doc = "10-Mbit/s operation."]
    #[inline(always)]
    pub fn rmii_10t_1(self) -> &'a mut W {
        self.variant(RMII_10T_A::RMII_10T_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable Frame Padding Remove On Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADEN_A {
    #[doc = "0: No padding is removed on receive by the MAC."]
    PADEN_0 = 0,
    #[doc = "1: Padding is removed from received frames."]
    PADEN_1 = 1,
}
impl From<PADEN_A> for bool {
    #[inline(always)]
    fn from(variant: PADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADEN`"]
pub type PADEN_R = crate::R<bool, PADEN_A>;
impl PADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADEN_A {
        match self.bits {
            false => PADEN_A::PADEN_0,
            true => PADEN_A::PADEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PADEN_0`"]
    #[inline(always)]
    pub fn is_paden_0(&self) -> bool {
        *self == PADEN_A::PADEN_0
    }
    #[doc = "Checks if the value of the field is `PADEN_1`"]
    #[inline(always)]
    pub fn is_paden_1(&self) -> bool {
        *self == PADEN_A::PADEN_1
    }
}
#[doc = "Write proxy for field `PADEN`"]
pub struct PADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No padding is removed on receive by the MAC."]
    #[inline(always)]
    pub fn paden_0(self) -> &'a mut W {
        self.variant(PADEN_A::PADEN_0)
    }
    #[doc = "Padding is removed from received frames."]
    #[inline(always)]
    pub fn paden_1(self) -> &'a mut W {
        self.variant(PADEN_A::PADEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Terminate/Forward Pause Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUFWD_A {
    #[doc = "0: Pause frames are terminated and discarded in the MAC."]
    PAUFWD_0 = 0,
    #[doc = "1: Pause frames are forwarded to the user application."]
    PAUFWD_1 = 1,
}
impl From<PAUFWD_A> for bool {
    #[inline(always)]
    fn from(variant: PAUFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAUFWD`"]
pub type PAUFWD_R = crate::R<bool, PAUFWD_A>;
impl PAUFWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUFWD_A {
        match self.bits {
            false => PAUFWD_A::PAUFWD_0,
            true => PAUFWD_A::PAUFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUFWD_0`"]
    #[inline(always)]
    pub fn is_paufwd_0(&self) -> bool {
        *self == PAUFWD_A::PAUFWD_0
    }
    #[doc = "Checks if the value of the field is `PAUFWD_1`"]
    #[inline(always)]
    pub fn is_paufwd_1(&self) -> bool {
        *self == PAUFWD_A::PAUFWD_1
    }
}
#[doc = "Write proxy for field `PAUFWD`"]
pub struct PAUFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUFWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    #[inline(always)]
    pub fn paufwd_0(self) -> &'a mut W {
        self.variant(PAUFWD_A::PAUFWD_0)
    }
    #[doc = "Pause frames are forwarded to the user application."]
    #[inline(always)]
    pub fn paufwd_1(self) -> &'a mut W {
        self.variant(PAUFWD_A::PAUFWD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Terminate/Forward Received CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWD_A {
    #[doc = "0: The CRC field of received frames is transmitted to the user application."]
    CRCFWD_0 = 0,
    #[doc = "1: The CRC field is stripped from the frame."]
    CRCFWD_1 = 1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCFWD`"]
pub type CRCFWD_R = crate::R<bool, CRCFWD_A>;
impl CRCFWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCFWD_A {
        match self.bits {
            false => CRCFWD_A::CRCFWD_0,
            true => CRCFWD_A::CRCFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCFWD_0`"]
    #[inline(always)]
    pub fn is_crcfwd_0(&self) -> bool {
        *self == CRCFWD_A::CRCFWD_0
    }
    #[doc = "Checks if the value of the field is `CRCFWD_1`"]
    #[inline(always)]
    pub fn is_crcfwd_1(&self) -> bool {
        *self == CRCFWD_A::CRCFWD_1
    }
}
#[doc = "Write proxy for field `CRCFWD`"]
pub struct CRCFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCFWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    #[inline(always)]
    pub fn crcfwd_0(self) -> &'a mut W {
        self.variant(CRCFWD_A::CRCFWD_0)
    }
    #[doc = "The CRC field is stripped from the frame."]
    #[inline(always)]
    pub fn crcfwd_1(self) -> &'a mut W {
        self.variant(CRCFWD_A::CRCFWD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "MAC Control Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEN_A {
    #[doc = "0: MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    CFEN_0 = 0,
    #[doc = "1: MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    CFEN_1 = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFEN`"]
pub type CFEN_R = crate::R<bool, CFEN_A>;
impl CFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::CFEN_0,
            true => CFEN_A::CFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFEN_0`"]
    #[inline(always)]
    pub fn is_cfen_0(&self) -> bool {
        *self == CFEN_A::CFEN_0
    }
    #[doc = "Checks if the value of the field is `CFEN_1`"]
    #[inline(always)]
    pub fn is_cfen_1(&self) -> bool {
        *self == CFEN_A::CFEN_1
    }
}
#[doc = "Write proxy for field `CFEN`"]
pub struct CFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    #[inline(always)]
    pub fn cfen_0(self) -> &'a mut W {
        self.variant(CFEN_A::CFEN_0)
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    #[inline(always)]
    pub fn cfen_1(self) -> &'a mut W {
        self.variant(CFEN_A::CFEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MAX_FL`"]
pub type MAX_FL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_FL`"]
pub struct MAX_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Payload Length Check Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NLC_A {
    #[doc = "0: The payload length check is disabled."]
    NLC_0 = 0,
    #[doc = "1: The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    NLC_1 = 1,
}
impl From<NLC_A> for bool {
    #[inline(always)]
    fn from(variant: NLC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NLC`"]
pub type NLC_R = crate::R<bool, NLC_A>;
impl NLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NLC_A {
        match self.bits {
            false => NLC_A::NLC_0,
            true => NLC_A::NLC_1,
        }
    }
    #[doc = "Checks if the value of the field is `NLC_0`"]
    #[inline(always)]
    pub fn is_nlc_0(&self) -> bool {
        *self == NLC_A::NLC_0
    }
    #[doc = "Checks if the value of the field is `NLC_1`"]
    #[inline(always)]
    pub fn is_nlc_1(&self) -> bool {
        *self == NLC_A::NLC_1
    }
}
#[doc = "Write proxy for field `NLC`"]
pub struct NLC_W<'a> {
    w: &'a mut W,
}
impl<'a> NLC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NLC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The payload length check is disabled."]
    #[inline(always)]
    pub fn nlc_0(self) -> &'a mut W {
        self.variant(NLC_A::NLC_0)
    }
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    #[inline(always)]
    pub fn nlc_1(self) -> &'a mut W {
        self.variant(NLC_A::NLC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `GRS`"]
pub type GRS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    pub fn drt(&self) -> DRT_R {
        DRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    pub fn mii_mode(&self) -> MII_MODE_R {
        MII_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prom(&self) -> PROM_R {
        PROM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    pub fn bc_rej(&self) -> BC_REJ_R {
        BC_REJ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    pub fn rmii_mode(&self) -> RMII_MODE_R {
        RMII_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub fn rmii_10t(&self) -> RMII_10T_R {
        RMII_10T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub fn paden(&self) -> PADEN_R {
        PADEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub fn paufwd(&self) -> PAUFWD_R {
        PAUFWD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    pub fn max_fl(&self) -> MAX_FL_R {
        MAX_FL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    pub fn nlc(&self) -> NLC_R {
        NLC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Graceful Receive Stopped"]
    #[inline(always)]
    pub fn grs(&self) -> GRS_R {
        GRS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    pub fn drt(&mut self) -> DRT_W {
        DRT_W { w: self }
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    pub fn mii_mode(&mut self) -> MII_MODE_W {
        MII_MODE_W { w: self }
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prom(&mut self) -> PROM_W {
        PROM_W { w: self }
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    pub fn bc_rej(&mut self) -> BC_REJ_W {
        BC_REJ_W { w: self }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    pub fn fce(&mut self) -> FCE_W {
        FCE_W { w: self }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    pub fn rmii_mode(&mut self) -> RMII_MODE_W {
        RMII_MODE_W { w: self }
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub fn rmii_10t(&mut self) -> RMII_10T_W {
        RMII_10T_W { w: self }
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub fn paden(&mut self) -> PADEN_W {
        PADEN_W { w: self }
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub fn paufwd(&mut self) -> PAUFWD_W {
        PAUFWD_W { w: self }
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    pub fn crcfwd(&mut self) -> CRCFWD_W {
        CRCFWD_W { w: self }
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W {
        CFEN_W { w: self }
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    pub fn max_fl(&mut self) -> MAX_FL_W {
        MAX_FL_W { w: self }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    pub fn nlc(&mut self) -> NLC_W {
        NLC_W { w: self }
    }
}
