#[doc = "Reader of register GPR6"]
pub type R = crate::R<u32, super::GPR6>;
#[doc = "Writer for register GPR6"]
pub type W = crate::W<u32, super::GPR6>;
#[doc = "Register GPR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "QTIMER1 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM0_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM0_INPUT_SEL_1,
}
impl From<QTIMER1_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM0_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0 => false,
            QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER1_TRM0_INPUT_SEL`"]
pub type QTIMER1_TRM0_INPUT_SEL_R = crate::R<bool, QTIMER1_TRM0_INPUT_SEL_A>;
impl QTIMER1_TRM0_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0,
            true => QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER1_TRM0_INPUT_SEL`"]
pub struct QTIMER1_TRM0_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER1_TRM0_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER1_TRM0_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1)
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
#[doc = "QTIMER1 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM1_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM1_INPUT_SEL_1,
}
impl From<QTIMER1_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM1_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0 => false,
            QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER1_TRM1_INPUT_SEL`"]
pub type QTIMER1_TRM1_INPUT_SEL_R = crate::R<bool, QTIMER1_TRM1_INPUT_SEL_A>;
impl QTIMER1_TRM1_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0,
            true => QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER1_TRM1_INPUT_SEL`"]
pub struct QTIMER1_TRM1_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER1_TRM1_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER1_TRM1_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1)
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
#[doc = "QTIMER1 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM2_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM2_INPUT_SEL_1,
}
impl From<QTIMER1_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM2_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0 => false,
            QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER1_TRM2_INPUT_SEL`"]
pub type QTIMER1_TRM2_INPUT_SEL_R = crate::R<bool, QTIMER1_TRM2_INPUT_SEL_A>;
impl QTIMER1_TRM2_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0,
            true => QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER1_TRM2_INPUT_SEL`"]
pub struct QTIMER1_TRM2_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER1_TRM2_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER1_TRM2_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1)
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
#[doc = "QTIMER1 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM3_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM3_INPUT_SEL_1,
}
impl From<QTIMER1_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM3_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0 => false,
            QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER1_TRM3_INPUT_SEL`"]
pub type QTIMER1_TRM3_INPUT_SEL_R = crate::R<bool, QTIMER1_TRM3_INPUT_SEL_A>;
impl QTIMER1_TRM3_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0,
            true => QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER1_TRM3_INPUT_SEL`"]
pub struct QTIMER1_TRM3_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER1_TRM3_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER1_TRM3_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1)
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
#[doc = "QTIMER2 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM0_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM0_INPUT_SEL_1,
}
impl From<QTIMER2_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM0_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0 => false,
            QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER2_TRM0_INPUT_SEL`"]
pub type QTIMER2_TRM0_INPUT_SEL_R = crate::R<bool, QTIMER2_TRM0_INPUT_SEL_A>;
impl QTIMER2_TRM0_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0,
            true => QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER2_TRM0_INPUT_SEL`"]
pub struct QTIMER2_TRM0_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER2_TRM0_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER2_TRM0_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1)
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
#[doc = "QTIMER2 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM1_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM1_INPUT_SEL_1,
}
impl From<QTIMER2_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM1_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0 => false,
            QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER2_TRM1_INPUT_SEL`"]
pub type QTIMER2_TRM1_INPUT_SEL_R = crate::R<bool, QTIMER2_TRM1_INPUT_SEL_A>;
impl QTIMER2_TRM1_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0,
            true => QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER2_TRM1_INPUT_SEL`"]
pub struct QTIMER2_TRM1_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER2_TRM1_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER2_TRM1_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1)
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
#[doc = "QTIMER2 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM2_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM2_INPUT_SEL_1,
}
impl From<QTIMER2_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM2_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0 => false,
            QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER2_TRM2_INPUT_SEL`"]
pub type QTIMER2_TRM2_INPUT_SEL_R = crate::R<bool, QTIMER2_TRM2_INPUT_SEL_A>;
impl QTIMER2_TRM2_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0,
            true => QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER2_TRM2_INPUT_SEL`"]
