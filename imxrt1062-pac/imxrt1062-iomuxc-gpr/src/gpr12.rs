#[doc = "Reader of register GPR12"]
pub type R = crate::R<u32, super::GPR12>;
#[doc = "Writer for register GPR12"]
pub type W = crate::W<u32, super::GPR12>;
#[doc = "Register GPR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_IPG_STOP_MODE_A {
    #[doc = "0: FlexIO1 is functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_1 = 1,
}
impl From<FLEXIO1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO1_IPG_STOP_MODE`"]
pub type FLEXIO1_IPG_STOP_MODE_R = crate::R<bool, FLEXIO1_IPG_STOP_MODE_A>;
impl FLEXIO1_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_IPG_STOP_MODE_A {
        match self.bits {
            false => FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0,
            true => FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `FLEXIO1_IPG_STOP_MODE`"]
pub struct FLEXIO1_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexIO1 is functional in Stop mode."]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_STOP_MODE_A::FLEXIO1_IPG_STOP_MODE_1)
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
#[doc = "FLEXIO1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_IPG_DOZE_A {
    #[doc = "0: FLEXIO1 is not in doze mode"]
    FLEXIO1_IPG_DOZE_0 = 0,
    #[doc = "1: FLEXIO1 is in doze mode"]
    FLEXIO1_IPG_DOZE_1 = 1,
}
impl From<FLEXIO1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO1_IPG_DOZE`"]
pub type FLEXIO1_IPG_DOZE_R = crate::R<bool, FLEXIO1_IPG_DOZE_A>;
impl FLEXIO1_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_IPG_DOZE_A {
        match self.bits {
            false => FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0,
            true => FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_doze_0(&self) -> bool {
        *self == FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_flexio1_ipg_doze_1(&self) -> bool {
        *self == FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `FLEXIO1_IPG_DOZE`"]
pub struct FLEXIO1_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLEXIO1 is not in doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_0)
    }
    #[doc = "FLEXIO1 is in doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO1_IPG_DOZE_A::FLEXIO1_IPG_DOZE_1)
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
#[doc = "FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_IPG_STOP_MODE_A {
    #[doc = "0: FlexIO2 is functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    FLEXIO2_IPG_STOP_MODE_1 = 1,
}
impl From<FLEXIO2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO2_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO2_IPG_STOP_MODE`"]
pub type FLEXIO2_IPG_STOP_MODE_R = crate::R<bool, FLEXIO2_IPG_STOP_MODE_A>;
impl FLEXIO2_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO2_IPG_STOP_MODE_A {
        match self.bits {
            false => FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_0,
            true => FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_flexio2_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_flexio2_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `FLEXIO2_IPG_STOP_MODE`"]
pub struct FLEXIO2_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO2_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO2_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexIO2 is functional in Stop mode."]
    #[inline(always)]
    pub fn flexio2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    #[inline(always)]
    pub fn flexio2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_STOP_MODE_A::FLEXIO2_IPG_STOP_MODE_1)
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
#[doc = "FLEXIO2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_IPG_DOZE_A {
    #[doc = "0: FLEXIO2 is not in doze mode"]
    FLEXIO2_IPG_DOZE_0 = 0,
    #[doc = "1: FLEXIO2 is in doze mode"]
    FLEXIO2_IPG_DOZE_1 = 1,
}
impl From<FLEXIO2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO2_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO2_IPG_DOZE`"]
pub type FLEXIO2_IPG_DOZE_R = crate::R<bool, FLEXIO2_IPG_DOZE_A>;
impl FLEXIO2_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO2_IPG_DOZE_A {
        match self.bits {
            false => FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_0,
            true => FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_flexio2_ipg_doze_0(&self) -> bool {
        *self == FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_flexio2_ipg_doze_1(&self) -> bool {
        *self == FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `FLEXIO2_IPG_DOZE`"]
pub struct FLEXIO2_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO2_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO2_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLEXIO2 is not in doze mode"]
    #[inline(always)]
    pub fn flexio2_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_0)
    }
    #[doc = "FLEXIO2 is in doze mode"]
    #[inline(always)]
    pub fn flexio2_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO2_IPG_DOZE_A::FLEXIO2_IPG_DOZE_1)
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
#[doc = "ACMP stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_IPG_STOP_MODE_A {
    #[doc = "0: ACMP is functional in Stop mode."]
    ACMP_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    ACMP_IPG_STOP_MODE_1 = 1,
}
impl From<ACMP_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_IPG_STOP_MODE`"]
pub type ACMP_IPG_STOP_MODE_R = crate::R<bool, ACMP_IPG_STOP_MODE_A>;
impl ACMP_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_IPG_STOP_MODE_A {
        match self.bits {
            false => ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0,
            true => ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_acmp_ipg_stop_mode_0(&self) -> bool {
        *self == ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `ACMP_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_acmp_ipg_stop_mode_1(&self) -> bool {
        *self == ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `ACMP_IPG_STOP_MODE`"]
pub struct ACMP_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP is functional in Stop mode."]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(ACMP_IPG_STOP_MODE_A::ACMP_IPG_STOP_MODE_1)
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
#[doc = "FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO3_IPG_STOP_MODE_A {
    #[doc = "0: FlexIO3 is functional in Stop mode."]
    FLEXIO3_IPG_STOP_MODE_0 = 0,
    #[doc = "1: When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO3 is not functional in Stop mode."]
    FLEXIO3_IPG_STOP_MODE_1 = 1,
}
impl From<FLEXIO3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO3_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO3_IPG_STOP_MODE`"]
pub type FLEXIO3_IPG_STOP_MODE_R = crate::R<bool, FLEXIO3_IPG_STOP_MODE_A>;
impl FLEXIO3_IPG_STOP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO3_IPG_STOP_MODE_A {
        match self.bits {
            false => FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_0,
            true => FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_flexio3_ipg_stop_mode_0(&self) -> bool {
        *self == FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_flexio3_ipg_stop_mode_1(&self) -> bool {
        *self == FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_1
    }
}
#[doc = "Write proxy for field `FLEXIO3_IPG_STOP_MODE`"]
pub struct FLEXIO3_IPG_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO3_IPG_STOP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO3_IPG_STOP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexIO3 is functional in Stop mode."]
    #[inline(always)]
    pub fn flexio3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_0)
    }
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO3 is not functional in Stop mode."]
    #[inline(always)]
    pub fn flexio3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(FLEXIO3_IPG_STOP_MODE_A::FLEXIO3_IPG_STOP_MODE_1)
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
#[doc = "FLEXIO3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO3_IPG_DOZE_A {
    #[doc = "0: FLEXIO3 is not in doze mode"]
    FLEXIO3_IPG_DOZE_0 = 0,
    #[doc = "1: FLEXIO3 is in doze mode"]
    FLEXIO3_IPG_DOZE_1 = 1,
}
impl From<FLEXIO3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO3_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLEXIO3_IPG_DOZE`"]
pub type FLEXIO3_IPG_DOZE_R = crate::R<bool, FLEXIO3_IPG_DOZE_A>;
impl FLEXIO3_IPG_DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO3_IPG_DOZE_A {
        match self.bits {
            false => FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_0,
            true => FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_flexio3_ipg_doze_0(&self) -> bool {
        *self == FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_flexio3_ipg_doze_1(&self) -> bool {
        *self == FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_1
    }
}
#[doc = "Write proxy for field `FLEXIO3_IPG_DOZE`"]
pub struct FLEXIO3_IPG_DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO3_IPG_DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO3_IPG_DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FLEXIO3 is not in doze mode"]
    #[inline(always)]
    pub fn flexio3_ipg_doze_0(self) -> &'a mut W {
        self.variant(FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_0)
    }
    #[doc = "FLEXIO3 is in doze mode"]
    #[inline(always)]
    pub fn flexio3_ipg_doze_1(self) -> &'a mut W {
        self.variant(FLEXIO3_IPG_DOZE_A::FLEXIO3_IPG_DOZE_1)
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
impl R {
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode(&self) -> FLEXIO1_IPG_STOP_MODE_R {
        FLEXIO1_IPG_STOP_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze(&self) -> FLEXIO1_IPG_DOZE_R {
        FLEXIO1_IPG_DOZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio2_ipg_stop_mode(&self) -> FLEXIO2_IPG_STOP_MODE_R {
        FLEXIO2_IPG_STOP_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FLEXIO2 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio2_ipg_doze(&self) -> FLEXIO2_IPG_DOZE_R {
        FLEXIO2_IPG_DOZE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode(&self) -> ACMP_IPG_STOP_MODE_R {
        ACMP_IPG_STOP_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio3_ipg_stop_mode(&self) -> FLEXIO3_IPG_STOP_MODE_R {
        FLEXIO3_IPG_STOP_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLEXIO3 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio3_ipg_doze(&self) -> FLEXIO3_IPG_DOZE_R {
        FLEXIO3_IPG_DOZE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio1_ipg_stop_mode(&mut self) -> FLEXIO1_IPG_STOP_MODE_W {
        FLEXIO1_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 1 - FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio1_ipg_doze(&mut self) -> FLEXIO1_IPG_DOZE_W {
        FLEXIO1_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 2 - FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio2_ipg_stop_mode(&mut self) -> FLEXIO2_IPG_STOP_MODE_W {
        FLEXIO2_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 3 - FLEXIO2 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio2_ipg_doze(&mut self) -> FLEXIO2_IPG_DOZE_W {
        FLEXIO2_IPG_DOZE_W { w: self }
    }
    #[doc = "Bit 4 - ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn acmp_ipg_stop_mode(&mut self) -> ACMP_IPG_STOP_MODE_W {
        ACMP_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 5 - FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn flexio3_ipg_stop_mode(&mut self) -> FLEXIO3_IPG_STOP_MODE_W {
        FLEXIO3_IPG_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 6 - FLEXIO3 ipg_doze mode"]
    #[inline(always)]
    pub fn flexio3_ipg_doze(&mut self) -> FLEXIO3_IPG_DOZE_W {
        FLEXIO3_IPG_DOZE_W { w: self }
    }
}
