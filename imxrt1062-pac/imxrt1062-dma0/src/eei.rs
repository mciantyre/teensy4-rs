#[doc = "Reader of register EEI"]
pub type R = crate::R<u32, super::EEI>;
#[doc = "Writer for register EEI"]
pub type W = crate::W<u32, super::EEI>;
#[doc = "Register EEI `reset()`'s with value 0"]
impl crate::ResetValue for super::EEI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI0_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI0_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI0_1 = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI0`"]
pub type EEI0_R = crate::R<bool, EEI0_A>;
impl EEI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::EEI0_0,
            true => EEI0_A::EEI0_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI0_0`"]
    #[inline(always)]
    pub fn is_eei0_0(&self) -> bool {
        *self == EEI0_A::EEI0_0
    }
    #[doc = "Checks if the value of the field is `EEI0_1`"]
    #[inline(always)]
    pub fn is_eei0_1(&self) -> bool {
        *self == EEI0_A::EEI0_1
    }
}
#[doc = "Write proxy for field `EEI0`"]
pub struct EEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei0_0(self) -> &'a mut W {
        self.variant(EEI0_A::EEI0_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei0_1(self) -> &'a mut W {
        self.variant(EEI0_A::EEI0_1)
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
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI1_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI1_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI1_1 = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI1`"]
pub type EEI1_R = crate::R<bool, EEI1_A>;
impl EEI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::EEI1_0,
            true => EEI1_A::EEI1_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI1_0`"]
    #[inline(always)]
    pub fn is_eei1_0(&self) -> bool {
        *self == EEI1_A::EEI1_0
    }
    #[doc = "Checks if the value of the field is `EEI1_1`"]
    #[inline(always)]
    pub fn is_eei1_1(&self) -> bool {
        *self == EEI1_A::EEI1_1
    }
}
#[doc = "Write proxy for field `EEI1`"]
pub struct EEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei1_0(self) -> &'a mut W {
        self.variant(EEI1_A::EEI1_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei1_1(self) -> &'a mut W {
        self.variant(EEI1_A::EEI1_1)
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
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI2_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI2_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI2_1 = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI2`"]
pub type EEI2_R = crate::R<bool, EEI2_A>;
impl EEI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::EEI2_0,
            true => EEI2_A::EEI2_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI2_0`"]
    #[inline(always)]
    pub fn is_eei2_0(&self) -> bool {
        *self == EEI2_A::EEI2_0
    }
    #[doc = "Checks if the value of the field is `EEI2_1`"]
    #[inline(always)]
    pub fn is_eei2_1(&self) -> bool {
        *self == EEI2_A::EEI2_1
    }
}
#[doc = "Write proxy for field `EEI2`"]
pub struct EEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei2_0(self) -> &'a mut W {
        self.variant(EEI2_A::EEI2_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei2_1(self) -> &'a mut W {
        self.variant(EEI2_A::EEI2_1)
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
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI3_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI3_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI3_1 = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI3`"]
pub type EEI3_R = crate::R<bool, EEI3_A>;
impl EEI3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::EEI3_0,
            true => EEI3_A::EEI3_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI3_0`"]
    #[inline(always)]
    pub fn is_eei3_0(&self) -> bool {
        *self == EEI3_A::EEI3_0
    }
    #[doc = "Checks if the value of the field is `EEI3_1`"]
    #[inline(always)]
    pub fn is_eei3_1(&self) -> bool {
        *self == EEI3_A::EEI3_1
    }
}
#[doc = "Write proxy for field `EEI3`"]
pub struct EEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei3_0(self) -> &'a mut W {
        self.variant(EEI3_A::EEI3_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei3_1(self) -> &'a mut W {
        self.variant(EEI3_A::EEI3_1)
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
#[doc = "Enable Error Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI4_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI4_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI4_1 = 1,
}
impl From<EEI4_A> for bool {
    #[inline(always)]
    fn from(variant: EEI4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI4`"]
pub type EEI4_R = crate::R<bool, EEI4_A>;
impl EEI4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI4_A {
        match self.bits {
            false => EEI4_A::EEI4_0,
            true => EEI4_A::EEI4_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI4_0`"]
    #[inline(always)]
    pub fn is_eei4_0(&self) -> bool {
        *self == EEI4_A::EEI4_0
    }
    #[doc = "Checks if the value of the field is `EEI4_1`"]
    #[inline(always)]
    pub fn is_eei4_1(&self) -> bool {
        *self == EEI4_A::EEI4_1
    }
}
#[doc = "Write proxy for field `EEI4`"]
pub struct EEI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei4_0(self) -> &'a mut W {
        self.variant(EEI4_A::EEI4_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei4_1(self) -> &'a mut W {
        self.variant(EEI4_A::EEI4_1)
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
#[doc = "Enable Error Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI5_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI5_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI5_1 = 1,
}
impl From<EEI5_A> for bool {
    #[inline(always)]
    fn from(variant: EEI5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI5`"]
pub type EEI5_R = crate::R<bool, EEI5_A>;
impl EEI5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI5_A {
        match self.bits {
            false => EEI5_A::EEI5_0,
            true => EEI5_A::EEI5_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI5_0`"]
    #[inline(always)]
    pub fn is_eei5_0(&self) -> bool {
        *self == EEI5_A::EEI5_0
    }
    #[doc = "Checks if the value of the field is `EEI5_1`"]
    #[inline(always)]
    pub fn is_eei5_1(&self) -> bool {
        *self == EEI5_A::EEI5_1
    }
}
#[doc = "Write proxy for field `EEI5`"]
pub struct EEI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei5_0(self) -> &'a mut W {
        self.variant(EEI5_A::EEI5_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei5_1(self) -> &'a mut W {
        self.variant(EEI5_A::EEI5_1)
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
#[doc = "Enable Error Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI6_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI6_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI6_1 = 1,
}
impl From<EEI6_A> for bool {
    #[inline(always)]
    fn from(variant: EEI6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI6`"]
pub type EEI6_R = crate::R<bool, EEI6_A>;
impl EEI6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI6_A {
        match self.bits {
            false => EEI6_A::EEI6_0,
            true => EEI6_A::EEI6_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI6_0`"]
    #[inline(always)]
    pub fn is_eei6_0(&self) -> bool {
        *self == EEI6_A::EEI6_0
    }
    #[doc = "Checks if the value of the field is `EEI6_1`"]
    #[inline(always)]
    pub fn is_eei6_1(&self) -> bool {
        *self == EEI6_A::EEI6_1
    }
}
#[doc = "Write proxy for field `EEI6`"]
pub struct EEI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei6_0(self) -> &'a mut W {
        self.variant(EEI6_A::EEI6_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei6_1(self) -> &'a mut W {
        self.variant(EEI6_A::EEI6_1)
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
#[doc = "Enable Error Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI7_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI7_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI7_1 = 1,
}
impl From<EEI7_A> for bool {
    #[inline(always)]
    fn from(variant: EEI7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI7`"]
pub type EEI7_R = crate::R<bool, EEI7_A>;
impl EEI7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI7_A {
        match self.bits {
            false => EEI7_A::EEI7_0,
            true => EEI7_A::EEI7_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI7_0`"]
    #[inline(always)]
    pub fn is_eei7_0(&self) -> bool {
        *self == EEI7_A::EEI7_0
    }
    #[doc = "Checks if the value of the field is `EEI7_1`"]
    #[inline(always)]
    pub fn is_eei7_1(&self) -> bool {
        *self == EEI7_A::EEI7_1
    }
}
#[doc = "Write proxy for field `EEI7`"]
pub struct EEI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei7_0(self) -> &'a mut W {
        self.variant(EEI7_A::EEI7_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei7_1(self) -> &'a mut W {
        self.variant(EEI7_A::EEI7_1)
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
#[doc = "Enable Error Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI8_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI8_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI8_1 = 1,
}
impl From<EEI8_A> for bool {
    #[inline(always)]
    fn from(variant: EEI8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI8`"]
pub type EEI8_R = crate::R<bool, EEI8_A>;
impl EEI8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI8_A {
        match self.bits {
            false => EEI8_A::EEI8_0,
            true => EEI8_A::EEI8_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI8_0`"]
    #[inline(always)]
    pub fn is_eei8_0(&self) -> bool {
        *self == EEI8_A::EEI8_0
    }
    #[doc = "Checks if the value of the field is `EEI8_1`"]
    #[inline(always)]
    pub fn is_eei8_1(&self) -> bool {
        *self == EEI8_A::EEI8_1
    }
}
#[doc = "Write proxy for field `EEI8`"]
pub struct EEI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei8_0(self) -> &'a mut W {
        self.variant(EEI8_A::EEI8_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei8_1(self) -> &'a mut W {
        self.variant(EEI8_A::EEI8_1)
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
#[doc = "Enable Error Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI9_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI9_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI9_1 = 1,
}
impl From<EEI9_A> for bool {
    #[inline(always)]
    fn from(variant: EEI9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI9`"]
pub type EEI9_R = crate::R<bool, EEI9_A>;
impl EEI9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI9_A {
        match self.bits {
            false => EEI9_A::EEI9_0,
            true => EEI9_A::EEI9_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI9_0`"]
    #[inline(always)]
    pub fn is_eei9_0(&self) -> bool {
        *self == EEI9_A::EEI9_0
    }
    #[doc = "Checks if the value of the field is `EEI9_1`"]
    #[inline(always)]
    pub fn is_eei9_1(&self) -> bool {
        *self == EEI9_A::EEI9_1
    }
}
#[doc = "Write proxy for field `EEI9`"]
pub struct EEI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei9_0(self) -> &'a mut W {
        self.variant(EEI9_A::EEI9_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei9_1(self) -> &'a mut W {
        self.variant(EEI9_A::EEI9_1)
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
#[doc = "Enable Error Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI10_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI10_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI10_1 = 1,
}
impl From<EEI10_A> for bool {
    #[inline(always)]
    fn from(variant: EEI10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI10`"]
pub type EEI10_R = crate::R<bool, EEI10_A>;
impl EEI10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI10_A {
        match self.bits {
            false => EEI10_A::EEI10_0,
            true => EEI10_A::EEI10_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI10_0`"]
    #[inline(always)]
    pub fn is_eei10_0(&self) -> bool {
        *self == EEI10_A::EEI10_0
    }
    #[doc = "Checks if the value of the field is `EEI10_1`"]
    #[inline(always)]
    pub fn is_eei10_1(&self) -> bool {
        *self == EEI10_A::EEI10_1
    }
}
#[doc = "Write proxy for field `EEI10`"]
pub struct EEI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei10_0(self) -> &'a mut W {
        self.variant(EEI10_A::EEI10_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei10_1(self) -> &'a mut W {
        self.variant(EEI10_A::EEI10_1)
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
#[doc = "Enable Error Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI11_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI11_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI11_1 = 1,
}
impl From<EEI11_A> for bool {
    #[inline(always)]
    fn from(variant: EEI11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI11`"]
pub type EEI11_R = crate::R<bool, EEI11_A>;
impl EEI11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI11_A {
        match self.bits {
            false => EEI11_A::EEI11_0,
            true => EEI11_A::EEI11_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI11_0`"]
    #[inline(always)]
    pub fn is_eei11_0(&self) -> bool {
        *self == EEI11_A::EEI11_0
    }
    #[doc = "Checks if the value of the field is `EEI11_1`"]
    #[inline(always)]
    pub fn is_eei11_1(&self) -> bool {
        *self == EEI11_A::EEI11_1
    }
}
#[doc = "Write proxy for field `EEI11`"]
pub struct EEI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei11_0(self) -> &'a mut W {
        self.variant(EEI11_A::EEI11_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei11_1(self) -> &'a mut W {
        self.variant(EEI11_A::EEI11_1)
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
#[doc = "Enable Error Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI12_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI12_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI12_1 = 1,
}
impl From<EEI12_A> for bool {
    #[inline(always)]
    fn from(variant: EEI12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI12`"]
pub type EEI12_R = crate::R<bool, EEI12_A>;
impl EEI12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI12_A {
        match self.bits {
            false => EEI12_A::EEI12_0,
            true => EEI12_A::EEI12_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI12_0`"]
    #[inline(always)]
    pub fn is_eei12_0(&self) -> bool {
        *self == EEI12_A::EEI12_0
    }
    #[doc = "Checks if the value of the field is `EEI12_1`"]
    #[inline(always)]
    pub fn is_eei12_1(&self) -> bool {
        *self == EEI12_A::EEI12_1
    }
}
#[doc = "Write proxy for field `EEI12`"]
pub struct EEI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei12_0(self) -> &'a mut W {
        self.variant(EEI12_A::EEI12_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei12_1(self) -> &'a mut W {
        self.variant(EEI12_A::EEI12_1)
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
#[doc = "Enable Error Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI13_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI13_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI13_1 = 1,
}
impl From<EEI13_A> for bool {
    #[inline(always)]
    fn from(variant: EEI13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI13`"]
pub type EEI13_R = crate::R<bool, EEI13_A>;
impl EEI13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI13_A {
        match self.bits {
            false => EEI13_A::EEI13_0,
            true => EEI13_A::EEI13_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI13_0`"]
    #[inline(always)]
    pub fn is_eei13_0(&self) -> bool {
        *self == EEI13_A::EEI13_0
    }
    #[doc = "Checks if the value of the field is `EEI13_1`"]
    #[inline(always)]
    pub fn is_eei13_1(&self) -> bool {
        *self == EEI13_A::EEI13_1
    }
}
#[doc = "Write proxy for field `EEI13`"]
pub struct EEI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei13_0(self) -> &'a mut W {
        self.variant(EEI13_A::EEI13_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei13_1(self) -> &'a mut W {
        self.variant(EEI13_A::EEI13_1)
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
#[doc = "Enable Error Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI14_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI14_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI14_1 = 1,
}
impl From<EEI14_A> for bool {
    #[inline(always)]
    fn from(variant: EEI14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI14`"]
pub type EEI14_R = crate::R<bool, EEI14_A>;
impl EEI14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI14_A {
        match self.bits {
            false => EEI14_A::EEI14_0,
            true => EEI14_A::EEI14_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI14_0`"]
    #[inline(always)]
    pub fn is_eei14_0(&self) -> bool {
        *self == EEI14_A::EEI14_0
    }
    #[doc = "Checks if the value of the field is `EEI14_1`"]
    #[inline(always)]
    pub fn is_eei14_1(&self) -> bool {
        *self == EEI14_A::EEI14_1
    }
}
#[doc = "Write proxy for field `EEI14`"]
pub struct EEI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei14_0(self) -> &'a mut W {
        self.variant(EEI14_A::EEI14_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei14_1(self) -> &'a mut W {
        self.variant(EEI14_A::EEI14_1)
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
#[doc = "Enable Error Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI15_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI15_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI15_1 = 1,
}
impl From<EEI15_A> for bool {
    #[inline(always)]
    fn from(variant: EEI15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI15`"]
pub type EEI15_R = crate::R<bool, EEI15_A>;
impl EEI15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI15_A {
        match self.bits {
            false => EEI15_A::EEI15_0,
            true => EEI15_A::EEI15_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI15_0`"]
    #[inline(always)]
    pub fn is_eei15_0(&self) -> bool {
        *self == EEI15_A::EEI15_0
    }
    #[doc = "Checks if the value of the field is `EEI15_1`"]
    #[inline(always)]
    pub fn is_eei15_1(&self) -> bool {
        *self == EEI15_A::EEI15_1
    }
}
#[doc = "Write proxy for field `EEI15`"]
pub struct EEI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei15_0(self) -> &'a mut W {
        self.variant(EEI15_A::EEI15_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei15_1(self) -> &'a mut W {
        self.variant(EEI15_A::EEI15_1)
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
#[doc = "Enable Error Interrupt 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI16_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI16_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI16_1 = 1,
}
impl From<EEI16_A> for bool {
    #[inline(always)]
    fn from(variant: EEI16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI16`"]
pub type EEI16_R = crate::R<bool, EEI16_A>;
impl EEI16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI16_A {
        match self.bits {
            false => EEI16_A::EEI16_0,
            true => EEI16_A::EEI16_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI16_0`"]
    #[inline(always)]
    pub fn is_eei16_0(&self) -> bool {
        *self == EEI16_A::EEI16_0
    }
    #[doc = "Checks if the value of the field is `EEI16_1`"]
    #[inline(always)]
    pub fn is_eei16_1(&self) -> bool {
        *self == EEI16_A::EEI16_1
    }
}
#[doc = "Write proxy for field `EEI16`"]
pub struct EEI16_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei16_0(self) -> &'a mut W {
        self.variant(EEI16_A::EEI16_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei16_1(self) -> &'a mut W {
        self.variant(EEI16_A::EEI16_1)
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
#[doc = "Enable Error Interrupt 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI17_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI17_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI17_1 = 1,
}
impl From<EEI17_A> for bool {
    #[inline(always)]
    fn from(variant: EEI17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI17`"]
pub type EEI17_R = crate::R<bool, EEI17_A>;
impl EEI17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI17_A {
        match self.bits {
            false => EEI17_A::EEI17_0,
            true => EEI17_A::EEI17_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI17_0`"]
    #[inline(always)]
    pub fn is_eei17_0(&self) -> bool {
        *self == EEI17_A::EEI17_0
    }
    #[doc = "Checks if the value of the field is `EEI17_1`"]
    #[inline(always)]
    pub fn is_eei17_1(&self) -> bool {
        *self == EEI17_A::EEI17_1
    }
}
#[doc = "Write proxy for field `EEI17`"]
pub struct EEI17_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei17_0(self) -> &'a mut W {
        self.variant(EEI17_A::EEI17_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei17_1(self) -> &'a mut W {
        self.variant(EEI17_A::EEI17_1)
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
#[doc = "Enable Error Interrupt 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI18_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI18_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI18_1 = 1,
}
impl From<EEI18_A> for bool {
    #[inline(always)]
    fn from(variant: EEI18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI18`"]
pub type EEI18_R = crate::R<bool, EEI18_A>;
impl EEI18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI18_A {
        match self.bits {
            false => EEI18_A::EEI18_0,
            true => EEI18_A::EEI18_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI18_0`"]
    #[inline(always)]
    pub fn is_eei18_0(&self) -> bool {
        *self == EEI18_A::EEI18_0
    }
    #[doc = "Checks if the value of the field is `EEI18_1`"]
    #[inline(always)]
    pub fn is_eei18_1(&self) -> bool {
        *self == EEI18_A::EEI18_1
    }
}
#[doc = "Write proxy for field `EEI18`"]
pub struct EEI18_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei18_0(self) -> &'a mut W {
        self.variant(EEI18_A::EEI18_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei18_1(self) -> &'a mut W {
        self.variant(EEI18_A::EEI18_1)
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
#[doc = "Enable Error Interrupt 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI19_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI19_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI19_1 = 1,
}
impl From<EEI19_A> for bool {
    #[inline(always)]
    fn from(variant: EEI19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI19`"]
pub type EEI19_R = crate::R<bool, EEI19_A>;
impl EEI19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI19_A {
        match self.bits {
            false => EEI19_A::EEI19_0,
            true => EEI19_A::EEI19_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI19_0`"]
    #[inline(always)]
    pub fn is_eei19_0(&self) -> bool {
        *self == EEI19_A::EEI19_0
    }
    #[doc = "Checks if the value of the field is `EEI19_1`"]
    #[inline(always)]
    pub fn is_eei19_1(&self) -> bool {
        *self == EEI19_A::EEI19_1
    }
}
#[doc = "Write proxy for field `EEI19`"]
pub struct EEI19_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei19_0(self) -> &'a mut W {
        self.variant(EEI19_A::EEI19_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei19_1(self) -> &'a mut W {
        self.variant(EEI19_A::EEI19_1)
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
#[doc = "Enable Error Interrupt 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI20_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI20_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI20_1 = 1,
}
impl From<EEI20_A> for bool {
    #[inline(always)]
    fn from(variant: EEI20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI20`"]
pub type EEI20_R = crate::R<bool, EEI20_A>;
impl EEI20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI20_A {
        match self.bits {
            false => EEI20_A::EEI20_0,
            true => EEI20_A::EEI20_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI20_0`"]
    #[inline(always)]
    pub fn is_eei20_0(&self) -> bool {
        *self == EEI20_A::EEI20_0
    }
    #[doc = "Checks if the value of the field is `EEI20_1`"]
    #[inline(always)]
    pub fn is_eei20_1(&self) -> bool {
        *self == EEI20_A::EEI20_1
    }
}
#[doc = "Write proxy for field `EEI20`"]
pub struct EEI20_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei20_0(self) -> &'a mut W {
        self.variant(EEI20_A::EEI20_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei20_1(self) -> &'a mut W {
        self.variant(EEI20_A::EEI20_1)
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
#[doc = "Enable Error Interrupt 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI21_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI21_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI21_1 = 1,
}
impl From<EEI21_A> for bool {
    #[inline(always)]
    fn from(variant: EEI21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI21`"]
pub type EEI21_R = crate::R<bool, EEI21_A>;
impl EEI21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI21_A {
        match self.bits {
            false => EEI21_A::EEI21_0,
            true => EEI21_A::EEI21_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI21_0`"]
    #[inline(always)]
    pub fn is_eei21_0(&self) -> bool {
        *self == EEI21_A::EEI21_0
    }
    #[doc = "Checks if the value of the field is `EEI21_1`"]
    #[inline(always)]
    pub fn is_eei21_1(&self) -> bool {
        *self == EEI21_A::EEI21_1
    }
}
#[doc = "Write proxy for field `EEI21`"]
pub struct EEI21_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei21_0(self) -> &'a mut W {
        self.variant(EEI21_A::EEI21_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei21_1(self) -> &'a mut W {
        self.variant(EEI21_A::EEI21_1)
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
#[doc = "Enable Error Interrupt 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI22_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI22_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI22_1 = 1,
}
impl From<EEI22_A> for bool {
    #[inline(always)]
    fn from(variant: EEI22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI22`"]
pub type EEI22_R = crate::R<bool, EEI22_A>;
impl EEI22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI22_A {
        match self.bits {
            false => EEI22_A::EEI22_0,
            true => EEI22_A::EEI22_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI22_0`"]
    #[inline(always)]
    pub fn is_eei22_0(&self) -> bool {
        *self == EEI22_A::EEI22_0
    }
    #[doc = "Checks if the value of the field is `EEI22_1`"]
    #[inline(always)]
    pub fn is_eei22_1(&self) -> bool {
        *self == EEI22_A::EEI22_1
    }
}
#[doc = "Write proxy for field `EEI22`"]
pub struct EEI22_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei22_0(self) -> &'a mut W {
        self.variant(EEI22_A::EEI22_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei22_1(self) -> &'a mut W {
        self.variant(EEI22_A::EEI22_1)
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
#[doc = "Enable Error Interrupt 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI23_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI23_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI23_1 = 1,
}
impl From<EEI23_A> for bool {
    #[inline(always)]
    fn from(variant: EEI23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI23`"]
pub type EEI23_R = crate::R<bool, EEI23_A>;
impl EEI23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI23_A {
        match self.bits {
            false => EEI23_A::EEI23_0,
            true => EEI23_A::EEI23_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI23_0`"]
    #[inline(always)]
    pub fn is_eei23_0(&self) -> bool {
        *self == EEI23_A::EEI23_0
    }
    #[doc = "Checks if the value of the field is `EEI23_1`"]
    #[inline(always)]
    pub fn is_eei23_1(&self) -> bool {
        *self == EEI23_A::EEI23_1
    }
}
#[doc = "Write proxy for field `EEI23`"]
pub struct EEI23_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei23_0(self) -> &'a mut W {
        self.variant(EEI23_A::EEI23_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei23_1(self) -> &'a mut W {
        self.variant(EEI23_A::EEI23_1)
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
#[doc = "Enable Error Interrupt 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI24_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI24_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI24_1 = 1,
}
impl From<EEI24_A> for bool {
    #[inline(always)]
    fn from(variant: EEI24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI24`"]
pub type EEI24_R = crate::R<bool, EEI24_A>;
impl EEI24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI24_A {
        match self.bits {
            false => EEI24_A::EEI24_0,
            true => EEI24_A::EEI24_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI24_0`"]
    #[inline(always)]
    pub fn is_eei24_0(&self) -> bool {
        *self == EEI24_A::EEI24_0
    }
    #[doc = "Checks if the value of the field is `EEI24_1`"]
    #[inline(always)]
    pub fn is_eei24_1(&self) -> bool {
        *self == EEI24_A::EEI24_1
    }
}
#[doc = "Write proxy for field `EEI24`"]
pub struct EEI24_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei24_0(self) -> &'a mut W {
        self.variant(EEI24_A::EEI24_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei24_1(self) -> &'a mut W {
        self.variant(EEI24_A::EEI24_1)
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
#[doc = "Enable Error Interrupt 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI25_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI25_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI25_1 = 1,
}
impl From<EEI25_A> for bool {
    #[inline(always)]
    fn from(variant: EEI25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI25`"]
pub type EEI25_R = crate::R<bool, EEI25_A>;
impl EEI25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI25_A {
        match self.bits {
            false => EEI25_A::EEI25_0,
            true => EEI25_A::EEI25_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI25_0`"]
    #[inline(always)]
    pub fn is_eei25_0(&self) -> bool {
        *self == EEI25_A::EEI25_0
    }
    #[doc = "Checks if the value of the field is `EEI25_1`"]
    #[inline(always)]
    pub fn is_eei25_1(&self) -> bool {
        *self == EEI25_A::EEI25_1
    }
}
#[doc = "Write proxy for field `EEI25`"]
pub struct EEI25_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei25_0(self) -> &'a mut W {
        self.variant(EEI25_A::EEI25_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei25_1(self) -> &'a mut W {
        self.variant(EEI25_A::EEI25_1)
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
#[doc = "Enable Error Interrupt 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI26_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI26_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI26_1 = 1,
}
impl From<EEI26_A> for bool {
    #[inline(always)]
    fn from(variant: EEI26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI26`"]
pub type EEI26_R = crate::R<bool, EEI26_A>;
impl EEI26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI26_A {
        match self.bits {
            false => EEI26_A::EEI26_0,
            true => EEI26_A::EEI26_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI26_0`"]
    #[inline(always)]
    pub fn is_eei26_0(&self) -> bool {
        *self == EEI26_A::EEI26_0
    }
    #[doc = "Checks if the value of the field is `EEI26_1`"]
    #[inline(always)]
    pub fn is_eei26_1(&self) -> bool {
        *self == EEI26_A::EEI26_1
    }
}
#[doc = "Write proxy for field `EEI26`"]
pub struct EEI26_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei26_0(self) -> &'a mut W {
        self.variant(EEI26_A::EEI26_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei26_1(self) -> &'a mut W {
        self.variant(EEI26_A::EEI26_1)
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
#[doc = "Enable Error Interrupt 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI27_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI27_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI27_1 = 1,
}
impl From<EEI27_A> for bool {
    #[inline(always)]
    fn from(variant: EEI27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI27`"]
pub type EEI27_R = crate::R<bool, EEI27_A>;
impl EEI27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI27_A {
        match self.bits {
            false => EEI27_A::EEI27_0,
            true => EEI27_A::EEI27_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI27_0`"]
    #[inline(always)]
    pub fn is_eei27_0(&self) -> bool {
        *self == EEI27_A::EEI27_0
    }
    #[doc = "Checks if the value of the field is `EEI27_1`"]
    #[inline(always)]
    pub fn is_eei27_1(&self) -> bool {
        *self == EEI27_A::EEI27_1
    }
}
#[doc = "Write proxy for field `EEI27`"]
pub struct EEI27_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei27_0(self) -> &'a mut W {
        self.variant(EEI27_A::EEI27_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei27_1(self) -> &'a mut W {
        self.variant(EEI27_A::EEI27_1)
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
#[doc = "Enable Error Interrupt 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI28_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI28_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI28_1 = 1,
}
impl From<EEI28_A> for bool {
    #[inline(always)]
    fn from(variant: EEI28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI28`"]
pub type EEI28_R = crate::R<bool, EEI28_A>;
impl EEI28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI28_A {
        match self.bits {
            false => EEI28_A::EEI28_0,
            true => EEI28_A::EEI28_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI28_0`"]
    #[inline(always)]
    pub fn is_eei28_0(&self) -> bool {
        *self == EEI28_A::EEI28_0
    }
    #[doc = "Checks if the value of the field is `EEI28_1`"]
    #[inline(always)]
    pub fn is_eei28_1(&self) -> bool {
        *self == EEI28_A::EEI28_1
    }
}
#[doc = "Write proxy for field `EEI28`"]
pub struct EEI28_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei28_0(self) -> &'a mut W {
        self.variant(EEI28_A::EEI28_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei28_1(self) -> &'a mut W {
        self.variant(EEI28_A::EEI28_1)
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
#[doc = "Enable Error Interrupt 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI29_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI29_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI29_1 = 1,
}
impl From<EEI29_A> for bool {
    #[inline(always)]
    fn from(variant: EEI29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI29`"]
pub type EEI29_R = crate::R<bool, EEI29_A>;
impl EEI29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI29_A {
        match self.bits {
            false => EEI29_A::EEI29_0,
            true => EEI29_A::EEI29_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI29_0`"]
    #[inline(always)]
    pub fn is_eei29_0(&self) -> bool {
        *self == EEI29_A::EEI29_0
    }
    #[doc = "Checks if the value of the field is `EEI29_1`"]
    #[inline(always)]
    pub fn is_eei29_1(&self) -> bool {
        *self == EEI29_A::EEI29_1
    }
}
#[doc = "Write proxy for field `EEI29`"]
pub struct EEI29_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei29_0(self) -> &'a mut W {
        self.variant(EEI29_A::EEI29_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei29_1(self) -> &'a mut W {
        self.variant(EEI29_A::EEI29_1)
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
#[doc = "Enable Error Interrupt 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI30_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI30_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI30_1 = 1,
}
impl From<EEI30_A> for bool {
    #[inline(always)]
    fn from(variant: EEI30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI30`"]
pub type EEI30_R = crate::R<bool, EEI30_A>;
impl EEI30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI30_A {
        match self.bits {
            false => EEI30_A::EEI30_0,
            true => EEI30_A::EEI30_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI30_0`"]
    #[inline(always)]
    pub fn is_eei30_0(&self) -> bool {
        *self == EEI30_A::EEI30_0
    }
    #[doc = "Checks if the value of the field is `EEI30_1`"]
    #[inline(always)]
    pub fn is_eei30_1(&self) -> bool {
        *self == EEI30_A::EEI30_1
    }
}
#[doc = "Write proxy for field `EEI30`"]
pub struct EEI30_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei30_0(self) -> &'a mut W {
        self.variant(EEI30_A::EEI30_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei30_1(self) -> &'a mut W {
        self.variant(EEI30_A::EEI30_1)
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
#[doc = "Enable Error Interrupt 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI31_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI31_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI31_1 = 1,
}
impl From<EEI31_A> for bool {
    #[inline(always)]
    fn from(variant: EEI31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEI31`"]
pub type EEI31_R = crate::R<bool, EEI31_A>;
impl EEI31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI31_A {
        match self.bits {
            false => EEI31_A::EEI31_0,
            true => EEI31_A::EEI31_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI31_0`"]
    #[inline(always)]
    pub fn is_eei31_0(&self) -> bool {
        *self == EEI31_A::EEI31_0
    }
    #[doc = "Checks if the value of the field is `EEI31_1`"]
    #[inline(always)]
    pub fn is_eei31_1(&self) -> bool {
        *self == EEI31_A::EEI31_1
    }
}
#[doc = "Write proxy for field `EEI31`"]
pub struct EEI31_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei31_0(self) -> &'a mut W {
        self.variant(EEI31_A::EEI31_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei31_1(self) -> &'a mut W {
        self.variant(EEI31_A::EEI31_1)
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&self) -> EEI4_R {
        EEI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&self) -> EEI5_R {
        EEI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&self) -> EEI6_R {
        EEI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&self) -> EEI7_R {
        EEI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&self) -> EEI8_R {
        EEI8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&self) -> EEI9_R {
        EEI9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&self) -> EEI10_R {
        EEI10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&self) -> EEI11_R {
        EEI11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&self) -> EEI12_R {
        EEI12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&self) -> EEI13_R {
        EEI13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&self) -> EEI14_R {
        EEI14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&self) -> EEI15_R {
        EEI15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline(always)]
    pub fn eei16(&self) -> EEI16_R {
        EEI16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline(always)]
    pub fn eei17(&self) -> EEI17_R {
        EEI17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline(always)]
    pub fn eei18(&self) -> EEI18_R {
        EEI18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline(always)]
    pub fn eei19(&self) -> EEI19_R {
        EEI19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline(always)]
    pub fn eei20(&self) -> EEI20_R {
        EEI20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline(always)]
    pub fn eei21(&self) -> EEI21_R {
        EEI21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline(always)]
    pub fn eei22(&self) -> EEI22_R {
        EEI22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline(always)]
    pub fn eei23(&self) -> EEI23_R {
        EEI23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline(always)]
    pub fn eei24(&self) -> EEI24_R {
        EEI24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline(always)]
    pub fn eei25(&self) -> EEI25_R {
        EEI25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline(always)]
    pub fn eei26(&self) -> EEI26_R {
        EEI26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline(always)]
    pub fn eei27(&self) -> EEI27_R {
        EEI27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline(always)]
    pub fn eei28(&self) -> EEI28_R {
        EEI28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline(always)]
    pub fn eei29(&self) -> EEI29_R {
        EEI29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline(always)]
    pub fn eei30(&self) -> EEI30_R {
        EEI30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline(always)]
    pub fn eei31(&self) -> EEI31_R {
        EEI31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&mut self) -> EEI0_W {
        EEI0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&mut self) -> EEI1_W {
        EEI1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&mut self) -> EEI2_W {
        EEI2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&mut self) -> EEI3_W {
        EEI3_W { w: self }
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&mut self) -> EEI4_W {
        EEI4_W { w: self }
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&mut self) -> EEI5_W {
        EEI5_W { w: self }
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&mut self) -> EEI6_W {
        EEI6_W { w: self }
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&mut self) -> EEI7_W {
        EEI7_W { w: self }
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&mut self) -> EEI8_W {
        EEI8_W { w: self }
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&mut self) -> EEI9_W {
        EEI9_W { w: self }
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&mut self) -> EEI10_W {
        EEI10_W { w: self }
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&mut self) -> EEI11_W {
        EEI11_W { w: self }
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&mut self) -> EEI12_W {
        EEI12_W { w: self }
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&mut self) -> EEI13_W {
        EEI13_W { w: self }
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&mut self) -> EEI14_W {
        EEI14_W { w: self }
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&mut self) -> EEI15_W {
        EEI15_W { w: self }
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline(always)]
    pub fn eei16(&mut self) -> EEI16_W {
        EEI16_W { w: self }
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline(always)]
    pub fn eei17(&mut self) -> EEI17_W {
        EEI17_W { w: self }
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline(always)]
    pub fn eei18(&mut self) -> EEI18_W {
        EEI18_W { w: self }
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline(always)]
    pub fn eei19(&mut self) -> EEI19_W {
        EEI19_W { w: self }
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline(always)]
    pub fn eei20(&mut self) -> EEI20_W {
        EEI20_W { w: self }
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline(always)]
    pub fn eei21(&mut self) -> EEI21_W {
        EEI21_W { w: self }
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline(always)]
    pub fn eei22(&mut self) -> EEI22_W {
        EEI22_W { w: self }
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline(always)]
    pub fn eei23(&mut self) -> EEI23_W {
        EEI23_W { w: self }
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline(always)]
    pub fn eei24(&mut self) -> EEI24_W {
        EEI24_W { w: self }
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline(always)]
    pub fn eei25(&mut self) -> EEI25_W {
        EEI25_W { w: self }
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline(always)]
    pub fn eei26(&mut self) -> EEI26_W {
        EEI26_W { w: self }
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline(always)]
    pub fn eei27(&mut self) -> EEI27_W {
        EEI27_W { w: self }
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline(always)]
    pub fn eei28(&mut self) -> EEI28_W {
        EEI28_W { w: self }
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline(always)]
    pub fn eei29(&mut self) -> EEI29_W {
        EEI29_W { w: self }
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline(always)]
    pub fn eei30(&mut self) -> EEI30_W {
        EEI30_W { w: self }
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline(always)]
    pub fn eei31(&mut self) -> EEI31_W {
        EEI31_W { w: self }
    }
}