pub struct QTIMER2_TRM2_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER2_TRM2_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER2_TRM2_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1)
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
#[doc = "QTIMER2 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM3_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM3_INPUT_SEL_1,
}
impl From<QTIMER2_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM3_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0 => false,
            QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER2_TRM3_INPUT_SEL`"]
pub type QTIMER2_TRM3_INPUT_SEL_R = crate::R<bool, QTIMER2_TRM3_INPUT_SEL_A>;
impl QTIMER2_TRM3_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0,
            true => QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER2_TRM3_INPUT_SEL`"]
pub struct QTIMER2_TRM3_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER2_TRM3_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER2_TRM3_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1)
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
#[doc = "QTIMER3 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER3_TRM0_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER3_TRM0_INPUT_SEL_1,
}
impl From<QTIMER3_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER3_TRM0_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_0 => false,
            QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER3_TRM0_INPUT_SEL`"]
pub type QTIMER3_TRM0_INPUT_SEL_R = crate::R<bool, QTIMER3_TRM0_INPUT_SEL_A>;
impl QTIMER3_TRM0_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER3_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_0,
            true => QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer3_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer3_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER3_TRM0_INPUT_SEL`"]
pub struct QTIMER3_TRM0_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER3_TRM0_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER3_TRM0_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer3_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer3_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM0_INPUT_SEL_A::QTIMER3_TRM0_INPUT_SEL_1)
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
#[doc = "QTIMER3 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER3_TRM1_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER3_TRM1_INPUT_SEL_1,
}
impl From<QTIMER3_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER3_TRM1_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_0 => false,
            QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER3_TRM1_INPUT_SEL`"]
pub type QTIMER3_TRM1_INPUT_SEL_R = crate::R<bool, QTIMER3_TRM1_INPUT_SEL_A>;
impl QTIMER3_TRM1_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER3_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_0,
            true => QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer3_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer3_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER3_TRM1_INPUT_SEL`"]
pub struct QTIMER3_TRM1_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER3_TRM1_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER3_TRM1_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer3_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer3_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM1_INPUT_SEL_A::QTIMER3_TRM1_INPUT_SEL_1)
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
#[doc = "QTIMER3 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER3_TRM2_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER3_TRM2_INPUT_SEL_1,
}
impl From<QTIMER3_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER3_TRM2_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_0 => false,
            QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER3_TRM2_INPUT_SEL`"]
pub type QTIMER3_TRM2_INPUT_SEL_R = crate::R<bool, QTIMER3_TRM2_INPUT_SEL_A>;
impl QTIMER3_TRM2_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER3_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_0,
            true => QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer3_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer3_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER3_TRM2_INPUT_SEL`"]
pub struct QTIMER3_TRM2_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER3_TRM2_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER3_TRM2_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer3_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer3_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM2_INPUT_SEL_A::QTIMER3_TRM2_INPUT_SEL_1)
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
#[doc = "QTIMER3 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER3_TRM3_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER3_TRM3_INPUT_SEL_1,
}
impl From<QTIMER3_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER3_TRM3_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_0 => false,
            QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER3_TRM3_INPUT_SEL`"]
pub type QTIMER3_TRM3_INPUT_SEL_R = crate::R<bool, QTIMER3_TRM3_INPUT_SEL_A>;
impl QTIMER3_TRM3_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER3_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_0,
            true => QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer3_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer3_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER3_TRM3_INPUT_SEL`"]
pub struct QTIMER3_TRM3_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER3_TRM3_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER3_TRM3_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer3_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer3_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER3_TRM3_INPUT_SEL_A::QTIMER3_TRM3_INPUT_SEL_1)
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
#[doc = "QTIMER4 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER4_TRM0_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER4_TRM0_INPUT_SEL_1,
}
impl From<QTIMER4_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER4_TRM0_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_0 => false,
            QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER4_TRM0_INPUT_SEL`"]
pub type QTIMER4_TRM0_INPUT_SEL_R = crate::R<bool, QTIMER4_TRM0_INPUT_SEL_A>;
impl QTIMER4_TRM0_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER4_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_0,
            true => QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer4_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer4_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER4_TRM0_INPUT_SEL`"]
