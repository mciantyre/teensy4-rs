#[doc = "Reader of register ERQ"]
pub type R = crate::R<u32, super::ERQ>;
#[doc = "Writer for register ERQ"]
pub type W = crate::W<u32, super::ERQ>;
#[doc = "Register ERQ `reset()`'s with value 0"]
impl crate::ResetValue for super::ERQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ0_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ0_1 = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ0`"]
pub type ERQ0_R = crate::R<bool, ERQ0_A>;
impl ERQ0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::ERQ0_0,
            true => ERQ0_A::ERQ0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ0_0`"]
    #[inline(always)]
    pub fn is_erq0_0(&self) -> bool {
        *self == ERQ0_A::ERQ0_0
    }
    #[doc = "Checks if the value of the field is `ERQ0_1`"]
    #[inline(always)]
    pub fn is_erq0_1(&self) -> bool {
        *self == ERQ0_A::ERQ0_1
    }
}
#[doc = "Write proxy for field `ERQ0`"]
pub struct ERQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq0_0(self) -> &'a mut W {
        self.variant(ERQ0_A::ERQ0_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq0_1(self) -> &'a mut W {
        self.variant(ERQ0_A::ERQ0_1)
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
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ1_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ1_1 = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ1`"]
pub type ERQ1_R = crate::R<bool, ERQ1_A>;
impl ERQ1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::ERQ1_0,
            true => ERQ1_A::ERQ1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ1_0`"]
    #[inline(always)]
    pub fn is_erq1_0(&self) -> bool {
        *self == ERQ1_A::ERQ1_0
    }
    #[doc = "Checks if the value of the field is `ERQ1_1`"]
    #[inline(always)]
    pub fn is_erq1_1(&self) -> bool {
        *self == ERQ1_A::ERQ1_1
    }
}
#[doc = "Write proxy for field `ERQ1`"]
pub struct ERQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq1_0(self) -> &'a mut W {
        self.variant(ERQ1_A::ERQ1_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq1_1(self) -> &'a mut W {
        self.variant(ERQ1_A::ERQ1_1)
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
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ2_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ2_1 = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ2`"]
pub type ERQ2_R = crate::R<bool, ERQ2_A>;
impl ERQ2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::ERQ2_0,
            true => ERQ2_A::ERQ2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ2_0`"]
    #[inline(always)]
    pub fn is_erq2_0(&self) -> bool {
        *self == ERQ2_A::ERQ2_0
    }
    #[doc = "Checks if the value of the field is `ERQ2_1`"]
    #[inline(always)]
    pub fn is_erq2_1(&self) -> bool {
        *self == ERQ2_A::ERQ2_1
    }
}
#[doc = "Write proxy for field `ERQ2`"]
pub struct ERQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq2_0(self) -> &'a mut W {
        self.variant(ERQ2_A::ERQ2_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq2_1(self) -> &'a mut W {
        self.variant(ERQ2_A::ERQ2_1)
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
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ3_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ3_1 = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ3`"]
pub type ERQ3_R = crate::R<bool, ERQ3_A>;
impl ERQ3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::ERQ3_0,
            true => ERQ3_A::ERQ3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ3_0`"]
    #[inline(always)]
    pub fn is_erq3_0(&self) -> bool {
        *self == ERQ3_A::ERQ3_0
    }
    #[doc = "Checks if the value of the field is `ERQ3_1`"]
    #[inline(always)]
    pub fn is_erq3_1(&self) -> bool {
        *self == ERQ3_A::ERQ3_1
    }
}
#[doc = "Write proxy for field `ERQ3`"]
pub struct ERQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq3_0(self) -> &'a mut W {
        self.variant(ERQ3_A::ERQ3_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq3_1(self) -> &'a mut W {
        self.variant(ERQ3_A::ERQ3_1)
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
#[doc = "Enable DMA Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ4_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ4_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ4_1 = 1,
}
impl From<ERQ4_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ4`"]
pub type ERQ4_R = crate::R<bool, ERQ4_A>;
impl ERQ4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ4_A {
        match self.bits {
            false => ERQ4_A::ERQ4_0,
            true => ERQ4_A::ERQ4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ4_0`"]
    #[inline(always)]
    pub fn is_erq4_0(&self) -> bool {
        *self == ERQ4_A::ERQ4_0
    }
    #[doc = "Checks if the value of the field is `ERQ4_1`"]
    #[inline(always)]
    pub fn is_erq4_1(&self) -> bool {
        *self == ERQ4_A::ERQ4_1
    }
}
#[doc = "Write proxy for field `ERQ4`"]
pub struct ERQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq4_0(self) -> &'a mut W {
        self.variant(ERQ4_A::ERQ4_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq4_1(self) -> &'a mut W {
        self.variant(ERQ4_A::ERQ4_1)
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
#[doc = "Enable DMA Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ5_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ5_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ5_1 = 1,
}
impl From<ERQ5_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ5`"]
pub type ERQ5_R = crate::R<bool, ERQ5_A>;
impl ERQ5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ5_A {
        match self.bits {
            false => ERQ5_A::ERQ5_0,
            true => ERQ5_A::ERQ5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ5_0`"]
    #[inline(always)]
    pub fn is_erq5_0(&self) -> bool {
        *self == ERQ5_A::ERQ5_0
    }
    #[doc = "Checks if the value of the field is `ERQ5_1`"]
    #[inline(always)]
    pub fn is_erq5_1(&self) -> bool {
        *self == ERQ5_A::ERQ5_1
    }
}
#[doc = "Write proxy for field `ERQ5`"]
pub struct ERQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq5_0(self) -> &'a mut W {
        self.variant(ERQ5_A::ERQ5_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq5_1(self) -> &'a mut W {
        self.variant(ERQ5_A::ERQ5_1)
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
#[doc = "Enable DMA Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ6_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ6_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ6_1 = 1,
}
impl From<ERQ6_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ6`"]
pub type ERQ6_R = crate::R<bool, ERQ6_A>;
impl ERQ6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ6_A {
        match self.bits {
            false => ERQ6_A::ERQ6_0,
            true => ERQ6_A::ERQ6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ6_0`"]
    #[inline(always)]
    pub fn is_erq6_0(&self) -> bool {
        *self == ERQ6_A::ERQ6_0
    }
    #[doc = "Checks if the value of the field is `ERQ6_1`"]
    #[inline(always)]
    pub fn is_erq6_1(&self) -> bool {
        *self == ERQ6_A::ERQ6_1
    }
}
#[doc = "Write proxy for field `ERQ6`"]
pub struct ERQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq6_0(self) -> &'a mut W {
        self.variant(ERQ6_A::ERQ6_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq6_1(self) -> &'a mut W {
        self.variant(ERQ6_A::ERQ6_1)
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
#[doc = "Enable DMA Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ7_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ7_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ7_1 = 1,
}
impl From<ERQ7_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ7`"]
pub type ERQ7_R = crate::R<bool, ERQ7_A>;
impl ERQ7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ7_A {
        match self.bits {
            false => ERQ7_A::ERQ7_0,
            true => ERQ7_A::ERQ7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ7_0`"]
    #[inline(always)]
    pub fn is_erq7_0(&self) -> bool {
        *self == ERQ7_A::ERQ7_0
    }
    #[doc = "Checks if the value of the field is `ERQ7_1`"]
    #[inline(always)]
    pub fn is_erq7_1(&self) -> bool {
        *self == ERQ7_A::ERQ7_1
    }
}
#[doc = "Write proxy for field `ERQ7`"]
pub struct ERQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq7_0(self) -> &'a mut W {
        self.variant(ERQ7_A::ERQ7_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq7_1(self) -> &'a mut W {
        self.variant(ERQ7_A::ERQ7_1)
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
#[doc = "Enable DMA Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ8_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ8_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ8_1 = 1,
}
impl From<ERQ8_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ8`"]
pub type ERQ8_R = crate::R<bool, ERQ8_A>;
impl ERQ8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ8_A {
        match self.bits {
            false => ERQ8_A::ERQ8_0,
            true => ERQ8_A::ERQ8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ8_0`"]
    #[inline(always)]
    pub fn is_erq8_0(&self) -> bool {
        *self == ERQ8_A::ERQ8_0
    }
    #[doc = "Checks if the value of the field is `ERQ8_1`"]
    #[inline(always)]
    pub fn is_erq8_1(&self) -> bool {
        *self == ERQ8_A::ERQ8_1
    }
}
#[doc = "Write proxy for field `ERQ8`"]
pub struct ERQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq8_0(self) -> &'a mut W {
        self.variant(ERQ8_A::ERQ8_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq8_1(self) -> &'a mut W {
        self.variant(ERQ8_A::ERQ8_1)
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
#[doc = "Enable DMA Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ9_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ9_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ9_1 = 1,
}
impl From<ERQ9_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ9`"]
pub type ERQ9_R = crate::R<bool, ERQ9_A>;
impl ERQ9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ9_A {
        match self.bits {
            false => ERQ9_A::ERQ9_0,
            true => ERQ9_A::ERQ9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ9_0`"]
    #[inline(always)]
    pub fn is_erq9_0(&self) -> bool {
        *self == ERQ9_A::ERQ9_0
    }
    #[doc = "Checks if the value of the field is `ERQ9_1`"]
    #[inline(always)]
    pub fn is_erq9_1(&self) -> bool {
        *self == ERQ9_A::ERQ9_1
    }
}
#[doc = "Write proxy for field `ERQ9`"]
pub struct ERQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq9_0(self) -> &'a mut W {
        self.variant(ERQ9_A::ERQ9_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq9_1(self) -> &'a mut W {
        self.variant(ERQ9_A::ERQ9_1)
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
#[doc = "Enable DMA Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ10_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ10_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ10_1 = 1,
}
impl From<ERQ10_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ10`"]
pub type ERQ10_R = crate::R<bool, ERQ10_A>;
impl ERQ10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ10_A {
        match self.bits {
            false => ERQ10_A::ERQ10_0,
            true => ERQ10_A::ERQ10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ10_0`"]
    #[inline(always)]
    pub fn is_erq10_0(&self) -> bool {
        *self == ERQ10_A::ERQ10_0
    }
    #[doc = "Checks if the value of the field is `ERQ10_1`"]
    #[inline(always)]
    pub fn is_erq10_1(&self) -> bool {
        *self == ERQ10_A::ERQ10_1
    }
}
#[doc = "Write proxy for field `ERQ10`"]
pub struct ERQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq10_0(self) -> &'a mut W {
        self.variant(ERQ10_A::ERQ10_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq10_1(self) -> &'a mut W {
        self.variant(ERQ10_A::ERQ10_1)
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
#[doc = "Enable DMA Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ11_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ11_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ11_1 = 1,
}
impl From<ERQ11_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ11`"]
pub type ERQ11_R = crate::R<bool, ERQ11_A>;
impl ERQ11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ11_A {
        match self.bits {
            false => ERQ11_A::ERQ11_0,
            true => ERQ11_A::ERQ11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ11_0`"]
    #[inline(always)]
    pub fn is_erq11_0(&self) -> bool {
        *self == ERQ11_A::ERQ11_0
    }
    #[doc = "Checks if the value of the field is `ERQ11_1`"]
    #[inline(always)]
    pub fn is_erq11_1(&self) -> bool {
        *self == ERQ11_A::ERQ11_1
    }
}
#[doc = "Write proxy for field `ERQ11`"]
pub struct ERQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq11_0(self) -> &'a mut W {
        self.variant(ERQ11_A::ERQ11_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq11_1(self) -> &'a mut W {
        self.variant(ERQ11_A::ERQ11_1)
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
#[doc = "Enable DMA Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ12_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ12_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ12_1 = 1,
}
impl From<ERQ12_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ12`"]
pub type ERQ12_R = crate::R<bool, ERQ12_A>;
impl ERQ12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ12_A {
        match self.bits {
            false => ERQ12_A::ERQ12_0,
            true => ERQ12_A::ERQ12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ12_0`"]
    #[inline(always)]
    pub fn is_erq12_0(&self) -> bool {
        *self == ERQ12_A::ERQ12_0
    }
    #[doc = "Checks if the value of the field is `ERQ12_1`"]
    #[inline(always)]
    pub fn is_erq12_1(&self) -> bool {
        *self == ERQ12_A::ERQ12_1
    }
}
#[doc = "Write proxy for field `ERQ12`"]
pub struct ERQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq12_0(self) -> &'a mut W {
        self.variant(ERQ12_A::ERQ12_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq12_1(self) -> &'a mut W {
        self.variant(ERQ12_A::ERQ12_1)
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
#[doc = "Enable DMA Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ13_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ13_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ13_1 = 1,
}
impl From<ERQ13_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ13`"]
pub type ERQ13_R = crate::R<bool, ERQ13_A>;
impl ERQ13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ13_A {
        match self.bits {
            false => ERQ13_A::ERQ13_0,
            true => ERQ13_A::ERQ13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ13_0`"]
    #[inline(always)]
    pub fn is_erq13_0(&self) -> bool {
        *self == ERQ13_A::ERQ13_0
    }
    #[doc = "Checks if the value of the field is `ERQ13_1`"]
    #[inline(always)]
    pub fn is_erq13_1(&self) -> bool {
        *self == ERQ13_A::ERQ13_1
    }
}
#[doc = "Write proxy for field `ERQ13`"]
pub struct ERQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq13_0(self) -> &'a mut W {
        self.variant(ERQ13_A::ERQ13_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq13_1(self) -> &'a mut W {
        self.variant(ERQ13_A::ERQ13_1)
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
#[doc = "Enable DMA Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ14_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ14_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ14_1 = 1,
}
impl From<ERQ14_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ14`"]
pub type ERQ14_R = crate::R<bool, ERQ14_A>;
impl ERQ14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ14_A {
        match self.bits {
            false => ERQ14_A::ERQ14_0,
            true => ERQ14_A::ERQ14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ14_0`"]
    #[inline(always)]
    pub fn is_erq14_0(&self) -> bool {
        *self == ERQ14_A::ERQ14_0
    }
    #[doc = "Checks if the value of the field is `ERQ14_1`"]
    #[inline(always)]
    pub fn is_erq14_1(&self) -> bool {
        *self == ERQ14_A::ERQ14_1
    }
}
#[doc = "Write proxy for field `ERQ14`"]
pub struct ERQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq14_0(self) -> &'a mut W {
        self.variant(ERQ14_A::ERQ14_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq14_1(self) -> &'a mut W {
        self.variant(ERQ14_A::ERQ14_1)
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
#[doc = "Enable DMA Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ15_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ15_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ15_1 = 1,
}
impl From<ERQ15_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ15`"]
pub type ERQ15_R = crate::R<bool, ERQ15_A>;
impl ERQ15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ15_A {
        match self.bits {
            false => ERQ15_A::ERQ15_0,
            true => ERQ15_A::ERQ15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ15_0`"]
    #[inline(always)]
    pub fn is_erq15_0(&self) -> bool {
        *self == ERQ15_A::ERQ15_0
    }
    #[doc = "Checks if the value of the field is `ERQ15_1`"]
    #[inline(always)]
    pub fn is_erq15_1(&self) -> bool {
        *self == ERQ15_A::ERQ15_1
    }
}
#[doc = "Write proxy for field `ERQ15`"]
pub struct ERQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq15_0(self) -> &'a mut W {
        self.variant(ERQ15_A::ERQ15_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq15_1(self) -> &'a mut W {
        self.variant(ERQ15_A::ERQ15_1)
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
#[doc = "Enable DMA Request 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ16_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ16_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ16_1 = 1,
}
impl From<ERQ16_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ16`"]
pub type ERQ16_R = crate::R<bool, ERQ16_A>;
impl ERQ16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ16_A {
        match self.bits {
            false => ERQ16_A::ERQ16_0,
            true => ERQ16_A::ERQ16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ16_0`"]
    #[inline(always)]
    pub fn is_erq16_0(&self) -> bool {
        *self == ERQ16_A::ERQ16_0
    }
    #[doc = "Checks if the value of the field is `ERQ16_1`"]
    #[inline(always)]
    pub fn is_erq16_1(&self) -> bool {
        *self == ERQ16_A::ERQ16_1
    }
}
#[doc = "Write proxy for field `ERQ16`"]
pub struct ERQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq16_0(self) -> &'a mut W {
        self.variant(ERQ16_A::ERQ16_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq16_1(self) -> &'a mut W {
        self.variant(ERQ16_A::ERQ16_1)
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
#[doc = "Enable DMA Request 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ17_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ17_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ17_1 = 1,
}
impl From<ERQ17_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ17`"]
pub type ERQ17_R = crate::R<bool, ERQ17_A>;
impl ERQ17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ17_A {
        match self.bits {
            false => ERQ17_A::ERQ17_0,
            true => ERQ17_A::ERQ17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ17_0`"]
    #[inline(always)]
    pub fn is_erq17_0(&self) -> bool {
        *self == ERQ17_A::ERQ17_0
    }
    #[doc = "Checks if the value of the field is `ERQ17_1`"]
    #[inline(always)]
    pub fn is_erq17_1(&self) -> bool {
        *self == ERQ17_A::ERQ17_1
    }
}
#[doc = "Write proxy for field `ERQ17`"]
pub struct ERQ17_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq17_0(self) -> &'a mut W {
        self.variant(ERQ17_A::ERQ17_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq17_1(self) -> &'a mut W {
        self.variant(ERQ17_A::ERQ17_1)
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
#[doc = "Enable DMA Request 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ18_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ18_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ18_1 = 1,
}
impl From<ERQ18_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ18`"]
pub type ERQ18_R = crate::R<bool, ERQ18_A>;
impl ERQ18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ18_A {
        match self.bits {
            false => ERQ18_A::ERQ18_0,
            true => ERQ18_A::ERQ18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ18_0`"]
    #[inline(always)]
    pub fn is_erq18_0(&self) -> bool {
        *self == ERQ18_A::ERQ18_0
    }
    #[doc = "Checks if the value of the field is `ERQ18_1`"]
    #[inline(always)]
    pub fn is_erq18_1(&self) -> bool {
        *self == ERQ18_A::ERQ18_1
    }
}
#[doc = "Write proxy for field `ERQ18`"]
pub struct ERQ18_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq18_0(self) -> &'a mut W {
        self.variant(ERQ18_A::ERQ18_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq18_1(self) -> &'a mut W {
        self.variant(ERQ18_A::ERQ18_1)
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
#[doc = "Enable DMA Request 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ19_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ19_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ19_1 = 1,
}
impl From<ERQ19_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ19`"]
pub type ERQ19_R = crate::R<bool, ERQ19_A>;
impl ERQ19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ19_A {
        match self.bits {
            false => ERQ19_A::ERQ19_0,
            true => ERQ19_A::ERQ19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ19_0`"]
    #[inline(always)]
    pub fn is_erq19_0(&self) -> bool {
        *self == ERQ19_A::ERQ19_0
    }
    #[doc = "Checks if the value of the field is `ERQ19_1`"]
    #[inline(always)]
    pub fn is_erq19_1(&self) -> bool {
        *self == ERQ19_A::ERQ19_1
    }
}
#[doc = "Write proxy for field `ERQ19`"]
pub struct ERQ19_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq19_0(self) -> &'a mut W {
        self.variant(ERQ19_A::ERQ19_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq19_1(self) -> &'a mut W {
        self.variant(ERQ19_A::ERQ19_1)
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
#[doc = "Enable DMA Request 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ20_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ20_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ20_1 = 1,
}
impl From<ERQ20_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ20`"]
pub type ERQ20_R = crate::R<bool, ERQ20_A>;
impl ERQ20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ20_A {
        match self.bits {
            false => ERQ20_A::ERQ20_0,
            true => ERQ20_A::ERQ20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ20_0`"]
    #[inline(always)]
    pub fn is_erq20_0(&self) -> bool {
        *self == ERQ20_A::ERQ20_0
    }
    #[doc = "Checks if the value of the field is `ERQ20_1`"]
    #[inline(always)]
    pub fn is_erq20_1(&self) -> bool {
        *self == ERQ20_A::ERQ20_1
    }
}
#[doc = "Write proxy for field `ERQ20`"]
pub struct ERQ20_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq20_0(self) -> &'a mut W {
        self.variant(ERQ20_A::ERQ20_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq20_1(self) -> &'a mut W {
        self.variant(ERQ20_A::ERQ20_1)
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
#[doc = "Enable DMA Request 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ21_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ21_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ21_1 = 1,
}
impl From<ERQ21_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ21`"]
pub type ERQ21_R = crate::R<bool, ERQ21_A>;
impl ERQ21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ21_A {
        match self.bits {
            false => ERQ21_A::ERQ21_0,
            true => ERQ21_A::ERQ21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ21_0`"]
    #[inline(always)]
    pub fn is_erq21_0(&self) -> bool {
        *self == ERQ21_A::ERQ21_0
    }
    #[doc = "Checks if the value of the field is `ERQ21_1`"]
    #[inline(always)]
    pub fn is_erq21_1(&self) -> bool {
        *self == ERQ21_A::ERQ21_1
    }
}
#[doc = "Write proxy for field `ERQ21`"]
pub struct ERQ21_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq21_0(self) -> &'a mut W {
        self.variant(ERQ21_A::ERQ21_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq21_1(self) -> &'a mut W {
        self.variant(ERQ21_A::ERQ21_1)
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
#[doc = "Enable DMA Request 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ22_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ22_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ22_1 = 1,
}
impl From<ERQ22_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ22`"]
pub type ERQ22_R = crate::R<bool, ERQ22_A>;
impl ERQ22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ22_A {
        match self.bits {
            false => ERQ22_A::ERQ22_0,
            true => ERQ22_A::ERQ22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ22_0`"]
    #[inline(always)]
    pub fn is_erq22_0(&self) -> bool {
        *self == ERQ22_A::ERQ22_0
    }
    #[doc = "Checks if the value of the field is `ERQ22_1`"]
    #[inline(always)]
    pub fn is_erq22_1(&self) -> bool {
        *self == ERQ22_A::ERQ22_1
    }
}
#[doc = "Write proxy for field `ERQ22`"]
pub struct ERQ22_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq22_0(self) -> &'a mut W {
        self.variant(ERQ22_A::ERQ22_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq22_1(self) -> &'a mut W {
        self.variant(ERQ22_A::ERQ22_1)
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
#[doc = "Enable DMA Request 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ23_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ23_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ23_1 = 1,
}
impl From<ERQ23_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ23`"]
pub type ERQ23_R = crate::R<bool, ERQ23_A>;
impl ERQ23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ23_A {
        match self.bits {
            false => ERQ23_A::ERQ23_0,
            true => ERQ23_A::ERQ23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ23_0`"]
    #[inline(always)]
    pub fn is_erq23_0(&self) -> bool {
        *self == ERQ23_A::ERQ23_0
    }
    #[doc = "Checks if the value of the field is `ERQ23_1`"]
    #[inline(always)]
    pub fn is_erq23_1(&self) -> bool {
        *self == ERQ23_A::ERQ23_1
    }
}
#[doc = "Write proxy for field `ERQ23`"]
pub struct ERQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq23_0(self) -> &'a mut W {
        self.variant(ERQ23_A::ERQ23_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq23_1(self) -> &'a mut W {
        self.variant(ERQ23_A::ERQ23_1)
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
#[doc = "Enable DMA Request 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ24_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ24_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ24_1 = 1,
}
impl From<ERQ24_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ24`"]
pub type ERQ24_R = crate::R<bool, ERQ24_A>;
impl ERQ24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ24_A {
        match self.bits {
            false => ERQ24_A::ERQ24_0,
            true => ERQ24_A::ERQ24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ24_0`"]
    #[inline(always)]
    pub fn is_erq24_0(&self) -> bool {
        *self == ERQ24_A::ERQ24_0
    }
    #[doc = "Checks if the value of the field is `ERQ24_1`"]
    #[inline(always)]
    pub fn is_erq24_1(&self) -> bool {
        *self == ERQ24_A::ERQ24_1
    }
}
#[doc = "Write proxy for field `ERQ24`"]
pub struct ERQ24_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq24_0(self) -> &'a mut W {
        self.variant(ERQ24_A::ERQ24_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq24_1(self) -> &'a mut W {
        self.variant(ERQ24_A::ERQ24_1)
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
#[doc = "Enable DMA Request 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ25_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ25_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ25_1 = 1,
}
impl From<ERQ25_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ25`"]
pub type ERQ25_R = crate::R<bool, ERQ25_A>;
impl ERQ25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ25_A {
        match self.bits {
            false => ERQ25_A::ERQ25_0,
            true => ERQ25_A::ERQ25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ25_0`"]
    #[inline(always)]
    pub fn is_erq25_0(&self) -> bool {
        *self == ERQ25_A::ERQ25_0
    }
    #[doc = "Checks if the value of the field is `ERQ25_1`"]
    #[inline(always)]
    pub fn is_erq25_1(&self) -> bool {
        *self == ERQ25_A::ERQ25_1
    }
}
#[doc = "Write proxy for field `ERQ25`"]
pub struct ERQ25_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq25_0(self) -> &'a mut W {
        self.variant(ERQ25_A::ERQ25_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq25_1(self) -> &'a mut W {
        self.variant(ERQ25_A::ERQ25_1)
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
#[doc = "Enable DMA Request 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ26_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ26_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ26_1 = 1,
}
impl From<ERQ26_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ26`"]
pub type ERQ26_R = crate::R<bool, ERQ26_A>;
impl ERQ26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ26_A {
        match self.bits {
            false => ERQ26_A::ERQ26_0,
            true => ERQ26_A::ERQ26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ26_0`"]
    #[inline(always)]
    pub fn is_erq26_0(&self) -> bool {
        *self == ERQ26_A::ERQ26_0
    }
    #[doc = "Checks if the value of the field is `ERQ26_1`"]
    #[inline(always)]
    pub fn is_erq26_1(&self) -> bool {
        *self == ERQ26_A::ERQ26_1
    }
}
#[doc = "Write proxy for field `ERQ26`"]
pub struct ERQ26_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq26_0(self) -> &'a mut W {
        self.variant(ERQ26_A::ERQ26_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq26_1(self) -> &'a mut W {
        self.variant(ERQ26_A::ERQ26_1)
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
#[doc = "Enable DMA Request 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ27_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ27_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ27_1 = 1,
}
impl From<ERQ27_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ27`"]
pub type ERQ27_R = crate::R<bool, ERQ27_A>;
impl ERQ27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ27_A {
        match self.bits {
            false => ERQ27_A::ERQ27_0,
            true => ERQ27_A::ERQ27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ27_0`"]
    #[inline(always)]
    pub fn is_erq27_0(&self) -> bool {
        *self == ERQ27_A::ERQ27_0
    }
    #[doc = "Checks if the value of the field is `ERQ27_1`"]
    #[inline(always)]
    pub fn is_erq27_1(&self) -> bool {
        *self == ERQ27_A::ERQ27_1
    }
}
#[doc = "Write proxy for field `ERQ27`"]
pub struct ERQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq27_0(self) -> &'a mut W {
        self.variant(ERQ27_A::ERQ27_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq27_1(self) -> &'a mut W {
        self.variant(ERQ27_A::ERQ27_1)
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
#[doc = "Enable DMA Request 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ28_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ28_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ28_1 = 1,
}
impl From<ERQ28_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ28`"]
pub type ERQ28_R = crate::R<bool, ERQ28_A>;
impl ERQ28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ28_A {
        match self.bits {
            false => ERQ28_A::ERQ28_0,
            true => ERQ28_A::ERQ28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ28_0`"]
    #[inline(always)]
    pub fn is_erq28_0(&self) -> bool {
        *self == ERQ28_A::ERQ28_0
    }
    #[doc = "Checks if the value of the field is `ERQ28_1`"]
    #[inline(always)]
    pub fn is_erq28_1(&self) -> bool {
        *self == ERQ28_A::ERQ28_1
    }
}
#[doc = "Write proxy for field `ERQ28`"]
pub struct ERQ28_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq28_0(self) -> &'a mut W {
        self.variant(ERQ28_A::ERQ28_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq28_1(self) -> &'a mut W {
        self.variant(ERQ28_A::ERQ28_1)
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
#[doc = "Enable DMA Request 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ29_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ29_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ29_1 = 1,
}
impl From<ERQ29_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ29`"]
pub type ERQ29_R = crate::R<bool, ERQ29_A>;
impl ERQ29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ29_A {
        match self.bits {
            false => ERQ29_A::ERQ29_0,
            true => ERQ29_A::ERQ29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ29_0`"]
    #[inline(always)]
    pub fn is_erq29_0(&self) -> bool {
        *self == ERQ29_A::ERQ29_0
    }
    #[doc = "Checks if the value of the field is `ERQ29_1`"]
    #[inline(always)]
    pub fn is_erq29_1(&self) -> bool {
        *self == ERQ29_A::ERQ29_1
    }
}
#[doc = "Write proxy for field `ERQ29`"]
pub struct ERQ29_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq29_0(self) -> &'a mut W {
        self.variant(ERQ29_A::ERQ29_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq29_1(self) -> &'a mut W {
        self.variant(ERQ29_A::ERQ29_1)
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
#[doc = "Enable DMA Request 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ30_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ30_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ30_1 = 1,
}
impl From<ERQ30_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ30`"]
pub type ERQ30_R = crate::R<bool, ERQ30_A>;
impl ERQ30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ30_A {
        match self.bits {
            false => ERQ30_A::ERQ30_0,
            true => ERQ30_A::ERQ30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ30_0`"]
    #[inline(always)]
    pub fn is_erq30_0(&self) -> bool {
        *self == ERQ30_A::ERQ30_0
    }
    #[doc = "Checks if the value of the field is `ERQ30_1`"]
    #[inline(always)]
    pub fn is_erq30_1(&self) -> bool {
        *self == ERQ30_A::ERQ30_1
    }
}
#[doc = "Write proxy for field `ERQ30`"]
pub struct ERQ30_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq30_0(self) -> &'a mut W {
        self.variant(ERQ30_A::ERQ30_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq30_1(self) -> &'a mut W {
        self.variant(ERQ30_A::ERQ30_1)
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
#[doc = "Enable DMA Request 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ31_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ31_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ31_1 = 1,
}
impl From<ERQ31_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERQ31`"]
pub type ERQ31_R = crate::R<bool, ERQ31_A>;
impl ERQ31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ31_A {
        match self.bits {
            false => ERQ31_A::ERQ31_0,
            true => ERQ31_A::ERQ31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ31_0`"]
    #[inline(always)]
    pub fn is_erq31_0(&self) -> bool {
        *self == ERQ31_A::ERQ31_0
    }
    #[doc = "Checks if the value of the field is `ERQ31_1`"]
    #[inline(always)]
    pub fn is_erq31_1(&self) -> bool {
        *self == ERQ31_A::ERQ31_1
    }
}
#[doc = "Write proxy for field `ERQ31`"]
pub struct ERQ31_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq31_0(self) -> &'a mut W {
        self.variant(ERQ31_A::ERQ31_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq31_1(self) -> &'a mut W {
        self.variant(ERQ31_A::ERQ31_1)
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&self) -> ERQ4_R {
        ERQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&self) -> ERQ5_R {
        ERQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&self) -> ERQ6_R {
        ERQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&self) -> ERQ7_R {
        ERQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&self) -> ERQ8_R {
        ERQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&self) -> ERQ9_R {
        ERQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&self) -> ERQ10_R {
        ERQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&self) -> ERQ11_R {
        ERQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&self) -> ERQ12_R {
        ERQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&self) -> ERQ13_R {
        ERQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&self) -> ERQ14_R {
        ERQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&self) -> ERQ15_R {
        ERQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline(always)]
    pub fn erq16(&self) -> ERQ16_R {
        ERQ16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline(always)]
    pub fn erq17(&self) -> ERQ17_R {
        ERQ17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline(always)]
    pub fn erq18(&self) -> ERQ18_R {
        ERQ18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline(always)]
    pub fn erq19(&self) -> ERQ19_R {
        ERQ19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline(always)]
    pub fn erq20(&self) -> ERQ20_R {
        ERQ20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline(always)]
    pub fn erq21(&self) -> ERQ21_R {
        ERQ21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline(always)]
    pub fn erq22(&self) -> ERQ22_R {
        ERQ22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline(always)]
    pub fn erq23(&self) -> ERQ23_R {
        ERQ23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline(always)]
    pub fn erq24(&self) -> ERQ24_R {
        ERQ24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline(always)]
    pub fn erq25(&self) -> ERQ25_R {
        ERQ25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline(always)]
    pub fn erq26(&self) -> ERQ26_R {
        ERQ26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline(always)]
    pub fn erq27(&self) -> ERQ27_R {
        ERQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline(always)]
    pub fn erq28(&self) -> ERQ28_R {
        ERQ28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline(always)]
    pub fn erq29(&self) -> ERQ29_R {
        ERQ29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline(always)]
    pub fn erq30(&self) -> ERQ30_R {
        ERQ30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline(always)]
    pub fn erq31(&self) -> ERQ31_R {
        ERQ31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&mut self) -> ERQ0_W {
        ERQ0_W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&mut self) -> ERQ1_W {
        ERQ1_W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&mut self) -> ERQ2_W {
        ERQ2_W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&mut self) -> ERQ3_W {
        ERQ3_W { w: self }
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&mut self) -> ERQ4_W {
        ERQ4_W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&mut self) -> ERQ5_W {
        ERQ5_W { w: self }
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&mut self) -> ERQ6_W {
        ERQ6_W { w: self }
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&mut self) -> ERQ7_W {
        ERQ7_W { w: self }
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&mut self) -> ERQ8_W {
        ERQ8_W { w: self }
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&mut self) -> ERQ9_W {
        ERQ9_W { w: self }
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&mut self) -> ERQ10_W {
        ERQ10_W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&mut self) -> ERQ11_W {
        ERQ11_W { w: self }
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&mut self) -> ERQ12_W {
        ERQ12_W { w: self }
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&mut self) -> ERQ13_W {
        ERQ13_W { w: self }
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&mut self) -> ERQ14_W {
        ERQ14_W { w: self }
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&mut self) -> ERQ15_W {
        ERQ15_W { w: self }
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline(always)]
    pub fn erq16(&mut self) -> ERQ16_W {
        ERQ16_W { w: self }
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline(always)]
    pub fn erq17(&mut self) -> ERQ17_W {
        ERQ17_W { w: self }
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline(always)]
    pub fn erq18(&mut self) -> ERQ18_W {
        ERQ18_W { w: self }
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline(always)]
    pub fn erq19(&mut self) -> ERQ19_W {
        ERQ19_W { w: self }
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline(always)]
    pub fn erq20(&mut self) -> ERQ20_W {
        ERQ20_W { w: self }
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline(always)]
    pub fn erq21(&mut self) -> ERQ21_W {
        ERQ21_W { w: self }
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline(always)]
    pub fn erq22(&mut self) -> ERQ22_W {
        ERQ22_W { w: self }
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline(always)]
    pub fn erq23(&mut self) -> ERQ23_W {
        ERQ23_W { w: self }
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline(always)]
    pub fn erq24(&mut self) -> ERQ24_W {
        ERQ24_W { w: self }
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline(always)]
    pub fn erq25(&mut self) -> ERQ25_W {
        ERQ25_W { w: self }
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline(always)]
    pub fn erq26(&mut self) -> ERQ26_W {
        ERQ26_W { w: self }
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline(always)]
    pub fn erq27(&mut self) -> ERQ27_W {
        ERQ27_W { w: self }
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline(always)]
    pub fn erq28(&mut self) -> ERQ28_W {
        ERQ28_W { w: self }
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline(always)]
    pub fn erq29(&mut self) -> ERQ29_W {
        ERQ29_W { w: self }
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline(always)]
    pub fn erq30(&mut self) -> ERQ30_W {
        ERQ30_W { w: self }
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline(always)]
    pub fn erq31(&mut self) -> ERQ31_W {
        ERQ31_W { w: self }
    }
}
