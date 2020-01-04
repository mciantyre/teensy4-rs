#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u16, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u16, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Update Hold Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDHLD_A {
    #[doc = "0: Disable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_0 = 0,
    #[doc = "1: Enable updates of hold registers on rising edge of TRIGGER"]
    UPDHLD_1 = 1,
}
impl From<UPDHLD_A> for bool {
    #[inline(always)]
    fn from(variant: UPDHLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDHLD`"]
pub type UPDHLD_R = crate::R<bool, UPDHLD_A>;
impl UPDHLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDHLD_A {
        match self.bits {
            false => UPDHLD_A::UPDHLD_0,
            true => UPDHLD_A::UPDHLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDHLD_0`"]
    #[inline(always)]
    pub fn is_updhld_0(&self) -> bool {
        *self == UPDHLD_A::UPDHLD_0
    }
    #[doc = "Checks if the value of the field is `UPDHLD_1`"]
    #[inline(always)]
    pub fn is_updhld_1(&self) -> bool {
        *self == UPDHLD_A::UPDHLD_1
    }
}
#[doc = "Write proxy for field `UPDHLD`"]
pub struct UPDHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDHLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDHLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable updates of hold registers on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updhld_0(self) -> &'a mut W {
        self.variant(UPDHLD_A::UPDHLD_0)
    }
    #[doc = "Enable updates of hold registers on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updhld_1(self) -> &'a mut W {
        self.variant(UPDHLD_A::UPDHLD_1)
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
#[doc = "Update Position Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDPOS_A {
    #[doc = "0: No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_0 = 0,
    #[doc = "1: Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    UPDPOS_1 = 1,
}
impl From<UPDPOS_A> for bool {
    #[inline(always)]
    fn from(variant: UPDPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDPOS`"]
pub type UPDPOS_R = crate::R<bool, UPDPOS_A>;
impl UPDPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDPOS_A {
        match self.bits {
            false => UPDPOS_A::UPDPOS_0,
            true => UPDPOS_A::UPDPOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDPOS_0`"]
    #[inline(always)]
    pub fn is_updpos_0(&self) -> bool {
        *self == UPDPOS_A::UPDPOS_0
    }
    #[doc = "Checks if the value of the field is `UPDPOS_1`"]
    #[inline(always)]
    pub fn is_updpos_1(&self) -> bool {
        *self == UPDPOS_A::UPDPOS_1
    }
}
#[doc = "Write proxy for field `UPDPOS`"]
pub struct UPDPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDPOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action for POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updpos_0(self) -> &'a mut W {
        self.variant(UPDPOS_A::UPDPOS_0)
    }
    #[doc = "Clear POSD, REV, UPOS and LPOS on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updpos_1(self) -> &'a mut W {
        self.variant(UPDPOS_A::UPDPOS_1)
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
#[doc = "Enable Modulo Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_A {
    #[doc = "0: Disable modulo counting"]
    MOD_0 = 0,
    #[doc = "1: Enable modulo counting"]
    MOD_1 = 1,
}
impl From<MOD_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<bool, MOD_A>;
impl MOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_A {
        match self.bits {
            false => MOD_A::MOD_0,
            true => MOD_A::MOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_0`"]
    #[inline(always)]
    pub fn is_mod_0(&self) -> bool {
        *self == MOD_A::MOD_0
    }
    #[doc = "Checks if the value of the field is `MOD_1`"]
    #[inline(always)]
    pub fn is_mod_1(&self) -> bool {
        *self == MOD_A::MOD_1
    }
}
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable modulo counting"]
    #[inline(always)]
    pub fn mod_0(self) -> &'a mut W {
        self.variant(MOD_A::MOD_0)
    }
    #[doc = "Enable modulo counting"]
    #[inline(always)]
    pub fn mod_1(self) -> &'a mut W {
        self.variant(MOD_A::MOD_1)
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
#[doc = "Count Direction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Last count was in the down direction"]
    DIR_0 = 0,
    #[doc = "1: Last count was in the up direction"]
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
#[doc = "Roll-under Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUIE_A {
    #[doc = "0: Roll-under interrupt is disabled"]
    RUIE_0 = 0,
    #[doc = "1: Roll-under interrupt is enabled"]
    RUIE_1 = 1,
}
impl From<RUIE_A> for bool {
    #[inline(always)]
    fn from(variant: RUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUIE`"]
pub type RUIE_R = crate::R<bool, RUIE_A>;
impl RUIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIE_A {
        match self.bits {
            false => RUIE_A::RUIE_0,
            true => RUIE_A::RUIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIE_0`"]
    #[inline(always)]
    pub fn is_ruie_0(&self) -> bool {
        *self == RUIE_A::RUIE_0
    }
    #[doc = "Checks if the value of the field is `RUIE_1`"]
    #[inline(always)]
    pub fn is_ruie_1(&self) -> bool {
        *self == RUIE_A::RUIE_1
    }
}
#[doc = "Write proxy for field `RUIE`"]
pub struct RUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Roll-under interrupt is disabled"]
    #[inline(always)]
    pub fn ruie_0(self) -> &'a mut W {
        self.variant(RUIE_A::RUIE_0)
    }
    #[doc = "Roll-under interrupt is enabled"]
    #[inline(always)]
    pub fn ruie_1(self) -> &'a mut W {
        self.variant(RUIE_A::RUIE_1)
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
#[doc = "Roll-under Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUIRQ_A {
    #[doc = "0: No roll-under has occurred"]
    RUIRQ_0 = 0,
    #[doc = "1: Roll-under has occurred"]
    RUIRQ_1 = 1,
}
impl From<RUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUIRQ`"]
pub type RUIRQ_R = crate::R<bool, RUIRQ_A>;
impl RUIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIRQ_A {
        match self.bits {
            false => RUIRQ_A::RUIRQ_0,
            true => RUIRQ_A::RUIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIRQ_0`"]
    #[inline(always)]
    pub fn is_ruirq_0(&self) -> bool {
        *self == RUIRQ_A::RUIRQ_0
    }
    #[doc = "Checks if the value of the field is `RUIRQ_1`"]
    #[inline(always)]
    pub fn is_ruirq_1(&self) -> bool {
        *self == RUIRQ_A::RUIRQ_1
    }
}
#[doc = "Write proxy for field `RUIRQ`"]
pub struct RUIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RUIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No roll-under has occurred"]
    #[inline(always)]
    pub fn ruirq_0(self) -> &'a mut W {
        self.variant(RUIRQ_A::RUIRQ_0)
    }
    #[doc = "Roll-under has occurred"]
    #[inline(always)]
    pub fn ruirq_1(self) -> &'a mut W {
        self.variant(RUIRQ_A::RUIRQ_1)
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
#[doc = "Roll-over Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROIE_A {
    #[doc = "0: Roll-over interrupt is disabled"]
    ROIE_0 = 0,
    #[doc = "1: Roll-over interrupt is enabled"]
    ROIE_1 = 1,
}
impl From<ROIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROIE`"]
pub type ROIE_R = crate::R<bool, ROIE_A>;
impl ROIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIE_A {
        match self.bits {
            false => ROIE_A::ROIE_0,
            true => ROIE_A::ROIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIE_0`"]
    #[inline(always)]
    pub fn is_roie_0(&self) -> bool {
        *self == ROIE_A::ROIE_0
    }
    #[doc = "Checks if the value of the field is `ROIE_1`"]
    #[inline(always)]
    pub fn is_roie_1(&self) -> bool {
        *self == ROIE_A::ROIE_1
    }
}
#[doc = "Write proxy for field `ROIE`"]
pub struct ROIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Roll-over interrupt is disabled"]
    #[inline(always)]
    pub fn roie_0(self) -> &'a mut W {
        self.variant(ROIE_A::ROIE_0)
    }
    #[doc = "Roll-over interrupt is enabled"]
    #[inline(always)]
    pub fn roie_1(self) -> &'a mut W {
        self.variant(ROIE_A::ROIE_1)
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
#[doc = "Roll-over Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROIRQ_A {
    #[doc = "0: No roll-over has occurred"]
    ROIRQ_0 = 0,
    #[doc = "1: Roll-over has occurred"]
    ROIRQ_1 = 1,
}
impl From<ROIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ROIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROIRQ`"]
pub type ROIRQ_R = crate::R<bool, ROIRQ_A>;
impl ROIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIRQ_A {
        match self.bits {
            false => ROIRQ_A::ROIRQ_0,
            true => ROIRQ_A::ROIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIRQ_0`"]
    #[inline(always)]
    pub fn is_roirq_0(&self) -> bool {
        *self == ROIRQ_A::ROIRQ_0
    }
    #[doc = "Checks if the value of the field is `ROIRQ_1`"]
    #[inline(always)]
    pub fn is_roirq_1(&self) -> bool {
        *self == ROIRQ_A::ROIRQ_1
    }
}
#[doc = "Write proxy for field `ROIRQ`"]
pub struct ROIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No roll-over has occurred"]
    #[inline(always)]
    pub fn roirq_0(self) -> &'a mut W {
        self.variant(ROIRQ_A::ROIRQ_0)
    }
    #[doc = "Roll-over has occurred"]
    #[inline(always)]
    pub fn roirq_1(self) -> &'a mut W {
        self.variant(ROIRQ_A::ROIRQ_1)
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
#[doc = "Revolution Counter Modulus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVMOD_A {
    #[doc = "0: Use INDEX pulse to increment/decrement revolution counter (REV)."]
    REVMOD_0 = 0,
    #[doc = "1: Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
    REVMOD_1 = 1,
}
impl From<REVMOD_A> for bool {
    #[inline(always)]
    fn from(variant: REVMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REVMOD`"]
pub type REVMOD_R = crate::R<bool, REVMOD_A>;
impl REVMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REVMOD_A {
        match self.bits {
            false => REVMOD_A::REVMOD_0,
            true => REVMOD_A::REVMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `REVMOD_0`"]
    #[inline(always)]
    pub fn is_revmod_0(&self) -> bool {
        *self == REVMOD_A::REVMOD_0
    }
    #[doc = "Checks if the value of the field is `REVMOD_1`"]
    #[inline(always)]
    pub fn is_revmod_1(&self) -> bool {
        *self == REVMOD_A::REVMOD_1
    }
}
#[doc = "Write proxy for field `REVMOD`"]
pub struct REVMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)."]
    #[inline(always)]
    pub fn revmod_0(self) -> &'a mut W {
        self.variant(REVMOD_A::REVMOD_0)
    }
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)."]
    #[inline(always)]
    pub fn revmod_1(self) -> &'a mut W {
        self.variant(REVMOD_A::REVMOD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCTL_A {
    #[doc = "0: POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
    OUTCTL_0 = 0,
    #[doc = "1: POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
    OUTCTL_1 = 1,
}
impl From<OUTCTL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCTL`"]
pub type OUTCTL_R = crate::R<bool, OUTCTL_A>;
impl OUTCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCTL_A {
        match self.bits {
            false => OUTCTL_A::OUTCTL_0,
            true => OUTCTL_A::OUTCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTCTL_0`"]
    #[inline(always)]
    pub fn is_outctl_0(&self) -> bool {
        *self == OUTCTL_A::OUTCTL_0
    }
    #[doc = "Checks if the value of the field is `OUTCTL_1`"]
    #[inline(always)]
    pub fn is_outctl_1(&self) -> bool {
        *self == OUTCTL_A::OUTCTL_1
    }
}
#[doc = "Write proxy for field `OUTCTL`"]
pub struct OUTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the compare value (COMP)."]
    #[inline(always)]
    pub fn outctl_0(self) -> &'a mut W {
        self.variant(OUTCTL_A::OUTCTL_0)
    }
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read."]
    #[inline(always)]
    pub fn outctl_1(self) -> &'a mut W {
        self.variant(OUTCTL_A::OUTCTL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABIE_A {
    #[doc = "0: Simultaneous PHASEA and PHASEB change interrupt disabled."]
    SABIE_0 = 0,
    #[doc = "1: Simultaneous PHASEA and PHASEB change interrupt enabled."]
    SABIE_1 = 1,
}
impl From<SABIE_A> for bool {
    #[inline(always)]
    fn from(variant: SABIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SABIE`"]
pub type SABIE_R = crate::R<bool, SABIE_A>;
impl SABIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABIE_A {
        match self.bits {
            false => SABIE_A::SABIE_0,
            true => SABIE_A::SABIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABIE_0`"]
    #[inline(always)]
    pub fn is_sabie_0(&self) -> bool {
        *self == SABIE_A::SABIE_0
    }
    #[doc = "Checks if the value of the field is `SABIE_1`"]
    #[inline(always)]
    pub fn is_sabie_1(&self) -> bool {
        *self == SABIE_A::SABIE_1
    }
}
#[doc = "Write proxy for field `SABIE`"]
pub struct SABIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SABIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SABIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt disabled."]
    #[inline(always)]
    pub fn sabie_0(self) -> &'a mut W {
        self.variant(SABIE_A::SABIE_0)
    }
    #[doc = "Simultaneous PHASEA and PHASEB change interrupt enabled."]
    #[inline(always)]
    pub fn sabie_1(self) -> &'a mut W {
        self.variant(SABIE_A::SABIE_1)
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
#[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABIRQ_A {
    #[doc = "0: No simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_0 = 0,
    #[doc = "1: A simultaneous change of PHASEA and PHASEB has occurred."]
    SABIRQ_1 = 1,
}
impl From<SABIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SABIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SABIRQ`"]
pub type SABIRQ_R = crate::R<bool, SABIRQ_A>;
impl SABIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABIRQ_A {
        match self.bits {
            false => SABIRQ_A::SABIRQ_0,
            true => SABIRQ_A::SABIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABIRQ_0`"]
    #[inline(always)]
    pub fn is_sabirq_0(&self) -> bool {
        *self == SABIRQ_A::SABIRQ_0
    }
    #[doc = "Checks if the value of the field is `SABIRQ_1`"]
    #[inline(always)]
    pub fn is_sabirq_1(&self) -> bool {
        *self == SABIRQ_A::SABIRQ_1
    }
}
#[doc = "Write proxy for field `SABIRQ`"]
pub struct SABIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SABIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SABIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No simultaneous change of PHASEA and PHASEB has occurred."]
    #[inline(always)]
    pub fn sabirq_0(self) -> &'a mut W {
        self.variant(SABIRQ_A::SABIRQ_0)
    }
    #[doc = "A simultaneous change of PHASEA and PHASEB has occurred."]
    #[inline(always)]
    pub fn sabirq_1(self) -> &'a mut W {
        self.variant(SABIRQ_A::SABIRQ_1)
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
impl R {
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline(always)]
    pub fn updhld(&self) -> UPDHLD_R {
        UPDHLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline(always)]
    pub fn updpos(&self) -> UPDPOS_R {
        UPDPOS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Count Direction Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline(always)]
    pub fn ruie(&self) -> RUIE_R {
        RUIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline(always)]
    pub fn ruirq(&self) -> RUIRQ_R {
        RUIRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline(always)]
    pub fn roirq(&self) -> ROIRQ_R {
        ROIRQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline(always)]
    pub fn revmod(&self) -> REVMOD_R {
        REVMOD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline(always)]
    pub fn outctl(&self) -> OUTCTL_R {
        OUTCTL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline(always)]
    pub fn sabie(&self) -> SABIE_R {
        SABIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline(always)]
    pub fn sabirq(&self) -> SABIRQ_R {
        SABIRQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline(always)]
    pub fn updhld(&mut self) -> UPDHLD_W {
        UPDHLD_W { w: self }
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline(always)]
    pub fn updpos(&mut self) -> UPDPOS_W {
        UPDPOS_W { w: self }
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline(always)]
    pub fn ruie(&mut self) -> RUIE_W {
        RUIE_W { w: self }
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline(always)]
    pub fn ruirq(&mut self) -> RUIRQ_W {
        RUIRQ_W { w: self }
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W {
        ROIE_W { w: self }
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline(always)]
    pub fn roirq(&mut self) -> ROIRQ_W {
        ROIRQ_W { w: self }
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline(always)]
    pub fn revmod(&mut self) -> REVMOD_W {
        REVMOD_W { w: self }
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline(always)]
    pub fn outctl(&mut self) -> OUTCTL_W {
        OUTCTL_W { w: self }
    }
    #[doc = "Bit 10 - Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    #[inline(always)]
    pub fn sabie(&mut self) -> SABIE_W {
        SABIE_W { w: self }
    }
    #[doc = "Bit 11 - Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    #[inline(always)]
    pub fn sabirq(&mut self) -> SABIRQ_W {
        SABIRQ_W { w: self }
    }
}