pub struct QTIMER4_TRM0_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER4_TRM0_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER4_TRM0_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer4_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer4_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM0_INPUT_SEL_A::QTIMER4_TRM0_INPUT_SEL_1)
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
#[doc = "QTIMER4 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER4_TRM1_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER4_TRM1_INPUT_SEL_1,
}
impl From<QTIMER4_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER4_TRM1_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_0 => false,
            QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER4_TRM1_INPUT_SEL`"]
pub type QTIMER4_TRM1_INPUT_SEL_R = crate::R<bool, QTIMER4_TRM1_INPUT_SEL_A>;
impl QTIMER4_TRM1_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER4_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_0,
            true => QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer4_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer4_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER4_TRM1_INPUT_SEL`"]
pub struct QTIMER4_TRM1_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER4_TRM1_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER4_TRM1_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer4_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer4_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM1_INPUT_SEL_A::QTIMER4_TRM1_INPUT_SEL_1)
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
#[doc = "QTIMER4 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER4_TRM2_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER4_TRM2_INPUT_SEL_1,
}
impl From<QTIMER4_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER4_TRM2_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_0 => false,
            QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER4_TRM2_INPUT_SEL`"]
pub type QTIMER4_TRM2_INPUT_SEL_R = crate::R<bool, QTIMER4_TRM2_INPUT_SEL_A>;
impl QTIMER4_TRM2_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER4_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_0,
            true => QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer4_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer4_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER4_TRM2_INPUT_SEL`"]
pub struct QTIMER4_TRM2_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER4_TRM2_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER4_TRM2_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer4_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer4_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM2_INPUT_SEL_A::QTIMER4_TRM2_INPUT_SEL_1)
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
#[doc = "QTIMER4 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER4_TRM3_INPUT_SEL_0,
    #[doc = "1: input from XBAR"]
    QTIMER4_TRM3_INPUT_SEL_1,
}
impl From<QTIMER4_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER4_TRM3_INPUT_SEL_A) -> Self {
        match variant {
            QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_0 => false,
            QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `QTIMER4_TRM3_INPUT_SEL`"]
pub type QTIMER4_TRM3_INPUT_SEL_R = crate::R<bool, QTIMER4_TRM3_INPUT_SEL_A>;
impl QTIMER4_TRM3_INPUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER4_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_0,
            true => QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer4_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer4_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_1
    }
}
#[doc = "Write proxy for field `QTIMER4_TRM3_INPUT_SEL`"]
pub struct QTIMER4_TRM3_INPUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER4_TRM3_INPUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER4_TRM3_INPUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer4_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer4_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER4_TRM3_INPUT_SEL_A::QTIMER4_TRM3_INPUT_SEL_1)
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
#[doc = "IOMUXC XBAR_INOUT4 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_4_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_4_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_4_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_4_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_4_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0 => false,
            IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_4`"]
pub type IOMUXC_XBAR_DIR_SEL_4_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_4_A>;
impl IOMUXC_XBAR_DIR_SEL_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_4_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0,
            true => IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_4_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_4_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_4`"]
pub struct IOMUXC_XBAR_DIR_SEL_4_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1)
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
#[doc = "IOMUXC XBAR_INOUT5 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_5_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_5_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_5_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_5_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_5_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0 => false,
            IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_5`"]
pub type IOMUXC_XBAR_DIR_SEL_5_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_5_A>;
impl IOMUXC_XBAR_DIR_SEL_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_5_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0,
            true => IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_5_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_5_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_5`"]
