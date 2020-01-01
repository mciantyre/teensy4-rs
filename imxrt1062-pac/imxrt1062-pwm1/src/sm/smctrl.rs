#[doc = "Reader of register SMCTRL"]
pub type R = crate::R<u16, super::SMCTRL>;
#[doc = "Writer for register SMCTRL"]
pub type W = crate::W<u16, super::SMCTRL>;
#[doc = "Register SMCTRL `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::SMCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Double Switching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLEN_A {
    #[doc = "0: Double switching disabled."]
    DBLEN_0 = 0,
    #[doc = "1: Double switching enabled."]
    DBLEN_1 = 1,
}
impl From<DBLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBLEN`"]
pub type DBLEN_R = crate::R<bool, DBLEN_A>;
impl DBLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLEN_A {
        match self.bits {
            false => DBLEN_A::DBLEN_0,
            true => DBLEN_A::DBLEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBLEN_0`"]
    #[inline(always)]
    pub fn is_dblen_0(&self) -> bool {
        *self == DBLEN_A::DBLEN_0
    }
    #[doc = "Checks if the value of the field is `DBLEN_1`"]
    #[inline(always)]
    pub fn is_dblen_1(&self) -> bool {
        *self == DBLEN_A::DBLEN_1
    }
}
#[doc = "Write proxy for field `DBLEN`"]
pub struct DBLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Double switching disabled."]
    #[inline(always)]
    pub fn dblen_0(self) -> &'a mut W {
        self.variant(DBLEN_A::DBLEN_0)
    }
    #[doc = "Double switching enabled."]
    #[inline(always)]
    pub fn dblen_1(self) -> &'a mut W {
        self.variant(DBLEN_A::DBLEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "PWMX Double Switching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLX_A {
    #[doc = "0: PWMX double pulse disabled."]
    DBLX_0 = 0,
    #[doc = "1: PWMX double pulse enabled."]
    DBLX_1 = 1,
}
impl From<DBLX_A> for bool {
    #[inline(always)]
    fn from(variant: DBLX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBLX`"]
pub type DBLX_R = crate::R<bool, DBLX_A>;
impl DBLX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLX_A {
        match self.bits {
            false => DBLX_A::DBLX_0,
            true => DBLX_A::DBLX_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBLX_0`"]
    #[inline(always)]
    pub fn is_dblx_0(&self) -> bool {
        *self == DBLX_A::DBLX_0
    }
    #[doc = "Checks if the value of the field is `DBLX_1`"]
    #[inline(always)]
    pub fn is_dblx_1(&self) -> bool {
        *self == DBLX_A::DBLX_1
    }
}
#[doc = "Write proxy for field `DBLX`"]
pub struct DBLX_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWMX double pulse disabled."]
    #[inline(always)]
    pub fn dblx_0(self) -> &'a mut W {
        self.variant(DBLX_A::DBLX_0)
    }
    #[doc = "PWMX double pulse enabled."]
    #[inline(always)]
    pub fn dblx_1(self) -> &'a mut W {
        self.variant(DBLX_A::DBLX_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Load Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMOD_A {
    #[doc = "0: Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\]
is set."]
    LDMOD_0 = 0,
    #[doc = "1: Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\]
being set. In this case it is not necessary to set CTRL\\[FULL\\]
or CTRL\\[HALF\\]."]
    LDMOD_1 = 1,
}
impl From<LDMOD_A> for bool {
    #[inline(always)]
    fn from(variant: LDMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDMOD`"]
pub type LDMOD_R = crate::R<bool, LDMOD_A>;
impl LDMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMOD_A {
        match self.bits {
            false => LDMOD_A::LDMOD_0,
            true => LDMOD_A::LDMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LDMOD_0`"]
    #[inline(always)]
    pub fn is_ldmod_0(&self) -> bool {
        *self == LDMOD_A::LDMOD_0
    }
    #[doc = "Checks if the value of the field is `LDMOD_1`"]
    #[inline(always)]
    pub fn is_ldmod_1(&self) -> bool {
        *self == LDMOD_A::LDMOD_1
    }
}
#[doc = "Write proxy for field `LDMOD`"]
pub struct LDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\]
is set."]
    #[inline(always)]
    pub fn ldmod_0(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_0)
    }
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\]
being set. In this case it is not necessary to set CTRL\\[FULL\\]
or CTRL\\[HALF\\]."]
    #[inline(always)]
    pub fn ldmod_1(self) -> &'a mut W {
        self.variant(LDMOD_A::LDMOD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Split the DBLPWM signal to PWMA and PWMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLIT_A {
    #[doc = "0: DBLPWM is not split. PWMA and PWMB each have double pulses."]
    SPLIT_0 = 0,
    #[doc = "1: DBLPWM is split to PWMA and PWMB."]
    SPLIT_1 = 1,
}
impl From<SPLIT_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPLIT`"]
pub type SPLIT_R = crate::R<bool, SPLIT_A>;
impl SPLIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIT_A {
        match self.bits {
            false => SPLIT_A::SPLIT_0,
            true => SPLIT_A::SPLIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPLIT_0`"]
    #[inline(always)]
    pub fn is_split_0(&self) -> bool {
        *self == SPLIT_A::SPLIT_0
    }
    #[doc = "Checks if the value of the field is `SPLIT_1`"]
    #[inline(always)]
    pub fn is_split_1(&self) -> bool {
        *self == SPLIT_A::SPLIT_1
    }
}
#[doc = "Write proxy for field `SPLIT`"]
pub struct SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
    #[inline(always)]
    pub fn split_0(self) -> &'a mut W {
        self.variant(SPLIT_A::SPLIT_0)
    }
    #[doc = "DBLPWM is split to PWMA and PWMB."]
    #[inline(always)]
    pub fn split_1(self) -> &'a mut W {
        self.variant(SPLIT_A::SPLIT_1)
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
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSC_A {
    #[doc = "0: PWM clock frequency = fclk"]
    PRSC_0 = 0,
    #[doc = "1: PWM clock frequency = fclk/2"]
    PRSC_1 = 1,
    #[doc = "2: PWM clock frequency = fclk/4"]
    PRSC_2 = 2,
    #[doc = "3: PWM clock frequency = fclk/8"]
    PRSC_3 = 3,
    #[doc = "4: PWM clock frequency = fclk/16"]
    PRSC_4 = 4,
    #[doc = "5: PWM clock frequency = fclk/32"]
    PRSC_5 = 5,
    #[doc = "6: PWM clock frequency = fclk/64"]
    PRSC_6 = 6,
    #[doc = "7: PWM clock frequency = fclk/128"]
    PRSC_7 = 7,
}
impl From<PRSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRSC`"]
pub type PRSC_R = crate::R<u8, PRSC_A>;
impl PRSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSC_A {
        match self.bits {
            0 => PRSC_A::PRSC_0,
            1 => PRSC_A::PRSC_1,
            2 => PRSC_A::PRSC_2,
            3 => PRSC_A::PRSC_3,
            4 => PRSC_A::PRSC_4,
            5 => PRSC_A::PRSC_5,
            6 => PRSC_A::PRSC_6,
            7 => PRSC_A::PRSC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSC_0`"]
    #[inline(always)]
    pub fn is_prsc_0(&self) -> bool {
        *self == PRSC_A::PRSC_0
    }
    #[doc = "Checks if the value of the field is `PRSC_1`"]
    #[inline(always)]
    pub fn is_prsc_1(&self) -> bool {
        *self == PRSC_A::PRSC_1
    }
    #[doc = "Checks if the value of the field is `PRSC_2`"]
    #[inline(always)]
    pub fn is_prsc_2(&self) -> bool {
        *self == PRSC_A::PRSC_2
    }
    #[doc = "Checks if the value of the field is `PRSC_3`"]
    #[inline(always)]
    pub fn is_prsc_3(&self) -> bool {
        *self == PRSC_A::PRSC_3
    }
    #[doc = "Checks if the value of the field is `PRSC_4`"]
    #[inline(always)]
    pub fn is_prsc_4(&self) -> bool {
        *self == PRSC_A::PRSC_4
    }
    #[doc = "Checks if the value of the field is `PRSC_5`"]
    #[inline(always)]
    pub fn is_prsc_5(&self) -> bool {
        *self == PRSC_A::PRSC_5
    }
    #[doc = "Checks if the value of the field is `PRSC_6`"]
    #[inline(always)]
    pub fn is_prsc_6(&self) -> bool {
        *self == PRSC_A::PRSC_6
    }
    #[doc = "Checks if the value of the field is `PRSC_7`"]
    #[inline(always)]
    pub fn is_prsc_7(&self) -> bool {
        *self == PRSC_A::PRSC_7
    }
}
#[doc = "Write proxy for field `PRSC`"]
pub struct PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRSC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWM clock frequency = fclk"]
    #[inline(always)]
    pub fn prsc_0(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_0)
    }
    #[doc = "PWM clock frequency = fclk/2"]
    #[inline(always)]
    pub fn prsc_1(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_1)
    }
    #[doc = "PWM clock frequency = fclk/4"]
    #[inline(always)]
    pub fn prsc_2(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_2)
    }
    #[doc = "PWM clock frequency = fclk/8"]
    #[inline(always)]
    pub fn prsc_3(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_3)
    }
    #[doc = "PWM clock frequency = fclk/16"]
    #[inline(always)]
    pub fn prsc_4(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_4)
    }
    #[doc = "PWM clock frequency = fclk/32"]
    #[inline(always)]
    pub fn prsc_5(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_5)
    }
    #[doc = "PWM clock frequency = fclk/64"]
    #[inline(always)]
    pub fn prsc_6(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_6)
    }
    #[doc = "PWM clock frequency = fclk/128"]
    #[inline(always)]
    pub fn prsc_7(self) -> &'a mut W {
        self.variant(PRSC_A::PRSC_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPMODE_A {
    #[doc = "0: The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    COMPMODE_0 = 0,
    #[doc = "1: The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    COMPMODE_1 = 1,
}
impl From<COMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPMODE`"]
pub type COMPMODE_R = crate::R<bool, COMPMODE_A>;
impl COMPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPMODE_A {
        match self.bits {
            false => COMPMODE_A::COMPMODE_0,
            true => COMPMODE_A::COMPMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `COMPMODE_0`"]
    #[inline(always)]
    pub fn is_compmode_0(&self) -> bool {
        *self == COMPMODE_A::COMPMODE_0
    }
    #[doc = "Checks if the value of the field is `COMPMODE_1`"]
    #[inline(always)]
    pub fn is_compmode_1(&self) -> bool {
        *self == COMPMODE_A::COMPMODE_1
    }
}
#[doc = "Write proxy for field `COMPMODE`"]
pub struct COMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    #[inline(always)]
    pub fn compmode_0(self) -> &'a mut W {
        self.variant(COMPMODE_A::COMPMODE_0)
    }
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    #[inline(always)]
    pub fn compmode_1(self) -> &'a mut W {
        self.variant(COMPMODE_A::COMPMODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
#[doc = "Full Cycle Reload\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_A {
    #[doc = "0: Full-cycle reloads disabled."]
    FULL_0 = 0,
    #[doc = "1: Full-cycle reloads enabled."]
    FULL_1 = 1,
}
impl From<FULL_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, FULL_A>;
impl FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL_A {
        match self.bits {
            false => FULL_A::FULL_0,
            true => FULL_A::FULL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_0`"]
    #[inline(always)]
    pub fn is_full_0(&self) -> bool {
        *self == FULL_A::FULL_0
    }
    #[doc = "Checks if the value of the field is `FULL_1`"]
    #[inline(always)]
    pub fn is_full_1(&self) -> bool {
        *self == FULL_A::FULL_1
    }
}
#[doc = "Write proxy for field `FULL`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Full-cycle reloads disabled."]
    #[inline(always)]
    pub fn full_0(self) -> &'a mut W {
        self.variant(FULL_A::FULL_0)
    }
    #[doc = "Full-cycle reloads enabled."]
    #[inline(always)]
    pub fn full_1(self) -> &'a mut W {
        self.variant(FULL_A::FULL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Half Cycle Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALF_A {
    #[doc = "0: Half-cycle reloads disabled."]
    HALF_0 = 0,
    #[doc = "1: Half-cycle reloads enabled."]
    HALF_1 = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALF`"]
pub type HALF_R = crate::R<bool, HALF_A>;
impl HALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::HALF_0,
            true => HALF_A::HALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_0`"]
    #[inline(always)]
    pub fn is_half_0(&self) -> bool {
        *self == HALF_A::HALF_0
    }
    #[doc = "Checks if the value of the field is `HALF_1`"]
    #[inline(always)]
    pub fn is_half_1(&self) -> bool {
        *self == HALF_A::HALF_1
    }
}
#[doc = "Write proxy for field `HALF`"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Half-cycle reloads disabled."]
    #[inline(always)]
    pub fn half_0(self) -> &'a mut W {
        self.variant(HALF_A::HALF_0)
    }
    #[doc = "Half-cycle reloads enabled."]
    #[inline(always)]
    pub fn half_1(self) -> &'a mut W {
        self.variant(HALF_A::HALF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Load Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDFQ_A {
    #[doc = "0: Every PWM opportunity"]
    LDFQ_0 = 0,
    #[doc = "1: Every 2 PWM opportunities"]
    LDFQ_1 = 1,
    #[doc = "2: Every 3 PWM opportunities"]
    LDFQ_2 = 2,
    #[doc = "3: Every 4 PWM opportunities"]
    LDFQ_3 = 3,
    #[doc = "4: Every 5 PWM opportunities"]
    LDFQ_4 = 4,
    #[doc = "5: Every 6 PWM opportunities"]
    LDFQ_5 = 5,
    #[doc = "6: Every 7 PWM opportunities"]
    LDFQ_6 = 6,
    #[doc = "7: Every 8 PWM opportunities"]
    LDFQ_7 = 7,
    #[doc = "8: Every 9 PWM opportunities"]
    LDFQ_8 = 8,
    #[doc = "9: Every 10 PWM opportunities"]
    LDFQ_9 = 9,
    #[doc = "10: Every 11 PWM opportunities"]
    LDFQ_10 = 10,
    #[doc = "11: Every 12 PWM opportunities"]
    LDFQ_11 = 11,
    #[doc = "12: Every 13 PWM opportunities"]
    LDFQ_12 = 12,
    #[doc = "13: Every 14 PWM opportunities"]
    LDFQ_13 = 13,
    #[doc = "14: Every 15 PWM opportunities"]
    LDFQ_14 = 14,
    #[doc = "15: Every 16 PWM opportunities"]
    LDFQ_15 = 15,
}
impl From<LDFQ_A> for u8 {
    #[inline(always)]
    fn from(variant: LDFQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDFQ`"]
pub type LDFQ_R = crate::R<u8, LDFQ_A>;
impl LDFQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDFQ_A {
        match self.bits {
            0 => LDFQ_A::LDFQ_0,
            1 => LDFQ_A::LDFQ_1,
            2 => LDFQ_A::LDFQ_2,
            3 => LDFQ_A::LDFQ_3,
            4 => LDFQ_A::LDFQ_4,
            5 => LDFQ_A::LDFQ_5,
            6 => LDFQ_A::LDFQ_6,
            7 => LDFQ_A::LDFQ_7,
            8 => LDFQ_A::LDFQ_8,
            9 => LDFQ_A::LDFQ_9,
            10 => LDFQ_A::LDFQ_10,
            11 => LDFQ_A::LDFQ_11,
            12 => LDFQ_A::LDFQ_12,
            13 => LDFQ_A::LDFQ_13,
            14 => LDFQ_A::LDFQ_14,
            15 => LDFQ_A::LDFQ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LDFQ_0`"]
    #[inline(always)]
    pub fn is_ldfq_0(&self) -> bool {
        *self == LDFQ_A::LDFQ_0
    }
    #[doc = "Checks if the value of the field is `LDFQ_1`"]
    #[inline(always)]
    pub fn is_ldfq_1(&self) -> bool {
        *self == LDFQ_A::LDFQ_1
    }
    #[doc = "Checks if the value of the field is `LDFQ_2`"]
    #[inline(always)]
    pub fn is_ldfq_2(&self) -> bool {
        *self == LDFQ_A::LDFQ_2
    }
    #[doc = "Checks if the value of the field is `LDFQ_3`"]
    #[inline(always)]
    pub fn is_ldfq_3(&self) -> bool {
        *self == LDFQ_A::LDFQ_3
    }
    #[doc = "Checks if the value of the field is `LDFQ_4`"]
    #[inline(always)]
    pub fn is_ldfq_4(&self) -> bool {
        *self == LDFQ_A::LDFQ_4
    }
    #[doc = "Checks if the value of the field is `LDFQ_5`"]
    #[inline(always)]
    pub fn is_ldfq_5(&self) -> bool {
        *self == LDFQ_A::LDFQ_5
    }
    #[doc = "Checks if the value of the field is `LDFQ_6`"]
    #[inline(always)]
    pub fn is_ldfq_6(&self) -> bool {
        *self == LDFQ_A::LDFQ_6
    }
    #[doc = "Checks if the value of the field is `LDFQ_7`"]
    #[inline(always)]
    pub fn is_ldfq_7(&self) -> bool {
        *self == LDFQ_A::LDFQ_7
    }
    #[doc = "Checks if the value of the field is `LDFQ_8`"]
    #[inline(always)]
    pub fn is_ldfq_8(&self) -> bool {
        *self == LDFQ_A::LDFQ_8
    }
    #[doc = "Checks if the value of the field is `LDFQ_9`"]
    #[inline(always)]
    pub fn is_ldfq_9(&self) -> bool {
        *self == LDFQ_A::LDFQ_9
    }
    #[doc = "Checks if the value of the field is `LDFQ_10`"]
    #[inline(always)]
    pub fn is_ldfq_10(&self) -> bool {
        *self == LDFQ_A::LDFQ_10
    }
    #[doc = "Checks if the value of the field is `LDFQ_11`"]
    #[inline(always)]
    pub fn is_ldfq_11(&self) -> bool {
        *self == LDFQ_A::LDFQ_11
    }
    #[doc = "Checks if the value of the field is `LDFQ_12`"]
    #[inline(always)]
    pub fn is_ldfq_12(&self) -> bool {
        *self == LDFQ_A::LDFQ_12
    }
    #[doc = "Checks if the value of the field is `LDFQ_13`"]
    #[inline(always)]
    pub fn is_ldfq_13(&self) -> bool {
        *self == LDFQ_A::LDFQ_13
    }
    #[doc = "Checks if the value of the field is `LDFQ_14`"]
    #[inline(always)]
    pub fn is_ldfq_14(&self) -> bool {
        *self == LDFQ_A::LDFQ_14
    }
    #[doc = "Checks if the value of the field is `LDFQ_15`"]
    #[inline(always)]
    pub fn is_ldfq_15(&self) -> bool {
        *self == LDFQ_A::LDFQ_15
    }
}
#[doc = "Write proxy for field `LDFQ`"]
pub struct LDFQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LDFQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDFQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Every PWM opportunity"]
    #[inline(always)]
    pub fn ldfq_0(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_0)
    }
    #[doc = "Every 2 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_1(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_1)
    }
    #[doc = "Every 3 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_2(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_2)
    }
    #[doc = "Every 4 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_3(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_3)
    }
    #[doc = "Every 5 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_4(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_4)
    }
    #[doc = "Every 6 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_5(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_5)
    }
    #[doc = "Every 7 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_6(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_6)
    }
    #[doc = "Every 8 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_7(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_7)
    }
    #[doc = "Every 9 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_8(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_8)
    }
    #[doc = "Every 10 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_9(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_9)
    }
    #[doc = "Every 11 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_10(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_10)
    }
    #[doc = "Every 12 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_11(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_11)
    }
    #[doc = "Every 13 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_12(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_12)
    }
    #[doc = "Every 14 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_13(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_13)
    }
    #[doc = "Every 15 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_14(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_14)
    }
    #[doc = "Every 16 PWM opportunities"]
    #[inline(always)]
    pub fn ldfq_15(self) -> &'a mut W {
        self.variant(LDFQ_A::LDFQ_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline(always)]
    pub fn dblen(&self) -> DBLEN_R {
        DBLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline(always)]
    pub fn dblx(&self) -> DBLX_R {
        DBLX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&self) -> LDMOD_R {
        LDMOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    pub fn split(&self) -> SPLIT_R {
        SPLIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline(always)]
    pub fn compmode(&self) -> COMPMODE_R {
        COMPMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Deadtime"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline(always)]
    pub fn ldfq(&self) -> LDFQ_R {
        LDFQ_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline(always)]
    pub fn dblen(&mut self) -> DBLEN_W {
        DBLEN_W { w: self }
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline(always)]
    pub fn dblx(&mut self) -> DBLX_W {
        DBLX_W { w: self }
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&mut self) -> LDMOD_W {
        LDMOD_W { w: self }
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    pub fn split(&mut self) -> SPLIT_W {
        SPLIT_W { w: self }
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline(always)]
    pub fn prsc(&mut self) -> PRSC_W {
        PRSC_W { w: self }
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline(always)]
    pub fn compmode(&mut self) -> COMPMODE_W {
        COMPMODE_W { w: self }
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline(always)]
    pub fn ldfq(&mut self) -> LDFQ_W {
        LDFQ_W { w: self }
    }
}
