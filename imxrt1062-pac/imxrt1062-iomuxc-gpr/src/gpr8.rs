#[doc = "Reader of register GPR8"]
pub type R = crate::R<u32, super::GPR8>;
#[doc = "Writer for register GPR8"]
pub type W = crate::W<u32, super::GPR8>;
#[doc = "Register GPR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C1_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C1_IPG_STOP_MODE_1,
}
impl From<LPI2C1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0 => false,
            LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C1_IPG_STOP_MODE`"]
pub type LPI2C1_IPG_STOP_MODE_R = crate::R<bool, LPI2C1_IPG_STOP_MODE_A>;
impl LPI2C1_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0,
            true => LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPI2C1_IPG_STOP_MODE`"]
pub struct LPI2C1_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C1_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C1_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1)
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
#[doc = "LPI2C1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C1_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPI2C1_IPG_DOZE_1,
}
impl From<LPI2C1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_IPG_DOZE_A) -> Self {
        match variant {
            LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0 => false,
            LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C1_IPG_DOZE`"]
pub type LPI2C1_IPG_DOZE_R = crate::R<bool, LPI2C1_IPG_DOZE_A>;
impl LPI2C1_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_IPG_DOZE_A {
        match self.bits {
            false => LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0,
            true => LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_doze_0(&self) -> bool {
        *self == LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_doze_1(&self) -> bool {
        *self == LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPI2C1_IPG_DOZE`"]
pub struct LPI2C1_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C1_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C1_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1)
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
#[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C2_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C2_IPG_STOP_MODE_1,
}
impl From<LPI2C2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0 => false,
            LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C2_IPG_STOP_MODE`"]
pub type LPI2C2_IPG_STOP_MODE_R = crate::R<bool, LPI2C2_IPG_STOP_MODE_A>;
impl LPI2C2_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0,
            true => LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPI2C2_IPG_STOP_MODE`"]
pub struct LPI2C2_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C2_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C2_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1)
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
#[doc = "LPI2C2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C2_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPI2C2_IPG_DOZE_1,
}
impl From<LPI2C2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_IPG_DOZE_A) -> Self {
        match variant {
            LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0 => false,
            LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C2_IPG_DOZE`"]
pub type LPI2C2_IPG_DOZE_R = crate::R<bool, LPI2C2_IPG_DOZE_A>;
impl LPI2C2_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_IPG_DOZE_A {
        match self.bits {
            false => LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0,
            true => LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_doze_0(&self) -> bool {
        *self == LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_doze_1(&self) -> bool {
        *self == LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPI2C2_IPG_DOZE`"]
pub struct LPI2C2_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C2_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C2_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1)
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
#[doc = "LPI2C3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C3_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C3_IPG_STOP_MODE_1,
}
impl From<LPI2C3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0 => false,
            LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C3_IPG_STOP_MODE`"]
pub type LPI2C3_IPG_STOP_MODE_R = crate::R<bool, LPI2C3_IPG_STOP_MODE_A>;
impl LPI2C3_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0,
            true => LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPI2C3_IPG_STOP_MODE`"]
pub struct LPI2C3_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C3_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C3_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "LPI2C3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C3_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPI2C3_IPG_DOZE_1,
}
impl From<LPI2C3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_IPG_DOZE_A) -> Self {
        match variant {
            LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0 => false,
            LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C3_IPG_DOZE`"]
pub type LPI2C3_IPG_DOZE_R = crate::R<bool, LPI2C3_IPG_DOZE_A>;
impl LPI2C3_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_IPG_DOZE_A {
        match self.bits {
            false => LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0,
            true => LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_doze_0(&self) -> bool {
        *self == LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_doze_1(&self) -> bool {
        *self == LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPI2C3_IPG_DOZE`"]
pub struct LPI2C3_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C3_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C3_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1)
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
#[doc = "LPI2C4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C4_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C4_IPG_STOP_MODE_1,
}
impl From<LPI2C4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0 => false,
            LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C4_IPG_STOP_MODE`"]
pub type LPI2C4_IPG_STOP_MODE_R = crate::R<bool, LPI2C4_IPG_STOP_MODE_A>;
impl LPI2C4_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0,
            true => LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPI2C4_IPG_STOP_MODE`"]