pub struct IOMUXC_XBAR_DIR_SEL_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1)
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
#[doc = "IOMUXC XBAR_INOUT6 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_6_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_6_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_6_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_6_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_6_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0 => false,
            IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_6`"]
pub type IOMUXC_XBAR_DIR_SEL_6_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_6_A>;
impl IOMUXC_XBAR_DIR_SEL_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_6_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0,
            true => IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_6_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_6_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_6`"]
pub struct IOMUXC_XBAR_DIR_SEL_6_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1)
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
#[doc = "IOMUXC XBAR_INOUT7 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_7_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_7_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_7_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_7_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_7_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0 => false,
            IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_7`"]
pub type IOMUXC_XBAR_DIR_SEL_7_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_7_A>;
impl IOMUXC_XBAR_DIR_SEL_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_7_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0,
            true => IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_7_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_7_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_7`"]
pub struct IOMUXC_XBAR_DIR_SEL_7_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1)
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
#[doc = "IOMUXC XBAR_INOUT8 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_8_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_8_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_8_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_8_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_8_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0 => false,
            IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_8`"]
pub type IOMUXC_XBAR_DIR_SEL_8_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_8_A>;
impl IOMUXC_XBAR_DIR_SEL_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_8_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0,
            true => IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_8_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_8_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_8`"]
pub struct IOMUXC_XBAR_DIR_SEL_8_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1)
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
#[doc = "IOMUXC XBAR_INOUT9 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_9_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_9_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_9_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_9_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_9_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0 => false,
            IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_9`"]
pub type IOMUXC_XBAR_DIR_SEL_9_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_9_A>;
impl IOMUXC_XBAR_DIR_SEL_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_9_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0,
            true => IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_9_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_9_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_9`"]
pub struct IOMUXC_XBAR_DIR_SEL_9_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1)
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
#[doc = "IOMUXC XBAR_INOUT10 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_10_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_10_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_10_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_10_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_10_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0 => false,
            IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_10`"]
pub type IOMUXC_XBAR_DIR_SEL_10_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_10_A>;
impl IOMUXC_XBAR_DIR_SEL_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_10_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0,
            true => IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_10_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_10_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_10`"]
pub struct IOMUXC_XBAR_DIR_SEL_10_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1)
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
#[doc = "IOMUXC XBAR_INOUT11 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_11_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_11_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_11_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_11_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_11_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0 => false,
            IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_11`"]
pub type IOMUXC_XBAR_DIR_SEL_11_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_11_A>;
impl IOMUXC_XBAR_DIR_SEL_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_11_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0,
            true => IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_11_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_11_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_11`"]
pub struct IOMUXC_XBAR_DIR_SEL_11_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1)
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
#[doc = "IOMUXC XBAR_INOUT12 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_12_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_12_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_12_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_12_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_12_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0 => false,
            IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_12`"]
pub type IOMUXC_XBAR_DIR_SEL_12_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_12_A>;
impl IOMUXC_XBAR_DIR_SEL_12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_12_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0,
            true => IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_12_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_12_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_12`"]
pub struct IOMUXC_XBAR_DIR_SEL_12_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1)
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
#[doc = "IOMUXC XBAR_INOUT13 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_13_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_13_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_13_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_13_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_13_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0 => false,
            IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_13`"]
pub type IOMUXC_XBAR_DIR_SEL_13_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_13_A>;
impl IOMUXC_XBAR_DIR_SEL_13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_13_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0,
            true => IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_13_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_13_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_13`"]
pub struct IOMUXC_XBAR_DIR_SEL_13_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1)
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
#[doc = "IOMUXC XBAR_INOUT14 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_14_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_14_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_14_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_14_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_14_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0 => false,
            IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_14`"]
pub type IOMUXC_XBAR_DIR_SEL_14_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_14_A>;
impl IOMUXC_XBAR_DIR_SEL_14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_14_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0,
            true => IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_14_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_14_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_14`"]
pub struct IOMUXC_XBAR_DIR_SEL_14_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1)
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
#[doc = "IOMUXC XBAR_INOUT15 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_15_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_15_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_15_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_15_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_15_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0 => false,
            IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_15`"]
pub type IOMUXC_XBAR_DIR_SEL_15_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_15_A>;
impl IOMUXC_XBAR_DIR_SEL_15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_15_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0,
            true => IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_15_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_15_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_15`"]
