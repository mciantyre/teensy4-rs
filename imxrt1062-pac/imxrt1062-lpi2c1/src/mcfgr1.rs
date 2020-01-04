#[doc = "Reader of register MCFGR1"]
pub type R = crate::R<u32, super::MCFGR1>;
#[doc = "Writer for register MCFGR1"]
pub type W = crate::W<u32, super::MCFGR1>;
#[doc = "Register MCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide by 1"]
    PRESCALE_0 = 0,
    #[doc = "1: Divide by 2"]
    PRESCALE_1 = 1,
    #[doc = "2: Divide by 4"]
    PRESCALE_2 = 2,
    #[doc = "3: Divide by 8"]
    PRESCALE_3 = 3,
    #[doc = "4: Divide by 16"]
    PRESCALE_4 = 4,
    #[doc = "5: Divide by 32"]
    PRESCALE_5 = 5,
    #[doc = "6: Divide by 64"]
    PRESCALE_6 = 6,
    #[doc = "7: Divide by 128"]
    PRESCALE_7 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::PRESCALE_0,
            1 => PRESCALE_A::PRESCALE_1,
            2 => PRESCALE_A::PRESCALE_2,
            3 => PRESCALE_A::PRESCALE_3,
            4 => PRESCALE_A::PRESCALE_4,
            5 => PRESCALE_A::PRESCALE_5,
            6 => PRESCALE_A::PRESCALE_6,
            7 => PRESCALE_A::PRESCALE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_3`"]
    #[inline(always)]
    pub fn is_prescale_3(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_5`"]
    #[inline(always)]
    pub fn is_prescale_5(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `PRESCALE_6`"]
    #[inline(always)]
    pub fn is_prescale_6(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `PRESCALE_7`"]
    #[inline(always)]
    pub fn is_prescale_7(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_7
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_3)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_4)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_5)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_6)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Automatic STOP Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSTOP_A {
    #[doc = "0: No effect"]
    AUTOSTOP_0 = 0,
    #[doc = "1: STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    AUTOSTOP_1 = 1,
}
impl From<AUTOSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOSTOP`"]
pub type AUTOSTOP_R = crate::R<bool, AUTOSTOP_A>;
impl AUTOSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSTOP_A {
        match self.bits {
            false => AUTOSTOP_A::AUTOSTOP_0,
            true => AUTOSTOP_A::AUTOSTOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOSTOP_0`"]
    #[inline(always)]
    pub fn is_autostop_0(&self) -> bool {
        *self == AUTOSTOP_A::AUTOSTOP_0
    }
    #[doc = "Checks if the value of the field is `AUTOSTOP_1`"]
    #[inline(always)]
    pub fn is_autostop_1(&self) -> bool {
        *self == AUTOSTOP_A::AUTOSTOP_1
    }
}
#[doc = "Write proxy for field `AUTOSTOP`"]
pub struct AUTOSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn autostop_0(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::AUTOSTOP_0)
    }
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    #[inline(always)]
    pub fn autostop_1(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::AUTOSTOP_1)
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
#[doc = "IGNACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACK_A {
    #[doc = "0: LPI2C Master will receive ACK and NACK normally"]
    IGNACK_0 = 0,
    #[doc = "1: LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
    IGNACK_1 = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IGNACK`"]
pub type IGNACK_R = crate::R<bool, IGNACK_A>;
impl IGNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::IGNACK_0,
            true => IGNACK_A::IGNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IGNACK_0`"]
    #[inline(always)]
    pub fn is_ignack_0(&self) -> bool {
        *self == IGNACK_A::IGNACK_0
    }
    #[doc = "Checks if the value of the field is `IGNACK_1`"]
    #[inline(always)]
    pub fn is_ignack_1(&self) -> bool {
        *self == IGNACK_A::IGNACK_1
    }
}
#[doc = "Write proxy for field `IGNACK`"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPI2C Master will receive ACK and NACK normally"]
    #[inline(always)]
    pub fn ignack_0(self) -> &'a mut W {
        self.variant(IGNACK_A::IGNACK_0)
    }
    #[doc = "LPI2C Master will treat a received NACK as if it (NACK) was an ACK"]
    #[inline(always)]
    pub fn ignack_1(self) -> &'a mut W {
        self.variant(IGNACK_A::IGNACK_1)
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
#[doc = "Timeout Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMECFG_A {
    #[doc = "0: Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
    TIMECFG_0 = 0,
    #[doc = "1: Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
    TIMECFG_1 = 1,
}
impl From<TIMECFG_A> for bool {
    #[inline(always)]
    fn from(variant: TIMECFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMECFG`"]
pub type TIMECFG_R = crate::R<bool, TIMECFG_A>;
impl TIMECFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMECFG_A {
        match self.bits {
            false => TIMECFG_A::TIMECFG_0,
            true => TIMECFG_A::TIMECFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMECFG_0`"]
    #[inline(always)]
    pub fn is_timecfg_0(&self) -> bool {
        *self == TIMECFG_A::TIMECFG_0
    }
    #[doc = "Checks if the value of the field is `TIMECFG_1`"]
    #[inline(always)]
    pub fn is_timecfg_1(&self) -> bool {
        *self == TIMECFG_A::TIMECFG_1
    }
}
#[doc = "Write proxy for field `TIMECFG`"]
pub struct TIMECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMECFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout"]
    #[inline(always)]
    pub fn timecfg_0(self) -> &'a mut W {
        self.variant(TIMECFG_A::TIMECFG_0)
    }
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout"]
    #[inline(always)]
    pub fn timecfg_1(self) -> &'a mut W {
        self.variant(TIMECFG_A::TIMECFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match is disabled"]
    MATCFG_0 = 0,
    #[doc = "2: Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
    MATCFG_2 = 2,
    #[doc = "3: Match is enabled (any data word equals MATCH0 OR MATCH1)"]
    MATCFG_3 = 3,
    #[doc = "4: Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
    MATCFG_4 = 4,
    #[doc = "5: Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
    MATCFG_5 = 5,
    #[doc = "6: Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_6 = 6,
    #[doc = "7: Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    MATCFG_7 = 7,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MATCFG`"]
pub type MATCFG_R = crate::R<u8, MATCFG_A>;
impl MATCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MATCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MATCFG_A::MATCFG_0),
            2 => Val(MATCFG_A::MATCFG_2),
            3 => Val(MATCFG_A::MATCFG_3),
            4 => Val(MATCFG_A::MATCFG_4),
            5 => Val(MATCFG_A::MATCFG_5),
            6 => Val(MATCFG_A::MATCFG_6),
            7 => Val(MATCFG_A::MATCFG_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline(always)]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFG_A::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline(always)]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFG_A::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline(always)]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFG_A::MATCFG_3
    }
    #[doc = "Checks if the value of the field is `MATCFG_4`"]
    #[inline(always)]
    pub fn is_matcfg_4(&self) -> bool {
        *self == MATCFG_A::MATCFG_4
    }
    #[doc = "Checks if the value of the field is `MATCFG_5`"]
    #[inline(always)]
    pub fn is_matcfg_5(&self) -> bool {
        *self == MATCFG_A::MATCFG_5
    }
    #[doc = "Checks if the value of the field is `MATCFG_6`"]
    #[inline(always)]
    pub fn is_matcfg_6(&self) -> bool {
        *self == MATCFG_A::MATCFG_6
    }
    #[doc = "Checks if the value of the field is `MATCFG_7`"]
    #[inline(always)]
    pub fn is_matcfg_7(&self) -> bool {
        *self == MATCFG_A::MATCFG_7
    }
}
#[doc = "Write proxy for field `MATCFG`"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Match is disabled"]
    #[inline(always)]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_0)
    }
    #[doc = "Match is enabled (1st data word equals MATCH0 OR MATCH1)"]
    #[inline(always)]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_2)
    }
    #[doc = "Match is enabled (any data word equals MATCH0 OR MATCH1)"]
    #[inline(always)]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_3)
    }
    #[doc = "Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)"]
    #[inline(always)]
    pub fn matcfg_4(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_4)
    }
    #[doc = "Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)"]
    #[inline(always)]
    pub fn matcfg_5(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_5)
    }
    #[doc = "Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    #[inline(always)]
    pub fn matcfg_6(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_6)
    }
    #[doc = "Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)"]
    #[inline(always)]
    pub fn matcfg_7(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: 2-pin open drain mode"]
    PINCFG_0 = 0,
    #[doc = "1: 2-pin output only mode (ultra-fast mode)"]
    PINCFG_1 = 1,
    #[doc = "2: 2-pin push-pull mode"]
    PINCFG_2 = 2,
    #[doc = "3: 4-pin push-pull mode"]
    PINCFG_3 = 3,
    #[doc = "4: 2-pin open drain mode with separate LPI2C slave"]
    PINCFG_4 = 4,
    #[doc = "5: 2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    PINCFG_5 = 5,
    #[doc = "6: 2-pin push-pull mode with separate LPI2C slave"]
    PINCFG_6 = 6,
    #[doc = "7: 4-pin push-pull mode (inverted outputs)"]
    PINCFG_7 = 7,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINCFG`"]
pub type PINCFG_R = crate::R<u8, PINCFG_A>;
impl PINCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::PINCFG_0,
            1 => PINCFG_A::PINCFG_1,
            2 => PINCFG_A::PINCFG_2,
            3 => PINCFG_A::PINCFG_3,
            4 => PINCFG_A::PINCFG_4,
            5 => PINCFG_A::PINCFG_5,
            6 => PINCFG_A::PINCFG_6,
            7 => PINCFG_A::PINCFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline(always)]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFG_A::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline(always)]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFG_A::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline(always)]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFG_A::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline(always)]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFG_A::PINCFG_3
    }
    #[doc = "Checks if the value of the field is `PINCFG_4`"]
    #[inline(always)]
    pub fn is_pincfg_4(&self) -> bool {
        *self == PINCFG_A::PINCFG_4
    }
    #[doc = "Checks if the value of the field is `PINCFG_5`"]
    #[inline(always)]
    pub fn is_pincfg_5(&self) -> bool {
        *self == PINCFG_A::PINCFG_5
    }
    #[doc = "Checks if the value of the field is `PINCFG_6`"]
    #[inline(always)]
    pub fn is_pincfg_6(&self) -> bool {
        *self == PINCFG_A::PINCFG_6
    }
    #[doc = "Checks if the value of the field is `PINCFG_7`"]
    #[inline(always)]
    pub fn is_pincfg_7(&self) -> bool {
        *self == PINCFG_A::PINCFG_7
    }
}
#[doc = "Write proxy for field `PINCFG`"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2-pin open drain mode"]
    #[inline(always)]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_0)
    }
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    #[inline(always)]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_1)
    }
    #[doc = "2-pin push-pull mode"]
    #[inline(always)]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_2)
    }
    #[doc = "4-pin push-pull mode"]
    #[inline(always)]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_3)
    }
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    #[inline(always)]
    pub fn pincfg_4(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_4)
    }
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    #[inline(always)]
    pub fn pincfg_5(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_5)
    }
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    #[inline(always)]
    pub fn pincfg_6(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_6)
    }
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    #[inline(always)]
    pub fn pincfg_7(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    pub fn timecfg(&self) -> TIMECFG_R {
        TIMECFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W {
        AUTOSTOP_W { w: self }
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    pub fn timecfg(&mut self) -> TIMECFG_W {
        TIMECFG_W { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
}
