#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Writer for register FIFO"]
pub type W = crate::W<u32, super::FIFO>;
#[doc = "Register FIFO `reset()`'s with value 0x00c0_0011"]
impl crate::ResetValue for super::FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00c0_0011
    }
}
#[doc = "Receive FIFO Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIFOSIZE_A {
    #[doc = "0: Receive FIFO/Buffer depth = 1 dataword."]
    RXFIFOSIZE_0 = 0,
    #[doc = "1: Receive FIFO/Buffer depth = 4 datawords."]
    RXFIFOSIZE_1 = 1,
    #[doc = "2: Receive FIFO/Buffer depth = 8 datawords."]
    RXFIFOSIZE_2 = 2,
    #[doc = "3: Receive FIFO/Buffer depth = 16 datawords."]
    RXFIFOSIZE_3 = 3,
    #[doc = "4: Receive FIFO/Buffer depth = 32 datawords."]
    RXFIFOSIZE_4 = 4,
    #[doc = "5: Receive FIFO/Buffer depth = 64 datawords."]
    RXFIFOSIZE_5 = 5,
    #[doc = "6: Receive FIFO/Buffer depth = 128 datawords."]
    RXFIFOSIZE_6 = 6,
    #[doc = "7: Receive FIFO/Buffer depth = 256 datawords."]
    RXFIFOSIZE_7 = 7,
}
impl From<RXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXFIFOSIZE`"]
pub type RXFIFOSIZE_R = crate::R<u8, RXFIFOSIZE_A>;
impl RXFIFOSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOSIZE_A {
        match self.bits {
            0 => RXFIFOSIZE_A::RXFIFOSIZE_0,
            1 => RXFIFOSIZE_A::RXFIFOSIZE_1,
            2 => RXFIFOSIZE_A::RXFIFOSIZE_2,
            3 => RXFIFOSIZE_A::RXFIFOSIZE_3,
            4 => RXFIFOSIZE_A::RXFIFOSIZE_4,
            5 => RXFIFOSIZE_A::RXFIFOSIZE_5,
            6 => RXFIFOSIZE_A::RXFIFOSIZE_6,
            7 => RXFIFOSIZE_A::RXFIFOSIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_0`"]
    #[inline(always)]
    pub fn is_rxfifosize_0(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_0
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_1`"]
    #[inline(always)]
    pub fn is_rxfifosize_1(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_2`"]
    #[inline(always)]
    pub fn is_rxfifosize_2(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_2
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_3`"]
    #[inline(always)]
    pub fn is_rxfifosize_3(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_3
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_4`"]
    #[inline(always)]
    pub fn is_rxfifosize_4(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_5`"]
    #[inline(always)]
    pub fn is_rxfifosize_5(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_5
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_6`"]
    #[inline(always)]
    pub fn is_rxfifosize_6(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_6
    }
    #[doc = "Checks if the value of the field is `RXFIFOSIZE_7`"]
    #[inline(always)]
    pub fn is_rxfifosize_7(&self) -> bool {
        *self == RXFIFOSIZE_A::RXFIFOSIZE_7
    }
}
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: Receive FIFO is not enabled. Buffer is depth 1."]
    RXFE_0 = 0,
    #[doc = "1: Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    RXFE_1 = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, RXFE_A>;
impl RXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::RXFE_0,
            true => RXFE_A::RXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFE_0`"]
    #[inline(always)]
    pub fn is_rxfe_0(&self) -> bool {
        *self == RXFE_A::RXFE_0
    }
    #[doc = "Checks if the value of the field is `RXFE_1`"]
    #[inline(always)]
    pub fn is_rxfe_1(&self) -> bool {
        *self == RXFE_A::RXFE_1
    }
}
#[doc = "Write proxy for field `RXFE`"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1."]
    #[inline(always)]
    pub fn rxfe_0(self) -> &'a mut W {
        self.variant(RXFE_A::RXFE_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline(always)]
    pub fn rxfe_1(self) -> &'a mut W {
        self.variant(RXFE_A::RXFE_1)
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
#[doc = "Transmit FIFO Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFOSIZE_A {
    #[doc = "0: Transmit FIFO/Buffer depth = 1 dataword."]
    TXFIFOSIZE_0 = 0,
    #[doc = "1: Transmit FIFO/Buffer depth = 4 datawords."]
    TXFIFOSIZE_1 = 1,
    #[doc = "2: Transmit FIFO/Buffer depth = 8 datawords."]
    TXFIFOSIZE_2 = 2,
    #[doc = "3: Transmit FIFO/Buffer depth = 16 datawords."]
    TXFIFOSIZE_3 = 3,
    #[doc = "4: Transmit FIFO/Buffer depth = 32 datawords."]
    TXFIFOSIZE_4 = 4,
    #[doc = "5: Transmit FIFO/Buffer depth = 64 datawords."]
    TXFIFOSIZE_5 = 5,
    #[doc = "6: Transmit FIFO/Buffer depth = 128 datawords."]
    TXFIFOSIZE_6 = 6,
    #[doc = "7: Transmit FIFO/Buffer depth = 256 datawords"]
    TXFIFOSIZE_7 = 7,
}
impl From<TXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXFIFOSIZE`"]
pub type TXFIFOSIZE_R = crate::R<u8, TXFIFOSIZE_A>;
impl TXFIFOSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOSIZE_A {
        match self.bits {
            0 => TXFIFOSIZE_A::TXFIFOSIZE_0,
            1 => TXFIFOSIZE_A::TXFIFOSIZE_1,
            2 => TXFIFOSIZE_A::TXFIFOSIZE_2,
            3 => TXFIFOSIZE_A::TXFIFOSIZE_3,
            4 => TXFIFOSIZE_A::TXFIFOSIZE_4,
            5 => TXFIFOSIZE_A::TXFIFOSIZE_5,
            6 => TXFIFOSIZE_A::TXFIFOSIZE_6,
            7 => TXFIFOSIZE_A::TXFIFOSIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_0`"]
    #[inline(always)]
    pub fn is_txfifosize_0(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_0
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_1`"]
    #[inline(always)]
    pub fn is_txfifosize_1(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_2`"]
    #[inline(always)]
    pub fn is_txfifosize_2(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_2
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_3`"]
    #[inline(always)]
    pub fn is_txfifosize_3(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_3
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_4`"]
    #[inline(always)]
    pub fn is_txfifosize_4(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_5`"]
    #[inline(always)]
    pub fn is_txfifosize_5(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_5
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_6`"]
    #[inline(always)]
    pub fn is_txfifosize_6(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_6
    }
    #[doc = "Checks if the value of the field is `TXFIFOSIZE_7`"]
    #[inline(always)]
    pub fn is_txfifosize_7(&self) -> bool {
        *self == TXFIFOSIZE_A::TXFIFOSIZE_7
    }
}
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "0: Transmit FIFO is not enabled. Buffer is depth 1."]
    TXFE_0 = 0,
    #[doc = "1: Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    TXFE_1 = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, TXFE_A>;
impl TXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::TXFE_0,
            true => TXFE_A::TXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXFE_0`"]
    #[inline(always)]
    pub fn is_txfe_0(&self) -> bool {
        *self == TXFE_A::TXFE_0
    }
    #[doc = "Checks if the value of the field is `TXFE_1`"]
    #[inline(always)]
    pub fn is_txfe_1(&self) -> bool {
        *self == TXFE_A::TXFE_1
    }
}
#[doc = "Write proxy for field `TXFE`"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1."]
    #[inline(always)]
    pub fn txfe_0(self) -> &'a mut W {
        self.variant(TXFE_A::TXFE_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline(always)]
    pub fn txfe_1(self) -> &'a mut W {
        self.variant(TXFE_A::TXFE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFE_A {
    #[doc = "0: RXUF flag does not generate an interrupt to the host."]
    RXUFE_0 = 0,
    #[doc = "1: RXUF flag generates an interrupt to the host."]
    RXUFE_1 = 1,
}
impl From<RXUFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXUFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXUFE`"]
pub type RXUFE_R = crate::R<bool, RXUFE_A>;
impl RXUFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUFE_A {
        match self.bits {
            false => RXUFE_A::RXUFE_0,
            true => RXUFE_A::RXUFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXUFE_0`"]
    #[inline(always)]
    pub fn is_rxufe_0(&self) -> bool {
        *self == RXUFE_A::RXUFE_0
    }
    #[doc = "Checks if the value of the field is `RXUFE_1`"]
    #[inline(always)]
    pub fn is_rxufe_1(&self) -> bool {
        *self == RXUFE_A::RXUFE_1
    }
}
#[doc = "Write proxy for field `RXUFE`"]
pub struct RXUFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn rxufe_0(self) -> &'a mut W {
        self.variant(RXUFE_A::RXUFE_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn rxufe_1(self) -> &'a mut W {
        self.variant(RXUFE_A::RXUFE_1)
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
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFE_A {
    #[doc = "0: TXOF flag does not generate an interrupt to the host."]
    TXOFE_0 = 0,
    #[doc = "1: TXOF flag generates an interrupt to the host."]
    TXOFE_1 = 1,
}
impl From<TXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXOFE`"]
pub type TXOFE_R = crate::R<bool, TXOFE_A>;
impl TXOFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFE_A {
        match self.bits {
            false => TXOFE_A::TXOFE_0,
            true => TXOFE_A::TXOFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXOFE_0`"]
    #[inline(always)]
    pub fn is_txofe_0(&self) -> bool {
        *self == TXOFE_A::TXOFE_0
    }
    #[doc = "Checks if the value of the field is `TXOFE_1`"]
    #[inline(always)]
    pub fn is_txofe_1(&self) -> bool {
        *self == TXOFE_A::TXOFE_1
    }
}
#[doc = "Write proxy for field `TXOFE`"]
pub struct TXOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn txofe_0(self) -> &'a mut W {
        self.variant(TXOFE_A::TXOFE_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn txofe_1(self) -> &'a mut W {
        self.variant(TXOFE_A::TXOFE_1)
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
#[doc = "Receiver Idle Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXIDEN_A {
    #[doc = "0: Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    RXIDEN_0 = 0,
    #[doc = "1: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    RXIDEN_1 = 1,
    #[doc = "2: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    RXIDEN_2 = 2,
    #[doc = "3: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    RXIDEN_3 = 3,
    #[doc = "4: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    RXIDEN_4 = 4,
    #[doc = "5: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    RXIDEN_5 = 5,
    #[doc = "6: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    RXIDEN_6 = 6,
    #[doc = "7: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    RXIDEN_7 = 7,
}
impl From<RXIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXIDEN`"]
pub type RXIDEN_R = crate::R<u8, RXIDEN_A>;
impl RXIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDEN_A {
        match self.bits {
            0 => RXIDEN_A::RXIDEN_0,
            1 => RXIDEN_A::RXIDEN_1,
            2 => RXIDEN_A::RXIDEN_2,
            3 => RXIDEN_A::RXIDEN_3,
            4 => RXIDEN_A::RXIDEN_4,
            5 => RXIDEN_A::RXIDEN_5,
            6 => RXIDEN_A::RXIDEN_6,
            7 => RXIDEN_A::RXIDEN_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXIDEN_0`"]
    #[inline(always)]
    pub fn is_rxiden_0(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_0
    }
    #[doc = "Checks if the value of the field is `RXIDEN_1`"]
    #[inline(always)]
    pub fn is_rxiden_1(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_1
    }
    #[doc = "Checks if the value of the field is `RXIDEN_2`"]
    #[inline(always)]
    pub fn is_rxiden_2(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_2
    }
    #[doc = "Checks if the value of the field is `RXIDEN_3`"]
    #[inline(always)]
    pub fn is_rxiden_3(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_3
    }
    #[doc = "Checks if the value of the field is `RXIDEN_4`"]
    #[inline(always)]
    pub fn is_rxiden_4(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_4
    }
    #[doc = "Checks if the value of the field is `RXIDEN_5`"]
    #[inline(always)]
    pub fn is_rxiden_5(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_5
    }
    #[doc = "Checks if the value of the field is `RXIDEN_6`"]
    #[inline(always)]
    pub fn is_rxiden_6(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_6
    }
    #[doc = "Checks if the value of the field is `RXIDEN_7`"]
    #[inline(always)]
    pub fn is_rxiden_7(&self) -> bool {
        *self == RXIDEN_A::RXIDEN_7
    }
}
#[doc = "Write proxy for field `RXIDEN`"]
pub struct RXIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIDEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    #[inline(always)]
    pub fn rxiden_0(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_0)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    #[inline(always)]
    pub fn rxiden_1(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_1)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    #[inline(always)]
    pub fn rxiden_2(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_2)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    #[inline(always)]
    pub fn rxiden_3(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_3)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    #[inline(always)]
    pub fn rxiden_4(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_4)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    #[inline(always)]
    pub fn rxiden_5(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_5)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    #[inline(always)]
    pub fn rxiden_6(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_6)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    #[inline(always)]
    pub fn rxiden_7(self) -> &'a mut W {
        self.variant(RXIDEN_A::RXIDEN_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Receive FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFLUSH_A {
    #[doc = "0: No flush operation occurs."]
    RXFLUSH_0 = 0,
    #[doc = "1: All data in the receive FIFO/buffer is cleared out."]
    RXFLUSH_1 = 1,
}
impl From<RXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFLUSH`"]
pub type RXFLUSH_R = crate::R<bool, RXFLUSH_A>;
impl RXFLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFLUSH_A {
        match self.bits {
            false => RXFLUSH_A::RXFLUSH_0,
            true => RXFLUSH_A::RXFLUSH_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFLUSH_0`"]
    #[inline(always)]
    pub fn is_rxflush_0(&self) -> bool {
        *self == RXFLUSH_A::RXFLUSH_0
    }
    #[doc = "Checks if the value of the field is `RXFLUSH_1`"]
    #[inline(always)]
    pub fn is_rxflush_1(&self) -> bool {
        *self == RXFLUSH_A::RXFLUSH_1
    }
}
#[doc = "Write proxy for field `RXFLUSH`"]
pub struct RXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn rxflush_0(self) -> &'a mut W {
        self.variant(RXFLUSH_A::RXFLUSH_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline(always)]
    pub fn rxflush_1(self) -> &'a mut W {
        self.variant(RXFLUSH_A::RXFLUSH_1)
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
#[doc = "Transmit FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFLUSH_A {
    #[doc = "0: No flush operation occurs."]
    TXFLUSH_0 = 0,
    #[doc = "1: All data in the transmit FIFO/Buffer is cleared out."]
    TXFLUSH_1 = 1,
}
impl From<TXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFLUSH`"]
pub type TXFLUSH_R = crate::R<bool, TXFLUSH_A>;
impl TXFLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFLUSH_A {
        match self.bits {
            false => TXFLUSH_A::TXFLUSH_0,
            true => TXFLUSH_A::TXFLUSH_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXFLUSH_0`"]
    #[inline(always)]
    pub fn is_txflush_0(&self) -> bool {
        *self == TXFLUSH_A::TXFLUSH_0
    }
    #[doc = "Checks if the value of the field is `TXFLUSH_1`"]
    #[inline(always)]
    pub fn is_txflush_1(&self) -> bool {
        *self == TXFLUSH_A::TXFLUSH_1
    }
}
#[doc = "Write proxy for field `TXFLUSH`"]
pub struct TXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn txflush_0(self) -> &'a mut W {
        self.variant(TXFLUSH_A::TXFLUSH_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline(always)]
    pub fn txflush_1(self) -> &'a mut W {
        self.variant(TXFLUSH_A::TXFLUSH_1)
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
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUF_A {
    #[doc = "0: No receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_0 = 0,
    #[doc = "1: At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    RXUF_1 = 1,
}
impl From<RXUF_A> for bool {
    #[inline(always)]
    fn from(variant: RXUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, RXUF_A>;
impl RXUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUF_A {
        match self.bits {
            false => RXUF_A::RXUF_0,
            true => RXUF_A::RXUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXUF_0`"]
    #[inline(always)]
    pub fn is_rxuf_0(&self) -> bool {
        *self == RXUF_A::RXUF_0
    }
    #[doc = "Checks if the value of the field is `RXUF_1`"]
    #[inline(always)]
    pub fn is_rxuf_1(&self) -> bool {
        *self == RXUF_A::RXUF_1
    }
}
#[doc = "Write proxy for field `RXUF`"]
pub struct RXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn rxuf_0(self) -> &'a mut W {
        self.variant(RXUF_A::RXUF_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn rxuf_1(self) -> &'a mut W {
        self.variant(RXUF_A::RXUF_1)
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
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOF_A {
    #[doc = "0: No transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_0 = 0,
    #[doc = "1: At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    TXOF_1 = 1,
}
impl From<TXOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, TXOF_A>;
impl TXOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOF_A {
        match self.bits {
            false => TXOF_A::TXOF_0,
            true => TXOF_A::TXOF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXOF_0`"]
    #[inline(always)]
    pub fn is_txof_0(&self) -> bool {
        *self == TXOF_A::TXOF_0
    }
    #[doc = "Checks if the value of the field is `TXOF_1`"]
    #[inline(always)]
    pub fn is_txof_1(&self) -> bool {
        *self == TXOF_A::TXOF_1
    }
}
#[doc = "Write proxy for field `TXOF`"]
pub struct TXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn txof_0(self) -> &'a mut W {
        self.variant(TXOF_A::TXOF_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn txof_1(self) -> &'a mut W {
        self.variant(TXOF_A::TXOF_1)
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
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer is not empty."]
    RXEMPT_0 = 0,
    #[doc = "1: Receive buffer is empty."]
    RXEMPT_1 = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEMPT`"]
pub type RXEMPT_R = crate::R<bool, RXEMPT_A>;
impl RXEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::RXEMPT_0,
            true => RXEMPT_A::RXEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPT_0`"]
    #[inline(always)]
    pub fn is_rxempt_0(&self) -> bool {
        *self == RXEMPT_A::RXEMPT_0
    }
    #[doc = "Checks if the value of the field is `RXEMPT_1`"]
    #[inline(always)]
    pub fn is_rxempt_1(&self) -> bool {
        *self == RXEMPT_A::RXEMPT_1
    }
}
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPT_A {
    #[doc = "0: Transmit buffer is not empty."]
    TXEMPT_0 = 0,
    #[doc = "1: Transmit buffer is empty."]
    TXEMPT_1 = 1,
}
impl From<TXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEMPT`"]
pub type TXEMPT_R = crate::R<bool, TXEMPT_A>;
impl TXEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPT_A {
        match self.bits {
            false => TXEMPT_A::TXEMPT_0,
            true => TXEMPT_A::TXEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXEMPT_0`"]
    #[inline(always)]
    pub fn is_txempt_0(&self) -> bool {
        *self == TXEMPT_A::TXEMPT_0
    }
    #[doc = "Checks if the value of the field is `TXEMPT_1`"]
    #[inline(always)]
    pub fn is_txempt_1(&self) -> bool {
        *self == TXEMPT_A::TXEMPT_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Buffer Depth"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO Buffer Depth"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RXUFE_R {
        RXUFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TXOFE_R {
        TXOFE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    pub fn rxiden(&self) -> RXIDEN_R {
        RXIDEN_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 14 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn rxflush(&self) -> RXFLUSH_R {
        RXFLUSH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn txflush(&self) -> TXFLUSH_R {
        TXFLUSH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&mut self) -> RXUFE_W {
        RXUFE_W { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&mut self) -> TXOFE_W {
        TXOFE_W { w: self }
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    pub fn rxiden(&mut self) -> RXIDEN_W {
        RXIDEN_W { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RXFLUSH_W {
        RXFLUSH_W { w: self }
    }
    #[doc = "Bit 15 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn txflush(&mut self) -> TXFLUSH_W {
        TXFLUSH_W { w: self }
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
}