pub struct IOMUXC_XBAR_DIR_SEL_15_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1)
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
#[doc = "IOMUXC XBAR_INOUT16 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_16_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_16_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_16_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_16_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_16_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0 => false,
            IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_16`"]
pub type IOMUXC_XBAR_DIR_SEL_16_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_16_A>;
impl IOMUXC_XBAR_DIR_SEL_16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_16_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0,
            true => IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_16_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_16_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_16`"]
pub struct IOMUXC_XBAR_DIR_SEL_16_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1)
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
#[doc = "IOMUXC XBAR_INOUT17 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_17_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_17_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_17_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_17_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_17_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0 => false,
            IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_17`"]
pub type IOMUXC_XBAR_DIR_SEL_17_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_17_A>;
impl IOMUXC_XBAR_DIR_SEL_17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_17_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0,
            true => IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_17_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_17_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_17`"]
pub struct IOMUXC_XBAR_DIR_SEL_17_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1)
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
#[doc = "IOMUXC XBAR_INOUT18 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_18_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_18_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_18_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_18_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_18_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0 => false,
            IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_18`"]
pub type IOMUXC_XBAR_DIR_SEL_18_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_18_A>;
impl IOMUXC_XBAR_DIR_SEL_18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_18_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0,
            true => IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_18_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_18_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_18`"]