pub struct LPI2C4_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C4_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C4_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "LPI2C4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C4_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPI2C4_IPG_DOZE_1,
}
impl From<LPI2C4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_IPG_DOZE_A) -> Self {
        match variant {
            LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0 => false,
            LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPI2C4_IPG_DOZE`"]
pub type LPI2C4_IPG_DOZE_R = crate::R<bool, LPI2C4_IPG_DOZE_A>;
impl LPI2C4_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_IPG_DOZE_A {
        match self.bits {
            false => LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0,
            true => LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_doze_0(&self) -> bool {
        *self == LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_doze_1(&self) -> bool {
        *self == LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPI2C4_IPG_DOZE`"]
pub struct LPI2C4_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C4_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C4_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1)
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
#[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI1_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI1_IPG_STOP_MODE_1,
}
impl From<LPSPI1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0 => false,
            LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI1_IPG_STOP_MODE`"]
pub type LPSPI1_IPG_STOP_MODE_R = crate::R<bool, LPSPI1_IPG_STOP_MODE_A>;
impl LPSPI1_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0,
            true => LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPSPI1_IPG_STOP_MODE`"]
pub struct LPSPI1_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI1_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI1_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1)
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
#[doc = "LPSPI1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI1_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPSPI1_IPG_DOZE_1,
}
impl From<LPSPI1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_IPG_DOZE_A) -> Self {
        match variant {
            LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0 => false,
            LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI1_IPG_DOZE`"]
pub type LPSPI1_IPG_DOZE_R = crate::R<bool, LPSPI1_IPG_DOZE_A>;
impl LPSPI1_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_IPG_DOZE_A {
        match self.bits {
            false => LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0,
            true => LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_doze_0(&self) -> bool {
        *self == LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_doze_1(&self) -> bool {
        *self == LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPSPI1_IPG_DOZE`"]
pub struct LPSPI1_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI1_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI1_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1)
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
#[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI2_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI2_IPG_STOP_MODE_1,
}
impl From<LPSPI2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0 => false,
            LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI2_IPG_STOP_MODE`"]
pub type LPSPI2_IPG_STOP_MODE_R = crate::R<bool, LPSPI2_IPG_STOP_MODE_A>;
impl LPSPI2_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0,
            true => LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPSPI2_IPG_STOP_MODE`"]
pub struct LPSPI2_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI2_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI2_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1)
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
#[doc = "LPSPI2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI2_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPSPI2_IPG_DOZE_1,
}
impl From<LPSPI2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_IPG_DOZE_A) -> Self {
        match variant {
            LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0 => false,
            LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI2_IPG_DOZE`"]
pub type LPSPI2_IPG_DOZE_R = crate::R<bool, LPSPI2_IPG_DOZE_A>;
impl LPSPI2_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_IPG_DOZE_A {
        match self.bits {
            false => LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0,
            true => LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_doze_0(&self) -> bool {
        *self == LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_doze_1(&self) -> bool {
        *self == LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPSPI2_IPG_DOZE`"]
pub struct LPSPI2_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI2_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI2_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "LPSPI3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI3_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI3_IPG_STOP_MODE_1,
}
impl From<LPSPI3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0 => false,
            LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI3_IPG_STOP_MODE`"]
pub type LPSPI3_IPG_STOP_MODE_R = crate::R<bool, LPSPI3_IPG_STOP_MODE_A>;
impl LPSPI3_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0,
            true => LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPSPI3_IPG_STOP_MODE`"]
pub struct LPSPI3_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI3_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI3_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1)
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
#[doc = "LPSPI3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI3_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPSPI3_IPG_DOZE_1,
}
impl From<LPSPI3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_IPG_DOZE_A) -> Self {
        match variant {
            LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0 => false,
            LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI3_IPG_DOZE`"]
pub type LPSPI3_IPG_DOZE_R = crate::R<bool, LPSPI3_IPG_DOZE_A>;
impl LPSPI3_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_IPG_DOZE_A {
        match self.bits {
            false => LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0,
            true => LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_doze_0(&self) -> bool {
        *self == LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_doze_1(&self) -> bool {
        *self == LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPSPI3_IPG_DOZE`"]
pub struct LPSPI3_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI3_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI3_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1)
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
#[doc = "LPSPI4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI4_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI4_IPG_STOP_MODE_1,
}
impl From<LPSPI4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0 => false,
            LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI4_IPG_STOP_MODE`"]
pub type LPSPI4_IPG_STOP_MODE_R = crate::R<bool, LPSPI4_IPG_STOP_MODE_A>;
impl LPSPI4_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0,
            true => LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPSPI4_IPG_STOP_MODE`"]
