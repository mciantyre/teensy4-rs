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
pub enum SMOUT45_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SMOUT45_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    SMOUT45_1 = 1,
}
impl From<SMOUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SMOUT45_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMOUT45`"]
pub type SMOUT45_R = crate::R<bool, SMOUT45_A>;
impl SMOUT45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOUT45_A {
        match self.bits {
            false => SMOUT45_A::SMOUT45_0,
            true => SMOUT45_A::SMOUT45_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMOUT45_0`"]
    #[inline(always)]
    pub fn is_smout45_0(&self) -> bool {
        *self == SMOUT45_A::SMOUT45_0
    }
    #[doc = "Checks if the value of the field is `SMOUT45_1`"]
    #[inline(always)]
    pub fn is_smout45_1(&self) -> bool {
        *self == SMOUT45_A::SMOUT45_1
    }
}
#[doc = "Write proxy for field `SMOUT45`"]
pub struct SMOUT45_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOUT45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOUT45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn smout45_0(self) -> &'a mut W {
        self.variant(SMOUT45_A::SMOUT45_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    #[inline(always)]
    pub fn smout45_1(self) -> &'a mut W {
        self.variant(SMOUT45_A::SMOUT45_1)
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
pub enum SMOUT23_A {
    #[doc = "0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SMOUT23_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    SMOUT23_1 = 1,
}
impl From<SMOUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SMOUT23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMOUT23`"]
pub type SMOUT23_R = crate::R<bool, SMOUT23_A>;
impl SMOUT23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOUT23_A {
        match self.bits {
            false => SMOUT23_A::SMOUT23_0,
            true => SMOUT23_A::SMOUT23_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMOUT23_0`"]
    #[inline(always)]
    pub fn is_smout23_0(&self) -> bool {
        *self == SMOUT23_A::SMOUT23_0
    }
    #[doc = "Checks if the value of the field is `SMOUT23_1`"]
    #[inline(always)]
    pub fn is_smout23_1(&self) -> bool {
        *self == SMOUT23_A::SMOUT23_1
    }
}
#[doc = "Write proxy for field `SMOUT23`"]
pub struct SMOUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOUT23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn smout23_0(self) -> &'a mut W {
        self.variant(SMOUT23_A::SMOUT23_0)
    }
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    #[inline(always)]
    pub fn smout23_1(self) -> &'a mut W {
        self.variant(SMOUT23_A::SMOUT23_1)
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
    SM1OUT45_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    SM1OUT45_1 = 1,
}
impl From<SM1OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT45_A) -> Self {
        variant as u8 != 0
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
    SM1OUT23_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    SM1OUT23_1 = 1,
}
impl From<SM1OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM1OUT23_A) -> Self {
        variant as u8 != 0
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
    SM2OUT45_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    SM2OUT45_1 = 1,
}
impl From<SM2OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT45_A) -> Self {
        variant as u8 != 0
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
    SM2OUT23_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    SM2OUT23_1 = 1,
}
impl From<SM2OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM2OUT23_A) -> Self {
        variant as u8 != 0
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
    SM3OUT45_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    SM3OUT45_1 = 1,
}
impl From<SM3OUT45_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT45_A) -> Self {
        variant as u8 != 0
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
    SM3OUT23_0 = 0,
    #[doc = "1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    SM3OUT23_1 = 1,
}
impl From<SM3OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: SM3OUT23_A) -> Self {
        variant as u8 != 0
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
    pub fn smout45(&self) -> SMOUT45_R {
        SMOUT45_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub fn smout23(&self) -> SMOUT23_R {
        SMOUT23_R::new(((self.bits >> 1) & 0x01) != 0)
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
    pub fn smout45(&mut self) -> SMOUT45_W {
        SMOUT45_W { w: self }
    }
    #[doc = "Bit 1 - Submodule 0 Software Controlled Output 23"]
    #[inline(always)]
    pub fn smout23(&mut self) -> SMOUT23_W {
        SMOUT23_W { w: self }
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