pub struct IOMUXC_XBAR_DIR_SEL_18_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1)
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
#[doc = "IOMUXC XBAR_INOUT19 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMUXC_XBAR_DIR_SEL_19_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_19_0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_19_1,
}
impl From<IOMUXC_XBAR_DIR_SEL_19_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_19_A) -> Self {
        match variant {
            IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0 => false,
            IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1 => true,
        }
    }
}
#[doc = "Reader of field `IOMUXC_XBAR_DIR_SEL_19`"]
pub type IOMUXC_XBAR_DIR_SEL_19_R = crate::R<bool, IOMUXC_XBAR_DIR_SEL_19_A>;
impl IOMUXC_XBAR_DIR_SEL_19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_19_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0,
            true => IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_19_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_19_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1
    }
}
#[doc = "Write proxy for field `IOMUXC_XBAR_DIR_SEL_19`"]
pub struct IOMUXC_XBAR_DIR_SEL_19_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUXC_XBAR_DIR_SEL_19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOMUXC_XBAR_DIR_SEL_19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1)
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
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel(&self) -> QTIMER1_TRM0_INPUT_SEL_R {
        QTIMER1_TRM0_INPUT_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel(&self) -> QTIMER1_TRM1_INPUT_SEL_R {
        QTIMER1_TRM1_INPUT_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel(&self) -> QTIMER1_TRM2_INPUT_SEL_R {
        QTIMER1_TRM2_INPUT_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel(&self) -> QTIMER1_TRM3_INPUT_SEL_R {
        QTIMER1_TRM3_INPUT_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel(&self) -> QTIMER2_TRM0_INPUT_SEL_R {
        QTIMER2_TRM0_INPUT_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel(&self) -> QTIMER2_TRM1_INPUT_SEL_R {
        QTIMER2_TRM1_INPUT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel(&self) -> QTIMER2_TRM2_INPUT_SEL_R {
        QTIMER2_TRM2_INPUT_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel(&self) -> QTIMER2_TRM3_INPUT_SEL_R {
        QTIMER2_TRM3_INPUT_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - QTIMER3 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer3_trm0_input_sel(&self) -> QTIMER3_TRM0_INPUT_SEL_R {
        QTIMER3_TRM0_INPUT_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - QTIMER3 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer3_trm1_input_sel(&self) -> QTIMER3_TRM1_INPUT_SEL_R {
        QTIMER3_TRM1_INPUT_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - QTIMER3 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer3_trm2_input_sel(&self) -> QTIMER3_TRM2_INPUT_SEL_R {
        QTIMER3_TRM2_INPUT_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - QTIMER3 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer3_trm3_input_sel(&self) -> QTIMER3_TRM3_INPUT_SEL_R {
        QTIMER3_TRM3_INPUT_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - QTIMER4 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer4_trm0_input_sel(&self) -> QTIMER4_TRM0_INPUT_SEL_R {
        QTIMER4_TRM0_INPUT_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - QTIMER4 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer4_trm1_input_sel(&self) -> QTIMER4_TRM1_INPUT_SEL_R {
        QTIMER4_TRM1_INPUT_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QTIMER4 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer4_trm2_input_sel(&self) -> QTIMER4_TRM2_INPUT_SEL_R {
        QTIMER4_TRM2_INPUT_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - QTIMER4 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer4_trm3_input_sel(&self) -> QTIMER4_TRM3_INPUT_SEL_R {
        QTIMER4_TRM3_INPUT_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4(&self) -> IOMUXC_XBAR_DIR_SEL_4_R {
        IOMUXC_XBAR_DIR_SEL_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5(&self) -> IOMUXC_XBAR_DIR_SEL_5_R {
        IOMUXC_XBAR_DIR_SEL_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6(&self) -> IOMUXC_XBAR_DIR_SEL_6_R {
        IOMUXC_XBAR_DIR_SEL_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7(&self) -> IOMUXC_XBAR_DIR_SEL_7_R {
        IOMUXC_XBAR_DIR_SEL_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8(&self) -> IOMUXC_XBAR_DIR_SEL_8_R {
        IOMUXC_XBAR_DIR_SEL_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9(&self) -> IOMUXC_XBAR_DIR_SEL_9_R {
        IOMUXC_XBAR_DIR_SEL_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10(&self) -> IOMUXC_XBAR_DIR_SEL_10_R {
        IOMUXC_XBAR_DIR_SEL_10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11(&self) -> IOMUXC_XBAR_DIR_SEL_11_R {
        IOMUXC_XBAR_DIR_SEL_11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12(&self) -> IOMUXC_XBAR_DIR_SEL_12_R {
        IOMUXC_XBAR_DIR_SEL_12_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13(&self) -> IOMUXC_XBAR_DIR_SEL_13_R {
        IOMUXC_XBAR_DIR_SEL_13_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14(&self) -> IOMUXC_XBAR_DIR_SEL_14_R {
        IOMUXC_XBAR_DIR_SEL_14_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15(&self) -> IOMUXC_XBAR_DIR_SEL_15_R {
        IOMUXC_XBAR_DIR_SEL_15_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16(&self) -> IOMUXC_XBAR_DIR_SEL_16_R {
        IOMUXC_XBAR_DIR_SEL_16_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17(&self) -> IOMUXC_XBAR_DIR_SEL_17_R {
        IOMUXC_XBAR_DIR_SEL_17_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18(&self) -> IOMUXC_XBAR_DIR_SEL_18_R {
        IOMUXC_XBAR_DIR_SEL_18_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19(&self) -> IOMUXC_XBAR_DIR_SEL_19_R {
        IOMUXC_XBAR_DIR_SEL_19_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel(&mut self) -> QTIMER1_TRM0_INPUT_SEL_W {
        QTIMER1_TRM0_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel(&mut self) -> QTIMER1_TRM1_INPUT_SEL_W {
        QTIMER1_TRM1_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel(&mut self) -> QTIMER1_TRM2_INPUT_SEL_W {
        QTIMER1_TRM2_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel(&mut self) -> QTIMER1_TRM3_INPUT_SEL_W {
        QTIMER1_TRM3_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel(&mut self) -> QTIMER2_TRM0_INPUT_SEL_W {
        QTIMER2_TRM0_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel(&mut self) -> QTIMER2_TRM1_INPUT_SEL_W {
        QTIMER2_TRM1_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel(&mut self) -> QTIMER2_TRM2_INPUT_SEL_W {
        QTIMER2_TRM2_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel(&mut self) -> QTIMER2_TRM3_INPUT_SEL_W {
        QTIMER2_TRM3_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 8 - QTIMER3 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer3_trm0_input_sel(&mut self) -> QTIMER3_TRM0_INPUT_SEL_W {
        QTIMER3_TRM0_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 9 - QTIMER3 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer3_trm1_input_sel(&mut self) -> QTIMER3_TRM1_INPUT_SEL_W {
        QTIMER3_TRM1_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 10 - QTIMER3 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer3_trm2_input_sel(&mut self) -> QTIMER3_TRM2_INPUT_SEL_W {
        QTIMER3_TRM2_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 11 - QTIMER3 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer3_trm3_input_sel(&mut self) -> QTIMER3_TRM3_INPUT_SEL_W {
        QTIMER3_TRM3_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 12 - QTIMER4 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer4_trm0_input_sel(&mut self) -> QTIMER4_TRM0_INPUT_SEL_W {
        QTIMER4_TRM0_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 13 - QTIMER4 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer4_trm1_input_sel(&mut self) -> QTIMER4_TRM1_INPUT_SEL_W {
        QTIMER4_TRM1_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 14 - QTIMER4 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer4_trm2_input_sel(&mut self) -> QTIMER4_TRM2_INPUT_SEL_W {
        QTIMER4_TRM2_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 15 - QTIMER4 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer4_trm3_input_sel(&mut self) -> QTIMER4_TRM3_INPUT_SEL_W {
        QTIMER4_TRM3_INPUT_SEL_W { w: self }
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4(&mut self) -> IOMUXC_XBAR_DIR_SEL_4_W {
        IOMUXC_XBAR_DIR_SEL_4_W { w: self }
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5(&mut self) -> IOMUXC_XBAR_DIR_SEL_5_W {
        IOMUXC_XBAR_DIR_SEL_5_W { w: self }
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6(&mut self) -> IOMUXC_XBAR_DIR_SEL_6_W {
        IOMUXC_XBAR_DIR_SEL_6_W { w: self }
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7(&mut self) -> IOMUXC_XBAR_DIR_SEL_7_W {
        IOMUXC_XBAR_DIR_SEL_7_W { w: self }
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8(&mut self) -> IOMUXC_XBAR_DIR_SEL_8_W {
        IOMUXC_XBAR_DIR_SEL_8_W { w: self }
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9(&mut self) -> IOMUXC_XBAR_DIR_SEL_9_W {
        IOMUXC_XBAR_DIR_SEL_9_W { w: self }
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10(&mut self) -> IOMUXC_XBAR_DIR_SEL_10_W {
        IOMUXC_XBAR_DIR_SEL_10_W { w: self }
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11(&mut self) -> IOMUXC_XBAR_DIR_SEL_11_W {
        IOMUXC_XBAR_DIR_SEL_11_W { w: self }
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12(&mut self) -> IOMUXC_XBAR_DIR_SEL_12_W {
        IOMUXC_XBAR_DIR_SEL_12_W { w: self }
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13(&mut self) -> IOMUXC_XBAR_DIR_SEL_13_W {
        IOMUXC_XBAR_DIR_SEL_13_W { w: self }
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14(&mut self) -> IOMUXC_XBAR_DIR_SEL_14_W {
        IOMUXC_XBAR_DIR_SEL_14_W { w: self }
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15(&mut self) -> IOMUXC_XBAR_DIR_SEL_15_W {
        IOMUXC_XBAR_DIR_SEL_15_W { w: self }
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16(&mut self) -> IOMUXC_XBAR_DIR_SEL_16_W {
        IOMUXC_XBAR_DIR_SEL_16_W { w: self }
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17(&mut self) -> IOMUXC_XBAR_DIR_SEL_17_W {
        IOMUXC_XBAR_DIR_SEL_17_W { w: self }
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18(&mut self) -> IOMUXC_XBAR_DIR_SEL_18_W {
        IOMUXC_XBAR_DIR_SEL_18_W { w: self }
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19(&mut self) -> IOMUXC_XBAR_DIR_SEL_19_W {
        IOMUXC_XBAR_DIR_SEL_19_W { w: self }
    }
}