pub struct LPSPI4_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI4_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI4_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1)
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
#[doc = "LPSPI4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI4_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPSPI4_IPG_DOZE_1,
}
impl From<LPSPI4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_IPG_DOZE_A) -> Self {
        match variant {
            LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0 => false,
            LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPSPI4_IPG_DOZE`"]
pub type LPSPI4_IPG_DOZE_R = crate::R<bool, LPSPI4_IPG_DOZE_A>;
impl LPSPI4_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_IPG_DOZE_A {
        match self.bits {
            false => LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0,
            true => LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_doze_0(&self) -> bool {
        *self == LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_doze_1(&self) -> bool {
        *self == LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPSPI4_IPG_DOZE`"]
pub struct LPSPI4_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI4_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI4_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1)
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
#[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART1_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART1_IPG_STOP_MODE_1,
}
impl From<LPUART1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0 => false,
            LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART1_IPG_STOP_MODE`"]
pub type LPUART1_IPG_STOP_MODE_R = crate::R<bool, LPUART1_IPG_STOP_MODE_A>;
impl LPUART1_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0,
            true => LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART1_IPG_STOP_MODE`"]
pub struct LPUART1_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1)
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
#[doc = "LPUART1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART1_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART1_IPG_DOZE_1,
}
impl From<LPUART1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_IPG_DOZE_A) -> Self {
        match variant {
            LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0 => false,
            LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART1_IPG_DOZE`"]
pub type LPUART1_IPG_DOZE_R = crate::R<bool, LPUART1_IPG_DOZE_A>;
impl LPUART1_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_IPG_DOZE_A {
        match self.bits {
            false => LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0,
            true => LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_doze_0(&self) -> bool {
        *self == LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_doze_1(&self) -> bool {
        *self == LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART1_IPG_DOZE`"]
pub struct LPUART1_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1)
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
#[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART2_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART2_IPG_STOP_MODE_1,
}
impl From<LPUART2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0 => false,
            LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART2_IPG_STOP_MODE`"]
pub type LPUART2_IPG_STOP_MODE_R = crate::R<bool, LPUART2_IPG_STOP_MODE_A>;
impl LPUART2_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0,
            true => LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART2_IPG_STOP_MODE`"]
pub struct LPUART2_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART2_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1)
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
#[doc = "LPUART2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART2_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART2_IPG_DOZE_1,
}
impl From<LPUART2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_IPG_DOZE_A) -> Self {
        match variant {
            LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0 => false,
            LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART2_IPG_DOZE`"]
pub type LPUART2_IPG_DOZE_R = crate::R<bool, LPUART2_IPG_DOZE_A>;
impl LPUART2_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_IPG_DOZE_A {
        match self.bits {
            false => LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0,
            true => LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_doze_0(&self) -> bool {
        *self == LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_doze_1(&self) -> bool {
        *self == LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART2_IPG_DOZE`"]
pub struct LPUART2_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART2_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1)
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
#[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART3_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART3_IPG_STOP_MODE_1,
}
impl From<LPUART3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0 => false,
            LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART3_IPG_STOP_MODE`"]
pub type LPUART3_IPG_STOP_MODE_R = crate::R<bool, LPUART3_IPG_STOP_MODE_A>;
impl LPUART3_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0,
            true => LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART3_IPG_STOP_MODE`"]
pub struct LPUART3_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART3_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART3_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1)
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
#[doc = "LPUART3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART3_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART3_IPG_DOZE_1,
}
impl From<LPUART3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_IPG_DOZE_A) -> Self {
        match variant {
            LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0 => false,
            LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART3_IPG_DOZE`"]
pub type LPUART3_IPG_DOZE_R = crate::R<bool, LPUART3_IPG_DOZE_A>;
impl LPUART3_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_IPG_DOZE_A {
        match self.bits {
            false => LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0,
            true => LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_doze_0(&self) -> bool {
        *self == LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_doze_1(&self) -> bool {
        *self == LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART3_IPG_DOZE`"]
pub struct LPUART3_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART3_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART3_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1)
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
#[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART4_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART4_IPG_STOP_MODE_1,
}
impl From<LPUART4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0 => false,
            LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART4_IPG_STOP_MODE`"]
pub type LPUART4_IPG_STOP_MODE_R = crate::R<bool, LPUART4_IPG_STOP_MODE_A>;
impl LPUART4_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0,
            true => LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART4_IPG_STOP_MODE`"]
