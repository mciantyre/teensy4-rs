#[doc = "Reader of register INT"]
pub type R = crate::R<u32, super::INT>;
#[doc = "Writer for register INT"]
pub type W = crate::W<u32, super::INT>;
#[doc = "Register INT `reset()`'s with value 0"]
impl crate::ResetValue for super::INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT0_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT0_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT0_1 = 1,
}
impl From<INT0_A> for bool {
    #[inline(always)]
    fn from(variant: INT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT0`"]
pub type INT0_R = crate::R<bool, INT0_A>;
impl INT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT0_A {
        match self.bits {
            false => INT0_A::INT0_0,
            true => INT0_A::INT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT0_0`"]
    #[inline(always)]
    pub fn is_int0_0(&self) -> bool {
        *self == INT0_A::INT0_0
    }
    #[doc = "Checks if the value of the field is `INT0_1`"]
    #[inline(always)]
    pub fn is_int0_1(&self) -> bool {
        *self == INT0_A::INT0_1
    }
}
#[doc = "Write proxy for field `INT0`"]
pub struct INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int0_0(self) -> &'a mut W {
        self.variant(INT0_A::INT0_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int0_1(self) -> &'a mut W {
        self.variant(INT0_A::INT0_1)
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
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT1_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT1_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT1_1 = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT1`"]
pub type INT1_R = crate::R<bool, INT1_A>;
impl INT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::INT1_0,
            true => INT1_A::INT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT1_0`"]
    #[inline(always)]
    pub fn is_int1_0(&self) -> bool {
        *self == INT1_A::INT1_0
    }
    #[doc = "Checks if the value of the field is `INT1_1`"]
    #[inline(always)]
    pub fn is_int1_1(&self) -> bool {
        *self == INT1_A::INT1_1
    }
}
#[doc = "Write proxy for field `INT1`"]
pub struct INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int1_0(self) -> &'a mut W {
        self.variant(INT1_A::INT1_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int1_1(self) -> &'a mut W {
        self.variant(INT1_A::INT1_1)
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
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT2_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT2_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT2_1 = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT2`"]
pub type INT2_R = crate::R<bool, INT2_A>;
impl INT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::INT2_0,
            true => INT2_A::INT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT2_0`"]
    #[inline(always)]
    pub fn is_int2_0(&self) -> bool {
        *self == INT2_A::INT2_0
    }
    #[doc = "Checks if the value of the field is `INT2_1`"]
    #[inline(always)]
    pub fn is_int2_1(&self) -> bool {
        *self == INT2_A::INT2_1
    }
}
#[doc = "Write proxy for field `INT2`"]
pub struct INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int2_0(self) -> &'a mut W {
        self.variant(INT2_A::INT2_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int2_1(self) -> &'a mut W {
        self.variant(INT2_A::INT2_1)
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
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT3_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT3_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT3_1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT3`"]
pub type INT3_R = crate::R<bool, INT3_A>;
impl INT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::INT3_0,
            true => INT3_A::INT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT3_0`"]
    #[inline(always)]
    pub fn is_int3_0(&self) -> bool {
        *self == INT3_A::INT3_0
    }
    #[doc = "Checks if the value of the field is `INT3_1`"]
    #[inline(always)]
    pub fn is_int3_1(&self) -> bool {
        *self == INT3_A::INT3_1
    }
}
#[doc = "Write proxy for field `INT3`"]
pub struct INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int3_0(self) -> &'a mut W {
        self.variant(INT3_A::INT3_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int3_1(self) -> &'a mut W {
        self.variant(INT3_A::INT3_1)
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
#[doc = "Interrupt Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT4_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT4_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT4_1 = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT4`"]
pub type INT4_R = crate::R<bool, INT4_A>;
impl INT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::INT4_0,
            true => INT4_A::INT4_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT4_0`"]
    #[inline(always)]
    pub fn is_int4_0(&self) -> bool {
        *self == INT4_A::INT4_0
    }
    #[doc = "Checks if the value of the field is `INT4_1`"]
    #[inline(always)]
    pub fn is_int4_1(&self) -> bool {
        *self == INT4_A::INT4_1
    }
}
#[doc = "Write proxy for field `INT4`"]
pub struct INT4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int4_0(self) -> &'a mut W {
        self.variant(INT4_A::INT4_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int4_1(self) -> &'a mut W {
        self.variant(INT4_A::INT4_1)
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
#[doc = "Interrupt Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT5_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT5_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT5_1 = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT5`"]
pub type INT5_R = crate::R<bool, INT5_A>;
impl INT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::INT5_0,
            true => INT5_A::INT5_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT5_0`"]
    #[inline(always)]
    pub fn is_int5_0(&self) -> bool {
        *self == INT5_A::INT5_0
    }
    #[doc = "Checks if the value of the field is `INT5_1`"]
    #[inline(always)]
    pub fn is_int5_1(&self) -> bool {
        *self == INT5_A::INT5_1
    }
}
#[doc = "Write proxy for field `INT5`"]
pub struct INT5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int5_0(self) -> &'a mut W {
        self.variant(INT5_A::INT5_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int5_1(self) -> &'a mut W {
        self.variant(INT5_A::INT5_1)
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
#[doc = "Interrupt Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT6_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT6_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT6_1 = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT6`"]
pub type INT6_R = crate::R<bool, INT6_A>;
impl INT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::INT6_0,
            true => INT6_A::INT6_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT6_0`"]
    #[inline(always)]
    pub fn is_int6_0(&self) -> bool {
        *self == INT6_A::INT6_0
    }
    #[doc = "Checks if the value of the field is `INT6_1`"]
    #[inline(always)]
    pub fn is_int6_1(&self) -> bool {
        *self == INT6_A::INT6_1
    }
}
#[doc = "Write proxy for field `INT6`"]
pub struct INT6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int6_0(self) -> &'a mut W {
        self.variant(INT6_A::INT6_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int6_1(self) -> &'a mut W {
        self.variant(INT6_A::INT6_1)
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
#[doc = "Interrupt Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT7_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT7_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT7_1 = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT7`"]
pub type INT7_R = crate::R<bool, INT7_A>;
impl INT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::INT7_0,
            true => INT7_A::INT7_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT7_0`"]
    #[inline(always)]
    pub fn is_int7_0(&self) -> bool {
        *self == INT7_A::INT7_0
    }
    #[doc = "Checks if the value of the field is `INT7_1`"]
    #[inline(always)]
    pub fn is_int7_1(&self) -> bool {
        *self == INT7_A::INT7_1
    }
}
#[doc = "Write proxy for field `INT7`"]
pub struct INT7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int7_0(self) -> &'a mut W {
        self.variant(INT7_A::INT7_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int7_1(self) -> &'a mut W {
        self.variant(INT7_A::INT7_1)
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
#[doc = "Interrupt Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT8_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT8_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT8_1 = 1,
}
impl From<INT8_A> for bool {
    #[inline(always)]
    fn from(variant: INT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT8`"]
pub type INT8_R = crate::R<bool, INT8_A>;
impl INT8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT8_A {
        match self.bits {
            false => INT8_A::INT8_0,
            true => INT8_A::INT8_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT8_0`"]
    #[inline(always)]
    pub fn is_int8_0(&self) -> bool {
        *self == INT8_A::INT8_0
    }
    #[doc = "Checks if the value of the field is `INT8_1`"]
    #[inline(always)]
    pub fn is_int8_1(&self) -> bool {
        *self == INT8_A::INT8_1
    }
}
#[doc = "Write proxy for field `INT8`"]
pub struct INT8_W<'a> {
    w: &'a mut W,
}
impl<'a> INT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int8_0(self) -> &'a mut W {
        self.variant(INT8_A::INT8_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int8_1(self) -> &'a mut W {
        self.variant(INT8_A::INT8_1)
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
#[doc = "Interrupt Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT9_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT9_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT9_1 = 1,
}
impl From<INT9_A> for bool {
    #[inline(always)]
    fn from(variant: INT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT9`"]
pub type INT9_R = crate::R<bool, INT9_A>;
impl INT9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT9_A {
        match self.bits {
            false => INT9_A::INT9_0,
            true => INT9_A::INT9_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT9_0`"]
    #[inline(always)]
    pub fn is_int9_0(&self) -> bool {
        *self == INT9_A::INT9_0
    }
    #[doc = "Checks if the value of the field is `INT9_1`"]
    #[inline(always)]
    pub fn is_int9_1(&self) -> bool {
        *self == INT9_A::INT9_1
    }
}
#[doc = "Write proxy for field `INT9`"]
pub struct INT9_W<'a> {
    w: &'a mut W,
}
impl<'a> INT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int9_0(self) -> &'a mut W {
        self.variant(INT9_A::INT9_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int9_1(self) -> &'a mut W {
        self.variant(INT9_A::INT9_1)
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
#[doc = "Interrupt Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT10_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT10_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT10_1 = 1,
}
impl From<INT10_A> for bool {
    #[inline(always)]
    fn from(variant: INT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT10`"]
pub type INT10_R = crate::R<bool, INT10_A>;
impl INT10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT10_A {
        match self.bits {
            false => INT10_A::INT10_0,
            true => INT10_A::INT10_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT10_0`"]
    #[inline(always)]
    pub fn is_int10_0(&self) -> bool {
        *self == INT10_A::INT10_0
    }
    #[doc = "Checks if the value of the field is `INT10_1`"]
    #[inline(always)]
    pub fn is_int10_1(&self) -> bool {
        *self == INT10_A::INT10_1
    }
}
#[doc = "Write proxy for field `INT10`"]
pub struct INT10_W<'a> {
    w: &'a mut W,
}
impl<'a> INT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int10_0(self) -> &'a mut W {
        self.variant(INT10_A::INT10_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int10_1(self) -> &'a mut W {
        self.variant(INT10_A::INT10_1)
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
#[doc = "Interrupt Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT11_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT11_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT11_1 = 1,
}
impl From<INT11_A> for bool {
    #[inline(always)]
    fn from(variant: INT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT11`"]
pub type INT11_R = crate::R<bool, INT11_A>;
impl INT11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT11_A {
        match self.bits {
            false => INT11_A::INT11_0,
            true => INT11_A::INT11_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT11_0`"]
    #[inline(always)]
    pub fn is_int11_0(&self) -> bool {
        *self == INT11_A::INT11_0
    }
    #[doc = "Checks if the value of the field is `INT11_1`"]
    #[inline(always)]
    pub fn is_int11_1(&self) -> bool {
        *self == INT11_A::INT11_1
    }
}
#[doc = "Write proxy for field `INT11`"]
pub struct INT11_W<'a> {
    w: &'a mut W,
}
impl<'a> INT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int11_0(self) -> &'a mut W {
        self.variant(INT11_A::INT11_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int11_1(self) -> &'a mut W {
        self.variant(INT11_A::INT11_1)
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
#[doc = "Interrupt Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT12_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT12_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT12_1 = 1,
}
impl From<INT12_A> for bool {
    #[inline(always)]
    fn from(variant: INT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT12`"]
pub type INT12_R = crate::R<bool, INT12_A>;
impl INT12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT12_A {
        match self.bits {
            false => INT12_A::INT12_0,
            true => INT12_A::INT12_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT12_0`"]
    #[inline(always)]
    pub fn is_int12_0(&self) -> bool {
        *self == INT12_A::INT12_0
    }
    #[doc = "Checks if the value of the field is `INT12_1`"]
    #[inline(always)]
    pub fn is_int12_1(&self) -> bool {
        *self == INT12_A::INT12_1
    }
}
#[doc = "Write proxy for field `INT12`"]
pub struct INT12_W<'a> {
    w: &'a mut W,
}
impl<'a> INT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int12_0(self) -> &'a mut W {
        self.variant(INT12_A::INT12_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int12_1(self) -> &'a mut W {
        self.variant(INT12_A::INT12_1)
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
#[doc = "Interrupt Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT13_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT13_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT13_1 = 1,
}
impl From<INT13_A> for bool {
    #[inline(always)]
    fn from(variant: INT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT13`"]
pub type INT13_R = crate::R<bool, INT13_A>;
impl INT13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT13_A {
        match self.bits {
            false => INT13_A::INT13_0,
            true => INT13_A::INT13_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT13_0`"]
    #[inline(always)]
    pub fn is_int13_0(&self) -> bool {
        *self == INT13_A::INT13_0
    }
    #[doc = "Checks if the value of the field is `INT13_1`"]
    #[inline(always)]
    pub fn is_int13_1(&self) -> bool {
        *self == INT13_A::INT13_1
    }
}
#[doc = "Write proxy for field `INT13`"]
pub struct INT13_W<'a> {
    w: &'a mut W,
}
impl<'a> INT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int13_0(self) -> &'a mut W {
        self.variant(INT13_A::INT13_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int13_1(self) -> &'a mut W {
        self.variant(INT13_A::INT13_1)
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
#[doc = "Interrupt Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT14_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT14_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT14_1 = 1,
}
impl From<INT14_A> for bool {
    #[inline(always)]
    fn from(variant: INT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT14`"]
pub type INT14_R = crate::R<bool, INT14_A>;
impl INT14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT14_A {
        match self.bits {
            false => INT14_A::INT14_0,
            true => INT14_A::INT14_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT14_0`"]
    #[inline(always)]
    pub fn is_int14_0(&self) -> bool {
        *self == INT14_A::INT14_0
    }
    #[doc = "Checks if the value of the field is `INT14_1`"]
    #[inline(always)]
    pub fn is_int14_1(&self) -> bool {
        *self == INT14_A::INT14_1
    }
}
#[doc = "Write proxy for field `INT14`"]
pub struct INT14_W<'a> {
    w: &'a mut W,
}
impl<'a> INT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int14_0(self) -> &'a mut W {
        self.variant(INT14_A::INT14_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int14_1(self) -> &'a mut W {
        self.variant(INT14_A::INT14_1)
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
#[doc = "Interrupt Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT15_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT15_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT15_1 = 1,
}
impl From<INT15_A> for bool {
    #[inline(always)]
    fn from(variant: INT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT15`"]
pub type INT15_R = crate::R<bool, INT15_A>;
impl INT15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT15_A {
        match self.bits {
            false => INT15_A::INT15_0,
            true => INT15_A::INT15_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT15_0`"]
    #[inline(always)]
    pub fn is_int15_0(&self) -> bool {
        *self == INT15_A::INT15_0
    }
    #[doc = "Checks if the value of the field is `INT15_1`"]
    #[inline(always)]
    pub fn is_int15_1(&self) -> bool {
        *self == INT15_A::INT15_1
    }
}
#[doc = "Write proxy for field `INT15`"]
pub struct INT15_W<'a> {
    w: &'a mut W,
}
impl<'a> INT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int15_0(self) -> &'a mut W {
        self.variant(INT15_A::INT15_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int15_1(self) -> &'a mut W {
        self.variant(INT15_A::INT15_1)
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
#[doc = "Interrupt Request 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT16_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT16_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT16_1 = 1,
}
impl From<INT16_A> for bool {
    #[inline(always)]
    fn from(variant: INT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT16`"]
pub type INT16_R = crate::R<bool, INT16_A>;
impl INT16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT16_A {
        match self.bits {
            false => INT16_A::INT16_0,
            true => INT16_A::INT16_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT16_0`"]
    #[inline(always)]
    pub fn is_int16_0(&self) -> bool {
        *self == INT16_A::INT16_0
    }
    #[doc = "Checks if the value of the field is `INT16_1`"]
    #[inline(always)]
    pub fn is_int16_1(&self) -> bool {
        *self == INT16_A::INT16_1
    }
}
#[doc = "Write proxy for field `INT16`"]
pub struct INT16_W<'a> {
    w: &'a mut W,
}
impl<'a> INT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int16_0(self) -> &'a mut W {
        self.variant(INT16_A::INT16_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int16_1(self) -> &'a mut W {
        self.variant(INT16_A::INT16_1)
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
#[doc = "Interrupt Request 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT17_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT17_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT17_1 = 1,
}
impl From<INT17_A> for bool {
    #[inline(always)]
    fn from(variant: INT17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT17`"]
pub type INT17_R = crate::R<bool, INT17_A>;
impl INT17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT17_A {
        match self.bits {
            false => INT17_A::INT17_0,
            true => INT17_A::INT17_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT17_0`"]
    #[inline(always)]
    pub fn is_int17_0(&self) -> bool {
        *self == INT17_A::INT17_0
    }
    #[doc = "Checks if the value of the field is `INT17_1`"]
    #[inline(always)]
    pub fn is_int17_1(&self) -> bool {
        *self == INT17_A::INT17_1
    }
}
#[doc = "Write proxy for field `INT17`"]
pub struct INT17_W<'a> {
    w: &'a mut W,
}
impl<'a> INT17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int17_0(self) -> &'a mut W {
        self.variant(INT17_A::INT17_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int17_1(self) -> &'a mut W {
        self.variant(INT17_A::INT17_1)
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
#[doc = "Interrupt Request 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT18_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT18_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT18_1 = 1,
}
impl From<INT18_A> for bool {
    #[inline(always)]
    fn from(variant: INT18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT18`"]
pub type INT18_R = crate::R<bool, INT18_A>;
impl INT18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT18_A {
        match self.bits {
            false => INT18_A::INT18_0,
            true => INT18_A::INT18_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT18_0`"]
    #[inline(always)]
    pub fn is_int18_0(&self) -> bool {
        *self == INT18_A::INT18_0
    }
    #[doc = "Checks if the value of the field is `INT18_1`"]
    #[inline(always)]
    pub fn is_int18_1(&self) -> bool {
        *self == INT18_A::INT18_1
    }
}
#[doc = "Write proxy for field `INT18`"]
pub struct INT18_W<'a> {
    w: &'a mut W,
}
impl<'a> INT18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int18_0(self) -> &'a mut W {
        self.variant(INT18_A::INT18_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int18_1(self) -> &'a mut W {
        self.variant(INT18_A::INT18_1)
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
#[doc = "Interrupt Request 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT19_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT19_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT19_1 = 1,
}
impl From<INT19_A> for bool {
    #[inline(always)]
    fn from(variant: INT19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT19`"]
pub type INT19_R = crate::R<bool, INT19_A>;
impl INT19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT19_A {
        match self.bits {
            false => INT19_A::INT19_0,
            true => INT19_A::INT19_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT19_0`"]
    #[inline(always)]
    pub fn is_int19_0(&self) -> bool {
        *self == INT19_A::INT19_0
    }
    #[doc = "Checks if the value of the field is `INT19_1`"]
    #[inline(always)]
    pub fn is_int19_1(&self) -> bool {
        *self == INT19_A::INT19_1
    }
}
#[doc = "Write proxy for field `INT19`"]
pub struct INT19_W<'a> {
    w: &'a mut W,
}
impl<'a> INT19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int19_0(self) -> &'a mut W {
        self.variant(INT19_A::INT19_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int19_1(self) -> &'a mut W {
        self.variant(INT19_A::INT19_1)
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
#[doc = "Interrupt Request 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT20_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT20_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT20_1 = 1,
}
impl From<INT20_A> for bool {
    #[inline(always)]
    fn from(variant: INT20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT20`"]
pub type INT20_R = crate::R<bool, INT20_A>;
impl INT20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT20_A {
        match self.bits {
            false => INT20_A::INT20_0,
            true => INT20_A::INT20_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT20_0`"]
    #[inline(always)]
    pub fn is_int20_0(&self) -> bool {
        *self == INT20_A::INT20_0
    }
    #[doc = "Checks if the value of the field is `INT20_1`"]
    #[inline(always)]
    pub fn is_int20_1(&self) -> bool {
        *self == INT20_A::INT20_1
    }
}
#[doc = "Write proxy for field `INT20`"]
pub struct INT20_W<'a> {
    w: &'a mut W,
}
impl<'a> INT20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int20_0(self) -> &'a mut W {
        self.variant(INT20_A::INT20_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int20_1(self) -> &'a mut W {
        self.variant(INT20_A::INT20_1)
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
#[doc = "Interrupt Request 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT21_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT21_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT21_1 = 1,
}
impl From<INT21_A> for bool {
    #[inline(always)]
    fn from(variant: INT21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT21`"]
pub type INT21_R = crate::R<bool, INT21_A>;
impl INT21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT21_A {
        match self.bits {
            false => INT21_A::INT21_0,
            true => INT21_A::INT21_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT21_0`"]
    #[inline(always)]
    pub fn is_int21_0(&self) -> bool {
        *self == INT21_A::INT21_0
    }
    #[doc = "Checks if the value of the field is `INT21_1`"]
    #[inline(always)]
    pub fn is_int21_1(&self) -> bool {
        *self == INT21_A::INT21_1
    }
}
#[doc = "Write proxy for field `INT21`"]
pub struct INT21_W<'a> {
    w: &'a mut W,
}
impl<'a> INT21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int21_0(self) -> &'a mut W {
        self.variant(INT21_A::INT21_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int21_1(self) -> &'a mut W {
        self.variant(INT21_A::INT21_1)
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
#[doc = "Interrupt Request 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT22_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT22_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT22_1 = 1,
}
impl From<INT22_A> for bool {
    #[inline(always)]
    fn from(variant: INT22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT22`"]
pub type INT22_R = crate::R<bool, INT22_A>;
impl INT22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT22_A {
        match self.bits {
            false => INT22_A::INT22_0,
            true => INT22_A::INT22_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT22_0`"]
    #[inline(always)]
    pub fn is_int22_0(&self) -> bool {
        *self == INT22_A::INT22_0
    }
    #[doc = "Checks if the value of the field is `INT22_1`"]
    #[inline(always)]
    pub fn is_int22_1(&self) -> bool {
        *self == INT22_A::INT22_1
    }
}
#[doc = "Write proxy for field `INT22`"]
pub struct INT22_W<'a> {
    w: &'a mut W,
}
impl<'a> INT22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int22_0(self) -> &'a mut W {
        self.variant(INT22_A::INT22_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int22_1(self) -> &'a mut W {
        self.variant(INT22_A::INT22_1)
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
#[doc = "Interrupt Request 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT23_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT23_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT23_1 = 1,
}
impl From<INT23_A> for bool {
    #[inline(always)]
    fn from(variant: INT23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT23`"]
pub type INT23_R = crate::R<bool, INT23_A>;
impl INT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT23_A {
        match self.bits {
            false => INT23_A::INT23_0,
            true => INT23_A::INT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT23_0`"]
    #[inline(always)]
    pub fn is_int23_0(&self) -> bool {
        *self == INT23_A::INT23_0
    }
    #[doc = "Checks if the value of the field is `INT23_1`"]
    #[inline(always)]
    pub fn is_int23_1(&self) -> bool {
        *self == INT23_A::INT23_1
    }
}
#[doc = "Write proxy for field `INT23`"]
pub struct INT23_W<'a> {
    w: &'a mut W,
}
impl<'a> INT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int23_0(self) -> &'a mut W {
        self.variant(INT23_A::INT23_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int23_1(self) -> &'a mut W {
        self.variant(INT23_A::INT23_1)
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
#[doc = "Interrupt Request 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT24_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT24_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT24_1 = 1,
}
impl From<INT24_A> for bool {
    #[inline(always)]
    fn from(variant: INT24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT24`"]
pub type INT24_R = crate::R<bool, INT24_A>;
impl INT24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT24_A {
        match self.bits {
            false => INT24_A::INT24_0,
            true => INT24_A::INT24_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT24_0`"]
    #[inline(always)]
    pub fn is_int24_0(&self) -> bool {
        *self == INT24_A::INT24_0
    }
    #[doc = "Checks if the value of the field is `INT24_1`"]
    #[inline(always)]
    pub fn is_int24_1(&self) -> bool {
        *self == INT24_A::INT24_1
    }
}
#[doc = "Write proxy for field `INT24`"]
pub struct INT24_W<'a> {
    w: &'a mut W,
}
impl<'a> INT24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int24_0(self) -> &'a mut W {
        self.variant(INT24_A::INT24_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int24_1(self) -> &'a mut W {
        self.variant(INT24_A::INT24_1)
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
#[doc = "Interrupt Request 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT25_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT25_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT25_1 = 1,
}
impl From<INT25_A> for bool {
    #[inline(always)]
    fn from(variant: INT25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT25`"]
pub type INT25_R = crate::R<bool, INT25_A>;
impl INT25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT25_A {
        match self.bits {
            false => INT25_A::INT25_0,
            true => INT25_A::INT25_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT25_0`"]
    #[inline(always)]
    pub fn is_int25_0(&self) -> bool {
        *self == INT25_A::INT25_0
    }
    #[doc = "Checks if the value of the field is `INT25_1`"]
    #[inline(always)]
    pub fn is_int25_1(&self) -> bool {
        *self == INT25_A::INT25_1
    }
}
#[doc = "Write proxy for field `INT25`"]
pub struct INT25_W<'a> {
    w: &'a mut W,
}
impl<'a> INT25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int25_0(self) -> &'a mut W {
        self.variant(INT25_A::INT25_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int25_1(self) -> &'a mut W {
        self.variant(INT25_A::INT25_1)
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
#[doc = "Interrupt Request 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT26_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT26_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT26_1 = 1,
}
impl From<INT26_A> for bool {
    #[inline(always)]
    fn from(variant: INT26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT26`"]
pub type INT26_R = crate::R<bool, INT26_A>;
impl INT26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT26_A {
        match self.bits {
            false => INT26_A::INT26_0,
            true => INT26_A::INT26_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT26_0`"]
    #[inline(always)]
    pub fn is_int26_0(&self) -> bool {
        *self == INT26_A::INT26_0
    }
    #[doc = "Checks if the value of the field is `INT26_1`"]
    #[inline(always)]
    pub fn is_int26_1(&self) -> bool {
        *self == INT26_A::INT26_1
    }
}
#[doc = "Write proxy for field `INT26`"]
pub struct INT26_W<'a> {
    w: &'a mut W,
}
impl<'a> INT26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int26_0(self) -> &'a mut W {
        self.variant(INT26_A::INT26_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int26_1(self) -> &'a mut W {
        self.variant(INT26_A::INT26_1)
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
#[doc = "Interrupt Request 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT27_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT27_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT27_1 = 1,
}
impl From<INT27_A> for bool {
    #[inline(always)]
    fn from(variant: INT27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT27`"]
pub type INT27_R = crate::R<bool, INT27_A>;
impl INT27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT27_A {
        match self.bits {
            false => INT27_A::INT27_0,
            true => INT27_A::INT27_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT27_0`"]
    #[inline(always)]
    pub fn is_int27_0(&self) -> bool {
        *self == INT27_A::INT27_0
    }
    #[doc = "Checks if the value of the field is `INT27_1`"]
    #[inline(always)]
    pub fn is_int27_1(&self) -> bool {
        *self == INT27_A::INT27_1
    }
}
#[doc = "Write proxy for field `INT27`"]
pub struct INT27_W<'a> {
    w: &'a mut W,
}
impl<'a> INT27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int27_0(self) -> &'a mut W {
        self.variant(INT27_A::INT27_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int27_1(self) -> &'a mut W {
        self.variant(INT27_A::INT27_1)
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
#[doc = "Interrupt Request 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT28_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT28_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT28_1 = 1,
}
impl From<INT28_A> for bool {
    #[inline(always)]
    fn from(variant: INT28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT28`"]
pub type INT28_R = crate::R<bool, INT28_A>;
impl INT28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT28_A {
        match self.bits {
            false => INT28_A::INT28_0,
            true => INT28_A::INT28_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT28_0`"]
    #[inline(always)]
    pub fn is_int28_0(&self) -> bool {
        *self == INT28_A::INT28_0
    }
    #[doc = "Checks if the value of the field is `INT28_1`"]
    #[inline(always)]
    pub fn is_int28_1(&self) -> bool {
        *self == INT28_A::INT28_1
    }
}
#[doc = "Write proxy for field `INT28`"]
pub struct INT28_W<'a> {
    w: &'a mut W,
}
impl<'a> INT28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int28_0(self) -> &'a mut W {
        self.variant(INT28_A::INT28_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int28_1(self) -> &'a mut W {
        self.variant(INT28_A::INT28_1)
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
#[doc = "Interrupt Request 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT29_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT29_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT29_1 = 1,
}
impl From<INT29_A> for bool {
    #[inline(always)]
    fn from(variant: INT29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT29`"]
pub type INT29_R = crate::R<bool, INT29_A>;
impl INT29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT29_A {
        match self.bits {
            false => INT29_A::INT29_0,
            true => INT29_A::INT29_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT29_0`"]
    #[inline(always)]
    pub fn is_int29_0(&self) -> bool {
        *self == INT29_A::INT29_0
    }
    #[doc = "Checks if the value of the field is `INT29_1`"]
    #[inline(always)]
    pub fn is_int29_1(&self) -> bool {
        *self == INT29_A::INT29_1
    }
}
#[doc = "Write proxy for field `INT29`"]
pub struct INT29_W<'a> {
    w: &'a mut W,
}
impl<'a> INT29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int29_0(self) -> &'a mut W {
        self.variant(INT29_A::INT29_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int29_1(self) -> &'a mut W {
        self.variant(INT29_A::INT29_1)
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
#[doc = "Interrupt Request 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT30_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT30_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT30_1 = 1,
}
impl From<INT30_A> for bool {
    #[inline(always)]
    fn from(variant: INT30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT30`"]
pub type INT30_R = crate::R<bool, INT30_A>;
impl INT30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT30_A {
        match self.bits {
            false => INT30_A::INT30_0,
            true => INT30_A::INT30_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT30_0`"]
    #[inline(always)]
    pub fn is_int30_0(&self) -> bool {
        *self == INT30_A::INT30_0
    }
    #[doc = "Checks if the value of the field is `INT30_1`"]
    #[inline(always)]
    pub fn is_int30_1(&self) -> bool {
        *self == INT30_A::INT30_1
    }
}
#[doc = "Write proxy for field `INT30`"]
pub struct INT30_W<'a> {
    w: &'a mut W,
}
impl<'a> INT30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int30_0(self) -> &'a mut W {
        self.variant(INT30_A::INT30_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int30_1(self) -> &'a mut W {
        self.variant(INT30_A::INT30_1)
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
#[doc = "Interrupt Request 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT31_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT31_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT31_1 = 1,
}
impl From<INT31_A> for bool {
    #[inline(always)]
    fn from(variant: INT31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT31`"]
pub type INT31_R = crate::R<bool, INT31_A>;
impl INT31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT31_A {
        match self.bits {
            false => INT31_A::INT31_0,
            true => INT31_A::INT31_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT31_0`"]
    #[inline(always)]
    pub fn is_int31_0(&self) -> bool {
        *self == INT31_A::INT31_0
    }
    #[doc = "Checks if the value of the field is `INT31_1`"]
    #[inline(always)]
    pub fn is_int31_1(&self) -> bool {
        *self == INT31_A::INT31_1
    }
}
#[doc = "Write proxy for field `INT31`"]
pub struct INT31_W<'a> {
    w: &'a mut W,
}
impl<'a> INT31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int31_0(self) -> &'a mut W {
        self.variant(INT31_A::INT31_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int31_1(self) -> &'a mut W {
        self.variant(INT31_A::INT31_1)
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
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&mut self) -> INT0_W {
        INT0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&mut self) -> INT1_W {
        INT1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&mut self) -> INT2_W {
        INT2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&mut self) -> INT3_W {
        INT3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&mut self) -> INT4_W {
        INT4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&mut self) -> INT5_W {
        INT5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&mut self) -> INT6_W {
        INT6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&mut self) -> INT7_W {
        INT7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&mut self) -> INT8_W {
        INT8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&mut self) -> INT9_W {
        INT9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&mut self) -> INT10_W {
        INT10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&mut self) -> INT11_W {
        INT11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&mut self) -> INT12_W {
        INT12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&mut self) -> INT13_W {
        INT13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&mut self) -> INT14_W {
        INT14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&mut self) -> INT15_W {
        INT15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline(always)]
    pub fn int16(&mut self) -> INT16_W {
        INT16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline(always)]
    pub fn int17(&mut self) -> INT17_W {
        INT17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline(always)]
    pub fn int18(&mut self) -> INT18_W {
        INT18_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline(always)]
    pub fn int19(&mut self) -> INT19_W {
        INT19_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline(always)]
    pub fn int20(&mut self) -> INT20_W {
        INT20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline(always)]
    pub fn int21(&mut self) -> INT21_W {
        INT21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline(always)]
    pub fn int22(&mut self) -> INT22_W {
        INT22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline(always)]
    pub fn int23(&mut self) -> INT23_W {
        INT23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline(always)]
    pub fn int24(&mut self) -> INT24_W {
        INT24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline(always)]
    pub fn int25(&mut self) -> INT25_W {
        INT25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline(always)]
    pub fn int26(&mut self) -> INT26_W {
        INT26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline(always)]
    pub fn int27(&mut self) -> INT27_W {
        INT27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline(always)]
    pub fn int28(&mut self) -> INT28_W {
        INT28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline(always)]
    pub fn int29(&mut self) -> INT29_W {
        INT29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline(always)]
    pub fn int30(&mut self) -> INT30_W {
        INT30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline(always)]
    pub fn int31(&mut self) -> INT31_W {
        INT31_W { w: self }
    }
}
