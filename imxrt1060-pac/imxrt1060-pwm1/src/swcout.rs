#[doc = "Reader of register SWCOUT"]
pub type R = crate::R<u16, super::SWCOUT>;
#[doc = "Writer for register SWCOUT"]
pub type W = crate::W<u16, super::SWCOUT>;
#[doc = "Register SWCOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::SWCOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Submodule 0 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SM0OUT45_1,
}
impl From<SM0OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM0OUT45_A) -> Self {
        match variant {
            SM0OUT45_A::SM0OUT45_0 => false,
            SM0OUT45_A::SM0OUT45_1 => true,
        }
    }
}
#[doc = "Reader of field `SM0OUT45`"]
pub type SM0OUT45_R = crate::R<bool, SM0OUT45_A>;
impl SM0OUT45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0OUT45_A {
        match self.bits {
            false => SM0OUT45_A::SM0OUT45_0,
            true => SM0OUT45_A::SM0OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM0OUT45_0`"]
    #[inline(always)]
    pub fn is_sm0out45_0(&self) -> bool {
        *self == SM0OUT45_A::SM0OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM0OUT45_1`"]
    #[inline(always)]
    pub fn is_sm0out45_1(&self) -> bool {
        *self == SM0OUT45_A::SM0OUT45_1
    }
}
#[doc = "Write proxy for field `SM0OUT45`"]
pub struct SM0OUT45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0OUT45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM0OUT45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn sm0out45_0(self) -> &'a mut W {
        self.variant(SM0OUT45_A::SM0OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn sm0out45_1(self) -> &'a mut W {
        self.variant(SM0OUT45_A::SM0OUT45_1)
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
#[doc = "Submodule 0 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM0OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SM0OUT23_1,
}
impl From<SM0OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM0OUT23_A) -> Self {
        match variant {
            SM0OUT23_A::SM0OUT23_0 => false,
            SM0OUT23_A::SM0OUT23_1 => true,
        }
    }
}
#[doc = "Reader of field `SM0OUT23`"]
pub type SM0OUT23_R = crate::R<bool, SM0OUT23_A>;
impl SM0OUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM0OUT23_A {
        match self.bits {
            false => SM0OUT23_A::SM0OUT23_0,
            true => SM0OUT23_A::SM0OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM0OUT23_0`"]
    #[inline(always)]
    pub fn is_sm0out23_0(&self) -> bool {
        *self == SM0OUT23_A::SM0OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM0OUT23_1`"]
    #[inline(always)]
    pub fn is_sm0out23_1(&self) -> bool {
        *self == SM0OUT23_A::SM0OUT23_1
    }
}
#[doc = "Write proxy for field `SM0OUT23`"]
pub struct SM0OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM0OUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn sm0out23_0(self) -> &'a mut W {
        self.variant(SM0OUT23_A::SM0OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn sm0out23_1(self) -> &'a mut W {
        self.variant(SM0OUT23_A::SM0OUT23_1)
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
#[doc = "Submodule 1 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_1,
}
impl From<SM1OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT45_A) -> Self {
        match variant {
            SM1OUT45_A::SM1OUT45_0 => false,
            SM1OUT45_A::SM1OUT45_1 => true,
        }
    }
}
#[doc = "Reader of field `SM1OUT45`"]
pub type SM1OUT45_R = crate::R<bool, SM1OUT45_A>;
impl SM1OUT45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1OUT45_A {
        match self.bits {
            false => SM1OUT45_A::SM1OUT45_0,
            true => SM1OUT45_A::SM1OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM1OUT45_0`"]
    #[inline(always)]
    pub fn is_sm1out45_0(&self) -> bool {
        *self == SM1OUT45_A::SM1OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM1OUT45_1`"]
    #[inline(always)]
    pub fn is_sm1out45_1(&self) -> bool {
        *self == SM1OUT45_A::SM1OUT45_1
    }
}
#[doc = "Write proxy for field `SM1OUT45`"]
pub struct SM1OUT45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1OUT45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1OUT45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline(always)]
    pub fn sm1out45_0(self) -> &'a mut W {
        self.variant(SM1OUT45_A::SM1OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    #[inline(always)]
    pub fn sm1out45_1(self) -> &'a mut W {
        self.variant(SM1OUT45_A::SM1OUT45_1)
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
#[doc = "Submodule 1 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM1OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_1,
}
impl From<SM1OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT23_A) -> Self {
        match variant {
            SM1OUT23_A::SM1OUT23_0 => false,
            SM1OUT23_A::SM1OUT23_1 => true,
        }
    }
}
#[doc = "Reader of field `SM1OUT23`"]
pub type SM1OUT23_R = crate::R<bool, SM1OUT23_A>;
impl SM1OUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1OUT23_A {
        match self.bits {
            false => SM1OUT23_A::SM1OUT23_0,
            true => SM1OUT23_A::SM1OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM1OUT23_0`"]
    #[inline(always)]
    pub fn is_sm1out23_0(&self) -> bool {
        *self == SM1OUT23_A::SM1OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM1OUT23_1`"]
    #[inline(always)]
    pub fn is_sm1out23_1(&self) -> bool {
        *self == SM1OUT23_A::SM1OUT23_1
    }
}
#[doc = "Write proxy for field `SM1OUT23`"]
pub struct SM1OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1OUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline(always)]
    pub fn sm1out23_0(self) -> &'a mut W {
        self.variant(SM1OUT23_A::SM1OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    #[inline(always)]
    pub fn sm1out23_1(self) -> &'a mut W {
        self.variant(SM1OUT23_A::SM1OUT23_1)
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
#[doc = "Submodule 2 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_1,
}
impl From<SM2OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT45_A) -> Self {
        match variant {
            SM2OUT45_A::SM2OUT45_0 => false,
            SM2OUT45_A::SM2OUT45_1 => true,
        }
    }
}
#[doc = "Reader of field `SM2OUT45`"]
pub type SM2OUT45_R = crate::R<bool, SM2OUT45_A>;
impl SM2OUT45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2OUT45_A {
        match self.bits {
            false => SM2OUT45_A::SM2OUT45_0,
            true => SM2OUT45_A::SM2OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM2OUT45_0`"]
    #[inline(always)]
    pub fn is_sm2out45_0(&self) -> bool {
        *self == SM2OUT45_A::SM2OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM2OUT45_1`"]
    #[inline(always)]
    pub fn is_sm2out45_1(&self) -> bool {
        *self == SM2OUT45_A::SM2OUT45_1
    }
}
#[doc = "Write proxy for field `SM2OUT45`"]
pub struct SM2OUT45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2OUT45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM2OUT45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline(always)]
    pub fn sm2out45_0(self) -> &'a mut W {
        self.variant(SM2OUT45_A::SM2OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    #[inline(always)]
    pub fn sm2out45_1(self) -> &'a mut W {
        self.variant(SM2OUT45_A::SM2OUT45_1)
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
#[doc = "Submodule 2 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM2OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_1,
}
impl From<SM2OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT23_A) -> Self {
        match variant {
            SM2OUT23_A::SM2OUT23_0 => false,
            SM2OUT23_A::SM2OUT23_1 => true,
        }
    }
}
#[doc = "Reader of field `SM2OUT23`"]
pub type SM2OUT23_R = crate::R<bool, SM2OUT23_A>;
impl SM2OUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2OUT23_A {
        match self.bits {
            false => SM2OUT23_A::SM2OUT23_0,
            true => SM2OUT23_A::SM2OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM2OUT23_0`"]
    #[inline(always)]
    pub fn is_sm2out23_0(&self) -> bool {
        *self == SM2OUT23_A::SM2OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM2OUT23_1`"]
    #[inline(always)]
    pub fn is_sm2out23_1(&self) -> bool {
        *self == SM2OUT23_A::SM2OUT23_1
    }
}
#[doc = "Write proxy for field `SM2OUT23`"]
pub struct SM2OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM2OUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline(always)]
    pub fn sm2out23_0(self) -> &'a mut W {
        self.variant(SM2OUT23_A::SM2OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    #[inline(always)]
    pub fn sm2out23_1(self) -> &'a mut W {
        self.variant(SM2OUT23_A::SM2OUT23_1)
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
#[doc = "Submodule 3 Software Controlled Output 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3OUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_1,
}
impl From<SM3OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT45_A) -> Self {
        match variant {
            SM3OUT45_A::SM3OUT45_0 => false,
            SM3OUT45_A::SM3OUT45_1 => true,
        }
    }
}
#[doc = "Reader of field `SM3OUT45`"]
pub type SM3OUT45_R = crate::R<bool, SM3OUT45_A>;
impl SM3OUT45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3OUT45_A {
        match self.bits {
            false => SM3OUT45_A::SM3OUT45_0,
            true => SM3OUT45_A::SM3OUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM3OUT45_0`"]
    #[inline(always)]
    pub fn is_sm3out45_0(&self) -> bool {
        *self == SM3OUT45_A::SM3OUT45_0
    }
    #[doc = "Checks if the value of the field is `SM3OUT45_1`"]
    #[inline(always)]
    pub fn is_sm3out45_1(&self) -> bool {
        *self == SM3OUT45_A::SM3OUT45_1
    }
}
#[doc = "Write proxy for field `SM3OUT45`"]
pub struct SM3OUT45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3OUT45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM3OUT45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline(always)]
    pub fn sm3out45_0(self) -> &'a mut W {
        self.variant(SM3OUT45_A::SM3OUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    #[inline(always)]
    pub fn sm3out45_1(self) -> &'a mut W {
        self.variant(SM3OUT45_A::SM3OUT45_1)
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
#[doc = "Submodule 3 Software Controlled Output 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM3OUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_1,
}
impl From<SM3OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT23_A) -> Self {
        match variant {
            SM3OUT23_A::SM3OUT23_0 => false,
            SM3OUT23_A::SM3OUT23_1 => true,
        }
    }
}
#[doc = "Reader of field `SM3OUT23`"]
pub type SM3OUT23_R = crate::R<bool, SM3OUT23_A>;
impl SM3OUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3OUT23_A {
        match self.bits {
            false => SM3OUT23_A::SM3OUT23_0,
            true => SM3OUT23_A::SM3OUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SM3OUT23_0`"]
    #[inline(always)]
    pub fn is_sm3out23_0(&self) -> bool {
        *self == SM3OUT23_A::SM3OUT23_0
    }
    #[doc = "Checks if the value of the field is `SM3OUT23_1`"]
    #[inline(always)]
    pub fn is_sm3out23_1(&self) -> bool {
        *self == SM3OUT23_A::SM3OUT23_1
    }
}
#[doc = "Write proxy for field `SM3OUT23`"]
pub struct SM3OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM3OUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline(always)]
    pub fn sm3out23_0(self) -> &'a mut W {
        self.variant(SM3OUT23_A::SM3OUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    #[inline(always)]
    pub fn sm3out23_1(self) -> &'a mut W {
        self.variant(SM3OUT23_A::SM3OUT23_1)
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
impl R {
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm0out45(&self) -> SM0OUT45_R {
        SM0OUT45_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm0out23(&self) -> SM0OUT23_R {
        SM0OUT23_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm1out45(&self) -> SM1OUT45_R {
        SM1OUT45_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm1out23(&self) -> SM1OUT23_R {
        SM1OUT23_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm2out45(&self) -> SM2OUT45_R {
        SM2OUT45_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm2out23(&self) -> SM2OUT23_R {
        SM2OUT23_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm3out45(&self) -> SM3OUT45_R {
        SM3OUT45_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm3out23(&self) -> SM3OUT23_R {
        SM3OUT23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Submodule 0 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm0out45(&mut self) -> SM0OUT45_W {
        SM0OUT45_W { w: self }
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm0out23(&mut self) -> SM0OUT23_W {
        SM0OUT23_W { w: self }
    }
    #[doc = "Bit 2 - Submodule 1 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm1out45(&mut self) -> SM1OUT45_W {
        SM1OUT45_W { w: self }
    }
    #[doc = "Bit 3 - Submodule 1 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm1out23(&mut self) -> SM1OUT23_W {
        SM1OUT23_W { w: self }
    }
    #[doc = "Bit 4 - Submodule 2 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm2out45(&mut self) -> SM2OUT45_W {
        SM2OUT45_W { w: self }
    }
    #[doc = "Bit 5 - Submodule 2 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm2out23(&mut self) -> SM2OUT23_W {
        SM2OUT23_W { w: self }
    }
    #[doc = "Bit 6 - Submodule 3 Software Controlled Output 45"]
    #[inline(always)]
    pub fn sm3out45(&mut self) -> SM3OUT45_W {
        SM3OUT45_W { w: self }
    }
    #[doc = "Bit 7 - Submodule 3 Software Controlled Output 23"]
    #[inline(always)]
    pub fn sm3out23(&mut self) -> SM3OUT23_W {
        SM3OUT23_W { w: self }
    }
}