pub struct LPUART4_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART4_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART4_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1)
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
#[doc = "LPUART4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART4_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART4_IPG_DOZE_1,
}
impl From<LPUART4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_IPG_DOZE_A) -> Self {
        match variant {
            LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0 => false,
            LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART4_IPG_DOZE`"]
pub type LPUART4_IPG_DOZE_R = crate::R<bool, LPUART4_IPG_DOZE_A>;
impl LPUART4_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_IPG_DOZE_A {
        match self.bits {
            false => LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0,
            true => LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_doze_0(&self) -> bool {
        *self == LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_doze_1(&self) -> bool {
        *self == LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART4_IPG_DOZE`"]
pub struct LPUART4_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART4_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART4_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1)
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
#[doc = "LPUART5 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART5_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART5_IPG_STOP_MODE_1,
}
impl From<LPUART5_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0 => false,
            LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART5_IPG_STOP_MODE`"]
pub type LPUART5_IPG_STOP_MODE_R = crate::R<bool, LPUART5_IPG_STOP_MODE_A>;
impl LPUART5_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0,
            true => LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART5_IPG_STOP_MODE`"]
pub struct LPUART5_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART5_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART5_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "LPUART5 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART5_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART5_IPG_DOZE_1,
}
impl From<LPUART5_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_IPG_DOZE_A) -> Self {
        match variant {
            LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0 => false,
            LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART5_IPG_DOZE`"]
pub type LPUART5_IPG_DOZE_R = crate::R<bool, LPUART5_IPG_DOZE_A>;
impl LPUART5_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_IPG_DOZE_A {
        match self.bits {
            false => LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0,
            true => LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_doze_0(&self) -> bool {
        *self == LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_doze_1(&self) -> bool {
        *self == LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART5_IPG_DOZE`"]
pub struct LPUART5_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART5_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART5_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "LPUART6 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART6_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART6_IPG_STOP_MODE_1,
}
impl From<LPUART6_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0 => false,
            LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART6_IPG_STOP_MODE`"]
pub type LPUART6_IPG_STOP_MODE_R = crate::R<bool, LPUART6_IPG_STOP_MODE_A>;
impl LPUART6_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0,
            true => LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART6_IPG_STOP_MODE`"]
pub struct LPUART6_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART6_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART6_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "LPUART6 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART6_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART6_IPG_DOZE_1,
}
impl From<LPUART6_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_IPG_DOZE_A) -> Self {
        match variant {
            LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0 => false,
            LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART6_IPG_DOZE`"]
pub type LPUART6_IPG_DOZE_R = crate::R<bool, LPUART6_IPG_DOZE_A>;
impl LPUART6_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_IPG_DOZE_A {
        match self.bits {
            false => LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0,
            true => LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_doze_0(&self) -> bool {
        *self == LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_doze_1(&self) -> bool {
        *self == LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART6_IPG_DOZE`"]
pub struct LPUART6_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART6_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART6_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "LPUART7 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART7_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART7_IPG_STOP_MODE_1,
}
impl From<LPUART7_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0 => false,
            LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART7_IPG_STOP_MODE`"]
pub type LPUART7_IPG_STOP_MODE_R = crate::R<bool, LPUART7_IPG_STOP_MODE_A>;
impl LPUART7_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0,
            true => LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART7_IPG_STOP_MODE`"]
pub struct LPUART7_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART7_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART7_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "LPUART7 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART7_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART7_IPG_DOZE_1,
}
impl From<LPUART7_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_IPG_DOZE_A) -> Self {
        match variant {
            LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0 => false,
            LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART7_IPG_DOZE`"]
pub type LPUART7_IPG_DOZE_R = crate::R<bool, LPUART7_IPG_DOZE_A>;
impl LPUART7_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_IPG_DOZE_A {
        match self.bits {
            false => LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0,
            true => LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_doze_0(&self) -> bool {
        *self == LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_doze_1(&self) -> bool {
        *self == LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART7_IPG_DOZE`"]
pub struct LPUART7_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART7_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART7_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "LPUART8 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART8_IPG_STOP_MODE_0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART8_IPG_STOP_MODE_1,
}
impl From<LPUART8_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_IPG_STOP_MODE_A) -> Self {
        match variant {
            LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0 => false,
            LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART8_IPG_STOP_MODE`"]
pub type LPUART8_IPG_STOP_MODE_R = crate::R<bool, LPUART8_IPG_STOP_MODE_A>;
impl LPUART8_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0,
            true => LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `LPUART8_IPG_STOP_MODE`"]
pub struct LPUART8_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART8_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART8_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1)
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
#[doc = "LPUART8 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART8_IPG_DOZE_0,
    #[doc = "1: in doze mode"]
    LPUART8_IPG_DOZE_1,
}
impl From<LPUART8_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_IPG_DOZE_A) -> Self {
        match variant {
            LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0 => false,
            LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1 => true,
        }
    }
}
#[doc = "Reader of field `LPUART8_IPG_DOZE`"]
pub type LPUART8_IPG_DOZE_R = crate::R<bool, LPUART8_IPG_DOZE_A>;
impl LPUART8_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_IPG_DOZE_A {
        match self.bits {
            false => LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0,
            true => LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_doze_0(&self) -> bool {
        *self == LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_doze_1(&self) -> bool {
        *self == LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `LPUART8_IPG_DOZE`"]
pub struct LPUART8_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART8_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART8_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode(&self) -> LPI2C1_IPG_STOP_MODE_R {
        LPI2C1_IPG_STOP_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze(&self) -> LPI2C1_IPG_DOZE_R {
        LPI2C1_IPG_DOZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode(&self) -> LPI2C2_IPG_STOP_MODE_R {
        LPI2C2_IPG_STOP_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze(&self) -> LPI2C2_IPG_DOZE_R {
        LPI2C2_IPG_DOZE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode(&self) -> LPI2C3_IPG_STOP_MODE_R {
        LPI2C3_IPG_STOP_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze(&self) -> LPI2C3_IPG_DOZE_R {
        LPI2C3_IPG_DOZE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode(&self) -> LPI2C4_IPG_STOP_MODE_R {
        LPI2C4_IPG_STOP_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze(&self) -> LPI2C4_IPG_DOZE_R {
        LPI2C4_IPG_DOZE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode(&self) -> LPSPI1_IPG_STOP_MODE_R {
        LPSPI1_IPG_STOP_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze(&self) -> LPSPI1_IPG_DOZE_R {
        LPSPI1_IPG_DOZE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode(&self) -> LPSPI2_IPG_STOP_MODE_R {
        LPSPI2_IPG_STOP_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze(&self) -> LPSPI2_IPG_DOZE_R {
        LPSPI2_IPG_DOZE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode(&self) -> LPSPI3_IPG_STOP_MODE_R {
        LPSPI3_IPG_STOP_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze(&self) -> LPSPI3_IPG_DOZE_R {
        LPSPI3_IPG_DOZE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode(&self) -> LPSPI4_IPG_STOP_MODE_R {
        LPSPI4_IPG_STOP_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze(&self) -> LPSPI4_IPG_DOZE_R {
        LPSPI4_IPG_DOZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode(&self) -> LPUART1_IPG_STOP_MODE_R {
        LPUART1_IPG_STOP_MODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze(&self) -> LPUART1_IPG_DOZE_R {
        LPUART1_IPG_DOZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode(&self) -> LPUART2_IPG_STOP_MODE_R {
        LPUART2_IPG_STOP_MODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze(&self) -> LPUART2_IPG_DOZE_R {
        LPUART2_IPG_DOZE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode(&self) -> LPUART3_IPG_STOP_MODE_R {
        LPUART3_IPG_STOP_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze(&self) -> LPUART3_IPG_DOZE_R {
        LPUART3_IPG_DOZE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode(&self) -> LPUART4_IPG_STOP_MODE_R {
        LPUART4_IPG_STOP_MODE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze(&self) -> LPUART4_IPG_DOZE_R {
        LPUART4_IPG_DOZE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode(&self) -> LPUART5_IPG_STOP_MODE_R {
        LPUART5_IPG_STOP_MODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze(&self) -> LPUART5_IPG_DOZE_R {
        LPUART5_IPG_DOZE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode(&self) -> LPUART6_IPG_STOP_MODE_R {
        LPUART6_IPG_STOP_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze(&self) -> LPUART6_IPG_DOZE_R {
        LPUART6_IPG_DOZE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode(&self) -> LPUART7_IPG_STOP_MODE_R {
        LPUART7_IPG_STOP_MODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze(&self) -> LPUART7_IPG_DOZE_R {
        LPUART7_IPG_DOZE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode(&self) -> LPUART8_IPG_STOP_MODE_R {
        LPUART8_IPG_STOP_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze(&self) -> LPUART8_IPG_DOZE_R {
        LPUART8_IPG_DOZE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode(&mut self) -> LPI2C1_IPG_STOP_MODE_W {
        LPI2C1_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze(&mut self) -> LPI2C1_IPG_DOZE_W {
        LPI2C1_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode(&mut self) -> LPI2C2_IPG_STOP_MODE_W {
        LPI2C2_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze(&mut self) -> LPI2C2_IPG_DOZE_W {
        LPI2C2_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode(&mut self) -> LPI2C3_IPG_STOP_MODE_W {
        LPI2C3_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze(&mut self) -> LPI2C3_IPG_DOZE_W {
        LPI2C3_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode(&mut self) -> LPI2C4_IPG_STOP_MODE_W {
        LPI2C4_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze(&mut self) -> LPI2C4_IPG_DOZE_W {
        LPI2C4_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode(&mut self) -> LPSPI1_IPG_STOP_MODE_W {
        LPSPI1_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze(&mut self) -> LPSPI1_IPG_DOZE_W {
        LPSPI1_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode(&mut self) -> LPSPI2_IPG_STOP_MODE_W {
        LPSPI2_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze(&mut self) -> LPSPI2_IPG_DOZE_W {
        LPSPI2_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode(&mut self) -> LPSPI3_IPG_STOP_MODE_W {
        LPSPI3_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze(&mut self) -> LPSPI3_IPG_DOZE_W {
        LPSPI3_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode(&mut self) -> LPSPI4_IPG_STOP_MODE_W {
        LPSPI4_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze(&mut self) -> LPSPI4_IPG_DOZE_W {
        LPSPI4_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode(&mut self) -> LPUART1_IPG_STOP_MODE_W {
        LPUART1_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze(&mut self) -> LPUART1_IPG_DOZE_W {
        LPUART1_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode(&mut self) -> LPUART2_IPG_STOP_MODE_W {
        LPUART2_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze(&mut self) -> LPUART2_IPG_DOZE_W {
        LPUART2_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode(&mut self) -> LPUART3_IPG_STOP_MODE_W {
        LPUART3_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze(&mut self) -> LPUART3_IPG_DOZE_W {
        LPUART3_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode(&mut self) -> LPUART4_IPG_STOP_MODE_W {
        LPUART4_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze(&mut self) -> LPUART4_IPG_DOZE_W {
        LPUART4_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode(&mut self) -> LPUART5_IPG_STOP_MODE_W {
        LPUART5_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze(&mut self) -> LPUART5_IPG_DOZE_W {
        LPUART5_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode(&mut self) -> LPUART6_IPG_STOP_MODE_W {
        LPUART6_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze(&mut self) -> LPUART6_IPG_DOZE_W {
        LPUART6_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode(&mut self) -> LPUART7_IPG_STOP_MODE_W {
        LPUART7_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze(&mut self) -> LPUART7_IPG_DOZE_W {
        LPUART7_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode(&mut self) -> LPUART8_IPG_STOP_MODE_W {
        LPUART8_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze(&mut self) -> LPUART8_IPG_DOZE_W {
        LPUART8_IPG_DOZE_W { w: self }
    }
}
