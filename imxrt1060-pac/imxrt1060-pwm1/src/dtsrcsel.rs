#[doc = "Reader of register DTSRCSEL"]
pub type R = crate::R<u16, super::DTSRCSEL>;
#[doc = "Writer for register DTSRCSEL"]
pub type W = crate::W<u16, super::DTSRCSEL>;
#[doc = "Register DTSRCSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DTSRCSEL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Submodule 0 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMSEL45_A {
    #[doc = "0: Generated SM0PWM45 signal is used by the deadtime logic."]
    SMSEL45_0 = 0,
    #[doc = "1: Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    SMSEL45_1 = 1,
    #[doc = "2: SWCOUT\\[SM0OUT45\\] is used by the deadtime logic."]
    SMSEL45_2 = 2,
    #[doc = "3: PWM0_EXTB signal is used by the deadtime logic."]
    SMSEL45_3 = 3,
}
impl From<SMSEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSEL45_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMSEL45`"]
pub type SMSEL45_R = crate::R<u8, SMSEL45_A>;
impl SMSEL45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMSEL45_A {
        match self.bits {
            0 => SMSEL45_A::SMSEL45_0,
            1 => SMSEL45_A::SMSEL45_1,
            2 => SMSEL45_A::SMSEL45_2,
            3 => SMSEL45_A::SMSEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SMSEL45_0`"]
    #[inline(always)]
    pub fn is_smsel45_0(&self) -> bool {
        *self == SMSEL45_A::SMSEL45_0
    }
    #[doc = "Checks if the value of the field is `SMSEL45_1`"]
    #[inline(always)]
    pub fn is_smsel45_1(&self) -> bool {
        *self == SMSEL45_A::SMSEL45_1
    }
    #[doc = "Checks if the value of the field is `SMSEL45_2`"]
    #[inline(always)]
    pub fn is_smsel45_2(&self) -> bool {
        *self == SMSEL45_A::SMSEL45_2
    }
    #[doc = "Checks if the value of the field is `SMSEL45_3`"]
    #[inline(always)]
    pub fn is_smsel45_3(&self) -> bool {
        *self == SMSEL45_A::SMSEL45_3
    }
}
#[doc = "Write proxy for field `SMSEL45`"]
pub struct SMSEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSEL45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSEL45_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel45_0(self) -> &'a mut W {
        self.variant(SMSEL45_A::SMSEL45_0)
    }
    #[doc = "Inverted generated SM0PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel45_1(self) -> &'a mut W {
        self.variant(SMSEL45_A::SMSEL45_1)
    }
    #[doc = "SWCOUT\\[SM0OUT45\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel45_2(self) -> &'a mut W {
        self.variant(SMSEL45_A::SMSEL45_2)
    }
    #[doc = "PWM0_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel45_3(self) -> &'a mut W {
        self.variant(SMSEL45_A::SMSEL45_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Submodule 0 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMSEL23_A {
    #[doc = "0: Generated SM0PWM23 signal is used by the deadtime logic."]
    SMSEL23_0 = 0,
    #[doc = "1: Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    SMSEL23_1 = 1,
    #[doc = "2: SWCOUT\\[SM0OUT23\\] is used by the deadtime logic."]
    SMSEL23_2 = 2,
    #[doc = "3: PWM0_EXTA signal is used by the deadtime logic."]
    SMSEL23_3 = 3,
}
impl From<SMSEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSEL23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMSEL23`"]
pub type SMSEL23_R = crate::R<u8, SMSEL23_A>;
impl SMSEL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMSEL23_A {
        match self.bits {
            0 => SMSEL23_A::SMSEL23_0,
            1 => SMSEL23_A::SMSEL23_1,
            2 => SMSEL23_A::SMSEL23_2,
            3 => SMSEL23_A::SMSEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SMSEL23_0`"]
    #[inline(always)]
    pub fn is_smsel23_0(&self) -> bool {
        *self == SMSEL23_A::SMSEL23_0
    }
    #[doc = "Checks if the value of the field is `SMSEL23_1`"]
    #[inline(always)]
    pub fn is_smsel23_1(&self) -> bool {
        *self == SMSEL23_A::SMSEL23_1
    }
    #[doc = "Checks if the value of the field is `SMSEL23_2`"]
    #[inline(always)]
    pub fn is_smsel23_2(&self) -> bool {
        *self == SMSEL23_A::SMSEL23_2
    }
    #[doc = "Checks if the value of the field is `SMSEL23_3`"]
    #[inline(always)]
    pub fn is_smsel23_3(&self) -> bool {
        *self == SMSEL23_A::SMSEL23_3
    }
}
#[doc = "Write proxy for field `SMSEL23`"]
pub struct SMSEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSEL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSEL23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel23_0(self) -> &'a mut W {
        self.variant(SMSEL23_A::SMSEL23_0)
    }
    #[doc = "Inverted generated SM0PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel23_1(self) -> &'a mut W {
        self.variant(SMSEL23_A::SMSEL23_1)
    }
    #[doc = "SWCOUT\\[SM0OUT23\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel23_2(self) -> &'a mut W {
        self.variant(SMSEL23_A::SMSEL23_2)
    }
    #[doc = "PWM0_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn smsel23_3(self) -> &'a mut W {
        self.variant(SMSEL23_A::SMSEL23_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Submodule 1 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM1SEL45_A {
    #[doc = "0: Generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_0 = 0,
    #[doc = "1: Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    SM1SEL45_1 = 1,
    #[doc = "2: SWCOUT\\[SM1OUT45\\] is used by the deadtime logic."]
    SM1SEL45_2 = 2,
    #[doc = "3: PWM1_EXTB signal is used by the deadtime logic."]
    SM1SEL45_3 = 3,
}
impl From<SM1SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM1SEL45_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM1SEL45`"]
pub type SM1SEL45_R = crate::R<u8, SM1SEL45_A>;
impl SM1SEL45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1SEL45_A {
        match self.bits {
            0 => SM1SEL45_A::SM1SEL45_0,
            1 => SM1SEL45_A::SM1SEL45_1,
            2 => SM1SEL45_A::SM1SEL45_2,
            3 => SM1SEL45_A::SM1SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_0`"]
    #[inline(always)]
    pub fn is_sm1sel45_0(&self) -> bool {
        *self == SM1SEL45_A::SM1SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_1`"]
    #[inline(always)]
    pub fn is_sm1sel45_1(&self) -> bool {
        *self == SM1SEL45_A::SM1SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_2`"]
    #[inline(always)]
    pub fn is_sm1sel45_2(&self) -> bool {
        *self == SM1SEL45_A::SM1SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM1SEL45_3`"]
    #[inline(always)]
    pub fn is_sm1sel45_3(&self) -> bool {
        *self == SM1SEL45_A::SM1SEL45_3
    }
}
#[doc = "Write proxy for field `SM1SEL45`"]
pub struct SM1SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1SEL45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1SEL45_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel45_0(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1SEL45_0)
    }
    #[doc = "Inverted generated SM1PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel45_1(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1SEL45_1)
    }
    #[doc = "SWCOUT\\[SM1OUT45\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel45_2(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1SEL45_2)
    }
    #[doc = "PWM1_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel45_3(self) -> &'a mut W {
        self.variant(SM1SEL45_A::SM1SEL45_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Submodule 1 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM1SEL23_A {
    #[doc = "0: Generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_0 = 0,
    #[doc = "1: Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    SM1SEL23_1 = 1,
    #[doc = "2: SWCOUT\\[SM1OUT23\\] is used by the deadtime logic."]
    SM1SEL23_2 = 2,
    #[doc = "3: PWM1_EXTA signal is used by the deadtime logic."]
    SM1SEL23_3 = 3,
}
impl From<SM1SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM1SEL23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM1SEL23`"]
pub type SM1SEL23_R = crate::R<u8, SM1SEL23_A>;
impl SM1SEL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM1SEL23_A {
        match self.bits {
            0 => SM1SEL23_A::SM1SEL23_0,
            1 => SM1SEL23_A::SM1SEL23_1,
            2 => SM1SEL23_A::SM1SEL23_2,
            3 => SM1SEL23_A::SM1SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_0`"]
    #[inline(always)]
    pub fn is_sm1sel23_0(&self) -> bool {
        *self == SM1SEL23_A::SM1SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_1`"]
    #[inline(always)]
    pub fn is_sm1sel23_1(&self) -> bool {
        *self == SM1SEL23_A::SM1SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_2`"]
    #[inline(always)]
    pub fn is_sm1sel23_2(&self) -> bool {
        *self == SM1SEL23_A::SM1SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM1SEL23_3`"]
    #[inline(always)]
    pub fn is_sm1sel23_3(&self) -> bool {
        *self == SM1SEL23_A::SM1SEL23_3
    }
}
#[doc = "Write proxy for field `SM1SEL23`"]
pub struct SM1SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1SEL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM1SEL23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel23_0(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1SEL23_0)
    }
    #[doc = "Inverted generated SM1PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel23_1(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1SEL23_1)
    }
    #[doc = "SWCOUT\\[SM1OUT23\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel23_2(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1SEL23_2)
    }
    #[doc = "PWM1_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm1sel23_3(self) -> &'a mut W {
        self.variant(SM1SEL23_A::SM1SEL23_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Submodule 2 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM2SEL45_A {
    #[doc = "0: Generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_0 = 0,
    #[doc = "1: Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    SM2SEL45_1 = 1,
    #[doc = "2: SWCOUT\\[SM2OUT45\\] is used by the deadtime logic."]
    SM2SEL45_2 = 2,
    #[doc = "3: PWM2_EXTB signal is used by the deadtime logic."]
    SM2SEL45_3 = 3,
}
impl From<SM2SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM2SEL45_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM2SEL45`"]
pub type SM2SEL45_R = crate::R<u8, SM2SEL45_A>;
impl SM2SEL45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2SEL45_A {
        match self.bits {
            0 => SM2SEL45_A::SM2SEL45_0,
            1 => SM2SEL45_A::SM2SEL45_1,
            2 => SM2SEL45_A::SM2SEL45_2,
            3 => SM2SEL45_A::SM2SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_0`"]
    #[inline(always)]
    pub fn is_sm2sel45_0(&self) -> bool {
        *self == SM2SEL45_A::SM2SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_1`"]
    #[inline(always)]
    pub fn is_sm2sel45_1(&self) -> bool {
        *self == SM2SEL45_A::SM2SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_2`"]
    #[inline(always)]
    pub fn is_sm2sel45_2(&self) -> bool {
        *self == SM2SEL45_A::SM2SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM2SEL45_3`"]
    #[inline(always)]
    pub fn is_sm2sel45_3(&self) -> bool {
        *self == SM2SEL45_A::SM2SEL45_3
    }
}
#[doc = "Write proxy for field `SM2SEL45`"]
pub struct SM2SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2SEL45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM2SEL45_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel45_0(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2SEL45_0)
    }
    #[doc = "Inverted generated SM2PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel45_1(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2SEL45_1)
    }
    #[doc = "SWCOUT\\[SM2OUT45\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel45_2(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2SEL45_2)
    }
    #[doc = "PWM2_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel45_3(self) -> &'a mut W {
        self.variant(SM2SEL45_A::SM2SEL45_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Submodule 2 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM2SEL23_A {
    #[doc = "0: Generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_0 = 0,
    #[doc = "1: Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    SM2SEL23_1 = 1,
    #[doc = "2: SWCOUT\\[SM2OUT23\\] is used by the deadtime logic."]
    SM2SEL23_2 = 2,
    #[doc = "3: PWM2_EXTA signal is used by the deadtime logic."]
    SM2SEL23_3 = 3,
}
impl From<SM2SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM2SEL23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM2SEL23`"]
pub type SM2SEL23_R = crate::R<u8, SM2SEL23_A>;
impl SM2SEL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM2SEL23_A {
        match self.bits {
            0 => SM2SEL23_A::SM2SEL23_0,
            1 => SM2SEL23_A::SM2SEL23_1,
            2 => SM2SEL23_A::SM2SEL23_2,
            3 => SM2SEL23_A::SM2SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_0`"]
    #[inline(always)]
    pub fn is_sm2sel23_0(&self) -> bool {
        *self == SM2SEL23_A::SM2SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_1`"]
    #[inline(always)]
    pub fn is_sm2sel23_1(&self) -> bool {
        *self == SM2SEL23_A::SM2SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_2`"]
    #[inline(always)]
    pub fn is_sm2sel23_2(&self) -> bool {
        *self == SM2SEL23_A::SM2SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM2SEL23_3`"]
    #[inline(always)]
    pub fn is_sm2sel23_3(&self) -> bool {
        *self == SM2SEL23_A::SM2SEL23_3
    }
}
#[doc = "Write proxy for field `SM2SEL23`"]
pub struct SM2SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2SEL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM2SEL23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel23_0(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2SEL23_0)
    }
    #[doc = "Inverted generated SM2PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel23_1(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2SEL23_1)
    }
    #[doc = "SWCOUT\\[SM2OUT23\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel23_2(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2SEL23_2)
    }
    #[doc = "PWM2_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm2sel23_3(self) -> &'a mut W {
        self.variant(SM2SEL23_A::SM2SEL23_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Submodule 3 PWM45 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM3SEL45_A {
    #[doc = "0: Generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_0 = 0,
    #[doc = "1: Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    SM3SEL45_1 = 1,
    #[doc = "2: SWCOUT\\[SM3OUT45\\] is used by the deadtime logic."]
    SM3SEL45_2 = 2,
    #[doc = "3: PWM3_EXTB signal is used by the deadtime logic."]
    SM3SEL45_3 = 3,
}
impl From<SM3SEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: SM3SEL45_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM3SEL45`"]
pub type SM3SEL45_R = crate::R<u8, SM3SEL45_A>;
impl SM3SEL45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3SEL45_A {
        match self.bits {
            0 => SM3SEL45_A::SM3SEL45_0,
            1 => SM3SEL45_A::SM3SEL45_1,
            2 => SM3SEL45_A::SM3SEL45_2,
            3 => SM3SEL45_A::SM3SEL45_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_0`"]
    #[inline(always)]
    pub fn is_sm3sel45_0(&self) -> bool {
        *self == SM3SEL45_A::SM3SEL45_0
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_1`"]
    #[inline(always)]
    pub fn is_sm3sel45_1(&self) -> bool {
        *self == SM3SEL45_A::SM3SEL45_1
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_2`"]
    #[inline(always)]
    pub fn is_sm3sel45_2(&self) -> bool {
        *self == SM3SEL45_A::SM3SEL45_2
    }
    #[doc = "Checks if the value of the field is `SM3SEL45_3`"]
    #[inline(always)]
    pub fn is_sm3sel45_3(&self) -> bool {
        *self == SM3SEL45_A::SM3SEL45_3
    }
}
#[doc = "Write proxy for field `SM3SEL45`"]
pub struct SM3SEL45_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3SEL45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM3SEL45_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel45_0(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3SEL45_0)
    }
    #[doc = "Inverted generated SM3PWM45 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel45_1(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3SEL45_1)
    }
    #[doc = "SWCOUT\\[SM3OUT45\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel45_2(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3SEL45_2)
    }
    #[doc = "PWM3_EXTB signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel45_3(self) -> &'a mut W {
        self.variant(SM3SEL45_A::SM3SEL45_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Submodule 3 PWM23 Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM3SEL23_A {
    #[doc = "0: Generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_0 = 0,
    #[doc = "1: Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    SM3SEL23_1 = 1,
    #[doc = "2: SWCOUT\\[SM3OUT23\\] is used by the deadtime logic."]
    SM3SEL23_2 = 2,
    #[doc = "3: PWM3_EXTA signal is used by the deadtime logic."]
    SM3SEL23_3 = 3,
}
impl From<SM3SEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: SM3SEL23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM3SEL23`"]
pub type SM3SEL23_R = crate::R<u8, SM3SEL23_A>;
impl SM3SEL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM3SEL23_A {
        match self.bits {
            0 => SM3SEL23_A::SM3SEL23_0,
            1 => SM3SEL23_A::SM3SEL23_1,
            2 => SM3SEL23_A::SM3SEL23_2,
            3 => SM3SEL23_A::SM3SEL23_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_0`"]
    #[inline(always)]
    pub fn is_sm3sel23_0(&self) -> bool {
        *self == SM3SEL23_A::SM3SEL23_0
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_1`"]
    #[inline(always)]
    pub fn is_sm3sel23_1(&self) -> bool {
        *self == SM3SEL23_A::SM3SEL23_1
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_2`"]
    #[inline(always)]
    pub fn is_sm3sel23_2(&self) -> bool {
        *self == SM3SEL23_A::SM3SEL23_2
    }
    #[doc = "Checks if the value of the field is `SM3SEL23_3`"]
    #[inline(always)]
    pub fn is_sm3sel23_3(&self) -> bool {
        *self == SM3SEL23_A::SM3SEL23_3
    }
}
#[doc = "Write proxy for field `SM3SEL23`"]
pub struct SM3SEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3SEL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM3SEL23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel23_0(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3SEL23_0)
    }
    #[doc = "Inverted generated SM3PWM23 signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel23_1(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3SEL23_1)
    }
    #[doc = "SWCOUT\\[SM3OUT23\\] is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel23_2(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3SEL23_2)
    }
    #[doc = "PWM3_EXTA signal is used by the deadtime logic."]
    #[inline(always)]
    pub fn sm3sel23_3(self) -> &'a mut W {
        self.variant(SM3SEL23_A::SM3SEL23_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub fn smsel45(&self) -> SMSEL45_R {
        SMSEL45_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub fn smsel23(&self) -> SMSEL23_R {
        SMSEL23_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm1sel45(&self) -> SM1SEL45_R {
        SM1SEL45_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm1sel23(&self) -> SM1SEL23_R {
        SM1SEL23_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm2sel45(&self) -> SM2SEL45_R {
        SM2SEL45_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm2sel23(&self) -> SM2SEL23_R {
        SM2SEL23_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm3sel45(&self) -> SM3SEL45_R {
        SM3SEL45_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm3sel23(&self) -> SM3SEL23_R {
        SM3SEL23_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Submodule 0 PWM45 Control Select"]
    #[inline(always)]
    pub fn smsel45(&mut self) -> SMSEL45_W {
        SMSEL45_W { w: self }
    }
    #[doc = "Bits 2:3 - Submodule 0 PWM23 Control Select"]
    #[inline(always)]
    pub fn smsel23(&mut self) -> SMSEL23_W {
        SMSEL23_W { w: self }
    }
    #[doc = "Bits 4:5 - Submodule 1 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm1sel45(&mut self) -> SM1SEL45_W {
        SM1SEL45_W { w: self }
    }
    #[doc = "Bits 6:7 - Submodule 1 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm1sel23(&mut self) -> SM1SEL23_W {
        SM1SEL23_W { w: self }
    }
    #[doc = "Bits 8:9 - Submodule 2 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm2sel45(&mut self) -> SM2SEL45_W {
        SM2SEL45_W { w: self }
    }
    #[doc = "Bits 10:11 - Submodule 2 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm2sel23(&mut self) -> SM2SEL23_W {
        SM2SEL23_W { w: self }
    }
    #[doc = "Bits 12:13 - Submodule 3 PWM45 Control Select"]
    #[inline(always)]
    pub fn sm3sel45(&mut self) -> SM3SEL45_W {
        SM3SEL45_W { w: self }
    }
    #[doc = "Bits 14:15 - Submodule 3 PWM23 Control Select"]
    #[inline(always)]
    pub fn sm3sel23(&mut self) -> SM3SEL23_W {
        SM3SEL23_W { w: self }
    }
}
