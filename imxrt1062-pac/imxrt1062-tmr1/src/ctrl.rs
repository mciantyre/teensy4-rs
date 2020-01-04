#[doc = "Reader of register CTRL%s"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL%s"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMODE_A {
    #[doc = "0: Asserted while counter is active"]
    OUTMODE_0 = 0,
    #[doc = "1: Clear OFLAG output on successful compare"]
    OUTMODE_1 = 1,
    #[doc = "2: Set OFLAG output on successful compare"]
    OUTMODE_2 = 2,
    #[doc = "3: Toggle OFLAG output on successful compare"]
    OUTMODE_3 = 3,
    #[doc = "4: Toggle OFLAG output using alternating compare registers"]
    OUTMODE_4 = 4,
    #[doc = "5: Set on compare, cleared on secondary source input edge"]
    OUTMODE_5 = 5,
    #[doc = "6: Set on compare, cleared on counter rollover"]
    OUTMODE_6 = 6,
    #[doc = "7: Enable gated clock output while counter is active"]
    OUTMODE_7 = 7,
}
impl From<OUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTMODE`"]
pub type OUTMODE_R = crate::R<u8, OUTMODE_A>;
impl OUTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMODE_A {
        match self.bits {
            0 => OUTMODE_A::OUTMODE_0,
            1 => OUTMODE_A::OUTMODE_1,
            2 => OUTMODE_A::OUTMODE_2,
            3 => OUTMODE_A::OUTMODE_3,
            4 => OUTMODE_A::OUTMODE_4,
            5 => OUTMODE_A::OUTMODE_5,
            6 => OUTMODE_A::OUTMODE_6,
            7 => OUTMODE_A::OUTMODE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMODE_0`"]
    #[inline(always)]
    pub fn is_outmode_0(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_0
    }
    #[doc = "Checks if the value of the field is `OUTMODE_1`"]
    #[inline(always)]
    pub fn is_outmode_1(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_1
    }
    #[doc = "Checks if the value of the field is `OUTMODE_2`"]
    #[inline(always)]
    pub fn is_outmode_2(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_2
    }
    #[doc = "Checks if the value of the field is `OUTMODE_3`"]
    #[inline(always)]
    pub fn is_outmode_3(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_3
    }
    #[doc = "Checks if the value of the field is `OUTMODE_4`"]
    #[inline(always)]
    pub fn is_outmode_4(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_4
    }
    #[doc = "Checks if the value of the field is `OUTMODE_5`"]
    #[inline(always)]
    pub fn is_outmode_5(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_5
    }
    #[doc = "Checks if the value of the field is `OUTMODE_6`"]
    #[inline(always)]
    pub fn is_outmode_6(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_6
    }
    #[doc = "Checks if the value of the field is `OUTMODE_7`"]
    #[inline(always)]
    pub fn is_outmode_7(&self) -> bool {
        *self == OUTMODE_A::OUTMODE_7
    }
}
#[doc = "Write proxy for field `OUTMODE`"]
pub struct OUTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Asserted while counter is active"]
    #[inline(always)]
    pub fn outmode_0(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_0)
    }
    #[doc = "Clear OFLAG output on successful compare"]
    #[inline(always)]
    pub fn outmode_1(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_1)
    }
    #[doc = "Set OFLAG output on successful compare"]
    #[inline(always)]
    pub fn outmode_2(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_2)
    }
    #[doc = "Toggle OFLAG output on successful compare"]
    #[inline(always)]
    pub fn outmode_3(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_3)
    }
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    #[inline(always)]
    pub fn outmode_4(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_4)
    }
    #[doc = "Set on compare, cleared on secondary source input edge"]
    #[inline(always)]
    pub fn outmode_5(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_5)
    }
    #[doc = "Set on compare, cleared on counter rollover"]
    #[inline(always)]
    pub fn outmode_6(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_6)
    }
    #[doc = "Enable gated clock output while counter is active"]
    #[inline(always)]
    pub fn outmode_7(self) -> &'a mut W {
        self.variant(OUTMODE_A::OUTMODE_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Co-Channel Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COINIT_A {
    #[doc = "0: Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    COINIT_0 = 0,
    #[doc = "1: Co-channel counter/timers may force a re-initialization of this counter/timer"]
    COINIT_1 = 1,
}
impl From<COINIT_A> for bool {
    #[inline(always)]
    fn from(variant: COINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COINIT`"]
pub type COINIT_R = crate::R<bool, COINIT_A>;
impl COINIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COINIT_A {
        match self.bits {
            false => COINIT_A::COINIT_0,
            true => COINIT_A::COINIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `COINIT_0`"]
    #[inline(always)]
    pub fn is_coinit_0(&self) -> bool {
        *self == COINIT_A::COINIT_0
    }
    #[doc = "Checks if the value of the field is `COINIT_1`"]
    #[inline(always)]
    pub fn is_coinit_1(&self) -> bool {
        *self == COINIT_A::COINIT_1
    }
}
#[doc = "Write proxy for field `COINIT`"]
pub struct COINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> COINIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COINIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn coinit_0(self) -> &'a mut W {
        self.variant(COINIT_A::COINIT_0)
    }
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn coinit_1(self) -> &'a mut W {
        self.variant(COINIT_A::COINIT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Count up."]
    DIR_0 = 0,
    #[doc = "1: Count down."]
    DIR_1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::DIR_0,
            true => DIR_A::DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIR_0`"]
    #[inline(always)]
    pub fn is_dir_0(&self) -> bool {
        *self == DIR_A::DIR_0
    }
    #[doc = "Checks if the value of the field is `DIR_1`"]
    #[inline(always)]
    pub fn is_dir_1(&self) -> bool {
        *self == DIR_A::DIR_1
    }
}
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count up."]
    #[inline(always)]
    pub fn dir_0(self) -> &'a mut W {
        self.variant(DIR_A::DIR_0)
    }
    #[doc = "Count down."]
    #[inline(always)]
    pub fn dir_1(self) -> &'a mut W {
        self.variant(DIR_A::DIR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Count Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTH_A {
    #[doc = "0: Count until roll over at $FFFF and continue from $0000."]
    LENGTH_0 = 0,
    #[doc = "1: Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    LENGTH_1 = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<bool, LENGTH_A>;
impl LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::LENGTH_0,
            true => LENGTH_A::LENGTH_1,
        }
    }
    #[doc = "Checks if the value of the field is `LENGTH_0`"]
    #[inline(always)]
    pub fn is_length_0(&self) -> bool {
        *self == LENGTH_A::LENGTH_0
    }
    #[doc = "Checks if the value of the field is `LENGTH_1`"]
    #[inline(always)]
    pub fn is_length_1(&self) -> bool {
        *self == LENGTH_A::LENGTH_1
    }
}
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LENGTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    #[inline(always)]
    pub fn length_0(self) -> &'a mut W {
        self.variant(LENGTH_A::LENGTH_0)
    }
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    #[inline(always)]
    pub fn length_1(self) -> &'a mut W {
        self.variant(LENGTH_A::LENGTH_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Count Once\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONCE_A {
    #[doc = "0: Count repeatedly."]
    ONCE_0 = 0,
    #[doc = "1: Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    ONCE_1 = 1,
}
impl From<ONCE_A> for bool {
    #[inline(always)]
    fn from(variant: ONCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONCE`"]
pub type ONCE_R = crate::R<bool, ONCE_A>;
impl ONCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONCE_A {
        match self.bits {
            false => ONCE_A::ONCE_0,
            true => ONCE_A::ONCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE_0`"]
    #[inline(always)]
    pub fn is_once_0(&self) -> bool {
        *self == ONCE_A::ONCE_0
    }
    #[doc = "Checks if the value of the field is `ONCE_1`"]
    #[inline(always)]
    pub fn is_once_1(&self) -> bool {
        *self == ONCE_A::ONCE_1
    }
}
#[doc = "Write proxy for field `ONCE`"]
pub struct ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count repeatedly."]
    #[inline(always)]
    pub fn once_0(self) -> &'a mut W {
        self.variant(ONCE_A::ONCE_0)
    }
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    #[inline(always)]
    pub fn once_1(self) -> &'a mut W {
        self.variant(ONCE_A::ONCE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Secondary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "0: Counter 0 input pin"]
    SCS_0 = 0,
    #[doc = "1: Counter 1 input pin"]
    SCS_1 = 1,
    #[doc = "2: Counter 2 input pin"]
    SCS_2 = 2,
    #[doc = "3: Counter 3 input pin"]
    SCS_3 = 3,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            0 => SCS_A::SCS_0,
            1 => SCS_A::SCS_1,
            2 => SCS_A::SCS_2,
            3 => SCS_A::SCS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SCS_0`"]
    #[inline(always)]
    pub fn is_scs_0(&self) -> bool {
        *self == SCS_A::SCS_0
    }
    #[doc = "Checks if the value of the field is `SCS_1`"]
    #[inline(always)]
    pub fn is_scs_1(&self) -> bool {
        *self == SCS_A::SCS_1
    }
    #[doc = "Checks if the value of the field is `SCS_2`"]
    #[inline(always)]
    pub fn is_scs_2(&self) -> bool {
        *self == SCS_A::SCS_2
    }
    #[doc = "Checks if the value of the field is `SCS_3`"]
    #[inline(always)]
    pub fn is_scs_3(&self) -> bool {
        *self == SCS_A::SCS_3
    }
}
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn scs_0(self) -> &'a mut W {
        self.variant(SCS_A::SCS_0)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn scs_1(self) -> &'a mut W {
        self.variant(SCS_A::SCS_1)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn scs_2(self) -> &'a mut W {
        self.variant(SCS_A::SCS_2)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn scs_3(self) -> &'a mut W {
        self.variant(SCS_A::SCS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u16) & 0x03) << 7);
        self.w
    }
}
#[doc = "Primary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Counter 0 input pin"]
    PCS_0 = 0,
    #[doc = "1: Counter 1 input pin"]
    PCS_1 = 1,
    #[doc = "2: Counter 2 input pin"]
    PCS_2 = 2,
    #[doc = "3: Counter 3 input pin"]
    PCS_3 = 3,
    #[doc = "4: Counter 0 output"]
    PCS_4 = 4,
    #[doc = "5: Counter 1 output"]
    PCS_5 = 5,
    #[doc = "6: Counter 2 output"]
    PCS_6 = 6,
    #[doc = "7: Counter 3 output"]
    PCS_7 = 7,
    #[doc = "8: IP bus clock divide by 1 prescaler"]
    PCS_8 = 8,
    #[doc = "9: IP bus clock divide by 2 prescaler"]
    PCS_9 = 9,
    #[doc = "10: IP bus clock divide by 4 prescaler"]
    PCS_10 = 10,
    #[doc = "11: IP bus clock divide by 8 prescaler"]
    PCS_11 = 11,
    #[doc = "12: IP bus clock divide by 16 prescaler"]
    PCS_12 = 12,
    #[doc = "13: IP bus clock divide by 32 prescaler"]
    PCS_13 = 13,
    #[doc = "14: IP bus clock divide by 64 prescaler"]
    PCS_14 = 14,
    #[doc = "15: IP bus clock divide by 128 prescaler"]
    PCS_15 = 15,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::PCS_0,
            1 => PCS_A::PCS_1,
            2 => PCS_A::PCS_2,
            3 => PCS_A::PCS_3,
            4 => PCS_A::PCS_4,
            5 => PCS_A::PCS_5,
            6 => PCS_A::PCS_6,
            7 => PCS_A::PCS_7,
            8 => PCS_A::PCS_8,
            9 => PCS_A::PCS_9,
            10 => PCS_A::PCS_10,
            11 => PCS_A::PCS_11,
            12 => PCS_A::PCS_12,
            13 => PCS_A::PCS_13,
            14 => PCS_A::PCS_14,
            15 => PCS_A::PCS_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCS_0`"]
    #[inline(always)]
    pub fn is_pcs_0(&self) -> bool {
        *self == PCS_A::PCS_0
    }
    #[doc = "Checks if the value of the field is `PCS_1`"]
    #[inline(always)]
    pub fn is_pcs_1(&self) -> bool {
        *self == PCS_A::PCS_1
    }
    #[doc = "Checks if the value of the field is `PCS_2`"]
    #[inline(always)]
    pub fn is_pcs_2(&self) -> bool {
        *self == PCS_A::PCS_2
    }
    #[doc = "Checks if the value of the field is `PCS_3`"]
    #[inline(always)]
    pub fn is_pcs_3(&self) -> bool {
        *self == PCS_A::PCS_3
    }
    #[doc = "Checks if the value of the field is `PCS_4`"]
    #[inline(always)]
    pub fn is_pcs_4(&self) -> bool {
        *self == PCS_A::PCS_4
    }
    #[doc = "Checks if the value of the field is `PCS_5`"]
    #[inline(always)]
    pub fn is_pcs_5(&self) -> bool {
        *self == PCS_A::PCS_5
    }
    #[doc = "Checks if the value of the field is `PCS_6`"]
    #[inline(always)]
    pub fn is_pcs_6(&self) -> bool {
        *self == PCS_A::PCS_6
    }
    #[doc = "Checks if the value of the field is `PCS_7`"]
    #[inline(always)]
    pub fn is_pcs_7(&self) -> bool {
        *self == PCS_A::PCS_7
    }
    #[doc = "Checks if the value of the field is `PCS_8`"]
    #[inline(always)]
    pub fn is_pcs_8(&self) -> bool {
        *self == PCS_A::PCS_8
    }
    #[doc = "Checks if the value of the field is `PCS_9`"]
    #[inline(always)]
    pub fn is_pcs_9(&self) -> bool {
        *self == PCS_A::PCS_9
    }
    #[doc = "Checks if the value of the field is `PCS_10`"]
    #[inline(always)]
    pub fn is_pcs_10(&self) -> bool {
        *self == PCS_A::PCS_10
    }
    #[doc = "Checks if the value of the field is `PCS_11`"]
    #[inline(always)]
    pub fn is_pcs_11(&self) -> bool {
        *self == PCS_A::PCS_11
    }
    #[doc = "Checks if the value of the field is `PCS_12`"]
    #[inline(always)]
    pub fn is_pcs_12(&self) -> bool {
        *self == PCS_A::PCS_12
    }
    #[doc = "Checks if the value of the field is `PCS_13`"]
    #[inline(always)]
    pub fn is_pcs_13(&self) -> bool {
        *self == PCS_A::PCS_13
    }
    #[doc = "Checks if the value of the field is `PCS_14`"]
    #[inline(always)]
    pub fn is_pcs_14(&self) -> bool {
        *self == PCS_A::PCS_14
    }
    #[doc = "Checks if the value of the field is `PCS_15`"]
    #[inline(always)]
    pub fn is_pcs_15(&self) -> bool {
        *self == PCS_A::PCS_15
    }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn pcs_0(self) -> &'a mut W {
        self.variant(PCS_A::PCS_0)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn pcs_1(self) -> &'a mut W {
        self.variant(PCS_A::PCS_1)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn pcs_2(self) -> &'a mut W {
        self.variant(PCS_A::PCS_2)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn pcs_3(self) -> &'a mut W {
        self.variant(PCS_A::PCS_3)
    }
    #[doc = "Counter 0 output"]
    #[inline(always)]
    pub fn pcs_4(self) -> &'a mut W {
        self.variant(PCS_A::PCS_4)
    }
    #[doc = "Counter 1 output"]
    #[inline(always)]
    pub fn pcs_5(self) -> &'a mut W {
        self.variant(PCS_A::PCS_5)
    }
    #[doc = "Counter 2 output"]
    #[inline(always)]
    pub fn pcs_6(self) -> &'a mut W {
        self.variant(PCS_A::PCS_6)
    }
    #[doc = "Counter 3 output"]
    #[inline(always)]
    pub fn pcs_7(self) -> &'a mut W {
        self.variant(PCS_A::PCS_7)
    }
    #[doc = "IP bus clock divide by 1 prescaler"]
    #[inline(always)]
    pub fn pcs_8(self) -> &'a mut W {
        self.variant(PCS_A::PCS_8)
    }
    #[doc = "IP bus clock divide by 2 prescaler"]
    #[inline(always)]
    pub fn pcs_9(self) -> &'a mut W {
        self.variant(PCS_A::PCS_9)
    }
    #[doc = "IP bus clock divide by 4 prescaler"]
    #[inline(always)]
    pub fn pcs_10(self) -> &'a mut W {
        self.variant(PCS_A::PCS_10)
    }
    #[doc = "IP bus clock divide by 8 prescaler"]
    #[inline(always)]
    pub fn pcs_11(self) -> &'a mut W {
        self.variant(PCS_A::PCS_11)
    }
    #[doc = "IP bus clock divide by 16 prescaler"]
    #[inline(always)]
    pub fn pcs_12(self) -> &'a mut W {
        self.variant(PCS_A::PCS_12)
    }
    #[doc = "IP bus clock divide by 32 prescaler"]
    #[inline(always)]
    pub fn pcs_13(self) -> &'a mut W {
        self.variant(PCS_A::PCS_13)
    }
    #[doc = "IP bus clock divide by 64 prescaler"]
    #[inline(always)]
    pub fn pcs_14(self) -> &'a mut W {
        self.variant(PCS_A::PCS_14)
    }
    #[doc = "IP bus clock divide by 128 prescaler"]
    #[inline(always)]
    pub fn pcs_15(self) -> &'a mut W {
        self.variant(PCS_A::PCS_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u16) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No operation"]
    CM_0 = 0,
    #[doc = "1: Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    CM_1 = 1,
    #[doc = "2: Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    CM_2 = 2,
    #[doc = "3: Count rising edges of primary source while secondary input high active"]
    CM_3 = 3,
    #[doc = "4: Quadrature count mode, uses primary and secondary sources"]
    CM_4 = 4,
    #[doc = "5: Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    CM_5 = 5,
    #[doc = "6: Edge of secondary source triggers primary count until compare"]
    CM_6 = 6,
    #[doc = "7: Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CM_7 = 7,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::CM_0,
            1 => CM_A::CM_1,
            2 => CM_A::CM_2,
            3 => CM_A::CM_3,
            4 => CM_A::CM_4,
            5 => CM_A::CM_5,
            6 => CM_A::CM_6,
            7 => CM_A::CM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_1`"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == CM_A::CM_1
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
    #[doc = "Checks if the value of the field is `CM_4`"]
    #[inline(always)]
    pub fn is_cm_4(&self) -> bool {
        *self == CM_A::CM_4
    }
    #[doc = "Checks if the value of the field is `CM_5`"]
    #[inline(always)]
    pub fn is_cm_5(&self) -> bool {
        *self == CM_A::CM_5
    }
    #[doc = "Checks if the value of the field is `CM_6`"]
    #[inline(always)]
    pub fn is_cm_6(&self) -> bool {
        *self == CM_A::CM_6
    }
    #[doc = "Checks if the value of the field is `CM_7`"]
    #[inline(always)]
    pub fn is_cm_7(&self) -> bool {
        *self == CM_A::CM_7
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut W {
        self.variant(CM_A::CM_1)
    }
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Count rising edges of primary source while secondary input high active"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CM_A::CM_3)
    }
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    #[inline(always)]
    pub fn cm_4(self) -> &'a mut W {
        self.variant(CM_A::CM_4)
    }
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    #[inline(always)]
    pub fn cm_5(self) -> &'a mut W {
        self.variant(CM_A::CM_5)
    }
    #[doc = "Edge of secondary source triggers primary count until compare"]
    #[inline(always)]
    pub fn cm_6(self) -> &'a mut W {
        self.variant(CM_A::CM_6)
    }
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    #[inline(always)]
    pub fn cm_7(self) -> &'a mut W {
        self.variant(CM_A::CM_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    pub fn coinit(&self) -> COINIT_R {
        COINIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    pub fn once(&self) -> ONCE_R {
        ONCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OUTMODE_W {
        OUTMODE_W { w: self }
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    pub fn coinit(&mut self) -> COINIT_W {
        COINIT_W { w: self }
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    pub fn once(&mut self) -> ONCE_W {
        ONCE_W { w: self }
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
