#[doc = "Reader of register TGSR"]
pub type R = crate::R<u32, super::TGSR>;
#[doc = "Writer for register TGSR"]
pub type W = crate::W<u32, super::TGSR>;
#[doc = "Register TGSR `reset()`'s with value 0"]
impl crate::ResetValue for super::TGSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Copy Of Timer Flag For Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF0_A {
    #[doc = "0: Timer Flag for Channel 0 is clear"]
    TF0_0 = 0,
    #[doc = "1: Timer Flag for Channel 0 is set"]
    TF0_1 = 1,
}
impl From<TF0_A> for bool {
    #[inline(always)]
    fn from(variant: TF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TF0`"]
pub type TF0_R = crate::R<bool, TF0_A>;
impl TF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF0_A {
        match self.bits {
            false => TF0_A::TF0_0,
            true => TF0_A::TF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF0_0`"]
    #[inline(always)]
    pub fn is_tf0_0(&self) -> bool {
        *self == TF0_A::TF0_0
    }
    #[doc = "Checks if the value of the field is `TF0_1`"]
    #[inline(always)]
    pub fn is_tf0_1(&self) -> bool {
        *self == TF0_A::TF0_1
    }
}
#[doc = "Write proxy for field `TF0`"]
pub struct TF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer Flag for Channel 0 is clear"]
    #[inline(always)]
    pub fn tf0_0(self) -> &'a mut W {
        self.variant(TF0_A::TF0_0)
    }
    #[doc = "Timer Flag for Channel 0 is set"]
    #[inline(always)]
    pub fn tf0_1(self) -> &'a mut W {
        self.variant(TF0_A::TF0_1)
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
#[doc = "Copy Of Timer Flag For Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF1_A {
    #[doc = "0: Timer Flag for Channel 1 is clear"]
    TF1_0 = 0,
    #[doc = "1: Timer Flag for Channel 1 is set"]
    TF1_1 = 1,
}
impl From<TF1_A> for bool {
    #[inline(always)]
    fn from(variant: TF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TF1`"]
pub type TF1_R = crate::R<bool, TF1_A>;
impl TF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF1_A {
        match self.bits {
            false => TF1_A::TF1_0,
            true => TF1_A::TF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF1_0`"]
    #[inline(always)]
    pub fn is_tf1_0(&self) -> bool {
        *self == TF1_A::TF1_0
    }
    #[doc = "Checks if the value of the field is `TF1_1`"]
    #[inline(always)]
    pub fn is_tf1_1(&self) -> bool {
        *self == TF1_A::TF1_1
    }
}
#[doc = "Write proxy for field `TF1`"]
pub struct TF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer Flag for Channel 1 is clear"]
    #[inline(always)]
    pub fn tf1_0(self) -> &'a mut W {
        self.variant(TF1_A::TF1_0)
    }
    #[doc = "Timer Flag for Channel 1 is set"]
    #[inline(always)]
    pub fn tf1_1(self) -> &'a mut W {
        self.variant(TF1_A::TF1_1)
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
#[doc = "Copy Of Timer Flag For Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF2_A {
    #[doc = "0: Timer Flag for Channel 2 is clear"]
    TF2_0 = 0,
    #[doc = "1: Timer Flag for Channel 2 is set"]
    TF2_1 = 1,
}
impl From<TF2_A> for bool {
    #[inline(always)]
    fn from(variant: TF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TF2`"]
pub type TF2_R = crate::R<bool, TF2_A>;
impl TF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF2_A {
        match self.bits {
            false => TF2_A::TF2_0,
            true => TF2_A::TF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF2_0`"]
    #[inline(always)]
    pub fn is_tf2_0(&self) -> bool {
        *self == TF2_A::TF2_0
    }
    #[doc = "Checks if the value of the field is `TF2_1`"]
    #[inline(always)]
    pub fn is_tf2_1(&self) -> bool {
        *self == TF2_A::TF2_1
    }
}
#[doc = "Write proxy for field `TF2`"]
pub struct TF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer Flag for Channel 2 is clear"]
    #[inline(always)]
    pub fn tf2_0(self) -> &'a mut W {
        self.variant(TF2_A::TF2_0)
    }
    #[doc = "Timer Flag for Channel 2 is set"]
    #[inline(always)]
    pub fn tf2_1(self) -> &'a mut W {
        self.variant(TF2_A::TF2_1)
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
#[doc = "Copy Of Timer Flag For Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF3_A {
    #[doc = "0: Timer Flag for Channel 3 is clear"]
    TF3_0 = 0,
    #[doc = "1: Timer Flag for Channel 3 is set"]
    TF3_1 = 1,
}
impl From<TF3_A> for bool {
    #[inline(always)]
    fn from(variant: TF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TF3`"]
pub type TF3_R = crate::R<bool, TF3_A>;
impl TF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF3_A {
        match self.bits {
            false => TF3_A::TF3_0,
            true => TF3_A::TF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF3_0`"]
    #[inline(always)]
    pub fn is_tf3_0(&self) -> bool {
        *self == TF3_A::TF3_0
    }
    #[doc = "Checks if the value of the field is `TF3_1`"]
    #[inline(always)]
    pub fn is_tf3_1(&self) -> bool {
        *self == TF3_A::TF3_1
    }
}
#[doc = "Write proxy for field `TF3`"]
pub struct TF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer Flag for Channel 3 is clear"]
    #[inline(always)]
    pub fn tf3_0(self) -> &'a mut W {
        self.variant(TF3_A::TF3_0)
    }
    #[doc = "Timer Flag for Channel 3 is set"]
    #[inline(always)]
    pub fn tf3_1(self) -> &'a mut W {
        self.variant(TF3_A::TF3_1)
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
impl R {
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub fn tf0(&self) -> TF0_R {
        TF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub fn tf1(&self) -> TF1_R {
        TF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub fn tf2(&self) -> TF2_R {
        TF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub fn tf3(&self) -> TF3_R {
        TF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Copy Of Timer Flag For Channel 0"]
    #[inline(always)]
    pub fn tf0(&mut self) -> TF0_W {
        TF0_W { w: self }
    }
    #[doc = "Bit 1 - Copy Of Timer Flag For Channel 1"]
    #[inline(always)]
    pub fn tf1(&mut self) -> TF1_W {
        TF1_W { w: self }
    }
    #[doc = "Bit 2 - Copy Of Timer Flag For Channel 2"]
    #[inline(always)]
    pub fn tf2(&mut self) -> TF2_W {
        TF2_W { w: self }
    }
    #[doc = "Bit 3 - Copy Of Timer Flag For Channel 3"]
    #[inline(always)]
    pub fn tf3(&mut self) -> TF3_W {
        TF3_W { w: self }
    }
}
